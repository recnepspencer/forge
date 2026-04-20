# Milestone 8 Closeout: Scopes, Templates, Saved Query Artifacts, And View-Shape Semantics

## Status

Milestone 8 is closed as of 2026-04-20 for the admitted composition,
saved-query, view-shape, and grouped-live scope in `forge-query`.

`forge-query` now owns:

- canonical scope and template composition that preserves one query meaning
- ephemeral saved-query freeze and typed reuse legality
- admitted view-shape semantics distinct from result shape
- planner-owned and live-owned view contracts for table, detail, observed
  inspector detail, focused inspector detail, and grouped kanban
- cross-crate grouped truth consumption through `forge-relational`,
  `forge-runtime-bridge`, and `forge-query`
- milestone-native certification, rejection rows, and compile-fail proof
  boundaries for the admitted Milestone 8 surface

The semantic center shipped in this milestone is:

one canonical query meaning can now be authored directly, through scopes,
through templates, frozen into ephemeral saved-query artifacts, admitted as
explicit view shapes, lowered into planner-visible and live-visible semantics,
and certified through named hostile rows without host repair, payload
rediscovery, cosmetic regrouping, or durable overclaim.

## Shipped Scope

Milestone 8 delivered:

- composition lineage, scope expansion, template instantiation, and canonical
  parity surfaces in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\composition](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\composition)
- ephemeral saved-query freeze, reuse legality, and support-profile truth in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\saved_query](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\saved_query)
- admitted view-shape planning, grouped planning contracts, and maintenance
  policy surfaces in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape)
- live view-shape lowering, grouped execution, grouped baseline, grouped
  desired state, grouped delta, and grouped replay/patch semantics in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live)
- grouped authoritative row truth and grouped projection authority in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-relational\src\grouped_truth](C:\Users\shepworth\Documents\programming\forge\crates\forge-relational\src\grouped_truth)
- grouped truth-view materialization and grouped boundary proofs in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source](C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source)
- milestone-native certification in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\harness\milestone_eight_certification](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\harness\milestone_eight_certification)
- compile-fail proof boundaries for composition, saved-query, view-shape,
  grouped truth, and grouped live admission in
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui)
  and
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\tests\ui](C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\tests\ui)

## Acceptance Mapping

Milestone 8 is considered closed against
[milestone-8.md](C:\Users\shepworth\Documents\programming\forge\_docs\forge-query\milestone-8.md),
[forge_query_roadmap.md](C:\Users\shepworth\Documents\programming\forge\_docs\forge-query\forge_query_roadmap.md),
[forge_query_vision.md](C:\Users\shepworth\Documents\programming\forge\_docs\forge-query\forge_query_vision.md),
and
[test-requirements.md](C:\Users\shepworth\Documents\programming\forge\_docs\forge-query\test-requirements.md)
because the required composition, saved-query, view-shape, grouped-live, and
closeout proof surfaces now exist directly.

### `Scope / Template / View-Shape Semantic Parity Test`

Covered by:

- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\harness\milestone_eight_certification\mod.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\harness\milestone_eight_certification\mod.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\harness\milestone_eight_certification\tests.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\harness\milestone_eight_certification\tests.rs)

What is proven:

- the named certification artifact exists as the Milestone 8 closeout suite
- required canonical rows are present and exercised:
  - `direct-vs-scope-parity`
  - `direct-vs-template-parity`
  - `saved-query-freeze-parity`
  - `view-shape-non-cosmetic-planning-live`
  - `kanban-desired-state-to-delta-parity`
  - `kanban-delta-admission-boundary`
  - `grouped-refresh-honesty`
  - `grouped-bridge-truth-view-authority`
  - `grouped-query-execution-surface-authority`
  - `grouped-proof-chain-no-payload-rediscovery`
  - `inspector-observed-focused-distinction`
  - `support-profile-honesty`
- required rejection rows are present and exercised:
  - `unsupported-scope-family`
  - `unsupported-template-family`
  - `saved-query-support-profile-drift`
  - `durable-saved-query-deferred-debt`
  - `post-admission-view-mutation-forbidden`
  - `grouped-hidden-refresh-forbidden`
- grouped proof-chain rows now use genuinely adversarial hostile inputs rather
  than cloned control lanes
- the matrix proves exact equality, inequality, typed rejection, and digest
  stability rather than row presence alone

