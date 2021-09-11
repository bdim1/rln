use sapling_crypto::bellman::pairing::ff::{PrimeField, from_hex};
use sapling_crypto::bellman::pairing::bn256::{Bn256, Fr};

pub mod bn254_x5_3;
pub mod bn254_x5_2;


pub fn decode_hex(s: &str) -> Vec<u8> {
	let s = &s[2..];
	let vec: Vec<u8> = (0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
		.collect();

	vec
}

pub fn get_bytes_array_from_hex(hex_str: &str) -> [u8; 32] {
	let bytes = decode_hex(hex_str);
	let mut result: [u8; 32] = [0; 32];
	result.copy_from_slice(&bytes);
	result
}

pub fn parse_vec<F: PrimeField>(arr: Vec<&str>) -> Vec<F> {
	let mut res = Vec::new();
	for r in arr.iter() {
		let c = from_hex(r).unwrap();
		res.push(c);
	}
	res
}

pub fn parse_matrix<F: PrimeField>(mds_entries: Vec<Vec<&str>>) -> Vec<F> {
	let width = mds_entries.len();
	let mut matrix: Vec<F> = Vec::with_capacity(width * width);
	for i in 0..width {
		for j in 0..width {
			let c = from_hex(mds_entries[i][j]).unwrap();
			matrix.insert((i * width) + j, c);
		}
	}
	matrix
}

// #[cfg(feature = "poseidon_bn254_x5_5")]
// pub fn get_results_poseidon_bn254_x5_5<F: PrimeField>() -> Vec<F> {
// 	parse_vec(bn254_x5_5_result::RESULT.to_vec())
// }

// #[cfg(feature = "poseidon_bn254_x5_3")]
// pub fn get_results_poseidon_bn254_x5_3<F: PrimeField>() -> Vec<F> {
// 	parse_vec(bn254_x5_3_result::RESULT.to_vec())
// }

pub fn get_rounds_poseidon_bn254_x5_3<F: PrimeField>() -> Vec<F> {
	parse_vec(bn254_x5_3::ROUND_CONSTS.to_vec())
}

pub fn get_rounds_poseidon_bn254_x5_2<F: PrimeField>() -> Vec<F> {
	parse_vec(bn254_x5_2::ROUND_CONSTS.to_vec())
}

pub fn get_mds_poseidon_bn254_x5_3<F: PrimeField>() -> Vec<F> {
	parse_matrix(
		bn254_x5_3::MDS_ENTRIES
			.iter()
			.map(|x| x.to_vec())
			.collect::<Vec<_>>(),
	)
}

pub fn get_mds_poseidon_bn254_x5_2<F: PrimeField>() -> Vec<F> {
	parse_matrix(
		bn254_x5_2::MDS_ENTRIES
			.iter()
			.map(|x| x.to_vec())
			.collect::<Vec<_>>(),
	)
}
