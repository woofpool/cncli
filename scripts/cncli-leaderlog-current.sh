#!/bin/bash
export CARDANO_NODE_SOCKET_PATH=/home/cardano/cardano-my-node/db/socket

/usr/local/bin/cncli sync --host 127.0.0.1 --port 6000 --no-service
echo "WOOF"
SNAPSHOT=$(/usr/local/bin/cardano-cli query stake-snapshot --stake-pool-id c22942e1b855136643d1e6e5a75266fb891d87727a8cbf06acd17208 --mainnet)
POOL_STAKE=$(echo "$SNAPSHOT" | grep -oP '(?<=    "poolStakeSet": )\d+(?=,?)')
ACTIVE_STAKE=$(echo "$SNAPSHOT" | grep -oP '(?<=    "activeStakeSet": )\d+(?=,?)')
WOOF=`/usr/local/bin/cncli leaderlog --pool-id c22942e1b855136643d1e6e5a75266fb891d87727a8cbf06acd17208 --pool-vrf-skey /home/cardano/cardano-my-node/vrf.skey --byron-genesis /home/cardano/cardano-my-node/mainnet-byron-genesis.json --shelley-genesis /home/cardano/cardano-my-node/mainnet-shelley-genesis.json --pool-stake $POOL_STAKE --active-stake $ACTIVE_STAKE --ledger-set current`
echo $WOOF | jq .

EPOCH=`echo $WOOF | jq .epoch`
echo "\`Epoch $EPOCH\` 🧙🔮:"

SLOTS=`echo $WOOF | jq .epochSlots`
IDEAL=`echo $WOOF | jq .epochSlotsIdeal`
PERFORMANCE=`echo $WOOF | jq .maxPerformance`
echo "\`WOOF  - $SLOTS \`🎰\`,  $PERFORMANCE% \`🍀max, \`$IDEAL\` 🧱ideal"
