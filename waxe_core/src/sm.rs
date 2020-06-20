#![warn(clippy::all)]
#![allow(clippy::missing_safety_doc)]
#![allow(dead_code)]

use crate::trio::Trio;
use libc::c_uint;
use mozjs::glue::SetBuildId;
use mozjs::jsapi::BuildIdCharVector;
use mozjs::jsapi::InitRealmStandardClasses;
use mozjs::jsapi::JSContext;
use mozjs::jsapi::JS_ReportErrorASCII;
use mozjs::jsapi::SetProcessBuildIdOp;

use core::ptr;
use core::slice::from_raw_parts;
use core::str;
use std::ffi::CStr;

use mozjs::jsapi::JSAutoRealm;
use mozjs::jsapi::JS_ClearPendingException;
use mozjs::jsapi::JS_DefineFunction;
use mozjs::jsapi::JS_IsExceptionPending;
use mozjs::jsapi::JS_NewGlobalObject;
use mozjs::jsapi::JS_SetGlobalJitCompilerOption;
use mozjs::jsapi::JS_Utf8BufferIsCompilableUnit;
use mozjs::jsapi::OnNewGlobalHookOption;
use mozjs::jsapi::{
    JSJitCompilerOption, JS_SetOffthreadIonCompilationEnabled, JS_SetParallelParsingEnabled,
};
use mozjs::rust::wrappers::JS_ErrorFromException;
use mozjs::rust::wrappers::JS_GetPendingException;
use mozjs::rust::{JSEngine, RealmOptions, Runtime, SIMPLE_GLOBAL_CLASS};

use mozjs::conversions::FromJSValConvertible;
use mozjs::conversions::ToJSValConvertible;
use mozjs::glue::EncodeStringToUTF8;
use mozjs::jsapi::JS::CallArgs;
use mozjs::jsapi::JS::Value;
use mozjs::jsval::UndefinedValue;
pub use mozjs::rust::Handle as global_handle;

#[derive(Copy, Clone, Default)]
pub struct JSOptions {
    pub disable_asmjs: bool,
    pub disable_wasm: bool,
    pub disable_wasm_ion: bool,
    pub disable_wasm_baseline: bool,
    pub disable_ion: bool,
    pub disable_baseline: bool,
    pub disable_parallel_parsing: bool,
    pub disable_offthread_compilation: bool,
    pub enable_baseline_unsafe_eager_compilation: bool,
    pub enable_ion_unsafe_eager_compilation: bool,
    pub enable_werror: bool,
    pub enable_strict: bool,
    pub disable_native_regexp: bool,
}

/// spider monkey engine data
pub struct SME {
    pub runtime: mozjs::rust::Runtime,
    _engine: JSEngine,
    pub global: *mut mozjs::jsapi::JSObject,
    _ac: JSAutoRealm,
}

impl Drop for SME {
    fn drop(&mut self) {
        println!("SME Dropping!");
    }
}

impl SME {
    pub fn start() -> Self {
        let _engine = JSEngine::init().unwrap();
        let runtime = Runtime::new(_engine.handle());
        let context = runtime.cx();
        let h_option = OnNewGlobalHookOption::FireOnNewGlobalHook;
        let c_option = RealmOptions::default();
        let global = unsafe {
            JS_NewGlobalObject(
                context,
                &SIMPLE_GLOBAL_CLASS,
                ptr::null_mut(),
                h_option,
                &*c_option,
            )
        };
        SME {
            runtime,
            _engine,
            global,
            _ac: JSAutoRealm::new(context, global_handle::new(&global).get()),
        }
    }

