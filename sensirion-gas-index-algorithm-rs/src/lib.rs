#![cfg_attr(not(feature = "std"), no_std)]

mod sensirion_gas_index_algorithm;
#[cfg(test)]
mod tests;

pub use sensirion_gas_index_algorithm::AlgorithmType;
pub use sensirion_gas_index_algorithm::GasIndexAlgorithm;
pub use sensirion_gas_index_algorithm::TuningParameters;
