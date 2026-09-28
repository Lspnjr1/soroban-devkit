//! CLI regression coverage for offline fee-bump wrapping and follow-up validation.

use assert_cmd::Command;
use sdkt_xdr::{build_invoke_transaction, InvokeTransactionParams};

const SOURCE: &str = "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF";
const CONTRACT: &str = "CAAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQCAIBAEAQC526";

#[test]
fn tx_wrap_produces_an_envelope_accepted_by_validate() {
    let inner = build_invoke_transaction(&InvokeTransactionParams {
        source_account: SOURCE.into(),
        sequence: 7,
        fee: 100,
        contract_id: CONTRACT.into(),
        function: "hello".into(),
        args: vec![],
    })
    .unwrap();

    let output = Command::cargo_bin("sdkt")
        .unwrap()
        .args([
            "tx",
            "wrap",
            "--envelope",
            &inner,
            "--fee-source",
            SOURCE,
            "--fee",
            "200",
            "--format",
            "json",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["inner_fee"], 100);
    assert_eq!(result["fee_bump_total"], 200);
    let wrapped = result["envelope"].as_str().expect("encoded envelope");

    Command::cargo_bin("sdkt")
        .unwrap()
        .args(["tx", "validate", "--envelope", wrapped])
        .assert()
        .success();
}
