//! Evidence consistency proposals, separate from semantic meaning and execution.
mod model;
mod validate;
pub use model::*;

/// Reviews only caller-supplied observations at the selected scope revision.
///
/// # Errors
/// Rejects malformed, duplicate, cross-subject or mismatched native references.
/// A supported claim is consistency with supplied observations, not their trust.
pub fn review(
    input: ReviewInput,
    expected_revision: u64,
    selected: &SemanticBinding,
) -> Result<ReviewOutput, &'static str> {
    validate::input(&input, expected_revision, selected)?;
    let findings = input
        .claims
        .into_iter()
        .map(|claim| {
            let matching: Vec<_> = claim
                .evidence
                .iter()
                .filter(|e| e.stage == claim.stage)
                .collect();
            let current: Vec<_> = matching
                .iter()
                .copied()
                .filter(|e| e.revision == input.revision)
                .collect();
            let has = |outcome| current.iter().any(|e| e.outcome == outcome);
            let kind = if matching.is_empty() {
                FindingKind::Missing
            } else if current.is_empty() {
                FindingKind::Stale
            } else if has(Outcome::Verified) && has(Outcome::Failed) {
                FindingKind::Conflicting
            } else if has(Outcome::Failed) {
                FindingKind::Failed
            } else if has(Outcome::Unknown) {
                FindingKind::Unresolved
            } else {
                FindingKind::Supported
            };
            Finding {
                subject_ref: claim.subject_ref,
                stage: claim.stage,
                kind,
                evidence_refs: matching.iter().map(|e| e.reference.clone()).collect(),
            }
        })
        .collect();
    Ok(ReviewOutput {
        schema: OUTPUT_SCHEMA.into(),
        scope_ref: input.scope_ref,
        revision: input.revision.checked_add(1).ok_or("revision-overflow")?,
        semantic: input.semantic,
        authority_granted: false,
        findings,
    })
}
