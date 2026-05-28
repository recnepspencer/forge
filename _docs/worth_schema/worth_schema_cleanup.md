# Worth-Schema Query Boundary Cleanup Audit

## Purpose

This document defines the post-`forge-query 9.3.7` and `9.3.8` cleanup work
that should happen in `worth-schema`.

`worth-schema` was built before Query had a complete public domain-capability,
declaration-entry, support, invariant, explanation, receipt, envelope, and
recovery model. That history left the crate with a meaningful amount of
public Query-shaped infrastructure that was reasonable while Query was still
incomplete, but is now architectural debt.

This audit is not a gentle suggestion list. It is an implementation contract
for shrinking `worth-schema` back to the responsibilities it should actually
own:

- truth vocabulary
- schema registry and raw truth structure
- lower authority substrate
- stable domain identifiers for aspects, collections, and invariants

It also names the responsibilities `worth-schema` should stop owning publicly:

- Query mutation readiness policy
- runtime capability and widening policy tables
- public invariant rollout posture
- public boundary-envelope and failure products
- public tracing and narration products
- public recovery-adjacent explanation surfaces

The goal is not to make `worth-schema` "smaller" in the abstract. The goal is
to stop teaching downstream crates that they should build on a parallel
runtime-facing boundary model now that Query already owns the real one.

## Missing Upstream Planning Docs

Unlike `forge-query` and several other subsystems, `worth-schema` currently
does not have an `_docs/worth_schema/...` vision, roadmap, or test-requirements
set.

That absence is itself part of the problem:

- the crate has a meaningful public boundary
- it influences lower-kernel vocabulary and truth semantics
- it now has real cleanup pressure caused by `forge-query 9.3.x`
- but there is no crate-local planning home that says what `worth-schema`
  should and should not own

This document therefore serves two jobs:

1. the immediate cleanup contract
2. the temporary planning anchor until `worth-schema` has a proper vision and
   roadmap

Nothing in this audit should be read as permission to guess future
`worth-schema` product direction loosely. Where the crate lacks an explicit
vision, this audit defers to:

- the coding-guideline documents
- the `forge-query` roadmap and `9.3.7` / `9.3.8` milestone decisions
- the actual current `worth-schema` code
- the already-established cleanup-audit style used in
  `_docs/worth_topo/query_boundary_cleanup_audit.md`

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects is foundation-first architecture.
  The strongest constraint here is that this cleanup must solve the kernel
  boundary problem directly instead of preserving transitional public surfaces
  because they are currently convenient.

- `arch_laws.md`
  The most important thing it protects is authority-honest subsystem
  boundaries. The strongest constraint here is that public envelopes,
  tracing, support, and recovery products must belong to the subsystem that
  actually owns that lifecycle now, which is Query rather than `worth-schema`.

- `composition_laws.md`
  The most important thing it protects is named responsibility over bag-shaped
  implementation. The strongest constraint here is that `worth-schema` cannot
  keep exporting vocabulary, runtime policy, support posture, tracing, and
  explanation as one blended public facade.

- `domain_structure_laws.md`
  The most important thing it protects is physical structure that teaches real
  authority and lifecycle distinctions. The strongest constraint here is that
  schema vocabulary, lower authority substrate, and Query-facing runtime
  product surfaces cannot continue sharing the same public structural space.

- `perf_laws.md`
  The most important thing it protects is honest cost boundaries. The
  strongest constraint here is that public Query policy, tracing, and
  explanation work should not be rediscovered through parallel schema surfaces
  that force downstream crates to reassemble meaning manually.

- `forge_query_roadmap.md`
  The most important thing it protects is Query as the platform-level facade
  for ordinary domain work. The strongest constraint here is that downstream
  domains should stay inside Query for runtime-facing artifact, orchestration,
  inspection, support, and recovery work rather than rebuilding pseudo-Query
  layers above it.

- `milestone-9.3.7.md`
  The most important thing it protects is the Query-owned domain capability
  contribution lifecycle. The strongest constraint here is that support,
  invariant, workflow, continuity, aftermath, and explanation posture are now
  explicit public Query categories rather than domain-local artifact folklore.

