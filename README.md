![](media/logo.png)

# Description

GServerManager is a comprehensive command-line interface (CLI) tool designed in Rust for the efficient management of multiple **WSGI** servers. It provides essential features that allow users to seamlessly add, remove, start, stop, restart, update servers, etc. Users can monitor server status, manage server logs, and adjust servers' GitHub settings, all from the same interface.

In an effort to simplify and streamline operations, GServerManager includes automatic event-triggered backups and intuitive visualization tools. This makes handling multiple servers on a single machine straightforward and less cumbersome.

GServerManager is built to be user-friendly and practical. Its design philosophy emphasizes simplicity and ease of use without compromising on functionality. Currently, the tool extends support to widely-used frameworks such as `Flask`, `FastAPI`, and `Django`, and it works in conjunction with the `Gunicorn` WSGI HTTP server.

Whether you're a novice or an experienced system administrator, GServerManager provides an intuitive, feature-packed solution for WSGI server management. Its blend of crucial features and user-centric design makes managing servers a hassle-free task.

# Getting Started

## Prerequisites
* **Rust**: You can download Rust from the [rust official website](https://www.rust-lang.org/tools/install).
* **Python**: You'll need python for the your WSGI servers as well as the included data processing and visualization tools. It's recommended to use `anaconda` for environment managements, check out the [anaconda official website](https://www.anaconda.com/) for details.
* **Redis**: This application supports Redis, if you would need to use it in your servers, check out the [redis official website](https://redis.io/docs/getting-started/) for more information.

## Installation
1. Clone the repo: `git clone https://github.com/SnowCheetos/GServerManager.git`
2. Navigate to the cloned directory: `cd GServerManager`
3. Build the project: `cargo build --release`
4. Install python dependencies via `pip install -r requirements.txt`

## Usage
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

# Examples
After installation, an executable can be found in `GServerManager/target/release/`
```bash
$ cd GServerManager/
$ ./target/release/GServerManager

# This is the welcome screen
╔════════════════════════════════════════════════════╗
║             Welcome to GServerManager              ║
╠════════════════════════════════════════════════════╣
║   This is a CLI tool for managing WSGI servers.    ║
║   Use '-h' command to see the available options.   ║
╚════════════════════════════════════════════════════╝
>>> # This is where you will type the commands
```






## Monitor hardware usage (beta)
### You can see a snapshot of the hardware usage by using the `hardware` command, it's still in the development phase and can be inaccurate.
```bash
>>> hardware
CPU USAGE:       39.38 % |========            |
MEMORY USAGE:    98.43 % |====================|
>>> 
```






## Adding a server and listing all available servers
### You can start a server by typing `add --name {server name} --path {path to server directory}`, or use `add --help` to see all configuration options.
```bash
>>> add --name test_server --path tests/test-servers/server-1 --framework flask # Use `add --help` or `add -h` to see all options 
Successfully added [test_server]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[ ] Name: test_server | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
>>> 
```



## Adding more and starting servers
```bash
>>> add --name test_server_2 --path tests/test-servers/server-2 --framework fastapi # Note that you can't add another server on the same port
[ERROR] Server port already exists
>>> add --name test_server_2 --path tests/test-servers/server-2 --framework fastapi --port 9000
Successfully added [test_server_2]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[ ] Name: test_server | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
[ ] Name: test_server_2 | Address: 0.0.0.0:9000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-2 |
>>> 
```



### Now we can start a server by typing `start --name {server name}` or `start -n {server name}`
```bash
>>> start --name test_server
Successfully started [test_server]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 
# Note that the list shows that test_server is running, which is correct
[*] Name: test_server | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
[ ] Name: test_server_2 | Address: 0.0.0.0:9000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-2 |
>>> 
```




## Redis support
If you're using redis as a caching layer or any other purpose, you can add a redis server by `redis --path {path to redis config file}`, if the path you provided does not contain `redis.conf`, then it'll automatically use the default redis configurations.
```bash
>>> redis --path tests/redis-logs/redis-1
Successfully added [redis-server]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[*] Name: test_server | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
[ ] Name: test_server_2 | Address: 0.0.0.0:9000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-2 |
[ ] Name: redis-server | Address: 127.0.0.1:6379 | Workers: 1 | Timeout: 30s | Log Path: tests/redis-logs/redis-1 |
>>> 
```
You can ass multiple redis servers if you need, as long as they're on different ports.
```bash
>>> redis --path tests/redis-logs/redis-2 -p 7000
Successfully added [redis-server-2]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[*] Name: test_server | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
[ ] Name: test_server_2 | Address: 0.0.0.0:9000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-2 |
[ ] Name: redis-server | Address: 127.0.0.1:6379 | Workers: 1 | Timeout: 30s | Log Path: tests/redis-logs/redis-1 |
[ ] Name: redis-server-2 | Address: 127.0.0.1:7000 | Workers: 1 | Timeout: 30s | Log Path: tests/redis-logs/redis-2 |
>>> 
```




## Starting redis server and monitoring servers
### You can start a redis server the same way you start other servers
```bash
>>> start -n redis-server-2
Successfully started [redis-server-2]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[*] Name: test_server | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
[ ] Name: test_server_2 | Address: 0.0.0.0:9000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-2 |
[ ] Name: redis-server | Address: 127.0.0.1:6379 | Workers: 1 | Timeout: 30s | Log Path: tests/redis-logs/redis-1 |
[*] Name: redis-server-2 | Address: 127.0.0.1:7000 | Workers: 1 | Timeout: 30s | Log Path: tests/redis-logs/redis-2 |
>>> 
```


### You can view the logs for the servers via `monitor --name {server name}`
```bash
>>> monitor -n test_server
Successfully retrieved server logs.
[2023-06-25 17:25:05 -0500] [22828] [INFO] Starting gunicorn 20.1.0
[2023-06-25 17:25:05 -0500] [22828] [INFO] Listening at: http://0.0.0.0:8000 (22828)
[2023-06-25 17:25:05 -0500] [22828] [INFO] Using worker: sync
[2023-06-25 17:25:05 -0500] [22829] [INFO] Booting worker with pid: 22829
[2023-06-25 17:25:05 -0500] [22830] [INFO] Booting worker with pid: 22830
[2023-06-25 17:25:05 -0500] [22831] [INFO] Booting worker with pid: 22831
[2023-06-25 17:25:05 -0500] [22832] [INFO] Booting worker with pid: 22832

>>> monitor -n redis-server-2
Successfully retrieved server logs.
23015:C 25 Jun 2023 17:33:20.462 # oO0OoO0OoO0Oo Redis is starting oO0OoO0OoO0Oo
23015:C 25 Jun 2023 17:33:20.462 # Redis version=7.0.11, bits=64, commit=00000000, modified=0, pid=23015, just started
23015:C 25 Jun 2023 17:33:20.462 # Configuration loaded
23015:M 25 Jun 2023 17:33:20.463 * monotonic clock: POSIX clock_gettime
23015:M 25 Jun 2023 17:33:20.465 * Running mode=standalone, port=7000.
23015:M 25 Jun 2023 17:33:20.465 # WARNING: The TCP backlog setting of 511 cannot be enforced because kern.ipc.somaxconn is set to the lower value of 128.
23015:M 25 Jun 2023 17:33:20.466 # Server initialized
23015:M 25 Jun 2023 17:33:20.466 * Loading RDB produced by version 7.0.11
23015:M 25 Jun 2023 17:33:20.467 * RDB age 3470 seconds
23015:M 25 Jun 2023 17:33:20.467 * RDB memory usage when created 1.10 Mb
23015:M 25 Jun 2023 17:33:20.467 * Done loading RDB, keys loaded: 0, keys expired: 0.
23015:M 25 Jun 2023 17:33:20.467 * DB loaded from disk: 0.001 seconds
23015:M 25 Jun 2023 17:33:20.467 * Ready to accept connections

>>> 
```





## Server log visualizations (currently only available for Flask servers)
```bash
>>> visualize -n test -s # display tag
```
![](media/demo.jpg)






## Example of server and event logs which will be saved in `data/logs`
### Server logs
```yaml
| Type   | Timestamp                 | IP          | RequestMethod | Endpoint | ResponseCode | UserAgent                                                                                                             |
|--------|---------------------------|-------------|---------------|----------|--------------|-----------------------------------------------------------------------------------------------------------------------|
| Server | 2023-06-24 02:37:00-05:00 | 127.0.0.1   | GET           | /        | 200          | Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 |
| Server | 2023-06-24 02:37:00-05:00 | 127.0.0.1   | GET           | /        | 200          | Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 |
| Server | 2023-06-24 02:37:00-05:00 | 127.0.0.1   | GET           | /        | 200          | Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 |
| Server | 2023-06-24 02:37:00-05:00 | 127.0.0.1   | GET           | /        | 200          | Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36 |

```
### Event logs
```yaml
|      | Type  | Timestamp                 | PID   | LogLevel      | EventMessage                                     |
|------|-------|---------------------------|-------|---------------|--------------------------------------------------|
| 3    | Event | 2023-06-24 02:37:51-05:00 | 80925 | INFO          | Booting worker with pid: 80925                   |
| 4    | Event | 2023-06-24 02:37:51-05:00 | 80926 | INFO          | Booting worker with pid: 80926                   |
| 5    | Event | 2023-06-24 02:37:51-05:00 | 80927 | INFO          | Booting worker with pid: 80927                   |
| 6    | Event | 2023-06-24 02:37:51-05:00 | 80928 | INFO          | Booting worker with pid: 80928                   |
| 7    | Event | 2023-06-24 02:38:28-05:00 | 80924 | CRITICAL      | WORKER TIMEOUT (pid:80927)                       |
| 8    | Event | 2023-06-24 02:38:28-05:00 | 80927 | INFO          | Worker exiting (pid: 80927)                      |
| 9    | Event | 2023-06-24 02:38:28-05:00 | 80992 | INFO          | Booting worker with pid: 80992                   |
| 10   | Event | 2023-06-24 02:58:43-05:00 | 80926 | INFO          | Worker exiting (pid: 80926)                      |
| 11   | Event | 2023-06-24 02:58:43-05:00 | 80925 | INFO          | Worker exiting (pid: 80925)                      |
```







## Stopping a server
### You can stop a server via `stop --name {server name}`
```bash
>>> stop -n redis-server-2
Successfully stopped [redis-server-2]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[*] Name: test_server | Address: 0.0.0.0:8000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
[ ] Name: test_server_2 | Address: 0.0.0.0:9000 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-2 |
[ ] Name: redis-server | Address: 127.0.0.1:6379 | Workers: 1 | Timeout: 30s | Log Path: tests/redis-logs/redis-1 |
[ ] Name: redis-server-2 | Address: 127.0.0.1:7000 | Workers: 1 | Timeout: 30s | Log Path: tests/redis-logs/redis-2 |
>>> 
```

### You can also stop all servers and clear the list via `flush`
```bash
>>> flush
Successfully stopped [test_server]
Server [redis-server] not currently running, doing nothing...
Server [test_server_2] not currently running, doing nothing...
Server [redis-server-2] not currently running, doing nothing...
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

>>> 
```

## Event triggered backups
### Persistent backups are available for this application. Each time a backup event is triggered (add, remove, start, etc...), the server states are saved in `backups/servers_backup.json`, if you exit the application, it'll automatically be restored next time you launch it.
```bash
>>> add -n test-server -d tests/test-servers/server-1 -p 7890 -f fastapi
Successfully added [test-server]
>>> start -n test-server # Backup event
Successfully started [test-server]
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[*] Name: test-server | Address: 0.0.0.0:7890 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
>>> exit

$ ./target/release/GServerManager
╔════════════════════════════════════════════════════╗
║             Welcome to GServerManager              ║
╠════════════════════════════════════════════════════╣
║   This is a CLI tool for managing WSGI servers.    ║
║   Use '-h' command to see the available options.   ║
╚════════════════════════════════════════════════════╝
>>> list
[INFO] Listing all available servers
[INFO] [*]: Running | [ ]: Not running 

[*] Name: test-server | Address: 0.0.0.0:7890 | Workers: 4 | Timeout: 30s | Log Path: tests/test-servers/server-1 |
>>> 
```




# Contributing
Contributions are welcome! Just make a branch, make your changes and create a pull request.

# License
Distributed under the GNU v3.0 License. See `LICENSE` for more information.