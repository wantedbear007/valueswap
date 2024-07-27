# use a specific identity 
dfx identity use yyy

dfx canister create swap
dfx build swap

dfx deploy valueswap_backend




MINTER=$(dfx --identity default identity get-principal)
DEFAULT=$(dfx --identity default identity get-principal)
RECIEVER=$(dfx --identity reciever identity get-principal)
TOKEN_SYMBOL=TOK
TOKEN_NAME="VALSWAP"
TRANSFER_FEE=1000
PRE_MINTED_TOKENS=100000000000
echo $RECIEVER

# dfx canister create --all

dfx deploy icrc1_ledger_canister --argument "(variant {Init = 
record {
     token_symbol = \"${TOKEN_SYMBOL}\";
     token_name = \"${TOKEN_NAME}\";
     minting_account = record { owner = principal \"${MINTER}\" };
     transfer_fee = ${TRANSFER_FEE};
     metadata = vec {};
     initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; ${PRE_MINTED_TOKENS}; }; };
     archive_options = record {
         num_blocks_to_archive = 100;
         trigger_threshold = 100;
         controller_id = principal \"${DEFAULT}\";
     };
     feature_flags = opt record {icrc2 = true;};
 }
})"