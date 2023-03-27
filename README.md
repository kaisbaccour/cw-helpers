# Contracts

- cw-escrow-121 This contracts allows two parties to share coins in a
  permissionless manner.
  - Party A places some source funds to exchange with the funds of party B and
    vice versa.
  - Until the exchange message is not executed both parties can still withdraw
    their source funds.
  - After the execute message no party can revert the action or withdraw their
    respective source funds

### Build the contracts

The repo root is a Rust workspace containing all the contracts. Basic tests can
be run like this:

```
cargo build --all-targets
cargo clippy --all-targets
cargo fmt
```

The production grade Wasm builds are compiled with:

```
#If you are running on OSX you might need to first run "brew install coreutils"
./devtools/build_integration_wasm.sh
```

### Run tests

```rust
cargo test
```

That's it 🎉

## Production build

This is a regular CosmWasm workspace. Use the latest version of
[workspace-optimizer](https://github.com/CosmWasm/rust-optimizer) to build it.

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.12
```
