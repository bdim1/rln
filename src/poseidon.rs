use blake2::{Blake2s, Digest};

use sapling_crypto::bellman::pairing::ff::{Field, PrimeField, PrimeFieldRepr, to_hex};
use sapling_crypto::bellman::pairing::Engine;

use crate::poseidon_utils::{
    get_mds_poseidon_bn254_x5_2, get_mds_poseidon_bn254_x5_3, get_rounds_poseidon_bn254_x5_2, get_rounds_poseidon_bn254_x5_3
};

#[derive(Clone)]
pub struct PoseidonParams<E: Engine> {
    rf: usize,
    rp: usize,
    t: usize,
    round_constants: Vec<E::Fr>,
    mds_matrix: Vec<E::Fr>,
}

#[derive(Clone)]
pub struct Poseidon<E: Engine> {
    state: Vec<E::Fr>,
    round: usize,
    params: PoseidonParams<E>,
}

impl<E: Engine> PoseidonParams<E> {
    pub fn new(t: usize) -> PoseidonParams<E> {
        let mut params = PoseidonParams::<E>::empty();

      // t can be only 2 or 3
        params.t = t;
        params.rf = 8;
        if t == 2 {
            params.rp = 56; 
            params.round_constants = get_rounds_poseidon_bn254_x5_2::<E::Fr>();
            params.mds_matrix = get_mds_poseidon_bn254_x5_2::<E::Fr>();
        } else if t == 3 {
            params.rp = 57;
            params.round_constants = get_rounds_poseidon_bn254_x5_3::<E::Fr>();
            params.mds_matrix = get_mds_poseidon_bn254_x5_3::<E::Fr>();
        }

        params
    }

    pub fn empty() -> PoseidonParams::<E> {
        PoseidonParams {
            rf:0,
            rp:0,
            t:0,
            round_constants: Vec::new(),
            mds_matrix: Vec::new()
        }
    }

    pub fn width(&self) -> usize {
        return self.t;
    }

    pub fn partial_round_len(&self) -> usize {
        return self.rp;
    }

    pub fn full_round_half_len(&self) -> usize {
        return self.rf / 2;
    }

    pub fn total_rounds(&self) -> usize {
        return self.rf + self.rp;
    }

    pub fn round_constant(&self, round: usize) -> E::Fr {
        return self.round_constants[round];
    }

    pub fn mds_matrix_row(&self, i: usize) -> Vec<E::Fr> {
        let w = self.width();
        self.mds_matrix[i * w..(i + 1) * w].to_vec()
    }

    pub fn mds_matrix(&self) -> Vec<E::Fr> {
        self.mds_matrix.clone()
    }

}

impl<E: Engine> Poseidon<E> {
    pub fn new() -> Poseidon<E> {
        Poseidon {
            round: 0,
            state: Vec::new(),
            params: PoseidonParams::<E>::empty()
        }
    }

    fn new_state(&mut self, inputs: Vec<E::Fr>) {
        let mut new_state = vec![E::Fr::zero()];

        for num in inputs.iter() {
            let c = num.clone();
            new_state.push(c);
        }

        self.state = new_state;
    }

    fn clear(&mut self) {
        self.round = 0;
        self.params = PoseidonParams::<E>::empty();
    }

    fn t(&self) -> usize {
        self.params.t
    }

    fn result(&self) -> E::Fr {
        self.state[0]
    }

    pub fn hash(&mut self, inputs: Vec<E::Fr>) -> E::Fr {
        let num_inputs = inputs.len();
        if num_inputs < 1 || num_inputs > 2  {
            panic!("Invalid number of inputs");
        }
        let t = num_inputs + 1;
        
        self.params = PoseidonParams::<E>::new(t);
        self.new_state(inputs);

        for round in 0..self.params.total_rounds() {
            let a1 = self.params.full_round_half_len();
            let a2 = a1 + self.params.partial_round_len();
    
            self.add_round_constants(round);

            if round < a1 || round >= a2 {
                self.apply_quintic_sbox(true);
            } else {
                self.apply_quintic_sbox(false);
            }
            self.mul_mds_matrix(); 
        }

        let r = self.result();
        self.clear();
        r
    }

    fn add_round_constants(&mut self, round: usize) {
        let width = self.t();
        for (i, b) in self.state.iter_mut().enumerate() {
            let c = self.params.round_constants[round * width + i];
            b.add_assign(&c);
        }
    }

    fn apply_quintic_sbox(&mut self, full: bool) {
        for s in self.state.iter_mut() {
            let mut b = s.clone();
            b.square();
            b.square();
            s.mul_assign(&b);
            if !full {
                break;
            }
        }
    }

    fn mul_mds_matrix(&mut self) {
        let w = self.params.t;
        let mut new_state = vec![E::Fr::zero(); w];
        for (i, ns) in new_state.iter_mut().enumerate() {
            for (j, s) in self.state.iter().enumerate() {
                let mut tmp = s.clone();
                tmp.mul_assign(&self.params.mds_matrix[i * w + j]);
                ns.add_assign(&tmp);
            }
        }
        self.state = new_state;
    }
}

#[test]
fn test_poseidon_hash() {
    use sapling_crypto::bellman::pairing::bn256;
    use sapling_crypto::bellman::pairing::bn256::{Bn256, Fr};
    use sapling_crypto::bellman::pairing::ff::{PrimeField, to_hex};
    let mut hasher = Poseidon::<Bn256>::new();
    let input1: Vec<Fr> = ["0"].iter().map(|e| Fr::from_str(e).unwrap()).collect();
    let r1: Fr = hasher.hash(input1.to_vec());
    let input2: Vec<Fr> = ["1", "0"]
        .iter()
        .map(|e| Fr::from_str(e).unwrap())
        .collect();
    let r2: Fr = hasher.hash(input2.to_vec());
    // println!("{:?}", r1);

    let hash1 = to_hex(&r1);
    let hash2 = to_hex(&r2);
    println!("hash (0): 0x{}", hash1);
    println!("hash (1, 0): 0x{}", hash2);
    // assert_eq!(r1, r2, "just to see if internal state resets");

}
