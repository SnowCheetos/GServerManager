# GServerManager

## Description

GServerManager is an interactive command-line interface (CLI) tool for managing multiple `gunicorn` servers. It allows users to add, remove, start, stop, restart, monitor, update servers, manage server logs, and handle servers' GitHub settings. The tool is built using `Rust`, and is designed to be intuitive and easy to use. It also provides event-triggered backups and visualization tools, making it much easier to manage multiple servers on the same machine.

## Getting Started

### Prerequisites
* Rust Programming Language: You can download Rust from the [official website](https://www.rust-lang.org/tools/install).

### Installation
1. Clone the repo: `git clone https://github.com/SnowCheetos/GServerManager.git`
2. Navigate to the cloned directory: `cd GServerManager`
3. Build the project: `cargo build --release`

### Usage
After building the project, you can start using the `GServerManager`. Below are examples of the available commands:

* `add`: Add a new server.
* `remove`: Remove an existing server.
* `start`: Start an existing server.
* `stop`: Stop an existing server.
* `restart`: Restart an existing server.
* `update`: Pull from server repository and rebuild (if applicable).
* `monitor`: Monitor the log of an existing server.
* `clear_logs`: Clear logs of an existing server.
* `git_init`: Initialize Git in the server's directory.
* `add_origin`: Add a remote GitHub repository to the server.
* `hardware`: Show hardware usage.
* `list`: List all servers.
* `flush`: Stop and remove all servers.
* `visualize`: Visualize server logs.

Each command has additional options that can be viewed by using the -h option with the command, like so: `command -h`.

## Examples
```bash
$ ls
TestServer/    GServerManager/

$ cd TestServer/
app.py    # The directory must have at least one of `app.py` or `main.py`

$ cd .. && cd GServerManager/
$ ./target/release/GServerManager # Assuming you have already built it

╔════════════════════════════════════════════════════╗
║             Welcome to GServerManager              ║
╠════════════════════════════════════════════════════╣
║  This is a CLI tool for managing Gunicorn servers. ║
║   Use '-h' command to see the available options.   ║
╚════════════════════════════════════════════════════╝
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

>>> hardware
CPU USAGE:       36.46 % |=======             |
MEMORY USAGE:    95.85 % |=================== |
```
### Adding a server
```bash
>>> add -N test -F ../TestServer # Adding a server
Server added successfully.
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[ ] Name: test | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | PID: 0 |
```
### Starting a server
```bash
>>> start -N test
Server started successfully.
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[*] Name: test | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | PID: 74578 |
```
### Stopping a server and exiting
```bash
>>> stop -N test
Stopping... 
Server stopped successfully.
>>> exit
```

## Contributing
Contributions are welcome! Just create a branch, make your changes and create a pull request.

## License
Distributed under the MIT License. See LICENSE for more information.