- `milestone-9.3.8.md`
  The most important thing it protects is Query as a first-class domain entry
  boundary. The strongest constraint here is that downstream crates are no
  longer justified in keeping a pre-Query public preparation, tracing, or
  envelope world for ordinary runtime-facing work.

- `test-requirements-milestone-9_3-and-runtime-gates.md`
  The most important thing it protects is certification-grade proof that
  Query-owned runtime artifacts remain typed, canonical, and boundary-honest.
  The strongest constraint here is that this cleanup must remove duplicate
  schema surfaces rather than letting them survive as shadow contracts beside
  the certified Query ones.

## Adversarial Constraint

This cleanup should be read with one hostile question in mind:

> could a good engineer follow the plan, move a few exports, rename a few
> wrappers, and still leave `worth-schema` teaching a second public
> runtime-facing boundary model beside Query?

If the answer is yes, the phase is underspecified.

The concrete failure modes we must defend against are:

- keeping schema-owned Query policy in place under a narrower helper name
- preserving bootstrap invariant rollout APIs because the enum identifiers are
  still useful
- replacing schema-owned boundary envelopes and failures with thin wrappers
  that still publish the same public contract
- keeping narration and tracing products public because they are typed and
  compile-checked
- demoting Query to "one option" while letting downstream crates continue to
  depend on schema-owned support, explanation, or boundary products

This audit therefore treats public overlap with Query as a kernel defect, not a
stylistic preference.

## Product Decision Lock

The following decisions are locked for this cleanup:

- `worth-schema` owns vocabulary, registry structure, and lower truth
  substrate. It does not own the ordinary public runtime workflow.
- `forge-query` owns the public domain-facing lifecycle for:
  - configured operating worlds
  - declaration entry
  - foundational evidence publication
  - route plans
  - receipts
  - envelopes
  - declaration-scoped support
  - invariant registration
  - capability gaps and invariant denials
  - lower-runtime support and explanation contributions
  - inspection
  - recovery
- Stable invariant identifiers may remain schema-owned if they are still useful
  as shared vocabulary. Runtime-facing invariant rollout posture may not remain
  schema-owned.
- Lower authority truth remains lower-crate-owned. This audit does not move
  relational, bridge, or signal authority into Query or into `worth-schema`.
- Internal substrate types may survive temporarily if they are required by
  lower truth authority, but survival as an internal detail does not justify
  survival as a public facade contract.
- Public compile-boundary tests should teach the post-`9.3.8` architecture,
  not preserve transitional API folklore for convenience.

## Query Capability Baseline

The relevant current Query docs and milestone decisions establish a much richer
public kernel than `worth-schema` was originally written against.

Most important public capability surfaces:

- configured domain entry and admitted-handle workflow:
  `crates/forge-query/docs/domain-capabilities/configured-domain-handles.md`
- typed binding:
  `crates/forge-query/docs/domain-capabilities/typed-binding-pipeline.md`
- ordinary outcome projection:
  `crates/forge-query/docs/domain-capabilities/ordinary-outcomes.md`
- family-native helper ergonomics:
  `crates/forge-query/docs/domain-capabilities/family-helpers.md`
- declaration-entry workflow:
  `crates/forge-query/docs/domain-capabilities/workflow/single-declaration-to-envelope.md`
- declaration receipts and envelopes:
  `crates/forge-query/docs/domain-capabilities/declaration-boundary-receipts.md`
  and
  `crates/forge-query/docs/domain-capabilities/declaration-boundary-envelopes.md`
- declaration-entry inspection:
  `crates/forge-query/docs/domain-capabilities/declaration-entry-inspection.md`
- declaration-scoped support:
  `crates/forge-query/docs/domain-capabilities/support/declaration-scoped-support-and-traceability.md`
- lower-runtime support:
  `crates/forge-query/docs/domain-capabilities/support/lower-runtime-support-and-boundary-traceability.md`
