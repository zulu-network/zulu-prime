help: ## Display this help screen
	@grep -h \
		-E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

login_db: ## if need use pswd: notsecurepassword
	@psql -h localhost -U postgres

login_geth: ## enter geth(L1)'s docker bash
	@docker container exec -it zulu-prime-geth-1  geth attach http://localhost:8545

fund: ## faught for l1: eth.sendTransaction({from: personal.listAccounts[0], to: "0x618263CE921F7dd5F4f40C29f6c524Aaf97b9bbd", value: "7400000000000000000"})
	@#eth.sendTransaction({from: personal.listAccounts[0], to: "0x618263CE921F7dd5F4f40C29f6c524Aaf97b9bbd", value: "7400000000000000000"})

bridge: ## transfer from l1->l2. Key in: Private key of the sender: 0x5090c024edb3bdf4ce2ebc2da96bedee925d9d77d729687e5e2d56382cf0a5a6  Recipient address on L2: 0x618263CE921F7dd5F4f40C29f6c524Aaf97b9bbd
	@npx zksync-cli bridge deposit


##  start l1/l2 nodes
current_date := $(shell date +%Y-%m-%d-%H-%M-%S)
server_log_file := "logs/server_$(current_date).log"

start_common: ## start db/geth, after start, need `zk stack init`. or just `zk init`
	@docker-compose -f docker-compose-zkstack-common.uml -d postgres geth


init: ## clean and init the server
	@zk && zk clean --all && zk init

start_server: ## start sequencer.
	@nohup zk server --components=api,tree,eth,state_keeper,housekeeper,proof_data_handler,basic_witness_input_producer,commitment_generator >  $(server_log_file) 2>&1 &
	@echo "output logs to $(server_log_file)"

.PHONY: clippy fmt test
