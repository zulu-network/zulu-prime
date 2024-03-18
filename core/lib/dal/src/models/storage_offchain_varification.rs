use std::{convert::TryInto, str::FromStr};

use bigdecimal::{BigDecimal, ToPrimitive};
use sqlx::{
    postgres::{PgArguments, Postgres},
    query::Query,
    types::chrono::{DateTime, NaiveDateTime, Utc},
};
use thiserror::Error;
use zksync_contracts::BaseSystemContractsHashes;
use zksync_types::{
    api,
    block::{L1BatchHeader, MiniblockHeader},
    commitment::{L1BatchMetaParameters, L1BatchMetadata},
    fee_model::{BatchFeeInput, L1PeggedBatchFeeModelInput, PubdataIndependentBatchFeeModelInput},
    l2_to_l1_log::{L2ToL1Log, SystemL2ToL1Log, UserL2ToL1Log},
    Address, L1BatchNumber, MiniblockNumber, ProtocolVersionId, H2048, H256,
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageProofOffchainVerification {
    pub l1_batch_number: i64,
    pub status: String,
    pub verifier_picked_at: Option<NaiveDateTime>,
    pub verifier_submit_at: Option<NaiveDateTime>,
}

impl From<StorageProofOffchainVerification>
    for api::proof_offchain_verification::OffChainVerificationDetails
{
    fn from(details: StorageProofOffchainVerification) -> Self {
        api::proof_offchain_verification::OffChainVerificationDetails {
            l1_batch_number: L1BatchNumber(details.l1_batch_number as u32),
            verifier_status: details.status, // Here we just return the string.
            verifier_picked_at: details
                .verifier_picked_at
                .map(|committed_at| DateTime::<Utc>::from_naive_utc_and_offset(committed_at, Utc)),
            verifier_submit_at: details
                .verifier_submit_at
                .map(|committed_at| DateTime::<Utc>::from_naive_utc_and_offset(committed_at, Utc)),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct StorageL1BatchDetailsWithOffchainVerification {
    // As same as StorageL1BatchDetails's fields
    pub number: i64,
    pub timestamp: i64,
    pub l1_tx_count: i32,
    pub l2_tx_count: i32,
    pub root_hash: Option<Vec<u8>>,
    pub commit_tx_hash: Option<String>,
    pub committed_at: Option<NaiveDateTime>,
    pub prove_tx_hash: Option<String>,
    pub proven_at: Option<NaiveDateTime>,
    pub execute_tx_hash: Option<String>,
    pub executed_at: Option<NaiveDateTime>,
    pub l1_gas_price: i64,
    pub l2_fair_gas_price: i64,
    pub bootloader_code_hash: Option<Vec<u8>>,
    pub default_aa_code_hash: Option<Vec<u8>>,
    // Below are offchain_verfication fields
    pub offchain_verfication_status: String,
    pub offchain_verifier_picked_at: Option<NaiveDateTime>,
    pub offchain_verifier_submit_at: Option<NaiveDateTime>,
}

impl From<StorageL1BatchDetailsWithOffchainVerification>
    for api::proof_offchain_verification::L1BatchDetailsWithOffchainVerification
{
    fn from(details: StorageL1BatchDetailsWithOffchainVerification) -> Self {
        let status = if details.number == 0 || details.execute_tx_hash.is_some() {
            api::BlockStatus::Verified
        } else {
            api::BlockStatus::Sealed
        };

        let base = api::BlockDetailsBase {
            timestamp: details.timestamp as u64,
            l1_tx_count: details.l1_tx_count as usize,
            l2_tx_count: details.l2_tx_count as usize,
            status,
            root_hash: details.root_hash.as_deref().map(H256::from_slice),
            commit_tx_hash: details
                .commit_tx_hash
                .as_deref()
                .map(|hash| H256::from_str(hash).expect("Incorrect commit_tx hash")),
            committed_at: details
                .committed_at
                .map(|committed_at| DateTime::<Utc>::from_naive_utc_and_offset(committed_at, Utc)),
            prove_tx_hash: details
                .prove_tx_hash
                .as_deref()
                .map(|hash| H256::from_str(hash).expect("Incorrect prove_tx hash")),
            proven_at: details
                .proven_at
                .map(|proven_at| DateTime::<Utc>::from_naive_utc_and_offset(proven_at, Utc)),
            execute_tx_hash: details
                .execute_tx_hash
                .as_deref()
                .map(|hash| H256::from_str(hash).expect("Incorrect execute_tx hash")),
            executed_at: details
                .executed_at
                .map(|executed_at| DateTime::<Utc>::from_naive_utc_and_offset(executed_at, Utc)),
            l1_gas_price: details.l1_gas_price as u64,
            l2_fair_gas_price: details.l2_fair_gas_price as u64,
            base_system_contracts_hashes: convert_base_system_contracts_hashes(
                details.bootloader_code_hash,
                details.default_aa_code_hash,
            ),
        };

        let offchain_verification = api::proof_offchain_verification::OffChainVerificationDetails {
            l1_batch_number: L1BatchNumber(details.number as u32),
            verifier_status: details.offchain_verfication_status, // Here we just return the string.
            verifier_picked_at: details
                .offchain_verifier_picked_at
                .map(|committed_at| DateTime::<Utc>::from_naive_utc_and_offset(committed_at, Utc)),
            verifier_submit_at: details
                .offchain_verifier_submit_at
                .map(|committed_at| DateTime::<Utc>::from_naive_utc_and_offset(committed_at, Utc)),
        };

        api::proof_offchain_verification::L1BatchDetailsWithOffchainVerification {
            base,
            number: L1BatchNumber(details.number as u32),
            offchain_verification,
        }
    }
}
