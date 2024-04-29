# Rest In Rust (R.I.R.)
A simple REST API written in Rust using [Actix](https://actix.rs/) and [Diesel](https://diesel.rs).  
This project is based on this repo: [Rust Twitter Clone](https://github.com/evoxmusic/twitter-clone-rust).

## Getting started
Install the Diesel CLI Tool

```sh
cargo install diesel_cli
```

Run the Diesel migration

```sh
diesel migration run
```
Start the PostgreSQL Database

```sh
docker compose up -d db
```

### Let's go!

```sh
cargo run
```
