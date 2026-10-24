//! Closed HAT-owned evidence review contract; no semantic compiler or authority.
use serde::{Deserialize, Serialize};

pub const INPUT_SCHEMA: &str =
    "hathq://hat-software-architect/review-architecture-boundary-input/v2";
pub const OUTPUT_SCHEMA: &str =
    "hathq://hat-software-architect/review-architecture-boundary-output/v2";

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticBinding {
    pub module_sha256: String,
    pub term_id: String,
    pub definition_revision: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stage {
    Implementation,
    Test,
    Publication,
    Deployment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Outcome {
    Verified,
    Failed,
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub scope_ref: String,
    pub reference: String,
    pub digest_sha256: String,
    pub subject_ref: String,
    pub stage: Stage,
    pub revision: u64,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Claim {
    pub subject_ref: String,
    pub stage: Stage,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewInput {
    pub schema: String,
    pub scope_ref: String,
    pub revision: u64,
    pub semantic: SemanticBinding,
    pub claims: Vec<Claim>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FindingKind {
    Supported,
    Missing,
    Stale,
    Failed,
    Conflicting,
    Unresolved,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub subject_ref: String,
    pub stage: Stage,
    pub kind: FindingKind,
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewOutput {
    pub schema: String,
    pub scope_ref: String,
    pub revision: u64,
    pub semantic: SemanticBinding,
    pub authority_granted: bool,
    pub findings: Vec<Finding>,
}
