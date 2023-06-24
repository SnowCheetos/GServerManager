use std::io::{stdout, Write};
use structopt::StructOpt;
use GServerManager::server::servers::Servers;
use GServerManager::commands::manager::ServerManager;
use GServerManager::commands::command::Command;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut manager = ServerManager::from_args();
    manager.servers = Some(Servers {
        servers: Vec::new(),
    });

    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">>> ");

        match readline {
            Ok(input) => {
                rl.add_history_entry(input.as_str());

                if input == "quit" || input == "exit" {
                    manager.servers.expect("REASON").flush();
                    break;
                }

                let input = format!("{} {}", std::env::args().next().unwrap(), input);
                let result = Command::from_iter_safe(input.split_whitespace());

                match result {
                    Ok(cmd) => {
                        manager.cmd = Some(cmd);
                        manager.execute();
                    }
                    Err(error) => {
                        println!("Invalid command: {}", error);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
