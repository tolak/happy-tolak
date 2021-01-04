# Tolak Chain

Tolak Chain is a substrate FRAME based chain build for tolak, for study and new feature test


# Developer build

 - Config you rustup toolchain

 ```sh
rustup install nightly-2020-10-06
rustup default nightly-2020-10-06
rustup target add wasm32-unknown-unknown --toolchain nightly-2020-10-06
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

# Run benchmarks

 - Enable benchmarks by running:

 ```sh
cd cli
cargo build --release --features runtime-benchmarks
 ```

 - Get a list of the available benchmarks by running:

```sh
./target/release/substrate benchmark --chain dev --pallet "*" --extrinsic "*" --repeat 0
```

 - run benchmark for a specific pallet(make sure pallet have written pallet benchmark test case) by running(pallet-balance for example):

 ```sh
./target/release/substrate benchmark \
--chain dev \
--execution=wasm \
--wasm-execution=compiled \
--pallet asset_claim \
--extrinsic "*" \
--steps 10 \
--repeat 2 \
--output pallets/asset-claim/src/weights.rs
```

This will output a file pallet_name.rs which implements the WeightInfo trait, add definition of WeightInfo in this pallet and give the dispatch a weight notation like this:

```sh
#[weight = T::WeightInfo::revoke_claim()]
fn revoke_claim(origin, proof: Vec<u8>) {
    // dispatch stuff
}
```

More information about substrate benchmarking can be found [here](https://github.com/paritytech/substrate/tree/master/frame/benchmarking)
