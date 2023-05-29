# Ping-Pong

A test project that uses [WebTransport](https://www.w3.org/TR/webtransport/) to send data back and forward.

## Build the project

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Clone the repository: `git clone https://github.com/bytefall/ping-pong.git`
3. `cd ping-pong`
4. Generate TLS certificate: `openssl req -newkey rsa:2048 -new -nodes -x509 -days 3650 -keyout key.pem -out cert.pem`
5. Start the server: `cargo run --bin server`
6. Start the client: `cargo run --bin client`
