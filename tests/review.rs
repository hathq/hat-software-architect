use hat_software_architect::review::*;
fn selected() -> SemanticBinding {
    serde_json::from_str(include_str!("fixtures/selected-binding.json")).unwrap()
}
fn input() -> ReviewInput {
    serde_json::from_str(include_str!("fixtures/review-v2.json")).unwrap()
}

#[test]
fn reviews_actual_stage_specific_observations_and_never_grants_authority() {
    let result = review(input(), 4, &selected()).unwrap();
    assert!(!result.authority_granted);
    assert_eq!(result.revision, 5);
    assert_eq!(result.semantic, selected());
    assert_eq!(
        result.findings.iter().map(|f| f.kind).collect::<Vec<_>>(),
        vec![
            FindingKind::Supported,
            FindingKind::Missing,
            FindingKind::Stale,
            FindingKind::Failed,
            FindingKind::Conflicting,
            FindingKind::Unresolved,
            FindingKind::Missing,
        ]
    );
}
#[test]
fn rejects_revision_unknown_semantics_and_cross_scope_subject_evidence() {
    assert!(review(input(), 3, &selected()).is_err());
    let mut changed = selected();
    changed.definition_revision.push('a');
    assert!(review(input(), 4, &changed).is_err());
    let mut changed = input();
    changed.semantic.term_id = "0".repeat(32);
    assert!(review(changed, 4, &selected()).is_err());
    let mut changed = input();
    changed.claims[0].evidence[0].subject_ref = "subject:other".into();
    assert!(review(changed, 4, &selected()).is_err());
    let mut changed = input();
    changed.claims[0].evidence[0].scope_ref = "scope:other".into();
    assert!(review(changed, 4, &selected()).is_err());
}
#[test]
fn rejects_duplicate_future_malformed_and_excessive_claims_or_evidence() {
    let mut value = input();
    value.claims.push(value.claims[0].clone());
    assert!(review(value, 4, &selected()).is_err());
    for mutate in [
        |v: &mut ReviewInput| v.claims[0].evidence[0].revision = 5,
        |v: &mut ReviewInput| v.claims[0].evidence[0].digest_sha256 = "../outside".into(),
        |v: &mut ReviewInput| {
            let e = v.claims[0].evidence[0].clone();
            v.claims[0].evidence.push(e);
        },
        |v: &mut ReviewInput| v.claims[0].evidence = vec![v.claims[0].evidence[0].clone(); 9],
        |v: &mut ReviewInput| v.claims = vec![v.claims[0].clone(); 65],
        |v: &mut ReviewInput| v.claims.clear(),
    ] {
        let mut value = input();
        mutate(&mut value);
        assert!(review(value, 4, &selected()).is_err());
    }
    let mut value = input();
    value.revision = u64::MAX;
    assert!(review(value, u64::MAX, &selected()).is_err());
}
#[test]
fn wire_is_closed_and_preserves_missing_evidence() {
    let mut value: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/review-v2.json")).unwrap();
    value["grant"] = true.into();
    assert!(serde_json::from_value::<ReviewInput>(value).is_err());
    let mut value = input();
    value.claims[0].evidence.clear();
    assert_eq!(
        review(value, 4, &selected()).unwrap().findings[0].kind,
        FindingKind::Missing
    );
}
