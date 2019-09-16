# DaQiao

启动双节点

./target/debug/daqiao --chain local \
--base-path /tmp/alice \
--alice \
--node-key 0000000000000000000000000000000000000000000000000000000000000001 \
--telemetry-url ws://telemetry.polkadot.io:1024 \
--validator \
--eth-chain ropsten \
--eth-base-path ${DATA_PATH_0}

./target/debug/daqiao --chain local \
--base-path /tmp/bob \
--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/QmRpheLN4JWdAnY7HGJfWFNbfkQCb6tFf4vvA6hgjMZKrR \
--bob \
--telemetry-url ws://telemetry.polkadot.io:1024 \
--validator \
--eth-chain ropsten \
--eth-base-path ${DATA_PATH_1}
