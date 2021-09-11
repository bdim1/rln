#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use sapling_crypto::bellman::pairing::bn256::Bn256;
    test_keys::export::<Bn256>();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    panic!("should not be run in wasm");
}

#[cfg(not(target_arch = "wasm32"))]
mod test_keys {
    use sapling_crypto::bellman::pairing::Engine;
    pub fn export<E: Engine>() {
        use sapling_crypto::bellman::pairing::ff::{PrimeField, to_hex};
        use sapling_crypto::bellman::pairing::bn256::{Bn256, Fr};

        use rln::poseidon::Poseidon;
        use rln::poseidon::PoseidonParams;

        // let inputs: Vec<Fr> = ["0", "0"]
        //     .iter()
        //     .map(|e| Fr::from_str(e).unwrap())
        //     .collect();

            let mut inputs: Vec<Fr> = Vec::new();
            inputs.push(Fr::from_str("1").unwrap());
            inputs.push(Fr::from_str("0").unwrap());

        let poseidon_params = PoseidonParams::<Bn256>::new(8, 55, 3, None, None, None);
        let mut poseidon_hasher = Poseidon::<Bn256>::new(poseidon_params.clone());

        let hashed = poseidon_hasher.hash(inputs);
        let hashed_str = to_hex(&hashed);
        println!("hashed str 0x{}", hashed_str)



        // let mut rng = XorShiftRng::from_seed([0x3dbe6258, 0x8d313d76, 0x3237db17, 0xe5bc0654]);
        // let hasher = PoseidonCircuit::new(poseidon_params.clone());
        // let circuit = RLNCircuit::<E> {
        //     inputs: RLNInputs::<E>::empty(merkle_depth),
        //     hasher: hasher.clone(),
        // };
        // let parameters = generate_random_parameters(circuit, &mut rng).unwrap();
        // let mut file_vk = File::create("verifier.key").unwrap();
        // let vk = parameters.vk.clone();
        // vk.write(&mut file_vk).unwrap();
        // let mut file_paramaters = File::create("parameters.key").unwrap();
        // parameters.write(&mut file_paramaters).unwrap();
    }
}
