# Actix Basics

Simple `actix_web` server example emulating the behavior of `https://jsonplaceholder.typicode.com/api/v1/users`

## Running the Server

```bash
cargo run
```

This server then is bound and listening on `127.0.0.1:9090`.

Basic `GET` and `POST` example tests are found in `./src/user.rs`.

## Testing

We use [cargo's testing system](https://doc.rust-lang.org/cargo/commands/cargo-test.html). To run tests, we want **two terminals open** - one running the server, and one to run tests against it.

### 1. Run the Server:

In a fresh terminal:

```bash
cargo run
```

### 2. Run the Tests against the Server:

In a fresh terminal:

```bash
cargo test
```
