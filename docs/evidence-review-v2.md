# Evidence review v2

This HAT owns a narrowly defined judgment: a claim about implementation, testing,
publication or deployment needs observations for **that same stage**, subject,
scope and revision. The criterion is Coela's evidence/uncertainty principle, not
an architecture scoring model or a second language compiler.

The versioned input/output JSON Schemas define the domain wire format. v1 package
operations, catalog, reducer, grants and active worker route stay unchanged.
The new `review-v2 INPUT SELECTED_BINDING OUTPUT` command evaluates an explicit
offline proposal; it never registers/claims/completes a Hatter invocation. The
Rust library additionally requires the caller's expected scope revision. CLI
revision checking is internal consistency, not concurrent state admission.

The caller explicitly selects a binding produced by the native semantic consumer:
`module_sha256`, `term_id`, `definition_revision`. Input must match that selection.
The supported Coela source module digest and native term ID are pinned; a changed
meaning needs a reviewed mapping. Definition revision must be the exact selected
native revision. Parsing or matching these fields does not prove native compilation,
trust, factual truth or runtime adoption. No value supplied by the CLI is trusted
merely because it is well formed. The independent Engineering consumer obtains the
binding from an actual native compiler/adoption/reload acceptance receipt.

Canonical knowledge: Coela Engineering content 0.2.4, principle on evidence and
uncertainty; semantic source 0.1.0, `coela/engineering/evidence::ImplementationEvidence`.
Source references, license, raw digests and mappings are in `integrations/review/source.json`.
This repository does not copy the prose or native meaning definition.

Each finding is `supported`, `missing`, `stale`, `failed`, `conflicting` or
`unresolved`. `supported` means consistency with caller-supplied verified observations;
it is not proof that a provider test ran or an artifact was published. Evidence
trust/signatures and independent byte verification remain the caller's admission
responsibility. A supplied digest is an identity reference, not evidence verification.
A later-stage claim is never inferred from an earlier-stage observation. Current
verified+failed observations conflict; unknown observations prevent support. Old
observations cannot establish the current revision. Same-reference duplicates,
future revisions, cross-scope/subject observations and unsupported references fail.

Input has at most 64 distinct subject/stage claims, eight observations per claim,
and a 1 MiB raw-file budget. Revision stays in the interoperable JSON integer range.
Source/selection/output parents must be canonical directories controlled by the
caller. Final symlinks, FIFOs, devices and nonregular files are rejected using
no-follow/nonblocking opens. Handle/path identity, size and modification time are
checked around bounded reads. The v1 digest store shares this admission; invalid
hex/path references fail before access. Unsupported secure-file platforms fail
closed. Writes are exclusive, private and never replace differing output.

`integrations/review/lifecycle.json` uses the existing Coela lifecycle schema and
verifier at a fixed public revision. Actual CLI output is validated against both
closed domain schemas, then independently compared in full. Finding records project
all scalar fields; the ordered evidence-reference array is represented by SHA256
of its JSON bytes and is separately schema/full-output checked. Seven cases include
all six finding kinds plus a wrong-stage refusal. This is projection coverage,
not semantic equivalence or authority acceptance. The report's flags stay false.

Publication is owner-built HAT 0.11.0 (Cargo 0.11.0), through the existing reviewed
`hat-public` Environment. Its attestation does not replace a signed installation
catalog. Before Issue #20 is complete, independently accept the published bytes
in Engineering, then accept real Hatter subject/scope/grant/expiry/revocation when
its approved environment is available. No held host/isolation test, frozen HatSpec
0.10.0 or initial-definition decision is changed by this additive source slice.

Reader reuse assessment: the existing MCP, custody, ecosystem and CLI readers are
private modules tied to their owners. Importing their runtimes would add unrelated
protocol/custody dependencies. This small OS-file adapter is shared by this HAT's
v1/v2 paths, uses standard handles and existing SHA2, and creates no new service,
parser, transport, cryptography implementation or generic framework. Other HAT
workers have the old post-allocation read pattern; migrating those owners requires
their own real consumer tests rather than copying this feature across blindly.
