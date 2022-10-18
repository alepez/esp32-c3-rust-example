```shell
yay -S python-pip
rustup install nightly-2022-03-10
rustup component add rust-src --toolchain nightly-2022-03-10
cargo install cargo-espflash ldproxy
cargo espflash --release --monitor /dev/serial/by-id/usb-1a86_USB_Single_Serial_5424026661-if00
```
