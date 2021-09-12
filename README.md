# RLN  implementation with canonical Poseidon (WIP)

For previous version of the construct with custom Poseidon implementation, please visit: https://github.com/kilic/rln

To learn more about RLN, please visit: https://medium.com/privacy-scaling-explorations/rate-limiting-nullifier-a-spam-protection-mechanism-for-anonymous-environments-bbe4006a57d

## TODO: 

Refactor

## Generate Test Keys

```
cargo run --release --example export_test_keys
```

## Test canonical Poseidon
```
cargo run --example canonical_poseidon
```

## Test canonical Poseidon circuit
```
cargo run --example canonical_poseidon_circuit
```
