#!/bin/bash 

SIGNER=<signer-account-id>

source ./scripts/setting.conf

# Test Get Info
near call $SUB_ACCOUNT get_address_info '{"msg": "Random message"}' --accountId $SIGNER