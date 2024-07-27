// use crate::api::deposit::{transfer_from_ckbtc , transfer_from_cketh};
use crate::constants::asset_address::*;
use crate::utils::types::*;
use candid::{Nat, Principal};
use ic_cdk::call;
use icrc_ledger_types::icrc1::transfer::BlockIndex;

// Function to handle deposits
// #[ic_cdk_macros::update]
// pub async fn deposit_ckbtc(amount: u64) -> Result<Nat, String> {
//     let ledger_canister_id =
//         Principal::from_text(CKBTC_LEDGER_ADDRESS).map_err(|e| e.to_string())?;

//     ic_cdk::println!("ckbtc canister principal {}", ledger_canister_id);
//     let user_principal = ic_cdk::api::caller();
//     let platform_principal =
//         Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").map_err(|e| e.to_string())?;
//     ic_cdk::println!("platform canister principal {}", platform_principal);

//     let amount_nat = Nat::from(amount);
//     transfer_from_ckbtc(
//         ledger_canister_id,
//         user_principal,
//         platform_principal,
//         amount_nat,
//     )
//     .await
// }

// the function above is just an sample function, deposit function will use validation logic, reserve logic and other checks according to aave
pub async fn transfer_from_ckbtc(
    // ledger_canister_id: Principal,
    to: Principal,
    from: Principal,
    amount: u64,
) -> Result<BlockIndex, String> {
    let args = TransferFromArgs {
        to: TransferAccount {
            owner: to,
            subaccount: None,
        },
        fee: None,
        spender_subaccount: None,
        from: TransferAccount {
            owner: from,
            subaccount: None,
        },
        memo: None,
        created_at_time: None,
        amount: amount.into(),
    };
    // let (result,): (TransferFromResult,) = call(ledger_canister_id, "icrc2_transfer_from", (args,))
    //     .await
    //     .map_err(|e| e.1)?;
    ic_cdk::call::<(TransferFromArgs,), (Result<BlockIndex, TransferFromError>,)>(
        Principal::from_text("mxzaz-hqaaa-aaaar-qaada-cai")
            .expect("Could not decode the principal."),
        "icrc2_transfer_from",
        (args,),
    )
    .await
    .map_err(|e| format!("failed to call ledger: {:?}", e))?
    .0
    .map_err(|e| format!("ledger transfer error {:?}", e))

    // match result {
    //     TransferFromResult::Ok(balance) => Ok(balance),
    //     TransferFromResult::Err(err) => Err(format!("Error while transferring {:?}", err)),
    // }
}

#[ic_cdk_macros::update]
async fn make_payment(tokens: u64, to: Principal, from: Principal) -> String {
    // add check for admin
    let response = transfer_from_ckbtc(to, from, tokens).await;
    ic_cdk::println!("response is {:?}", response);
    // response
    format!("response is {:?}", response)
}