    pub unsafe fn set(&mut self, js_options: JSOptions) {
        let rt = &self.runtime;
        let cx = rt.cx();
        let rt_opts = &mut *mozjs::jsapi::ContextOptionsRef(cx);
        rt_opts.set_asmJS_(!js_options.disable_asmjs);
        rt_opts.set_extraWarnings_(js_options.enable_strict);
        rt_opts.set_werror_(js_options.enable_werror);
        rt_opts.set_wasm_(!js_options.disable_wasm);
        rt_opts.set_wasmBaseline_(!js_options.disable_wasm_baseline);
        rt_opts.set_wasmIon_(!js_options.disable_wasm_ion);
        if !js_options.disable_wasm {
            // If WASM is enabled without setting the buildIdOp,
            // initializing a module will report an out of memory error.
            // https://dxr.mozilla.org/mozilla-central/source/js/src/wasm/WasmTypes.cpp#458
            SetProcessBuildIdOp(Some(servo_build_id));
        }

        JS_SetParallelParsingEnabled(cx, !js_options.disable_parallel_parsing);
        JS_SetOffthreadIonCompilationEnabled(cx, !js_options.disable_offthread_compilation);
        JS_SetGlobalJitCompilerOption(
            cx,
            JSJitCompilerOption::JSJITCOMPILER_BASELINE_WARMUP_TRIGGER,
            if js_options.enable_baseline_unsafe_eager_compilation {
                0i32
            } else {
                -1i32
            } as u32,
        );
        JS_SetGlobalJitCompilerOption(
            cx,
            JSJitCompilerOption::JSJITCOMPILER_ION_FULL_WARMUP_TRIGGER,
            if js_options.enable_ion_unsafe_eager_compilation {
                0i32
            } else {
                -1i32
            } as u32,
        );
        JS_SetGlobalJitCompilerOption(
            cx,
            JSJitCompilerOption::JSJITCOMPILER_BASELINE_ENABLE,
            if !js_options.disable_baseline {
                0i32
            } else {
                -1i32
            } as u32,
        );
        JS_SetGlobalJitCompilerOption(
            cx,
            JSJitCompilerOption::JSJITCOMPILER_ION_ENABLE,
            if !js_options.disable_ion { 0i32 } else { -1i32 } as u32,
        );
        JS_SetGlobalJitCompilerOption(
            cx,
            JSJitCompilerOption::JSJITCOMPILER_NATIVE_REGEXP_ENABLE,
            if !js_options.disable_native_regexp {
                0i32
            } else {
                -1i32
            } as u32,
        );
    }

    pub unsafe fn config_engine(&mut self) {
        let cx = self.runtime.cx();
        let global = global_handle::new(&self.global);
        assert!(InitRealmStandardClasses(cx));
        JS_DefineFunction(
            cx,
            global.into(),
            b"version\0".as_ptr() as *const libc::c_char,
            Some(info::version),
            1,
            0,
        );
        JS_DefineFunction(
            cx,
            global.into(),
            b"input\0".as_ptr() as *const libc::c_char,
            Some(input),
            1,
            0,
        );
        JS_DefineFunction(
            cx,
            global.into(),
            b"exit\0".as_ptr() as *const _,
            Some(exit),
            1,
            0,
        );
        let function = JS_DefineFunction(
            cx,
            global.into(),
            b"print\0".as_ptr() as *const libc::c_char,
            Some(print),
            1,
            0,
        );
        assert!(!function.is_null());
    }
    ///
    /// Alternative to JS_BufferIsCompilableUnit with less input.
    ///
    pub fn is_full_js(&mut self, buffer: &str) -> bool {
        let script_utf8: Vec<u8> = buffer.bytes().collect();
        let script_ptr: *const i8 = buffer.as_ptr() as *const i8; // error
        let script_len = script_utf8.len() as usize;
        unsafe {
            JS_Utf8BufferIsCompilableUnit(
                self.runtime.cx(),
                global_handle::new(&self.global).into(),
                script_ptr,
                script_len,
            )
        }
    }