### `Canonical composition and saved-query freeze honesty`

Covered by:

- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\composition](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\composition)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\saved_query](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\saved_query)

What is proven:

- direct construction, scope expansion, and template instantiation preserve one
  canonical query meaning for admitted families
- saved-query artifacts freeze canonical meaning plus composition/view metadata
  instead of introducing a second AST or host-owned bag
- saved-query reuse now classifies legality through an explicit rebinding
  matrix rather than convenience equality or host-side guesses
- durable persistence remains honest deferred debt rather than an ambient claim

### `View-shape planning and live semantic honesty`

Covered by:

- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape\tests.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape\tests.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\tests.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\tests.rs)

What is proven:

- view shape is now distinct from result shape and planner-visible
- table, detail, observed inspector detail, focused inspector detail, and
  kanban grouped are explicit admitted families
- grouped live execution requires grouped admission and next authoritative
  grouped truth instead of piggybacking on the ungrouped live entrypoint
- grouped desired state and grouped delta are derived from prior and next
  authoritative grouped truth rather than payload heuristics
- grouped refresh remains explicit debt or explicit denial rather than silent
  fallback
- focused inspector widening and post-admission mutation remain denied by type
  and boundary

### `Cross-crate grouped truth proof chain`

Covered by:

- [C:\Users\shepworth\Documents\programming\forge\crates\forge-relational\src\grouped_truth\row_set.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-relational\src\grouped_truth\row_set.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-relational\src\grouped_truth\grouped_projection.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-relational\src\grouped_truth\grouped_projection.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source\row_set.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source\row_set.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source\grouped_contract.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source\grouped_contract.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source\grouped_truth_view.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\src\source\grouped_truth_view.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\grouped_execution.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\grouped_execution.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\grouped_baseline.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\grouped_baseline.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\grouped_delta.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\src\view_shape_live\grouped_delta.rs)

What is proven:

- grouped meaning is no longer rediscovered from `Vec<String>` payload indexing
- relational authority, bridge truth-view materialization, and query grouped
  execution now form one typed proof chain
- bridge grouped truth-view preserves row identity, grouping value, basis
  identity, and grouped contract evidence
- grouped execution validates plan, grouped binding proof, and snapshot/basis
  compatibility before materialization
- grouped baseline and delta artifacts now mean one thing each and no longer
  carry stale or dead proof fields

### `Compile-time enforcement and exact spec traceability`

Covered by:

- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\phase_boundaries_compile_fail.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\phase_boundaries_compile_fail.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\tests\phase_boundaries_compile_fail.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\tests\phase_boundaries_compile_fail.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\legacy_grouped_execution_entrypoint_not_exported.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\legacy_grouped_execution_entrypoint_not_exported.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\grouped_live_execution_requires_grouped_admission.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\grouped_live_execution_requires_grouped_admission.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\post_admission_view_mutation_forbidden.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\post_admission_view_mutation_forbidden.rs)
- [C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\tests\ui\grouped_truth_view_artifact_fields_private.rs](C:\Users\shepworth\Documents\programming\forge\crates\forge-runtime-bridge\tests\ui\grouped_truth_view_artifact_fields_private.rs)

What is proven:

- legacy payload-based grouped execution entrypoints are no longer part of the
  public daily-driver surface
- grouped live execution cannot bypass grouped admission
- grouped truth-view artifacts cannot be fabricated through public field access
- post-admission view mutation is not just named in the certification matrix;
  it is backed by an exact compile-fail trace through
  [C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\post_admission_view_mutation_forbidden.stderr](C:\Users\shepworth\Documents\programming\forge\crates\forge-query\tests\ui\post_admission_view_mutation_forbidden.stderr)

## Final QA Outcome

The final hostile QA loop was run directly against the Milestone 8 spec, the
expanded grouped cross-crate surface, the grouped truth proof chain, the
Milestone 8 certification matrix, the compile-fail boundary set, and the test
suite itself.

The last meaningful findings were:

- grouped proof-chain rows in the certification matrix were still too friendly
  and not meaningfully hostile
- grouped bridge truth-view coverage was mostly happy-path despite a rich
  rejection taxonomy
- grouped execution and grouped live still had a few basis and grouped-truth
  proof gaps
- exact spec traceability for `post-admission-view-mutation-forbidden` was not
  yet explicit

