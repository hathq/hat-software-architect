use super::{INPUT_SCHEMA, ReviewInput, SemanticBinding};
use crate::document::valid_digest;
use std::collections::BTreeSet;

pub(super) fn input(
    input: &ReviewInput,
    revision: u64,
    selected: &SemanticBinding,
) -> Result<(), &'static str> {
    if input.schema != INPUT_SCHEMA
        || input.revision != revision
        || revision > 9_007_199_254_740_990
    {
        return Err("schema-or-revision-conflict");
    }
    if !crate::valid_subject_reference(&input.scope_ref)
        || input.scope_ref.len() > 96
        || input.claims.is_empty()
        || input.claims.len() > 64
    {
        return Err("scope-or-claim-limit");
    }
    if input.semantic != *selected
        || selected.module_sha256
            != "6736c2fc5fc6819f2baa9b403362b1b6873600f0abb96ae37ae2b10f723116da"
        || selected.term_id != "737c04107b2d09fe17a865a3c9b4718f"
        || !selected
            .definition_revision
            .strip_prefix("definitions-")
            .is_some_and(valid_digest)
    {
        return Err("semantic-selection-conflict");
    }
    let mut claims = BTreeSet::new();
    for claim in &input.claims {
        if !crate::valid_subject_reference(&claim.subject_ref)
            || claim.evidence.len() > 8
            || !claims.insert((&claim.subject_ref, claim.stage))
        {
            return Err("invalid-or-duplicate-claim");
        }
        let mut refs = BTreeSet::new();
        for evidence in &claim.evidence {
            if !crate::valid_subject_reference(&evidence.reference)
                || evidence.subject_ref != claim.subject_ref
                || evidence.scope_ref != input.scope_ref
                || evidence.revision > input.revision
                || !valid_digest(&evidence.digest_sha256)
                || !refs.insert(&evidence.reference)
            {
                return Err("invalid-or-conflicting-evidence-reference");
            }
        }
    }
    Ok(())
}
