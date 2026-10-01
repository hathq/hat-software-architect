# HAT repository checklist

- [ ] Separate HATHQ repository and canonical `repository_id` / `package_id`
- [ ] Closed package, manifest, vocabulary and operation schemas
- [ ] Exact context selectors and unresolved-value policy
- [ ] Deterministic reducer with revision and conflict checks
- [ ] Bounded input, output, event and projection schemas
- [ ] Worker implementation outside Hatter
- [ ] Exact package/action/digest binding at the worker boundary
- [ ] Signed location set and iHAT/Crowsi worker identity
- [ ] Registration, finite lease renewal and clean shutdown
- [ ] Provider/source references without copied bodies or credentials
- [ ] Signed result and federation receipt
- [ ] Positive, unknown-field, substitution and authority tests
- [ ] Restart, idempotency, ambiguity and capacity tests
- [ ] Light-reasoning conformance; explicit rationale if unavailable
- [ ] Immutable signed release and official-catalog entry
