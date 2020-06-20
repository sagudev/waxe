use crate::sm::SME;
use crate::trio::Trio;
/// Runtime engine
pub struct RTX {
    engine: SME,
}

impl RTX {
    pub fn start() -> Self {
        let mut sme = SME::start();
        unsafe { sme.config_engine() };
        RTX { engine: sme }
    }
    pub fn eval(&mut self, buffer: String, filename: &str, line: u32) -> Trio<String, String> {
        match self.engine.eval(&buffer, filename, line) {
            Trio::Ok(x) => Trio::Ok(x),
            Trio::Empty => Trio::Empty,
            Trio::Err(x) => {
                if let Some(i) = x {
                    Trio::Err((*i.format()).to_string())
                } else {
                    Trio::Err("".to_string())
                }
            }
        }
    }
    pub fn is_full(&mut self, buffer: String) -> bool {
        self.engine.is_full_js(&buffer)
    }
}

#[test]
fn js_version_test() {
    let mut rtx = RTX::start();
    assert_eq!(rtx.is_full("version(".to_string()), false);
    assert_eq!(rtx.is_full("version()".to_string()), true);
    rtx.eval("version()".to_string(), "typein", 0);
}
