# Hadwiger Research

`hadwiger-research` is the planned Query-first research artifact pipeline for
Hadwiger-Nelson proof search.

The crate owns Hadwiger-Nelson domain meaning: candidate graphs, embeddings,
checker artifacts, aspect postures, invalidation rules, and theorem-like proof
claims. Forge Query remains the entry point for declaration, progression,
support/readiness, contribution posture, recovery, and later runtime
continuation.

Start with [Milestone 1](docs/milestone-1.md), then move into
[Milestone 2](docs/milestone-2.md) for the tiling candidate language and
iteration harness that makes the evidence stack usable for closed-loop
exploration.

## Local Verification

Use `cargo test -p hadwiger-research` for ordinary development checks. Public
compile-boundary tests are intentionally consolidated into one ignored trybuild
suite because they are expensive.

For normal implementation work, run only the UI scope you touched:

```powershell
$env:HADWIGER_TRYBUILD_SCOPE="tiling_iteration"
cargo test -p hadwiger-research --test compile_boundaries -- --ignored
Remove-Item Env:\HADWIGER_TRYBUILD_SCOPE
```

Multiple scopes can be comma-separated:

```powershell
$env:HADWIGER_TRYBUILD_SCOPE="tiling_geometry,tiling_iteration"
cargo test -p hadwiger-research --test compile_boundaries -- --ignored
Remove-Item Env:\HADWIGER_TRYBUILD_SCOPE
```

Run the full suite explicitly for closeout QA:

```powershell
cargo test -p hadwiger-research --test compile_boundaries -- --ignored
```
