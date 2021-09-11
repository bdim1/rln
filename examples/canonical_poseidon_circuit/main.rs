#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use rln::poseidon_canonical::Poseidon;
    use rln::circuit::poseidon_canonical::PoseidonCircuit;
    use sapling_crypto::bellman::pairing::bn256::{Bn256, Fr};
    use sapling_crypto::bellman::pairing::ff::{PrimeField, to_hex};
    use sapling_crypto::circuit::test::TestConstraintSystem;
    use sapling_crypto::bellman::{ ConstraintSystem, };
    use sapling_crypto::circuit::{num };

    let mut cs = TestConstraintSystem::<Bn256>::new();

    let inputs: Vec<Fr> = ["1", "2"]
        .iter()
        .map(|e| Fr::from_str(e).unwrap())
        .collect();
    let allocated_inputs = inputs
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, e)| {
            let a = num::AllocatedNum::alloc(cs.namespace(|| format!("input {}", i)), || Ok(e));
            a.unwrap()
        })
        .collect();

    let mut circuit = PoseidonCircuit::<Bn256>::new();
    let res_allocated = circuit
        .alloc(cs.namespace(|| "hash alloc"), allocated_inputs)
        .unwrap();
    let result = res_allocated.get_value().unwrap();
    let mut poseidon = Poseidon::<Bn256>::new();
    let expected = poseidon.hash(inputs);

    
    println!("circuit hash (1,2): 0x{}", to_hex(&result));
    println!("hash (1,2): 0x{}", to_hex(&expected));

    assert!(cs.is_satisfied());
    assert_eq!(result, expected);

    println!(
        "number of constraints for {}",
        cs.num_constraints()
    );

}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("should not be run in wasm");
}