- invariant registration and invariant denial posture:
  `crates/forge-query/docs/domain-capabilities/invariants/registering-domain-invariants-through-query.md`
  and
  `crates/forge-query/docs/domain-capabilities/invariants/capability-gaps-and-invariant-denials.md`
- lower-runtime explanation:
  `crates/forge-query/docs/domain-capabilities/explanation/lower-runtime-explanation-contributions.md`
- recovery:
  `crates/forge-query/docs/domain-capabilities/recovery-boundary.md`

The most important consequence for `worth-schema` is simple:

> Query now has both the semantic coverage and the ergonomic quality required
> to be the ordinary public boundary for runtime-facing work.

That means schema-owned public support, envelope, tracing, explanation, and
repair-adjacent products are no longer justified as normal downstream entry
points.

## Findings

### 1. Schema still exports a public Query mutation-policy subsystem

Severity: High

Files:

- `crates/worth-schema/src/data/query/mutation_admission.rs`
- `crates/worth-schema/src/data/query/tests.rs`
- `crates/worth-schema/src/data/query/mutation_admission_tests.rs`
- `crates/worth-schema/tests/public_api_contract.rs`

Evidence:

`mutation_admission.rs` defines and exports:

- `QueryMutationAdmissionBlocker`
- `QueryMutationSupportContract`
- `query_mutation_support_contract(...)`
- `admit_query_mutation_batch(...)`

This is not just schema vocabulary. It is a domain-local public policy engine
for:

- substrate readiness
- workflow widening blocks
- naming writeback posture
- geometry and diagnostics truth exclusion

That work now overlaps directly with Query-owned public capability, support,
invariant, and recovery lanes. The compile-boundary test in
`tests/public_api_contract.rs` also freezes this surface as part of the public
contract, which means the current crate is teaching downstream code to depend
on an obsolete architecture.

Required judgment:

- this surface should leave the public facade
- most of it should be deleted rather than wrapped
- any surviving lower-authority logic should reappear only through Query-owned
  entry and contribution surfaces

### 2. Schema still exports public invariant rollout posture instead of just invariant identity

Severity: High

Files:

- `crates/worth-schema/src/data/bootstrap/invariant_plan.rs`
- `crates/worth-schema/src/data/bootstrap/runtime_invariants.rs`
- `crates/worth-schema/src/data/invariants/mod.rs`
- `crates/worth-schema/src/data/invariants/geometry.rs`
- `crates/worth-schema/src/data/invariants/topology.rs`
- `crates/worth-schema/src/data/invariants/diagnostics.rs`
- `crates/worth-schema/src/facade.rs`

Evidence:

The crate currently exports:

- invariant identifier enums
- `BootstrapInvariantPlan`
- `BootstrapRuntimeInvariant`
- `bootstrap_invariant_plan()`
- `bootstrap_runtime_invariant_plan()`

The identifier enums may still be legitimate shared vocabulary. The bootstrap
plan APIs are not.

After Query `9.3.7`, invariant registration, capability gaps, and runtime
invariant denial posture have an explicit Query-owned public home. Keeping
schema-owned public rollout posture beside that surface creates two competing
foundation stories:

- "register or explain invariants through Query"
- "consult schema's bootstrap invariant plans"

That ambiguity is unacceptable at kernel-foundation level.

Required judgment:

- invariant identity vocabulary may remain if still useful
- bootstrap invariant planning should leave the public schema contract
- runtime-facing invariant installation and denial posture should move fully to
  Query-owned lanes

### 3. Schema still exports a public boundary-envelope and failure model beside Query

Severity: High

Files:

- `crates/worth-schema/src/data/tracing/mod.rs`
- `crates/worth-schema/src/data/authority/gateway.rs`
- `crates/worth-schema/src/facade.rs`
- `crates/worth-schema/tests/compile_fail/mint_boundary_envelope.rs`
- `crates/worth-schema/tests/compile_fail/mint_boundary_failure.rs`

Evidence:

The tracing module defines and the facade exports:

- `DecisionTrace`
- `BoundaryEnvelope<T>`
- `BoundaryFailure<E>`
- multiple trace anchor and evidence structs

