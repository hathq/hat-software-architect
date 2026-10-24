mod support;
use hat_software_architect::review::{FindingKind, ReviewOutput};
use std::{fs, process::Command};
#[test]
fn actual_offline_cli_returns_proposals_without_hatter_or_credentials() {
    let temp = support::Scratch::new();
    let input = temp.0.join("input.json");
    let selection = temp.0.join("selection.json");
    let output = temp.0.join("output.json");
    fs::write(&input, include_bytes!("fixtures/review-v2.json")).unwrap();
    fs::write(&selection, include_bytes!("fixtures/selected-binding.json")).unwrap();
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_hat-software-architect-worker"))
            .env_clear()
            .args(["review-v2"])
            .arg(&input)
            .arg(&selection)
            .arg(&output)
            .output()
            .unwrap()
    };
    let result = run();
    assert!(result.status.success(), "{:?}", result);
    let bytes = fs::read(&output).unwrap();
    let value: ReviewOutput = serde_json::from_slice(&bytes).unwrap();
    assert!(!value.authority_granted);
    assert_eq!(value.findings.len(), 7);
    assert_eq!(value.findings[1].kind, FindingKind::Missing);
    assert!(run().status.success()); // Identical output is an inert readback.
    fs::write(&output, b"changed").unwrap();
    assert!(!run().status.success()); // Never overwrites a different proposal.
    assert_eq!(fs::read(&output).unwrap(), b"changed");
    fs::write(&selection, b"{\"grant\":true}").unwrap();
    assert!(!run().status.success());
}
