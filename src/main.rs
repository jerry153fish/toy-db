use std::env;
use rustyline::error::ReadlineError;
use rustyline::Editor;

enum MetaCommand {
    Exit,
    Unknown(String),
}

impl MetaCommand {
    fn new(command: String) -> MetaCommand {
        match command.as_ref() {
            ".exit" => MetaCommand::Exit,
            _ => MetaCommand::Unknown(command),
        }
    }
}

enum DbCommand {
    Insert(String),
    Delete(String),
    Update(String),
    CreateTable(String),
    Select(String),
    Unknown(String),
}

impl DbCommand {
    fn new(command: String) -> DbCommand {
        let v = command.split(" ").collect::<Vec<&str>>();
        match v[0] {
            "insert" => DbCommand::Insert(command),
            "update" => DbCommand::Update(command),
            "delete" => DbCommand::Delete(command),
            "create" => DbCommand::CreateTable(command),
            "select" => DbCommand::Select(command),
            _ => DbCommand::Unknown(command),
        }
    }
}

enum CommandType {
    MetaCommand(MetaCommand),
    DbCommand(DbCommand),
}

fn get_command_type(cmd: &String) -> CommandType {
    match cmd.starts_with(".") {
        true => CommandType::MetaCommand(MetaCommand::new(cmd.to_owned())),
        false => CommandType::DbCommand(DbCommand::new(cmd.to_owned())),
    }
}

fn handle_meta_command(cmd: MetaCommand) {
    match cmd {
        MetaCommand::Exit => std::process::exit(0),
        MetaCommand::Unknown(cmd) => println!("Unrecognized meta command {}", cmd),
    }
}

fn process_command(cmd: DbCommand) {
    match cmd {
        DbCommand::Insert(cmd) => println!("Insert db command {}", cmd),
        DbCommand::Delete(cmd) => println!("Delete db command {}", cmd),
        DbCommand::Update(cmd) => println!("Update db command {}", cmd),
        DbCommand::CreateTable(cmd) => println!("Create table command {}", cmd),
        DbCommand::Select(cmd) => println!("select db command {}", cmd),
        DbCommand::Unknown(cmd) => println!("Unknowm db command {}", cmd),
    }
}

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
                        process_command(_cmd);
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