    ///
    /// Alternative to rt.evaluate_script with less input, auto rooting,
    /// AutoReportException and better results.
    /// ``` ignore
    /// match eval(rt, global, &buffer, "typein", start_line) {
    ///            Ok(Some(output)) => println!("{}", output),// sucessfully, with output
    ///            Ok(_) => (),// sucessfully, without output
    ///            Err(_) => (),// error (which is handled with _are)
    /// }
    /// ```
    pub fn eval(
        &mut self,
        buffer: &str,
        filename: &str,
        line_num: u32,
    ) -> Trio<String, Option<ErrorInfo>> {
        let cx = self.runtime.cx();
        //let _are = AutoReportException { cx };
        rooted!(in(cx) let mut rval = UndefinedValue());
        match self.runtime.evaluate_script(
            mozjs::rust::Handle::new(&global_handle::new(&self.global).get()),
            &buffer,
            filename,
            line_num,
            rval.handle_mut(),
        ) {
            Ok(_) => {
                if !rval.handle().is_undefined() {
                    Trio::Ok(fmt_js_value(cx, rval.handle()))
                } else {
                    Trio::Empty
                }
            }
            Err(_) => Trio::Err(unsafe {
                match report_pending_exception(cx) {
                    Trio::Ok(x) => Some(x),
                    Trio::Err(x) => Some(ErrorInfo::gen_fail(x)),
                    Trio::Empty => None,
                }
            }),
        }
    }
}

#[allow(unsafe_code)]
unsafe extern "C" fn servo_build_id(build_id: *mut BuildIdCharVector) -> bool {
    let servo_id = b"NanoJS\0";
    SetBuildId(build_id, &servo_id[0], servo_id.len())
}

// pub functions

// alternate version: https://github.com/Ms2ger/js-shell/blob/master/src/script.rs#L110
pub fn fmt_js_value(cx: *mut JSContext, val: mozjs::rust::HandleValue) -> String {
    unsafe {
        String::from_jsval(cx, val, ())
            .unwrap()
            .get_success_value()
            .unwrap()
            .to_string()
    }
}

pub unsafe fn neki_to_string(arg: *const i8) -> String {
    let x = arg as *const u8;
    let length = (0..).find(|idx| *x.offset(*idx) == 0).unwrap();
    let x = from_raw_parts(x, length as usize);
    String::from_utf8_lossy(x).into_owned()
}

pub mod info {
    use crate::sm::neki_to_string;
    use mozjs::conversions::ToJSValConvertible;
    use mozjs::jsapi::JSContext;
    use mozjs::jsapi::JS::CallArgs;
    use mozjs::jsapi::JS::Value;

    /// export version
    /// ```bash
    /// js> version()
    /// JavaScript-C67.0.3
    /// ```
    pub unsafe extern "C" fn version(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
        let args = CallArgs::from_vp(vp, argc);
        let cx = context;
        neki_to_string(mozjs::jsapi::JS_GetImplementationVersion())
            .to_jsval(cx, mozjs::rust::MutableHandle::from_raw(args.rval()));
        true
    }
}

/// read function
/// ```bash
/// js> let a = read("history.txt")
/// js> a
///  <content of history.txt>
/// ```
/* unsafe extern "C" fn read(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);
    if args.argc_ != 1 {
        mozjs::jsapi::JS_ReportErrorASCII(
            context,
            b"print() requires 1 argument\0".as_ptr() as *const libc::c_char,
        );
        return false;
    }
    let val = mozjs::rust::Handle::from_raw(args.get(0));
    let s = mozjs::rust::ToString(context, val);
    if s.is_null() {
        JS_ReportErrorASCII(
            context,
            b"No file is given\0".as_ptr() as *const libc::c_char,
        );
        return false;
    }
    let mut filename = std::env::current_dir().unwrap();
    rooted!(in(context) let path_root = s);
    let path = JS_EncodeStringToUTF8(context, path_root.handle().into());
    let path = CStr::from_ptr(path);
    filename.push(str::from_utf8(path.to_bytes()).unwrap());
    let mut file = match File::open(&filename) {
        Ok(file) => file,
        _ => {
            JS_ReportErrorASCII(
                context,
                b"Can not open file\0".as_ptr() as *const libc::c_char,
            );
            return false;
        }
    };
    let mut source = String::new();
    if let Err(_) = file.read_to_string(&mut source) {
        // TODO: report error
        JS_ReportErrorASCII(
            context,
            b"Can not read file\0".as_ptr() as *const libc::c_char,
        );
        return false;
    }
    source.to_jsval(context, mozjs::rust::MutableHandle::from_raw(args.rval()));
    true
} */

