use super::*;
#[test]
fn current_v1_projection_keeps_wire_schema_and_deduplication() {
    let input: WorkerInput = serde_json::from_value(serde_json::json!({
        "schema": INPUT_SCHEMA, "scope_ref": "scope:existing", "revision": 4,
        "subject_refs": ["subject:b", "subject:a", "subject:b"]
    }))
    .unwrap();
    validate_input(&input, 4).unwrap();
    assert_eq!(
        serde_json::to_value(project(input).unwrap()).unwrap(),
        serde_json::json!({
            "schema": OUTPUT_SCHEMA, "scope_ref": "scope:existing", "revision": 5,
            "subject_refs": ["subject:a", "subject:b"]
        })
    );
}
#[test]
fn v1_rejects_v2_fields_and_revision_conflicts() {
    let input: WorkerInput = serde_json::from_value(serde_json::json!({
        "schema": INPUT_SCHEMA, "scope_ref": "scope:existing", "revision": 4,
        "subject_refs": ["subject:a"]
    }))
    .unwrap();
    assert!(validate_input(&input, 3).is_err());
    assert!(
        serde_json::from_value::<WorkerInput>(serde_json::json!({
            "schema": INPUT_SCHEMA, "scope_ref": "scope:existing", "revision": 4,
            "subject_refs": [], "claims": []
        }))
        .is_err()
    );
}
