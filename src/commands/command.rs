use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "add")]
    Add {
        #[structopt(short="n", long)]
        name: String,

        #[structopt(short="f", long, default_value = "flask")]
        framework: String,

        #[structopt(short="d", long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short="w", long, default_value = "4")]
        workers: u32,

        #[structopt(short="b", long, default_value = "0.0.0.0")]
        bind: String,

        #[structopt(short="p", long, default_value = "8000")]
        port: u32,

        #[structopt(short="t", long, default_value = "30")]
        timeout: u32,

        #[structopt(short="l", long, parse(from_os_str))]
        log_path: Option<PathBuf>
    },

    #[structopt(name = "remove")]
    Remove {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "start")]
    Start {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "stop")]
    Stop {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "restart")]
    Restart {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "update")]
    Update {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "monitor")]
    Monitor {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "clear_logs")]
    ClearLogs {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "visualize")]
    Visualize {
        #[structopt(short="n", long)]
        name: String,
        
        #[structopt(short="s", long)]
        show: bool
    },

    #[structopt(name = "git_init")]
    GitInit {
        #[structopt(short="n", long)]
        name: String,
    },

    #[structopt(name = "add_origin")]
    AddOrigin {
        #[structopt(short="n", long)]
        name: String,

        #[structopt(short="u", long)]
        remote_url: String
    },

    #[structopt(name = "redis")]
    Redis {
        #[structopt(short="d", long, parse(from_os_str))]
        path: PathBuf,

        #[structopt(short="b", long, default_value = "127.0.0.1")]
        bind: String,

        #[structopt(short="p", long, default_value = "6379")]
        port: u32,

        #[structopt(short="l", long, parse(from_os_str))]
        log_path: Option<PathBuf>
    },


    #[structopt(name = "hardware")]
    Hardware, // Show hardware usage

    #[structopt(name = "list")]
    List, // List all servers

    #[structopt(name = "flush")]
    Flush

}