/// input function
/// ```bash
/// js> let a = input("enter: ")
/// enter: samo
/// js> a
/// samo
/// ```
unsafe extern "C" fn input(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);
    let cx = context;
    let val = mozjs::rust::Handle::from_raw(args.get(0));
    let s = mozjs::rust::ToString(cx, val);
    if s.is_null() {
        JS_ReportErrorASCII(
            context,
            b"No input is here\0".as_ptr() as *const libc::c_char,
        );
        return false;
    }
    rooted!(in(cx) let message_root = s);
    EncodeStringToUTF8(context, message_root.handle().into(), |message| {
        let message = CStr::from_ptr(message);
        let message = str::from_utf8(message.to_bytes()).unwrap();
        print!("{}", message);
    });
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    let line = String::from(s.trim());
    match line.trim().parse::<f64>() {
        // is float
        Ok(ok) => ok.to_jsval(cx, mozjs::rust::MutableHandle::from_raw(args.rval())),
        _ => match line.trim().parse::<u64>() {
            // is int
            Ok(ok) => ok.to_jsval(cx, mozjs::rust::MutableHandle::from_raw(args.rval())),
            _ => match line.trim().parse::<bool>() {
                // is bool
                Ok(ok) => ok.to_jsval(cx, mozjs::rust::MutableHandle::from_raw(args.rval())),
                // else it is string
                _ => line.to_jsval(cx, mozjs::rust::MutableHandle::from_raw(args.rval())),
            },
        },
    }
    true
}

/// exit function
/// ```bash
/// js> exit(0)
/// ```
unsafe extern "C" fn exit(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);
    let cx = context;
    let val = mozjs::rust::Handle::from_raw(args.get(0));
    let s: i32 = {
        match mozjs::rust::ToInt32(cx, val) {
            Ok(num) => num,
            Err(_) => {
                JS_ReportErrorASCII(context, b"No input here\0".as_ptr() as *const libc::c_char);
                return false;
            }
        }
    };
    std::process::exit(s);
}

/// print function
/// ```bash
/// js> print('Test Iñtërnâtiônàlizætiøn ┬─┬ノ( º _ ºノ) '); print('hi')
/// Test Iñtërnâtiônàlizætiøn ┬─┬ノ( º _ ºノ)
/// hi
/// js> print('hi', false); print('ho', false)
/// hiho
/// ```
unsafe extern "C" fn print(context: *mut JSContext, argc: u32, vp: *mut Value) -> bool {
    let args = CallArgs::from_vp(vp, argc);

    if args.argc_ != 1 && args.argc_ != 2 {
        mozjs::jsapi::JS_ReportErrorASCII(
            context,
            b"print() requires 1 or 2 arguments\0".as_ptr() as *const libc::c_char,
        );
        return false;
    }

    let arg = mozjs::rust::Handle::from_raw(args.get(0));
    let arg1 = mozjs::rust::Handle::from_raw(args.get(1));
    let ln = if fmt_js_value(context, arg1).is_empty() || args.argc_ != 2 {
        true
    } else {
        mozjs::rust::ToBoolean(arg1)
    };
    let js = mozjs::rust::ToString(context, arg);
    rooted!(in(context) let message_root = js);
    if ln {
        EncodeStringToUTF8(context, message_root.handle().into(), |message| {
            let message = CStr::from_ptr(message);
            let message = str::from_utf8(message.to_bytes()).unwrap();
            println!("{}", message);
        });
    } else {
        EncodeStringToUTF8(context, message_root.handle().into(), |message| {
            let message = CStr::from_ptr(message);
            let message = str::from_utf8(message.to_bytes()).unwrap();
            print!("{}", message);
        });
    }

    args.rval().set(UndefinedValue());
    true
}

