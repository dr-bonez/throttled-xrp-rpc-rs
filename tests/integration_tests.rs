use serde_json::json;
use serde_json::value::Value;
use throttled_xrp_rpc::{Account, AccountInfoParams, AccountTxParams, LedgerIndex, XRPClient};

// Used https://s1.ripple.com:51234/
const URL: &'static str = "http://10.68.2.90:5005/";
#[test]
fn account_info_tests() {
    let bitpay_account_id: Account = "r9HwsqBnAUN4nF6nDqxd4sgP8DrDnDcZP3".parse().unwrap();
    let client = reqwest::Client::new();

    let account_params = AccountInfoParams {
        account: &bitpay_account_id,
        strict: true,
        ledger_index: LedgerIndex::StrValue {
            ledger_index: "current".into(),
        },
        queue: true,
    };
    let raw_response = client
        .post(URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response =
        XRPClient::new(URL.into(), None, None, 0, 0, 0).account_info(account_params.clone());
    assert!(
        account_response.is_ok(),
        "Getting back an error {:?} from the server given the input {:?}, raw was {:?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );

    let account_params = AccountInfoParams {
        account: &bitpay_account_id,
        strict: false,
        ledger_index: LedgerIndex::Number {
            ledger_index: 48694757.into(),
        },
        queue: false,
    };
    let raw_response = client
        .post(URL.clone())
        .json(&json!({
        "method": "account_info",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_response =
        XRPClient::new(URL.into(), None, None, 0, 0, 0).account_info(account_params.clone());
    assert!(
        account_response.is_ok(),
        "Getting back an error {:?} from the server given the input {:?}, raw was {:?}",
        account_response,
        serde_json::to_string(&account_params),
        raw_response
    );
}

#[test]
fn account_tx_test() {
    let bitpay_account_id: Account = "r9HwsqBnAUN4nF6nDqxd4sgP8DrDnDcZP3".parse().unwrap();
    let client = reqwest::Client::new();

    let account_params = AccountTxParams {
        account: &bitpay_account_id,
        binary: Some(false),
        forward: Some(false),
        ledger_hash: None,
        ledger_index: Some(LedgerIndex::StrValue {
            ledger_index: "current".into(),
        }),
        ledger_index_max: Some(-1),
        ledger_index_min: Some(-1),
        limit: Some(2),
    };
    let raw_response = client
        .post(URL.clone())
        .json(&json!({
        "method": "account_tx",
        "params": [
            account_params
        ]
        }))
        .send()
        .unwrap()
        .json::<Value>();
    let account_tx =
        XRPClient::new(URL.into(), None, None, 0, 0, 0).account_tx(account_params.clone());
    assert!(
        account_tx.is_ok(),
        "Getting back an error {:?} from the server given the input {:?}, raw was {:?}",
        account_tx,
        serde_json::to_string(&account_params),
        raw_response
    );
}