Those findings were corrected before this closeout. After the correction pass,
I do not see a remaining meaningful Milestone 8 spec/code mismatch for the
admitted scope.

The final test-quality pass also closed the remaining certification softness:

- grouped proof-chain rows now use real hostile inputs
- grouped truth-view rejection taxonomy now has adversarial unit coverage
- grouped execution snapshot-identity mismatch is tested directly
- the Milestone 8 certification matrix now carries exact named rows for the
  expanded grouped and parity surface
- `post-admission-view-mutation-forbidden` now has exact compile-fail
  traceability in the closeout bundle

With those corrections in place, the admitted Milestone 8 scope is considered
production-ready rather than merely implemented.

## Explicit Deferred Scope

Milestone 8 is closed for the admitted composition, ephemeral saved-query,
view-shape, grouped-live, and grouped truth-view scope only.

The following remain later work, not implied completeness:

- durable saved-query reload, import/export portability, and restart-stable
  continuation
- additional scope families and additional template families beyond the current
  admitted surface
- timeline/chart/nested-grouped families beyond the current grouped kanban
  family
- richer lower-engine grouped execution beyond the current admitted grouped
  truth-view substrate
- any future `forge-signal` lane-aware invalidation refinements that might be
  enabled by the new grouped substrate

The current surface is intentionally strict and honest about those limits.

## What Later Milestones May Now Assume

Milestones 9, 10, and 11 and later may safely assume:

- composition semantics are already canonical and query-owned
- saved-query freeze is already one semantic model with explicit reuse legality
- view shape is already distinct from result shape
- grouped truth is already carried through relational authority, bridge
  truth-view materialization, and query grouped execution as explicit typed
  artifacts
- grouped live semantics are already planner-owned, baseline-owned, and
  delta-owned rather than host-regrouped
- the Milestone 8 certification suite already proves exact row coverage, exact
  denial coverage, grouped proof-chain honesty, and compile-time boundary
  enforcement

Those later milestones must not assume:

- durable saved-query semantics are already available
- grouped truth may be rediscovered from raw payloads
- grouped refresh fallback may hide inside generic collection success paths
- post-admission view mutation is tolerated as a convenience
- future grouped richness requires re-architecting the proof chain again

## Verification Baseline

Milestone 8 closeout was verified with:

- `cargo check -p forge-relational --lib`
- `cargo check -p forge-runtime-bridge --lib`
- `cargo check -p forge-query --lib`
- `cargo test -p forge-relational grouped_truth -- --nocapture`
- `cargo test -p forge-runtime-bridge grouped_truth_view -- --nocapture`
- `cargo test -p forge-query view_shape::tests -- --nocapture`
- `cargo test -p forge-query view_shape_live::tests -- --nocapture`
- `cargo test -p forge-query milestone_eight_certification -- --nocapture`
- `cargo test -p forge-query support_report_includes_query_composition_capability_and_profile -- --nocapture`
- `cargo test -p forge-query --test phase_boundaries_compile_fail -- --nocapture`
- `cargo test -p forge-runtime-bridge --test phase_boundaries_compile_fail -- --nocapture`

This passes cleanly and includes:

- composition and saved-query regression coverage
- grouped truth relational and bridge hostile coverage
- view-shape and grouped live unit tests
- milestone-native certification and rejection rows
- trybuild compile-fail proof-boundary tests across query and bridge

## Operational Conclusion

Milestone 8 is now closed at the composition, saved-query, admitted view-shape,
grouped truth-view, and grouped live semantic layer.

`forge-query` no longer depends on host-local scope/template sugar, controller-
owned saved-query conventions, payload-based grouped rediscovery, cosmetic
view labels, host-side regrouping, or untyped refresh fallback to express the
admitted Milestone 8 product surface. It now has canonical composition
artifacts, proof-bearing saved-query freeze, explicit view-shape planning,
cross-crate grouped truth authority, grouped desired-state and delta semantics,
milestone-native certification, exact compile-time enforcement, and adversarial
test coverage that later policy, tenant, durability, and richer grouped-engine
milestones can build on safely.

Production-readiness statement:

Milestone 8 is ready to treat as the normative composition and view-semantics
surface for the admitted runtime-backed `forge-query` product scope. The
remaining work is explicit later-milestone scope, not hidden architectural debt
inside the Milestone 8 boundary.
