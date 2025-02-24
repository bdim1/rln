use crate::circuit::poseidon::PoseidonCircuit;
use crate::circuit::rln::{RLNCircuit, RLNInputs};
use crate::merkle::MerkleTree;
use crate::poseidon::{Poseidon as PoseidonHasher, PoseidonParams};

use rand::{Rand, SeedableRng, XorShiftRng};
use sapling_crypto::bellman::groth16::*;
use sapling_crypto::bellman::pairing::ff::{Field, PrimeField, PrimeFieldRepr};
use sapling_crypto::bellman::pairing::Engine;
use sapling_crypto::bellman::Circuit;
use sapling_crypto::circuit::test::TestConstraintSystem;
use std::error::Error;
use std::io::{self, ErrorKind, Read, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::public::RLN;

pub struct ProverBenchResult {
    pub prover_key_size: usize,
    pub prover_time: f64,
}

impl ProverBenchResult {
    pub fn new() -> ProverBenchResult {
        ProverBenchResult {
            prover_key_size: 0,
            prover_time: 0f64,
        }
    }
}

pub fn run_rln_prover_bench<E: Engine>(
    merkle_depth: usize
) -> ProverBenchResult {
    RLNTest::<E>::new(merkle_depth).run_prover_bench()
}

pub struct RLNTest<E>
where
    E: Engine,
{
    rln: RLN<E>,
    merkle_depth: usize,
}


impl<E> RLNTest<E>
where
    E: Engine,
{
    fn rng() -> XorShiftRng {
        XorShiftRng::from_seed([0x3dbe6258, 0x8d313d76, 0x3237db17, 0xe5bc0654])
    }

    fn empty_inputs(merkle_depth: usize) -> RLNInputs<E> {
        RLNInputs::<E> {
            share_x: None,
            share_y: None,
            epoch: None,
            nullifier: None,
            root: None,
            id_key: None,
            auth_path: vec![None; merkle_depth],
        }
    }

    pub fn new(merkle_depth: usize) -> RLNTest<E> {
        RLNTest {
            rln: RLN::new(merkle_depth),
            merkle_depth,
        }
    }

    pub fn hasher(&self) -> PoseidonHasher<E> {
        self.rln.hasher()
    }

    pub fn valid_inputs(&self) -> RLNInputs<E> {
        let mut rng = Self::rng();
        let mut hasher = self.rln.hasher();

        // Initialize empty merkle tree
        let merkle_depth = self.merkle_depth;
        let mut membership_tree = MerkleTree::empty(hasher.clone(), merkle_depth);

        // A. setup an identity

        let id_key = E::Fr::rand(&mut rng);
        let id_comm: E::Fr = hasher.hash(vec![id_key.clone()]);

        // B. insert to the membership tree

        let id_index = 6; // any number below 2^depth will work
        membership_tree.update(id_index, id_comm);

        // C.1 get membership witness

        let auth_path = membership_tree.witness(id_index);
        assert!(membership_tree.check_inclusion(auth_path.clone(), id_index, id_key.clone()));

        // C.2 prepare sss

        // get current epoch
        let epoch = E::Fr::rand(&mut rng);

        let signal_hash = E::Fr::rand(&mut rng);
        // evaluation point is the signal_hash
        let share_x = signal_hash.clone();

        // calculate current line equation
        let a_0 = id_key.clone();
        let a_1: E::Fr = hasher.hash(vec![a_0, epoch]);

        // evaluate line equation
        let mut share_y = a_1.clone();
        share_y.mul_assign(&share_x);
        share_y.add_assign(&a_0);

        // calculate nullfier
        let nullifier = hasher.hash(vec![a_1]);

        // compose the circuit

        let inputs = RLNInputs::<E> {
            share_x: Some(share_x),
            share_y: Some(share_y),
            epoch: Some(epoch),
            nullifier: Some(nullifier),
            root: Some(membership_tree.root()),
            id_key: Some(id_key),
            auth_path: auth_path.into_iter().map(|w| Some(w)).collect(),
        };

        inputs
    }

    pub fn synthesize(&self) -> usize {
        let hasher = PoseidonCircuit::<E>::new();
        let inputs = self.valid_inputs();
        let circuit = RLNCircuit::<E> {
            inputs: inputs.clone(),
            hasher: hasher.clone(),
        };

        let mut cs = TestConstraintSystem::<E>::new();

        let circuit = circuit.clone();
        circuit.synthesize(&mut cs).unwrap();
        let unsatisfied = cs.which_is_unsatisfied();
        if unsatisfied.is_some() {
            panic!("unsatisfied\n{}", unsatisfied.unwrap());
        }
        let unconstrained = cs.find_unconstrained();
        if !unconstrained.is_empty() {
            panic!("unconstrained\n{}", unconstrained);
        }
        assert!(cs.is_satisfied());
        cs.num_constraints()
    }

    pub fn run_prover_bench(&self) -> ProverBenchResult {
        let mut raw_inputs: Vec<u8> = Vec::new();
        let inputs = self.valid_inputs();
        inputs.write(&mut raw_inputs).unwrap();

        let mut proof: Vec<u8> = Vec::new();
        let now = Instant::now();
        self.rln
            .generate_proof(raw_inputs.as_slice(), &mut proof)
            .unwrap();
        let prover_time = now.elapsed().as_millis() as f64 / 1000.0;

        let mut raw_public_inputs: Vec<u8> = Vec::new();
        inputs.write_public_inputs(&mut raw_public_inputs).unwrap();

        assert!(
            self.rln
                .verify(proof.as_slice(), raw_public_inputs.as_slice())
                .unwrap(),
            "invalid proof"
        );

        let mut circuit_parameters: Vec<u8> = Vec::new();
        self.rln
            .export_circuit_parameters(&mut circuit_parameters)
            .unwrap();
        let prover_key_size = circuit_parameters.len();

        ProverBenchResult {
            prover_time,
            prover_key_size,
        }
    }

    pub fn export_circuit_parameters<W: Write>(&self, w: W) -> io::Result<()> {
        self.rln.export_circuit_parameters(w)
    }
}