At the same time, Query now owns:

- declaration foundational evidence
- declaration receipts
- declaration envelopes
- lower-runtime boundary envelopes
- declaration-entry inspection
- recovery

Schema's boundary model therefore no longer reads as "shared substrate that the
public product still lacks." It reads as a second public boundary artifact
system.

The compile-fail tests are especially revealing: they prove these types are
carefully protected and intentionally public. That is exactly why they are
dangerous now. We are spending public-boundary rigor on the wrong facade.

Required judgment:

- these types should stop being ordinary public downstream contracts
- if any must survive temporarily for lower truth authority internals, they
  should be demoted behind narrower internal or crate-limited seams
- Query should be the only ordinary public artifact system that downstream
  domains learn

### 4. Schema still exports a public runtime explanation and narration layer beside Query

Severity: High

Files:

- `crates/worth-schema/src/data/explanation/mod.rs`
- `crates/worth-schema/src/facade.rs`

Evidence:

The crate exports:

- `NarratedTrace`
- `AuthorityNarrative`
- `BridgeNarrative`
- `DerivedNarrative`
- `SignalNarrative`
- `explain_*` helpers
- `narrate_*` helpers

This is not just raw substrate inspection. It is a schema-owned public runtime
storytelling layer.

That overlaps directly with Query-owned public surfaces for:

- declaration-scoped support and traceability
- lower-runtime support and boundary traceability
- lower-runtime explanation contributions
- declaration-entry inspection
- recovery explanations

The core problem is not that schema explanations are "badly typed." The
problem is that they are public runtime-facing products in the wrong subsystem.

Required judgment:

- these explanation products should leave the public schema facade
- downstream consumers should reach runtime-facing explanation through Query
- any surviving substrate helpers should be explicitly lower-level and not
  masquerade as the ordinary story

### 5. The schema facade currently blends nucleus vocabulary with obsolete runtime-facing exports

Severity: High

Files:

- `crates/worth-schema/src/lib.rs`
- `crates/worth-schema/src/facade.rs`

Evidence:

`lib.rs` says schema defines truth vocabulary and does not own runtime
materialization or mutation execution.

But `facade.rs` currently exports, in one public bundle:

- vocabulary and kinds
- bootstrap invariant plans
- Query mutation readiness policy
- tracing envelopes
- narration/explanation products

That is an architecture lie. The crate description and the facade contract no
longer match.

Required judgment:

- the public facade should teach the narrowed post-Query role of schema
- anything runtime-facing enough to feel like an ordinary product workflow
  should be presumed out of place unless explicitly justified as lower
  substrate

### 6. There is still a real schema nucleus, and the cleanup should preserve it aggressively

Severity: Important

Files:

- `crates/worth-schema/src/data/query/mod.rs`
- `crates/worth-schema/src/data/query/declarations.rs`
- `crates/worth-schema/src/data/bootstrap/registry.rs`
- `crates/worth-schema/src/data/bootstrap/domain_registry.rs`
- `crates/worth-schema/src/data/aspects/...`
- `crates/worth-schema/src/data/entities/...`
- `crates/worth-schema/src/data/relations/...`

Evidence:

Not everything Query-adjacent in schema is stale.

The strongest legitimate schema-owned surfaces still appear to be:

- aspect vocabulary
- aspect-path mapping
- collection names
- schema-basis names
- entity and relation kind vocabulary
- raw registry/bootstrap of truth structure
- possibly thin declaration-lowering helpers where they only carry naming and
  schema vocabulary

This matters because the cleanup should narrow schema, not hollow it out.
The target state is not "Query owns everything." The target state is:

- schema owns vocabulary and raw truth structure
- Query owns the public runtime-facing lifecycle built on top of that

## Phases

### Phase 1: Public Query Policy Surface Removal

Goal:

- remove schema-owned public Query gating and capability-policy contracts before
  they continue teaching downstream crates the wrong kernel story

Primary targets:

- `data/query/mutation_admission.rs`
- `data/query/tests.rs`
- `data/query/mutation_admission_tests.rs`
- `tests/public_api_contract.rs`
- `facade.rs`

