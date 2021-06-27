use std::env;
use rustyline::error::ReadlineError;
use rustyline::Editor;

use command::{CommandType, get_command_type, handle_meta_command, process_command};

mod command;

fn main() {

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    // if rl.load_history("history.txt").is_err() {
    //     println!("No previous history.");
    // }
    loop {
        let readline = rl.readline("toy_db >> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match get_command_type(&line) {
                    CommandType::DbCommand(_cmd) => {
                        process_command(&line);
                    }
                    CommandType::MetaCommand(cmd) => {
                        handle_meta_command(cmd);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    // rl.save_history("history.txt").unwrap();
}
