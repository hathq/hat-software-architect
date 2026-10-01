# Software Architect HAT

Independent HATHQ HAT repository. It owns the vocabulary, context plan, exact-term reducer and procedure for `review-architecture-boundary`. It contains no credentials and grants no authority. Consumers load the immutable package and invoke its declared worker interface.

The reducer accepts only canonical vocabulary IDs, rejects revision conflicts and returns `vocabulary-term-unknown` for every unrecognized value. Unknown input is never guessed or completed.

The signed package declares that GitHub issue and pull-request operations are supplied by
`hat-github-operator`. This is topology metadata only; it grants no GitHub authority and does
not claim that a worker or binding is available.

`authoring/authoring-kit-v1.json` is a source-candidate, digest-pinned Codex
authoring kit for creating a separate HAT repository or improving one exact HAT
package. It includes closed request schema, prompt templates, repository
instructions and a release checklist. It is not runtime-discovered, does not
upload packages and cannot submit a request implicitly. The authoring action is
not advertised in `hat.package.json` until the shared request contract and an
authenticated worker are released.
