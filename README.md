# Tolak Chain

Tolak Chain is a substrate FRAME based chain build for tolak, for study and new feature test


# Developer build

 - Config you rustup toolchain

 ```sh
rustup uninstall nightly
rustup install nightly-2020-10-06
rustup default nightly-2020-10-06
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
 ```

 - Build release node binary
 
```sh
cargo build --release
```

Then you can run ```tolak-node``` under ```target/release/```.

 - Build a single WASM

 ```sh
 ./script/build-only-wasm.sh tolak-node-runtime
 ```

 Argument ```tolak-node-runtime``` is the package name of runtime. By default you 
 can see WASM file ```tolak_node_runtime.wasm``` was created in current directory.
