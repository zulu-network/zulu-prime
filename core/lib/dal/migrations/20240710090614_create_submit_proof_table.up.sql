CREATE TABLE IF NOT EXISTS submit_proof
(
    l1_batch_number BIGINT NOT NULL PRIMARY KEY REFERENCES l1_batches (number) ON DELETE CASCADE,
    proof TEXT NOT NULL,
    vk TEXT NOT NULL,
    public_input TEXT NOT NULL,
    proof_id BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);