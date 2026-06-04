# worth-kernel

`worth-kernel` owns primitive-construction semantics over Worth topology and
spatial authority surfaces.

Its public API is intentionally narrow and Query-first:

- `worth_kernel::facade::authoring`
  - authored primitive-construction vocabulary
  - explicit Query-facing authoring entry
- `worth_kernel::facade::outcome`
  - prepared result truth
  - accepted and rejected construction outcomes
- `worth_kernel::facade::diagnostics`
  - public family, witness, preview, arbitration, policy, continuity, motion,
    and rejection diagnostics

## Ownership Boundary

`worth-kernel` owns:

- primitive-construction authored grammar
- primitive family semantics
- kernel-local lowering and construction interpretation
- selected kernel-local diagnostics that remain intentionally public

`forge-query` owns:

- runtime-facing entry and support posture
- declaration admission and progression
- receipts, envelopes, and retained artifacts
- inspection, workflow, and recovery lifecycle

## Supported Public Flow

1. Enter through `facade::authoring`
2. Produce a prepared result or outcome through `facade::outcome`
3. Use `facade::diagnostics` when you need witness, family, preview,
   arbitration, policy, continuity, motion, or rejection diagnostic
   explanation

What is not part of the public operating API:

- a flat root happy path
- a public certification bucket
- public corpus or closeout proof products
- public query-proof or realization-proof report buckets
- public replay, branch-preview-runtime, or hostility-suite proof products
- a second local runtime that competes with `forge-query`
