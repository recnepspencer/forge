# worth-harness

`worth-harness` is shared certification and parity infrastructure for WORTH runtimes.

It is the crate that lets runtime libraries run the same scenario matrix across:

- execution profiles
- parity comparisons
- workflow certifications
- diagnostics captures
- deterministic feed and tick stream generation for hostile workload driving

This is not meant to be the product-facing API for `worth-relational` or
`worth-signal`.

It is the shared harness those runtimes use for heavy test, certification, and
matrix-style verification work.

The stream substrate is intentionally domain-neutral. It can model mostly
stable feeds with drift and occasional shifts, but it does not know what a
"product", "fuel surcharge", or "shipping lane" means. Runtime crates own that
domain layer on top.
