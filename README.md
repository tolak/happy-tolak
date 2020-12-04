# Tolak Chain

Tolak Chain is a substrate FRAME based chain build for tolak, for study and new feature test


# Developer build

 - Config you rustup toolchain

 ```sh
 rustup uninstall nightly
 rustup install nightly-2020-10-06
 rustup default nightly-2020-10-06
 rustup target add wasm32-unknown-unknown
 ```

 - Build release target
 
 ```sh
 cargo build --release
 ```

Then you can run ```tolak-node``` under ```target/release/```.
