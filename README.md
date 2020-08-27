# Actix Basics 

Basic `GET` and `POST` example using the endpoints at: `https://jsonplaceholder.typicode.com/<API>`

# Organization

## Library

`./src/lib.rs` provides the `get_user` and `post_user` methods. For documentation on use, run:

```bash
cargo doc --no-deps --open
```

## Binary

`./src/main.rs` is a complete example. Run with:

```bash
cargo run
```

# Tests

We use [cargo's testing system](https://doc.rust-lang.org/cargo/commands/cargo-test.html). To run tests in `/tests/*`:

```bash
cargo test
```