Out of scope:

- full tracing/explanation demotion
- invariant identifier cleanup
- lower-authority gateway internals

Must stop doing:

- exporting `admit_query_mutation_batch(...)` as an ordinary public seam
- exporting `query_mutation_support_contract(...)` as an ordinary public seam
- compile-locking these transitional Query policy surfaces as part of the
  intended public API
- teaching workflow widening and truth-lane blocking through schema-owned
  stringly families

Must introduce:

- a narrower public facade that no longer advertises schema-owned Query policy
  as first-class foundation
- compile-boundary tests that reflect the post-`9.3.8` role of schema
- explicit migration notes in code or docs where callers must move to Query
  entry, support, invariant, or recovery surfaces

Allowed survivors:

- internal helpers may temporarily survive only if they are no longer public
  and are on a clear deletion path

Verification:

- `facade.rs` no longer re-exports mutation admission or query support
  contract policy helpers
- public compile-boundary tests no longer certify those surfaces as ordinary
  downstream API
- the cleanup does not replace the public API with thin wrappers that carry the
  same semantics under a new name

### Phase 2: Invariant Ownership Narrowing

Goal:

- reduce `worth-schema` invariant ownership to stable identity vocabulary and
  raw structural declarations, while moving public runtime-facing invariant
  posture fully to Query

Primary targets:

- `data/bootstrap/invariant_plan.rs`
- `data/bootstrap/runtime_invariants.rs`
- `data/invariants/...`
- `facade.rs`

Out of scope:

- deleting invariant enums if they still serve as stable shared vocabulary
- broad tracing cleanup
- lower-authority gateway refactors

Must stop doing:

- exporting bootstrap invariant rollout plans as part of the ordinary schema
  public contract
- teaching runtime invariant posture through schema-owned bootstrap surfaces
- leaving ambiguity about whether downstream domains should consult schema
  plans or Query invariant lanes

Must introduce:

- a narrower invariant story:
  - schema-owned invariant identifiers and raw structural declarations where
    needed
  - Query-owned public registration, denial, and capability posture
- a facade that makes the distinction physically obvious

Allowed survivors:

- invariant identifier enums
- raw structural invariant declarations that truly belong to lower truth
  substrate

Verification:

- bootstrap invariant plan exports are removed from the public facade
- the remaining public invariant surfaces in schema read like vocabulary or raw
  declaration substrate rather than runtime policy
- downstream planning docs can point to one ordinary invariant workflow, which
  is Query's

### Phase 3: Boundary And Explanation Surface Demotion

Goal:

- stop `worth-schema` from exporting a parallel public envelope, tracing, and
  runtime explanation model beside Query

Primary targets:

- `data/tracing/mod.rs`
- `data/explanation/mod.rs`
- `data/authority/gateway.rs`
- `facade.rs`
- compile-boundary tests related to boundary envelope/failure construction

Out of scope:

- deleting every lower-authority trace carrier immediately if internal users
  still need them
- redesigning Query itself

Must stop doing:

- exporting `BoundaryEnvelope`, `BoundaryFailure`, `DecisionTrace`, and the
  trace anchor/evidence types as ordinary downstream public contracts
- exporting `NarratedTrace` and `explain_*` / `narrate_*` helpers as ordinary
  runtime-facing explanation API
- teaching downstream domains to solve support, explanation, and repair
  questions through schema-owned boundary products

Must introduce:

- a deliberate split between:
  - internal or crate-limited lower-authority substrate, if still needed
  - Query-owned public receipts, envelopes, inspection, support, explanation,
    and recovery
- tighter visibility and narrower re-export posture

Allowed survivors:

- internal authority-facing evidence carriers that are still needed by
  lower-truth substrate and are no longer sold as ordinary public API

Verification:

- `facade.rs` no longer exposes schema-owned public tracing and explanation
  product surfaces
- downstream public documentation and compile boundaries no longer imply schema
  is an alternate boundary-artifact system
- any surviving trace or envelope substrate has a clearly locatable, narrow,
  non-ordinary ownership boundary

