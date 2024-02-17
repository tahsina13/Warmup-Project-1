# Warmup-Project-1

Example config.toml:

```TOML
ip = [127, 0, 0, 1]
submission_id = "foobarbooblaz1234"
http_port = 80
```

## To Run

From root:

```Shell
cargo build
cargo install cargo-watch
cargo watch -x run
```

```Shell
# allow binary to bind to low-number ports
sudo setcap CAP_NET_BIND_SERVICE=+eip target/debug/axum-server
./target/debug/axum-server
```
