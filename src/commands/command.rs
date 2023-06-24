use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "add")]
    Add {
        #[structopt(short="N", long)]
        name: String,

        #[structopt(short="F", long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short="W", long, default_value = "4")]
        workers: u32,

        #[structopt(short="H", long, default_value = "0.0.0.0")]
        host: String,

        #[structopt(short="P", long, default_value = "8000")]
        port: u32,

        #[structopt(short="T", long, default_value = "30")]
        timeout: u32,

        #[structopt(long="L", parse(from_os_str))]
        log_file: Option<PathBuf>,
    },

    #[structopt(name = "remove")]
    Remove {
        #[structopt(short="N", long)]
        name: String,
    },

    #[structopt(name = "start")]
    Start {
        #[structopt(short="N", long)]
        name: String,
    },

    #[structopt(name = "stop")]
    Stop {
        #[structopt(short="N", long)]
        name: String,
    },

    #[structopt(name = "restart")]
    Restart {
        #[structopt(short="N", long)]
        name: String,
    },

    #[structopt(name = "update")]
    Update {
        #[structopt(short="N", long)]
        name: String,
    },

    #[structopt(name = "monitor")]
    Monitor {
        #[structopt(short="N", long)]
        name: String,
    },

    #[structopt(name = "hardware")]
    Hardware, // Show hardware usage

    #[structopt(name = "list")]
    List, // List all servers

    #[structopt(name = "flush")]
    Flush

}