# M5Stamp-C3 ESP32 RISC-V

## Software requisites

You need nightly Rust and `espflash`. To install both of them, just use commands
below:

```shell
rustup install nightly-2022-10-18
rustup component add rust-src --toolchain nightly-2022-10-18
cargo install cargo-espflash ldproxy
```

## Flash

```shell
cargo espflash --release --monitor /dev/ttyACM0
```

## Configuration

You can configure some options in `cfg.toml`. This options will be embedded in
the resulting binary. This feature is provided
by [toml-cfg](https://crates.io/crates/toml-cfg)

The file `cfg.toml` is not provided. But you can copy `cfg.toml.example`
to `cfg.toml` and edit it. 