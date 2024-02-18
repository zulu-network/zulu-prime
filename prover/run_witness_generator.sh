#!/usr/bin/env bash
API_PROMETHEUS_LISTENER_PORT=3116 zk f cargo run --release --bin zksync_witness_generator -- --all_rounds