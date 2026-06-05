# Hadwiger Research

`hadwiger-research` is the planned Query-first research artifact pipeline for
Hadwiger-Nelson proof search.

The crate owns Hadwiger-Nelson domain meaning: candidate graphs, embeddings,
checker artifacts, aspect postures, invalidation rules, and theorem-like proof
claims. Forge Query remains the entry point for declaration, progression,
support/readiness, contribution posture, recovery, and later runtime
continuation.

Start with [Milestone 1](docs/milestone-1.md).

## Local Verification

Use `cargo test -p hadwiger-research` for ordinary development checks. Public
compile-boundary tests are intentionally consolidated into one ignored trybuild
suite because they are expensive. Run them explicitly for closeout QA:

```powershell
cargo test -p hadwiger-research --test compile_boundaries -- --ignored
```
