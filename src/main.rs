use std::io::{stdin, stdout, Write};
use structopt::StructOpt;
use GServerManager::server::servers::Servers;
use GServerManager::commands::manager::ServerManager;
use GServerManager::commands::command::Command;


fn main() {
    let mut manager = ServerManager::from_args();
    manager.servers = Some(Servers {
        servers: Vec::new(),
    });

    loop {
        print!(">>> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim_end();

        if input == "quit" || input == "exit" {
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
}
