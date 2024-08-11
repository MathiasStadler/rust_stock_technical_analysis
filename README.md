# thirtyfour_download_file

## download file from url

## init project

```bash
mkdir thirtyfour_download_file && cd $_
touch README.md
ln -s README.md README
cargo init .
cargo add rustfmt
sudo chmod -R 0777 /opt/rust
rustup component add rustfmt
# need for rust analyzer
rustup component add rust-src
export RUST_SRC_PATH="$(rustc --print sysroot)/lib/rustlib/src/rust/src"
mkdir examples
cp src/main.rs examples/example.rs
# not understood
# sudo apt update
# sudo apt install libssl-dev pkg-config
# cargo install cargo-udeps
# cargo udeps --all-targets --backend depinfo
cargo add tokio --features full
cargo add thirtyfour
cargo update
# spezial for these project
cargo add ta
cargo add csv
```

## [crates.io ta](https://crates.io/crates/ta)

cargo run --example ema
