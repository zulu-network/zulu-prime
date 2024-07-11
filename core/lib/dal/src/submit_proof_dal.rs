use zksync_types::{bitvm_proof::BitvmProof, L1BatchNumber, H256};

use crate::StorageProcessor;

#[derive(Debug)]
pub struct SubmitProofDal<'a, 'c> {
    pub storage: &'a mut StorageProcessor<'c>,
}

impl SubmitProofDal<'_, '_> {
    pub async fn insert_proof_submition(&mut self, bp: BitvmProof) {
        sqlx::query!(
            "INSERT INTO submit_proof (
                l1_batch_number, proof, vk, public_input, \
                proof_id, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, now(), now())",
            bp.number.0 as i64,
            bp.proof,
            bp.vk,
            bp.public_input,
            bp.proof_id.as_bytes(),
        )
        .execute(self.storage.conn())
        .await
        .unwrap();
    }

    pub async fn get_proof_id_by_batch_number(
        &mut self,
        l1_batch_number: L1BatchNumber,
    ) -> Option<String> {
        let row = sqlx::query!(
            "SELECT proof_id FROM submit_proof WHERE l1_batch_number = $1",
            l1_batch_number.0 as i64
        )
        .fetch_one(self.storage.conn())
        .await;

        if let Ok(row) = row {
            Some(hex::encode(row.proof_id))
        } else {
            None
        }
    }
}
