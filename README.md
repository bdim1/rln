# RLN implementation with canonical Poseidon

RLN construct implementation using a canonical Poseidon implementation, the recommended implementation from the research [paper](https://eprint.iacr.org/2019/458.pdf) for `x^5` S-box.
The hasher supports 1 or 2 inputs only, as those are the necessities for the RLN construct.

For previous version of the construct with custom Poseidon implementation, please visit: https://github.com/kilic/rln
To learn more about RLN, please visit: https://medium.com/privacy-scaling-explorations/rate-limiting-nullifier-a-spam-protection-mechanism-for-anonymous-environments-bbe4006a57d


## Test

```
cargo test --release --features multicore rln_32 -- --nocapture
```

## Examples

#### Generate Test Keys

```
cargo run --release --example export_test_keys
```

#### Poseidon hash
```
cargo run --example poseidon
```

#### Poseidon circuit hash
```
cargo run --example poseidon_circuit
```


## Wasm Support

#### Build

```
wasm-pack build --release --target=nodejs --scope=rln --out-name=rlnwasm --out-dir=rlnwasm -- --features wasm
```

#### Test

With wasm-pack:

```
wasm-pack test --release --node -- --features wasm
```

With cargo:

Follow the steps [here](https://rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/usage.html#appendix-using-wasm-bindgen-test-without-wasm-pack) before running the test, then run:

```
cargo test --release --target wasm32-unknown-unknown --features wasm
```