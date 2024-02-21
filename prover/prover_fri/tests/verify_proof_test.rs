use std::fs;
use std::sync::Arc;
use zkevm_test_harness::prover_utils::{verify_base_layer_proof, verify_recursion_layer_proof};
use zksync_prover_fri_types::{
    circuit_definitions::boojum::cs::implementations::{pow::NoPow, proof::Proof},
    keys::FriCircuitKey,
    CircuitWrapper, CircuitWrapper, ProverJob, ProverServiceDataKey,
};
use zksync_types::{
    basic_fri_types::{AggregationRound, CircuitIdRoundTuple},
    L1BatchNumber,
};

use anyhow::Context as _;
use zksync_config::configs::{object_store::ObjectStoreMode, FriProverConfig, ObjectStoreConfig};
use zksync_object_store::{bincode, ObjectStoreFactory};
use zksync_prover_fri::prover_job_processor::Prover;

#[tokio::test]
async fn test_load_and_verify_schedule_proof() {
    // 1. load proof.
    //    Proof name: proof_{sequence_number}.bin
    let expected_proof_id = 107;
    let object_store_config = ObjectStoreConfig {
        mode: ObjectStoreMode::FileBacked {
            file_backed_base_path: "./tests/data/".to_owned(),
        },
        max_retries: 5,
    };
    let object_store = ObjectStoreFactory::new(object_store_config)
        .create_store()
        .await;
    let proof = object_store
        .get(expected_proof_id)
        .await
        .expect("missing expected proof");
    println!("scheduler_proof: {:?}", proof);

    // 2. load circuit.
    //    circuit name:
    //      {block_number}_{sequence_number}_{circuit_id}_{AggregationRound}_{depth}.bin
    let block_number = L1BatchNumber(1);
    let sequence_number = 0;
    let circuit_id = 1;
    let aggregation_round = AggregationRound::Scheduler;
    let blob_key = FriCircuitKey {
        block_number,
        circuit_id,
        sequence_number,
        depth: 0,
        aggregation_round,
    };
    let circuit_wrapper = object_store
        .get(blob_key)
        .await
        .context("circuit missing")
        .unwrap();
    let circuit = match &circuit_wrapper {
        CircuitWrapper::Base(base) => base.clone(),
        CircuitWrapper::Recursive(_) => anyhow::bail!("Expected base layer circuit"),
    };

    // 3. load scheduler_vk
    let vk = get_recursive_layer_vk_for_circuit_type(1).unwrap();

    // 3. verify the proof.
    let (is_valid, circuit_id) = match circuit_wrapper {
        CircuitWrapper::Base(base_circuit) => (
            verify_base_layer_proof::<NoPow>(&base_circuit, proof, vk),
            base_circuit.numeric_circuit_type(),
        ),
        CircuitWrapper::Recursive(recursive_circuit) => (
            verify_recursion_layer_proof::<NoPow>(&recursive_circuit, proof, vk),
            recursive_circuit.numeric_circuit_type(),
        ),
    };
}
