# Performance, Layout, And Enforcement Vocabulary

This folder documents the Milestone 8 performance surface in
`forge-foundational`.

Use these docs when you need to answer questions like:

- How do I describe a performance claim without faking executed truth?
- How do I keep layout intent separate from claim boundary and evidence
  strength?
- When do I stay in common-path claim authoring, and when do I lower into
  canonical bundles, receipts, or explicit reports?
- How do I keep hot operational surfaces narrow while still allowing colder
  support expansion on purpose?
- When does a lower-lane artifact become eligible for proof-bearing certified
  or readmitted strengthening?

Read the docs in this order if you are new to the surface:

1. [Common Performance Claims And Layout Intent](./common-performance-claims-and-layout-intent.md)
2. [Policy Admission Receipts](./policy-admission-receipts.md)
3. [Canonical Bundles And Comparison](./canonical-bundles-and-comparison.md)
4. [Counter-Backed Performance Receipts](./counter-backed-performance-receipts.md)
5. [Performance Report Planning And Materialization](./performance-report-planning-and-materialization.md)
6. [Certified And Readmitted Performance Bundles](./certified-and-readmitted-performance-bundles.md)
7. [Grouped Public Lanes And Stronger Readiness](./grouped-public-lanes-and-stronger-readiness.md)
8. [Performance Production Readiness](./performance-production-readiness.md)

Capability order matters.

- Start with common-path claims so boundary, evidence strength, temperature,
  freshness, fallback, and work disclosure already mean one thing.
- Add policy admission before you claim executed counter-backed work.
- Lower into canonical bundles before you compare independently produced
  performance meaning.
- Add counter-backed receipts only after execution has really happened and
  exact structural rows exist.
- Plan reports before you materialize them, especially when support rows or
  counter detail may widen the surface.
- Use certified bundles only when stronger proof or trust-boundary readmission
  is real, never as a substitute for lower-lane honesty.
- Use the grouped public lanes when you want the supported first-contact API.
- Use readiness when you need the exact machine-checkable closure contract for
  what Milestone 8 really ships today.

These docs are feature-first on purpose. They are not milestone notes, closeout
notes, or test tours. If a performance capability shipped, it has one primary
home in this folder.

The crate-facing API surface these docs describe lives under:

- `forge_foundational::performance_api::common_path`
- `forge_foundational::performance_api::lower_lane`
- `forge_foundational::performance_api::stronger_lane`
- `forge_foundational::performance_api::performance_public_surface_inventory()`
