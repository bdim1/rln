#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use sapling_crypto::bellman::pairing::ff::{PrimeField, to_hex};
    use sapling_crypto::bellman::pairing::bn256::{Bn256, Fr};

    use rln::poseidon_canonical::Poseidon;

    let mut hasher = Poseidon::<Bn256>::new();
    let input1: Vec<Fr> = ["0"].iter().map(|e| Fr::from_str(e).unwrap()).collect();
    let r1: Fr = hasher.hash(input1.to_vec());
    let input2: Vec<Fr> = ["1", "0"]
        .iter()
        .map(|e| Fr::from_str(e).unwrap())
        .collect();
    let r2: Fr = hasher.hash(input2.to_vec());
    // println!("{:?}", r1);
    let input3: Vec<Fr> = ["1", "2"]
        .iter()
        .map(|e| Fr::from_str(e).unwrap())
        .collect();
    let r3: Fr = hasher.hash(input3.to_vec());
    

    let hash1 = to_hex(&r1);
    let hash2 = to_hex(&r2);
    let hash3 = to_hex(&r3);

    println!("hash (0): 0x{}", hash1);
    println!("hash (1, 0): 0x{}", hash2);
    println!("hash (1, 2): 0x{}", hash3);

}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("should not be run in wasm");
}