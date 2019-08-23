# TcpClientSpeedtest


# Installation

The installation requires cargo (installation via [rustup](https://rustup.rs/))


```
rustup toolchain install nightly
cargo +nightly install --git https://github.com/julianbieber/TcpClientSpeedtest.git
```

# Run

tcp_client --address IP --threads 1000 --new_connections --mb N
