# Milestone 9.13 Core Closeout: Declarative Query Experience

Core Phases 1-12 closed on 2026-07-14 for the runtime-backed declarative
product boundary. Add-on Phases 13-20 were added afterward and are not covered
by this closeout.

## Closed Boundary

Ordinary Query usage is capability-oriented and declarative across ten
namespaces:

- `facade::read`
- `facade::aggregate`
- `facade::live`
- `facade::history`
- `facade::comparison`
- `facade::preview`
- `facade::mutation`
- `facade::workflow`
- `facade::inspection`
- `facade::domain`

Consumers declare desired meaning, attach typed context, and run or open the
capability. Query owns canonicalization, admission, planning, lower-runtime
routing, execution, managed lifecycle, receipts, and typed outcomes. Internal
phase artifacts remain observable only where required for evidence or
certification; ordinary consumers cannot construct or independently advance
them.

The aggregate family is a separate ordinary namespace. Its current shipped
operation is count over an admitted collection declaration, producing
`WorthQueryCountOutcome` and a receipt-backed `WorthQueryCountResult`.

## Closure Evidence

The Phase 12 product-boundary bundle is assembled by
`certify_declarative_product_boundary()` from a source-backed evidence
registry. It covers:

- the complete capability grammar and executable transcript ownership
- equivalent declaration convergence and ordinary/internal oracle parity
- cross-capability, cross-basis, stale-context, and receipt-promotion denial
- one-shot/live parity, historical ambiguity, preview/workflow denial, and
  diagnostic-policy equivalence
- managed live open/close lifecycle evidence
- exact-zero planning/runtime work for invalid context and bounded ergonomic
  lowering under unrelated workspace growth
- facade snapshots, hard prohibitions, residue audits, sabotage cases, and
  reference-consumer adoption

The documentation closeout additionally proves that all ten ordinary
namespaces are discoverable, the read/projection/inspection and aggregate
examples execute, ordinary collection guidance does not expose internal
planner functions, ordinary projection guidance does not require manual
authority assembly, and relative Markdown links under `crates/worth-query/docs`
resolve.

## Verification Run

The closeout was rechecked on 2026-07-14 with:

- `cargo fmt -p worth-query -- --check`
- `cargo test -p worth-query declarative_facade_docs --lib -- --nocapture`
  (5 passed)
- `cargo test -p worth-query declarative_product_boundary_certification --lib -- --quiet`
  (29 passed)
- `cargo test -p worth-query declarative_product_boundary_compile_fail --lib -- --nocapture`
  (2 passed)
- `cargo test -p worth-query intent_admission_doc --lib -- --nocapture`
  (2 passed)
- `cargo test -p worth-query --lib --no-run`
  (library and test target compiled)

The crate still emits pre-existing unused/dead-code warnings during these
builds. They are not represented as documentation or product-boundary
failures.

## Claims Deliberately Not Made

This closeout does not claim:

- store-backed execution or pushdown parity
- durable restore, saved-query survival, or restart-stable continuation
- a general ordinary collection page-size/cursor continuation API
- durable cursor persistence
- blob-backed delivery
- completion of Milestones 10, 11, 12, or 13
- completion of Milestone 9.13 add-on Phases 13-20 or runtime-installed domain
  package authority
- a fresh uninterrupted pass of every `worth-query` unit test as part of this
  documentation correction

Runtime-backed cursor substrate and graph-access streaming cursors exist, but
the ordinary read/count journey does not currently expose a general paged
collection continuation contract. Store-backed parity belongs to Milestone 10;
durable artifacts and continuations belong to Milestone 11.

## Discovery Contract

Product discovery starts at:

- [`crates/worth-query/docs/AI_README.md`](../../crates/worth-query/docs/AI_README.md)
- [Declarative Query Experience](../../crates/worth-query/docs/capabilities/declarative-query-experience.md)
- [Collections, Ordering, Aggregates, And Cursors](../../crates/worth-query/docs/authoring/collections-cursors-ordering-and-aggregations.md)
- [Projection Consumption](../../crates/worth-query/docs/capabilities/projection-consumption.md)

These documents teach the current product surface. Historical phase assembly
belongs in milestone and certification records, not ordinary discovery.