### Phase 4: Facade And Compile-Boundary Tightening

Goal:

- make the crate's public contract match its actual intended role after the
  Query cleanup

Primary targets:

- `lib.rs`
- `facade.rs`
- public API tests
- any crate-local docs that still teach the transitional surface

Out of scope:

- broad implementation redesign outside the already-named cleanup targets
- inventing a fake roadmap if the crate still lacks one

Must stop doing:

- keeping the facade broad enough that downstream consumers cannot tell whether
  schema owns vocabulary or runtime-facing products
- leaving compile-boundary tests that preserve transitional surface area by
  inertia
- describing the crate as vocabulary-only while exporting policy, tracing, and
  explanation products that contradict that claim

Must introduce:

- a public facade that teaches one crisp role:
  `worth-schema` is vocabulary and raw truth-structure substrate
- compile-boundary tests aligned to that narrower role
- explicit planning follow-up that `worth-schema` still needs a real crate-local
  vision and roadmap

Allowed survivors:

- thin declaration-vocabulary helpers if they only carry naming and schema
  semantics and do not reintroduce runtime policy or boundary products

Verification:

- a new contributor reading only `lib.rs`, `facade.rs`, and the public API test
  can infer the correct role of the crate
- public exports align with the crate description
- the next correct downstream edit is more likely to enter Query for runtime
  work than to stay inside schema for convenience

## Expected End State

After this cleanup, `worth-schema` should look different in a few important
ways:

- the public crate reads like vocabulary and raw truth substrate, not like a
  second runtime facade
- downstream domains enter Query for runtime-facing support, invariant,
  envelope, explanation, inspection, and recovery work
- invariant identifiers may still originate in schema, but runtime-facing
  invariant lifecycle no longer does
- any surviving lower-authority evidence carriers are narrow and internal
  enough that ordinary downstream code does not learn them as the normal path
- the public compile boundary teaches post-`9.3.8` kernel architecture rather
  than transitional pre-stabilization scaffolding

That is the real payoff of the Query work for `worth-schema`: less parallel
infrastructure, less public ambiguity, and a much cleaner foundation split
between "schema-owned truth vocabulary" and "Query-owned runtime-facing
lifecycle."

## Sequencing Notes

- do not start by cosmetically rewriting docs while the public facade still
  exports the obsolete surfaces
- do not keep mutation-admission policy public just because it is already
  tested and typed
- do not treat invariant identifier usefulness as a reason to preserve
  bootstrap invariant rollout APIs
- do not solve tracing cleanup by keeping the same public artifact model under
  a different module name
- do not let lower-authority internal needs become an excuse for keeping the
  whole public schema boundary model alive
- do not declare this cleanup done until the compile-boundary story matches the
  intended architecture

## Acceptance Evidence

Before this cleanup is called done, the implementation should be able to show:

- `worth-schema` no longer exports public Query mutation policy surfaces
- `worth-schema` no longer exports public invariant rollout posture
- `worth-schema` no longer exports public boundary-envelope, tracing, or
  runtime narration products as ordinary downstream contracts
- the remaining public schema API is overwhelmingly vocabulary, registry, and
  raw truth-structure substrate
- downstream runtime-facing docs and examples route through Query rather than
  schema-owned helper systems
- compile-boundary tests prove the narrowed facade and do not preserve
  transitional surfaces by accident

## Self-Check

If an engineer finishes this plan and any of the following are still true, the
cleanup is not done:

- a downstream crate can still reasonably learn "Query mutation readiness" from
  `worth-schema`
- a downstream crate can still reasonably learn runtime invariant rollout
  posture from `worth-schema`
- `BoundaryEnvelope`, `BoundaryFailure`, `DecisionTrace`, or `NarratedTrace`
  are still part of the ordinary public schema teaching surface
- the public facade still blends vocabulary ownership with runtime-facing
  support or explanation products
- a reviewer reading the public API could not say, in one sentence, that Query
  owns the ordinary runtime-facing workflow and schema owns vocabulary and raw
  truth structure
