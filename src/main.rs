use rustyline::error::ReadlineError;
use rustyline::Editor;
use waxe_core::rtx::RTX;
use waxe_core::trio::Trio;

fn main() {
    let mut rtx = RTX::start();
    // TODO: check for script here
    let mut line_no = 1u32;
    loop {
        let mut brake = false;
        let mut multiline = false;
        let start_line = line_no;
        let mut buffer = String::new();
        let mut rl = Editor::<()>::new();
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }
        loop {
            let readline = if multiline {
                rl.readline("> ")
            } else {
                rl.readline("js> ")
            };
            match readline {
                Ok(line) => {
                    rl.add_history_entry(line.as_str());
                    //println!("Line: {}", line);
                    buffer.push_str(&line);
                    line_no += 1;
                    if rtx.is_full(buffer.clone()) {
                        break;
                    } else {
                        multiline = true;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    brake = true;
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    brake = true;
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
        if brake {
            break;
        }
        rl.save_history("history.txt").unwrap();
        match rtx.eval(buffer, "typein", line_no) {
            Trio::Ok(x) => println!("{}", x),
            Trio::Empty => { /* blank */ }
            Trio::Err(x) => println!("{}", x),
        }
    }
}
