use zksync_types::L1BatchNumber;
use strum::{Display, EnumString};

use crate::{time_utils::pg_interval_from_duration, SqlxError, StorageProcessor};

#[derive(Debug, EnumString, Display)]
enum ProofVerificationStatus {
    #[strum(serialize = "ready_to_be_verified")]
    ReadyToBeVerified,
    #[strum(serialize = "picked_by_offchain_verifier")]
    PickedByOffChainVerifier,
    #[strum(serialize = "offchain_verified")]
    OffChainVerified,
}

#[derive(Debug)]
pub struct ProofVerificationDal<'a, 'c> {
    pub(crate) storage: &'a mut StorageProcessor<'c>,
}

impl ProofVerificationDal<'_, '_> {
    pub async fn insert_l1_batch_to_be_verified(
        &mut self,
        block_number: L1BatchNumber,
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            INSERT INTO
                proof_offchain_verification_details (l1_batch_number, status, created_at, updated_at)
            VALUES
                ($1, $2, NOW(), NOW())
            ON CONFLICT (l1_batch_number) DO NOTHING
            "#,
            block_number.0 as i64,
            ProofVerificationStatus::ReadyToBeVerified.to_string(),
        )
        .execute(self.storage.conn())
        .await?
        .rows_affected()
        .eq(&1)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn mark_l1_batch_as_verified(
        &mut self,
        block_number: L1BatchNumber,
    ) -> Result<(), SqlxError> {
        sqlx::query!(
            r#"
            UPDATE proof_offchain_verification_details
            SET
                status = $1,
                updated_at = NOW()
            WHERE
                l1_batch_number = $2
            "#,
            ProofVerificationStatus::OffChainVerified.to_string(),
            block_number.0 as i64,
        )
        .execute(self.storage.conn())
        .await?
        .rows_affected()
        .eq(&1)
        .then_some(())
        .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn get_last_l1_batch_verified(&mut self) -> Option<L1BatchNumber> {
        let row = sqlx::query!(
            r#"
            SELECT
                MAX(l1_batch_number) AS "number"
            FROM
                proof_offchain_verification_details
            WHERE
                status = $1
            "#,
            ProofVerificationStatus::OffChainVerified.to_string(),
        )
        .instrument("get_last_l1_batch_verified")
        .report_latency()
        .fetch_one(self.storage)
        .await?;

        Ok(row.number.map(|num| L1BatchNumber(num as u32)))
    }
}