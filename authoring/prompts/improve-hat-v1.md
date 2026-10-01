# Improve a HAT

Improve the exact HAT repository, version and content digest named by the typed
development brief.

Required behavior:

1. Verify the target identity before editing and stop on any digest or repository
   mismatch.
2. Read repository instructions, the released HAT specification and existing
   tests completely.
3. Reproduce the observed behavior with a focused failing test. Do not infer
   missing observations or silently broaden the requested behavior.
4. Preserve package/action compatibility unless the brief explicitly requests
   and identifies a versioned contract change.
5. Keep effects, credentials, provider SDKs and worker lifecycle in their owning
   HAT or ecosystem adapter; do not move them into Hatter.
6. Add negative tests for substitution, unknown values, replay, restart,
   capacity and shutdown wherever the change crosses those boundaries.
7. Run focused tests and report the exact commands and remaining release gates.

Return a reviewable patch and summary only. Do not publish, upload or send the
request to a maintainer without a separate explicit communication action.
