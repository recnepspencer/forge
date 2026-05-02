# Forge Query Runtime API Public Stabilization Closeout

This closeout is the dependency contract for downstream runtime work that wants
to build against the Forge Query public runtime facade before temporal and async
milestones are implemented.

## Closed Scope

The stabilized public API is safe for runtime-backed, synchronous surfaces that
enter through `ForgeQueryRuntime::workspace` and compose through retained,
inspectable handles:

- `workspace.live_view(...)`
- `workspace.computed(...)`
- `workspace.effect(...)`
- `workspace.preview(...)` / `workspace.branch(...)`
- aspect-native `workspace.insert(...)`, `workspace.update(...)`,
  `workspace.delete(...)`, and `workspace.batch(...)`
- `workspace.read(...)`, `workspace.observe(...)`, and
  `workspace.materialize(...)`
- `workspace.state(...)`
- `workspace.inspect(...)`
- `workspace.public_api_contract()`
- `workspace.public_handle_contract()`
- `workspace.public_support_matrix()`
- `workspace.admit_public_api_family(...)`

The public support matrix is the source of truth for whether a family is stable,
deferred, or unsupported. Method presence is not a support claim.

The runtime API stabilization closeout defines the workspace facade surface
generally. The mutation-specific dependency contract is further narrowed by the
aspect finalization closeout: ordinary downstream mutation authoring should use
the aspect-native workspace mutation methods, while `workspace.write(...)`
remains a compatibility or expert seam rather than the preferred public story.

`workspace.intent(...)` remains part of the public vocabulary, but it is not in
the stable compatibility support set yet. Downstream runtimes must gate it
through `workspace.admit_public_api_family(...)` and backend support admission.

## Deferred Scope

These are explicit future gates, not implied support:

- temporal basis and time-aware subscriptions: Milestone 9.4
- async/resource query families: Milestone 9.5
- mixed truth/time/async delivery: Milestone 9.6
- temporal/async certification: Milestone 9.7
- store-backed parity: Milestone 10
- durable restart and artifact reload: Milestone 11

Each future milestone must extend the stabilized handle, state, authority lane,
aspect, support matrix, and inspection contracts. It must not introduce a
parallel public API family.

## Compatibility Names

Preferred names are the names downstream runtimes should use in new code.
Compatibility names remain adapters for existing call sites and tests.

- `live_view_request` and `declare_live_view` should migrate to `live_view`
  where closure-builder DX is appropriate.
- `computed_view`, `computed_definition`,
  `declare_maintained_derived_view`, and `declare_derived_view` should migrate
  toward `computed` for ordinary public DX.
- `effect_declaration` and `declare_effect` should migrate toward `effect`.
- `preview_with_options` and `branch_with_options` remain option-bearing
  variants of `preview` and `branch`.
- `execute_intent` should migrate toward `intent`.
- `execute_next_effect_write_intent` should migrate toward `next_effect_intent`.
- `read_live`, `drain_patches`, and `read_derived` should migrate toward
  `read`, `observe`, and `materialize`.

`computed_declaration` is intentionally not part of the compatibility surface.

For mutation naming specifically:

- ordinary downstream runtime code should use `insert`, `update`, `delete`,
  and `batch`
- `workspace.write(...)` remains available as a lower-level compatibility seam
  and should not be taught as the daily-driver API for new runtime work

## Safe To Build Now

Downstream runtimes may build domain-neutral public APIs that:

- keep the `Workspace` as the public context
- expose durable live, computed, effect, preview, branch, receipt, state, and
  inspection handles
- expose aspect-native mutation entrypoints without teaching payload-first
  authoring as the runtime's public mutation model
- use aspects to make reads, produces, triggers, and condition inputs auditable
- use authority lanes to distinguish truth, branch-local truth, preview truth,
  derived runtime state, effect delivery, pending write intent, bridge external
  state, temporal execution state, and async resource state
- call `admit_public_api_family(...)` before exposing future-neighbor behavior
- rely on typed early denials for unsupported temporal/async/store/durable
  surfaces

## Must Not Assume Yet

Downstream runtimes must not assume:

- `workspace.write(...)` is the preferred ordinary mutation story for new code
- the current public mutation contract already closes full authoritative target
  evidence, existing-truth identity binding, naming writeback evidence, or
  continuity-sensitive mutation evidence beyond the admitted runtime facade
- temporal basis execution is implemented
- async/resource lifecycle execution is implemented
- mixed truth/time/async delivery is implemented
- temporal/async certification has closed
- store-backed parity is admitted
- durable restart or artifact reload is admitted
- geometry, workflow, table, or other domain semantics belong inside
  `forge-query`

## Closeout Evidence

The `Runtime API Golden DX And Async-Safe Facade Test` now emits a closeout
artifact derived from the same certification matrix as the golden transcripts.
The closeout self-check answers:

- golden transcripts execute through the public facade
- unsupported future neighbors fail typed and early
- ordinary DX uses no lower-runtime plumbing
- support metadata is synchronized with admission gates
- handle, state, and inspection contracts are extension-ready
- temporal, async, store, and durable behavior remains deferred
- downstream examples are pressure tests, not `forge-query` domain semantics

Required verification commands:

- `cargo fmt -p forge-query`
- `cargo check -p forge-query --tests`
- `cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail`
- `cargo test -p forge-query`
- `cargo test -p forge-query runtime_api_stabilization`
- `cargo test -p forge-query runtime_public_support`
- `git diff --check`
