# HAT repository instructions

- This repository owns one HAT implementation; Hatter only installs, binds,
  invokes and projects it.
- Read the released `hat-specifications` contract before changing package,
  vocabulary, action, reducer, worker, location or receipt schemas.
- Unknown or ambiguous input stays unresolved. Never guess missing fields or
  search merely to complete them.
- Keep schemas closed, identifiers canonical and authority explicit.
- Do not embed credentials, source bodies, provider SDKs, dynamic code, MCP,
  browser upload or Hatter-local execution fallbacks.
- A declared action is unavailable until a separately released worker, signed
  execution location and authenticated transport pass conformance.
- Add focused negative tests before implementation and verify restart,
  idempotency, substitution, capacity and shutdown behavior.
- Do not publish, push, contact third parties or run a full ecosystem build
  without explicit authorization.
