# UDP-Server-CLI

Simple UDP server written in Rust. It receives requests, processes them and sends a responses.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [SQLite](https://sqlite.org/)
- [Sqlx-cli](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

### Deployment

1. Fork this repository to your folder
1. Edit `.env.example` by putting there your `DATABASE_URL`
1. Rename `.env.example` to `.env`
1. Create database with command `sqlx database create`
1. Run this command `sqlx migrate run`. It will create tables in your database add fill them.
1. Run this command `cargo test` to run some module tests.
1. Now, after all these preparations, just execute `cargo run` from your terminal.

## Usage

First of all, start the server with the command:

`cargo run`

The default port for listening is `8080`. You can specify another IP and port for listening:

`cargo run -- 127.0.0.1:8888`

After starting the server, open a new terminal window. With the help of the Netcat utility, you can send UDP packets to the server. Netcat (nc) command is installed by default in Linux OS. To connect to server using nc command use below command in system terminal:

`nc -u 127.0.0.1 8080`

Now we can send UDP packet to the server. Here is some requests and responses:

```
1
one
```

```
2
two
```

```
3
three
```

```
42
Database error: no rows returned by a query that expected to return at least one row
```

```
qwerty
Request error: invalid digit found in string
```

### Why did we get these results?

This server accepts the command, validates it and sends a response. If package is a number, it tries to find the text for this ID in the database. The database consists of one table. It has 3 lines by default:

| id  | text  |
| --- | ----- |
| 1   | one   |
| 2   | two   |
| 3   | three |

The server also responds with an error if it receives a non-number or no record is found for this number in the database.

## License

This project is licensed under the [MIT license](LICENSE).
