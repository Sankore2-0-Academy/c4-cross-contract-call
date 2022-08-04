#!/bin/bash 

SIGNER=<signer-account-id>

source ./scripts/setting.conf

# Test XCC calls
near call $SUB_ACCOUNT call_accounts_contract '{"msg": "Random message from xcc"}' --accountId $SIGNER


# SIGNER WALLET ===> XCC CONTRACT ===> ACCOUNTS CONTRACT -> {signer: SIGNER WALLET, predecessor: XCC WALLET}