# Rust

Fundermental rust and unit test.

## Install Rust

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source $HOME/.cargo/env
```

## Initial

```bash
$ cargo init demo-rust
```

## Build (creates binary in target/)

```bash
$ cargo build
```

## Run

```bash
$ cargo run --example array
$ cargo run --example conditional
$ cargo run --example controlflow
$ cargo run --example number
$ cargo run --example string
$ cargo run --example struct
$ cargo run --example trait
$ cargo run --example tuple
$ cargo run --example vector
```

## Test

```bash
$ cargo test --example array
```

## Code Formatting

```bash
$ cargo fmt
```
