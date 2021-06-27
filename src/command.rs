use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;

pub enum MetaCommand {
    Exit,
    Unknown(String),
}

impl MetaCommand {
    pub fn new(command: String) -> MetaCommand {
        match command.as_ref() {
            ".exit" => MetaCommand::Exit,
            _ => MetaCommand::Unknown(command),
        }
    }
}

pub enum CommandType {
    MetaCommand(MetaCommand),
    DbCommand(String),
}

pub fn get_command_type(cmd: &String) -> CommandType {
    match cmd.starts_with(".") {
        true => CommandType::MetaCommand(MetaCommand::new(cmd.to_owned())),
        false => CommandType::DbCommand(cmd.to_owned()),
    }
}

pub fn handle_meta_command(cmd: MetaCommand) {
    match cmd {
        MetaCommand::Exit => std::process::exit(0),
        MetaCommand::Unknown(cmd) => println!("Unrecognized meta command {}", cmd),
    }
}

pub fn process_command(cmd: &String) {
    let dialect = MySqlDialect {};
    let statements = &Parser::parse_sql(&dialect, cmd);

    match statements {
        Ok(_sts) => {
            for s in _sts {
                println!("{:?}", s);
            }
        },
        Err(_err) => println!("Can not parse command {}", cmd),
    }
}