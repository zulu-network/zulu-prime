use serde::{Deserialize, Serialize};
use zksync_utils::concat_and_hash;

use crate::{L1BatchNumber, H256};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitvmProof {
    /// Numeric ID of the block. Starts from 1, 0 block is considered genesis block and has no transactions.
    pub number: L1BatchNumber,
    /// Proof of bitvm.
    pub proof: String,
    /// Verification key of bitvm.
    pub vk: String,
    /// Public input of bitvm.
    pub public_input: String,
    /// Id of the proof = Sha256(proof_system || proof || public_input || vk)
    pub proof_id: H256,
}
