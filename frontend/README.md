# Frontend

## Setup

```bash
rustup
rustup target add wasm32-unknown-unknown
cargo install --locked trunk wasm-bindgen-cli
```

## Run (dev)

It is important to run the command in the frontend directory for permissions.

```bash
trunk serve --open
```

## Build

```bash
trunk build --release
```

Unless overwritten, the output will be located in the `dist` directory.