pub struct AutoReportException {
    pub cx: *mut JSContext,
}

impl Drop for AutoReportException {
    fn drop(&mut self) {
        unsafe { report_pending_exception(self.cx) };
    }
}

#[derive(Clone, Debug)]
/// A struct encapsulating information about a runtime script error.
pub struct ErrorInfo {
    /// The error message.
    pub message: String,
    /// The file name.
    pub filename: String,
    /// The line number.
    pub lineno: c_uint,
    /// The column number.
    pub column: c_uint,
    /// The stack.
    pub stack: Option<String>,
}

impl ErrorInfo {
    pub fn format(self) -> String {
        format!(
            "Uncaught exception at {}:{}:{} - {}",
            self.filename, self.lineno, self.column, self.message
        )
    }
    fn gen_fail(x: String) -> Self {
        ErrorInfo {
            message: format!("Failed with reporting exeption: {}", x),
            filename: String::new(),
            lineno: 0,
            column: 0,
            stack: None,
        }
    }
}

pub unsafe fn report_pending_exception(cx: *mut JSContext) -> Trio<ErrorInfo, String> {
    if !JS_IsExceptionPending(cx) {
        return Trio::Empty;
    }
    rooted!(in(cx) let mut value = UndefinedValue());
    if !JS_GetPendingException(cx, value.handle_mut()) {
        JS_ClearPendingException(cx);
        //panic!("Uncaught exception: JS_GetPendingException failed");
        return Trio::Err("Uncaught exception: JS_GetPendingException failed".to_string());
    }

    JS_ClearPendingException(cx);
    let maybe_report = if value.is_object() {
        rooted!(in(cx) let object = value.to_object());
        let report = JS_ErrorFromException(cx, object.handle());
        if report.is_null() {
            None
        } else {
            Some(report)
        }
    } else {
        None
    };
    let mut error_info = match maybe_report {
        Some(report) => {
            let filename = {
                let filename = (*report)._base.filename as *const u8;
                if !filename.is_null() {
                    let length = (0..).find(|idx| *filename.offset(*idx) == 0).unwrap();
                    let filename = from_raw_parts(filename, length as usize);
                    String::from_utf8_lossy(filename).into_owned()
                } else {
                    "none".to_string()
                }
            };

            let lineno = (*report)._base.lineno;
            let column = (*report)._base.column;
            let message = {
                let message = (*report)._base.message_.data_ as *const u8;
                let length = (0..).find(|idx| *message.offset(*idx) == 0).unwrap();
                let message = from_raw_parts(message, length as usize);
                let message = String::from_utf8_lossy(message);
                String::from(message)
                // neki_to_string
            };
            ErrorInfo {
                filename,
                message,
                lineno,
                column,
                stack: None,
            }
        }
        None => ErrorInfo {
            message: format!("Thrown value: {}", fmt_js_value(cx, value.handle())),
            filename: String::new(),
            lineno: 0,
            column: 0,
            stack: None,
        },
    };
    /* println!(
        "Uncaught exception at {}:{}:{} - {}",
        error_info.filename, error_info.lineno, error_info.column, error_info.message
    ); */
    // stack print
    capture_stack!(in(cx) let stack);
    error_info.stack = Some(
        stack
            .unwrap()
            .as_string(None, mozjs::jsapi::StackFormat::SpiderMonkey)
            .unwrap(),
    );
    //println!("STACK: {}", str_stack);
    Trio::Ok(error_info)
}
