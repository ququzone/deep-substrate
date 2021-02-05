ink! contracts
==============

## Setup

### Substrate Prerequisites

```
rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain stable
```

### Installing The Canvas Node

```
cargo install canvas-node --git https://github.com/paritytech/canvas-node.git --tag v0.1.4 --force --locked
```

### ink! CLI

```
cargo install cargo-contract --vers 0.8.0 --force --locked
```

## Create project

```
cargo contract new erc20
cargo +nightly test
cargo +nightly contract build
```

## Run

https://paritytech.github.io/canvas-ui/

```
canvas --dev --tmp
```
