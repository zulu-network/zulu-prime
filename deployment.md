# Deploy

## Local Deploy
Build and deploy with the source code

### Setup
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# NVM
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
# All necessary stuff
sudo apt-get install build-essential pkg-config cmake clang lldb lld libssl-dev postgresql docker-compose
sudo snap install solc

## TO install docker's docker-compose-plugin
sudo apt-get update 
sudo apt-get install docker-compose-plugin # meet error's to see Unbuntu Env

# Docker    
## sudo usermod -aG docker YOUR_USER # Add current user to docker group
sudo usermod -aG docker ${USER}


## You might need to re-connect (due to usermod change).

# Node & yarn
nvm install node
npm install -g yarn
yarn set version 1.22.19

# SQL tools
cargo install sqlx-cli --version 0.5.13
# Stop default postgres (as we'll use the docker one)
sudo systemctl stop postgresql
# Start docker.
sudo systemctl start docker


# Make sure in the root of the repository
export ZKSYNC_HOME=$(pwd)

# Or add this on /etc/profile
export ZKSYNC_HOME=/path/to/zksync/repo/you/cloned 
export PATH=$ZKSYNC_HOME/bin:$PATH

```

### Run locally
### Run Sequencer
```bash
# 1. start geth,postgres
zk init

# 2.start sequencer
make start_server
```

### Run Prover
```bash
cd prover 
# 0. setup first: generate related key. It's might takes 2 hours.
make start_prover_setup

# 1. run gateway
make start_prover_gateway

# 2. run witness generator
make start_prover_witness

# 3. run start_prover_fri
make start_prover_fri

# 4. run prover_agg
make start_prover_agg

```



## Decker Deploy


