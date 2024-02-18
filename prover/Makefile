help: ## Display this help screen
	@grep -h \
		-E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

##  start provers
current_date := $(shell date +%Y-%m-%d-%H-%M-%S)
prover_setup_log_file := "../logs/prover_setup_$(current_date).log"
prover_gateway_log_file := "../logs/prover_gateway_$(current_date).log"
prover_witness_log_file := "../logs/prover_witness_$(current_date).log"
prover_fri_log_file := "../logs/prover_fri_$(current_date).log"
prover_agg_log_file := "../logs/prover_agg_$(current_date).log"

start_prover_gateway: ## 4. start gateway: Run prover gateway to fetch blocks to be proven from server:
	@nohup zk f cargo run --release --bin zksync_prover_fri_gateway > $(prover_gateway_log_file) 2>&1 &
	@echo "output logs to $(prover_gateway_log_file)"

start_prover_witness: ## start witness
	@nohup ./run_witness_generator.sh > $(prover_witness_log_file) 2>&1 &
	@echo "output logs to $(prover_witness_log_file)"


start_prover_setup: ## prover setup
	@nohup ./setup.sh > $(prover_setup_log_file) 2>&1 &
	@echo "output logs to $(prover_setup_log_file)"

start_prover_fri: ## start stark(fri) prover
	@nohup zk f cargo run --release --bin zksync_prover_fri > $(prover_fri_log_file) 2>&1 &
	@echo "output logs to $(prover_fri_log_file)"

start_prover_agg: ## start agg(snark) prover
	@nohup zk f cargo run --release --bin zksync_proof_fri_compressor > $(prover_agg_log_file) 2>&1 &
	@echo "output logs to $(prover_agg_log_file)"


.PHONY: clippy fmt test