PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="../../../wallet-owner.pem"
######################################################################

WEGLD_TOKEN_ID="WEGLD-d7c6bb"
WEGLD_TOKEN_ID_HEX="0x$(echo -n ${WEGLD_TOKEN_ID} | xxd -p -u | tr -d '\n')"

USDC_TOKEN_ID="USDC-8d4068"
USDC_TOKEN_ID_HEX="0x$(echo -n ${USDC_TOKEN_ID} | xxd -p -u | tr -d '\n')"

BUSD_TOKEN_ID="BUSD-632f7d"
BUSD_TOKEN_ID_HEX="0x$(echo -n ${BUSD_TOKEN_ID} | xxd -p -u | tr -d '\n')"

USDT_TOKEN_ID="USDT-188935"
USDT_TOKEN_ID_HEX="0x$(echo -n ${USDT_TOKEN_ID} | xxd -p -u | tr -d '\n')"

TREASURY_ADDRESS="erd1zgufzwktnmg7g4qj22qs3advtkvz6zw7xtpctezr4zhph5drd6qqcggynq"
TREASURY_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${TREASURY_ADDRESS})"

STAKING_REWARD_ADDRESS="erd1zgufzwktnmg7g4qj22qs3advtkvz6zw7xtpctezr4zhph5drd6qqcggynq"
STAKING_REWARD_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${STAKING_REWARD_ADDRESS})"

BURNER_ADDRESS="erd16f3ds56uls2jq6yf7sqm5yalsetngg4jz77tqqem2kdndhujgn8qct3cdg"
BURNER_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${BURNER_ADDRESS})"

UNWRAP_ADDRESS="erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh"
UNWRAP_ADDRESS_HEX="0x$(mxpy wallet bech32 --decode ${UNWRAP_ADDRESS})"

TOTAL_FEE_PERCENT=30
SPECIAL_FEE_PERCENT=1
STKAING_REWARD_PERCENT=9

REGISTERING_COST=2000000000000000000

deploy() {
    mxpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-devnet.interaction.json" \
    --metadata-payable \
    --gas-limit=200000000
    
    ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

ADDRESS=$(mxpy data load --key=address-devnet)

setConfig() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="setConfig" \
    --arguments ${WEGLD_TOKEN_ID_HEX} ${USDC_TOKEN_ID_HEX} ${BUSD_TOKEN_ID_HEX} ${USDT_TOKEN_ID_HEX} ${TOTAL_FEE_PERCENT} ${SPECIAL_FEE_PERCENT} ${STKAING_REWARD_PERCENT} ${TREASURY_ADDRESS_HEX} ${STAKING_REWARD_ADDRESS_HEX} ${BURNER_ADDRESS_HEX} ${UNWRAP_ADDRESS_HEX} ${REGISTERING_COST} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

ONE_TOKEN_ID="EGLDCPALP-3cc91e"
ONE_TOKEN_ID_HEX="0x$(echo -n ${ONE_TOKEN_ID} | xxd -p -u | tr -d '\n')"

createPair() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="createPair" \
    --arguments ${ONE_TOKEN_ID_HEX} ${WEGLD_TOKEN_ID_HEX} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

PAIR_ID=2
COST=50000000000000000

issueLpToken() {
    mxpy --verbose contract call ${ADDRESS} \
    --recall-nonce --pem=${WALLET} \
    --gas-limit=300000000 \
    --function="issueLpToken" \
    --arguments ${PAIR_ID} \
    --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --value=${COST}
}

UPGRADE_SC_ADDRESS="erd1qqqqqqqqqqqqqpgqeg8g9z4fxgsmja4c3c2lcc3xlr6f8kt7kqysjxk6gu"
UPGRADE_SC_ADDRESS_HEX="$(mxpy wallet bech32 --decode ${UPGRADE_SC_ADDRESS})"


upgrade() {
    mxpy --verbose contract upgrade ${UPGRADE_SC_ADDRESS_HEX} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=200000000
}