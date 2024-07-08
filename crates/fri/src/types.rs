use crate::config::Config;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use starknet_core::serde::unsigned_field_element::UfeHex;
use starknet_crypto::Felt;

// Commitment values for FRI. Used to generate a commitment by "reading" these values
// from the channel.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UnsentCommitment {
    // Array of size n_layers - 1 containing unsent table commitments for each inner layer.
    #[serde_as(as = "Vec<UfeHex>")]
    inner_layers: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    #[serde_as(as = "Vec<UfeHex>")]
    last_layer_coefficients: Vec<Felt>,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Commitment {
    pub config: Config,
    // Array of size n_layers - 1 containing table commitments for each inner layer.
    pub inner_layers: Vec<cairovm_verifier_commitment::table::types::Commitment>,
    // Array of size n_layers, of one evaluation point for each layer.
    #[serde_as(as = "Vec<UfeHex>")]
    pub eval_points: Vec<Felt>,
    // Array of size 2**log_last_layer_degree_bound containing coefficients for the last layer
    // polynomial.
    #[serde_as(as = "Vec<UfeHex>")]
    pub last_layer_coefficients: Vec<Felt>,
}

#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Decommitment {
    // Array of size n_values, containing the values of the input layer at query indices.
    #[serde_as(as = "Vec<UfeHex>")]
    pub values: Vec<Felt>,
    // Array of size n_values, containing the field elements that correspond to the query indices
    // (See queries_to_points).
    #[serde_as(as = "Vec<UfeHex>")]
    pub points: Vec<Felt>,
}

// A witness for the decommitment of the FRI layers over queries.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Witness {
    // An array of size n_layers - 1, containing a witness for each inner layer.
    layers: Vec<LayerWitness>,
}

// A witness for a single FRI layer. This witness is required to verify the transition from an
// inner layer to the following layer.
#[serde_as]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LayerWitness {
    // Values for the sibling leaves required for decommitment.
    #[serde_as(as = "Vec<UfeHex>")]
    leaves: Vec<Felt>,
    // Table commitment witnesses for decommiting all the leaves.
    table_witness: cairovm_verifier_commitment::table::types::Witness,
}
