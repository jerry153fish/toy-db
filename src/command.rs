use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use sqlparser::ast::Statement;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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
                parse_db_command(s);
            }
        }
        Err(_err) => println!("Can not parse command {}", cmd),
    }
}

fn parse_db_command(s: &Statement) {
    match s {
        Statement::Query (_query) => {
            println!("{:?}", _query);
        }
        Statement::Insert { .. } => {
            println!("{:?}", s);
        }
        Statement::Copy { .. } => {
            println!("{:?}", s);
        }
        Statement::Update { .. } => {
            println!("{:?}", s);
        }
        Statement::Delete { .. } => {
            println!("{:?}", s);
        }
        Statement::CreateView { .. } => {
            println!("{:?}", s);
        }
        Statement::CreateTable { .. } => {
            println!("{:?}", s);
        }
        Statement::AlterTable { .. } => {
            println!("{:?}", s);
        }
        Statement::Drop { .. } => {
            println!("{:?}", s);
        },
        Statement::SetVariable { .. } => {
            println!("{:?}", s);
        },
        Statement::ShowVariable { .. } => {
            println!("{:?}", s);
        },
        Statement::ShowColumns { .. } => {
            println!("{:?}", s);
        },
        Statement::StartTransaction { modes } => {
            println!("{:?}", modes);
        }
        Statement::SetTransaction { modes } => {
            println!("{:?}", modes);
        }
        Statement::Commit {
            chain
        } => {
            println!("{:?}", chain)
        }
        Statement::Rollback {
            chain
        } => {
            println!("{:?}", chain)
        }
        _ => {
            println!("Not valid query");
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_exit_command() {
        let cmd = String::from(".exit");

        let result : CommandType = get_command_type(&cmd); 

        assert_eq!(CommandType::MetaCommand(MetaCommand::Exit), result);
    }

    #[test]
    fn test_db_command() {
        let cmd = String::from("select");

        let result : CommandType = get_command_type(&cmd); 

        assert_eq!(CommandType::DbCommand(cmd), result);
    }
}