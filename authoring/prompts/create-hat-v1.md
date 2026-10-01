# Create a HAT

Implement a new HAT in its own HATHQ repository from the supplied typed
development brief.

Required behavior:

1. Read the repository `AGENTS.md` and the released HAT specification before
   editing.
2. Treat every unresolved field as unresolved. Ask a focused question or emit
   the declared unresolved result; never guess a repository, provider, schema,
   authority, user fact or destination.
3. Define one closed vocabulary and action contract. Reject unknown fields,
   aliases, implicit authority, implicit model escalation and dynamic code.
4. Keep the implementation outside Hatter. Hatter may install, bind, invoke and
   project status only.
5. Add reducer, worker boundary, conformance tests, negative substitution tests,
   restart/idempotency tests and bounded resource/shutdown tests before claiming
   the action is available.
6. Use external provider references and digests instead of copying source
   bodies or credentials.
7. Run only focused validation unless the brief explicitly authorizes a broader
   build.

Deliver a concise change summary, exact tests run, unresolved items and release
gates. Do not publish, upload, create a remote repository or contact a maintainer
without a separate explicit action.
