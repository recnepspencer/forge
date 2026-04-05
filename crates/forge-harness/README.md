# forge-harness

`forge-harness` is shared certification and parity infrastructure for Forge runtimes.

It is the crate that lets runtime libraries run the same scenario matrix across:

- execution profiles
- parity comparisons
- workflow certifications
- diagnostics captures

This is not meant to be the product-facing API for `forge-relational` or
`forge-signal`.

It is the shared harness those runtimes use for heavy test, certification, and
matrix-style verification work.
