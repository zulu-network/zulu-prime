use super::BlockDetailsBase;
use serde::{Deserialize, Serialize};
use zksync_basic_types::L1BatchNumber;

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OffChainVerificationResult {
    pub l1_batch_number: u64,
    pub is_passed: bool,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OffChainVerificationDetails {
    pub l1_batch_number: u64,
    pub status: String,
    pub verifier_picked_at: DateTime<Utc>,
    pub verifier_submit_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct L1BatchDetailsWithOffchainVerification {
    pub number: L1BatchNumber,
    #[serde(flatten)]
    pub base: BlockDetailsBase,
    #[serde(flatten)]
    pub offchain_verification: OffChainVerificationDetails,
}
