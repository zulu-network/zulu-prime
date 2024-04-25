FROM zulunetwork/zk-environment:latest2.0

WORKDIR /usr/src/zksync
COPY . .

RUN zk

# Build all the required TS packages

# Build `zk` tool
RUN cd /infrastructure/zk && yarn && yarn build && cd /
# Build `local-setup-preparation` tool
RUN cd /infrastructure/local-setup-preparation && yarn && cd /
# Build L1 contracts package (contracts themselves should be already built)
RUN cd /contracts/l1-contracts && yarn && cd /
# Same for L2 contracts
RUN cd /contracts/l2-contracts && yarn && cd /

ENTRYPOINT [ "zk" ]