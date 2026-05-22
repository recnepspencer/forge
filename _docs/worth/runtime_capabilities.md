# Worth Runtime Capabilities

## 1. Runtime Thesis

Forge runtime is not a helper layer beneath Worth.

Forge runtime is the main operating substrate for nearly all runtime-facing
truth, routing, execution, evidence, diagnostics, lineage, and live behavior.

The mental model is simple: Relational is the truth-bearing graph and state
authority, Signal is the derived-computation DAG and invalidation engine, the
Bridge keeps truth and derivation coherent across runtime boundaries, and Query
is the ergonomic public operating layer that turns all of that lower machinery
into admissions, workflows, receipts, provenance, lineage, diagnostics,
subscriptions, and fact contracts. Because truth, derivation, and semantic
slicing are structural here rather than bolted on, many bad patterns become
unnecessary and many illegal states become much harder to express honestly.
The runtime does not need to invent Worth's domain semantics in order to carry
them. Query already has symbolic, extension-hook, invariant-pack, support-row,
trace, receipt, and inspection surfaces that let domain-originated semantics be
declared, attached, explained, and preserved much earlier than a normal kernel
stack would allow.

Aspects are a central part of how this runtime carries semantic meaning. They
are not optional metadata. They are one of the main ways the runtime lets us
declare semantic slices once and then reuse them across admission, execution,
invalidation, projection, diagnostics, subscriptions, and certification.

For Worth work, the ordinary rule is:

- Worth owns pre-runtime domain semantics
- Query owns the ordinary runtime-facing surface
- Relational, Bridge, and Signal remain lower authorities behind Query
- direct lower-runtime use is exceptional and explicit
- domain-originated semantics should be promoted into Query-owned declarations,
  traces, and evidence as soon as they stop being disposable local facts

If a concern is runtime-facing, the default assumption must be:

- the runtime already covers it
- Query should already provide the usable public/runtime-facing form
- rebuilding it in Worth is probably architectural error

This document exists to make that stance impossible to forget during planning.

## 2. Runtime Laws

### Authority Laws

1. Worth does not rebuild runtime-facing truth.
2. Worth does not rebuild runtime-facing authority boundaries.
3. Worth does not rebuild runtime-facing lifecycle vocabulary.
4. Query is the only ordinary runtime door for Worth.
5. Direct Relational use is forbidden except for declared extension seams.
6. Direct Bridge use is forbidden for normal Worth architecture.
7. Direct Signal use is forbidden for normal Worth architecture.
8. Lower runtimes keep authority; Query keeps the ordinary public operating
   language over that authority.
9. If a runtime-facing concern already belongs to Query, Worth must consume it
   there instead of routing around it.
10. Worth must not act like a mini runtime.
11. Worth must not treat aspects like decorative labels.
12. Runtime-facing semantic slicing should prefer aspects over local ad hoc
   categories.
13. Domain-originated semantics do not need to stay trapped in Worth-local
   structs once Query can carry them honestly.
14. If Query provides extension-hook, symbolic-reference, invariant-pack,
   support-row, trace, or inspection surfaces for a domain semantic family,
   Worth should use them instead of keeping that meaning disposable and local.

### Evidence Laws

15. Worth does not rebuild runtime-facing receipts.
16. Worth does not rebuild runtime-facing envelopes.
17. Worth does not rebuild runtime-facing provenance.
18. Worth does not rebuild runtime-facing lineage.
19. Worth does not rebuild runtime-facing diagnostics.
20. Worth does not rebuild runtime-facing certification bundles.
21. Worth does not rebuild runtime-facing support and discovery matrices.
22. Worth does not rebuild runtime-facing audits, closeout reports, or proof
   shape summaries.
23. If Query already provides a receipt-backed, envelope-backed, or
   certification-backed artifact, that artifact is the public contract.
24. Worth must not make a second nicer evidence layer over Query.
25. Domain-specific explanation should usually be layered onto Query artifacts,
   not built as a parallel runtime-facing evidence system.

### Basis And Admission Laws

26. Basis is not ambient context.
27. Basis is a runtime capability lifecycle.
28. Raw branch IDs, snapshot IDs, preview handles, tenant IDs, policy digests,
   and similar identifiers are not permission tokens in Worth.
29. Worth does not create its own basis protocol.
30. Worth does not create its own admission lattice for runtime-facing work.
31. Worth does not create its own runtime-facing denial taxonomy when Query
   already owns the family.
32. Runtime-facing decisions should enter Worth as admitted, advisory, denied,
   deferred, or equivalent Query-owned artifacts, not as local
   reinterpretations of raw runtime state.
33. Aspect-aware semantics should be admitted and consumed as runtime slices,
   not re-expanded into broad local object categories.
34. Domain-originated semantic admission should be promoted into Query-owned
   capability, trace, and inspection surfaces as soon as the semantics become
   runtime-facing.

### Execution Laws

35. Worth does not rebuild runtime-facing workflow languages.
36. Worth does not rebuild runtime-facing execution pipelines.
37. Worth does not rebuild runtime-facing effect, merge, or writeback
   orchestration.
38. Worth does not rebuild runtime-facing lower-runtime routing.
39. Worth does not choose lower-runtime paths by convenience.
40. Worth does not treat direct lower-runtime imports as harmless shortcuts.
41. If Query already owns the request -> eligibility -> plan -> receipt ->
   envelope story, Worth must use that story directly.
42. If the runtime can carry aspect-aware invalidation, execution, or delivery
   semantics, Worth must not flatten them back into object-level broad phases.
43. If a domain semantic fact can be carried through Query declaration,
   workflow, effect, or inspection surfaces, Worth should not downgrade it to
   throwaway local glue.

### Fact And Binding Laws

44. Worth does not reopen installed truth just to rediscover facts.
45. Worth does not rebuild materialized fact-consumption contracts locally.
46. Worth does not rebuild target binding or existing-truth binding protocols
   locally.
47. Worth does not rebuild graph-composition legality matrices locally.
48. Worth does not rebuild read-composition legality matrices locally.
49. If Query already owns the consumed-fact family, Worth must treat Query's
   fact artifacts as the finished surface.
50. Fact families should usually be aspect-aware slices, not broad local
   re-categorizations of the object.
51. Domain semantic classifications that later matter to reads, bindings, or
   diagnostics should usually be promoted into fact, binding, or inspection
   surfaces rather than re-derived from payloads.

### Live And Derived Laws

52. Worth does not build its own watcher protocol.
53. Worth does not build its own live subscription protocol.
54. Worth does not build its own invalidation engine.
55. Worth does not build its own derived-computation scheduler.
56. Worth does not build its own temporal or async/runtime-execution substrate.
57. If the concern is live, maintained, replayed, invalidated, delivered,
   scheduled, or convergence-sensitive, assume the runtime already owns it.
58. Query should be the public/runtime-facing bow over those capabilities.
59. Aspect-aware invalidation and delivery should be treated as runtime-native
   default power, not as advanced decoration.

### Planning Laws

60. If a concern is installed/runtime-facing, assume Query already has the
   right artifact until proven otherwise.
61. If Query feels awkward, do not immediately build a local wrapper; first
   verify whether the correct Query artifact already exists.
62. A genuinely missing Query surface is explicit debt, not permission to
   normalize a new Worth pattern.
63. The burden of proof is on any proposal that adds a new Worth-local
   runtime-facing concept.
64. The fact that lower-runtime pieces exist somewhere is not enough reason to
   rebuild the composition in Worth; Query is supposed to provide the
   composition.
65. When planning, treat the runtime as larger than your first instinct.
66. If you are unsure whether the runtime already owns something, assume it
   probably does and go check before designing locally.
67. If you are inventing a new runtime-facing category, ask whether it should
   really be an aspect slice, a fact family, a workflow family, or a
   subscription family first.
68. If you are holding a domain semantic fact locally, ask whether Query
   already has a declaration, extension-hook, symbolic, support, trace, or
   inspection surface that should carry it forward.

## 3. Exceptions

The normal exceptions are narrow.

1. Pre-runtime semantic truth originates in Worth.
   - authored spatial intent
   - anchor meaning
   - geometric ambiguity
   - non-finite or degenerate local semantics
   - local lowering legality before installed/runtime truth exists
   - this does not mean the semantics must remain invisible to Query after
     admission; many should be promoted into Query-owned declarations,
     traces, receipts, or diagnostics as soon as they become runtime-facing

2. Pre-runtime numeric and geometric admission belongs in Worth and
   `worth-math`.
   - admitted numeric witnesses
   - local geometric witness admission
   - local lowering and classification before runtime installation

3. Declared lower-runtime extension seams may go below Query.
   - relational invariant registration
   - relational custom invariant support
   - relational custom merge strategy registration
   - relational custom commit strategy registration

4. A genuinely missing Query surface may justify temporary debt.
   - this must be named explicitly as debt
   - it is not permission to establish a permanent Worth-local runtime pattern

These exceptions do not weaken the main rule.

They are the places where Query does not yet own the final public/runtime-facing
bow because the truth is either still pre-runtime or is an intentional
lower-runtime extension seam.

## 4. Planning Gate

Before planning or implementing new Worth architecture, answer these
questions:

1. Is this pre-runtime semantic truth, or installed/runtime-facing truth?
2. Is this really a runtime concern in disguise?
3. Does Query already own the ordinary artifact for this family?
4. Does Query already own the decision model for this family?
5. Does Query already own the receipt or envelope for this family?
6. Does Query already own the provenance, lineage, or diagnostics for this
   family?
7. Should this be modeled as an aspect-aware slice instead of a new local
   category?
8. Am I about to rebuild a runtime-facing wrapper language that Query is
   supposed to provide?
9. Am I about to reach below Query without being in an explicit extension seam?
10. Am I keeping a domain semantic fact local even though Query may already
    have a declaration, trace, support-row, symbolic-reference, or inspection
    surface that should carry it?

If the answer to 2 through 8 is yes, stop and use Query.

If the answer to 9 is yes, stop unless the task is one of the declared
extension seams.

If the concern is still purely pre-runtime domain semantics, keep it in Worth
and prove it locally.

## 5. Appendix: Runtime Coverage Inventory

This appendix exists so the claim that the runtime covers everything is
concrete rather than vague.

The runtime already covers:

- installed truth-state identity
- transactional commit authority
- canonical mutation publication
- savepoints and rollback
- MVCC snapshots
- retained history and replay
- branch heads and branch-local truth
- historical truth access
- lineage and correspondence
- structural identity and structural introspection
- graph traversal and graph introspection
- relation integrity
- schema-defined invariants
- custom and extensible invariants
- invariant execution and failure reporting
- schema evolution
- schema reconciliation
- merge ontology
- merge policy
- authoritative merge execution
- deletion and topology merge execution
- bulk query and traversal
- bulk mutation
- extensible commit strategies
- current-state certification
- generic certification bundles
- domain certification bundles

- typed query authoring
- canonical query identity
- canonical result-shape identity
- binding and rebinding
- schema-basis equivalence
- saved queries and templates
- aspects as semantic slicing
- aspect-aware invalidation
- aspect-aware projection and bulk query
- aspect-aware historical read surfaces
- aspect-aware committed diff and CDC surfaces
- aspect-aware subscriptions and derived-state propagation
- aspect-aware diagnostics and explanation
- aspect-aware merge and strategy semantics where declared
- authorized projection
- policy masking
- policy influence tracking
- projection visibility control
- declared projection consumption
- consumed fact families
- materialized projection contracts
- projection-consumption receipts
- projection-consumption envelopes
- projection-consumption certification

- cross-runtime causal inspection
- observation anchors
- evidence reference sets
- causal evidence resolution
- admitted, advisory, and denied inspection artifacts
- redaction policy
- materialization policy
- causal materialization receipts
- representative evidence matrices
- proof-flow and proof-shape audits
- cross-runtime explanation artifacts
- runtime-facing diagnostics bundles
- support matrices
- support traceability
- golden transcripts
- public-boundary audits
- compile-fail boundary enforcement
- non-bypass audits
- closeout registries
- gap registries
- phase manifests
- certification output manifests

- basis intent normalization
- basis eligibility
- admitted basis capability
- denied basis capability
- scoped basis use
- basis-use receipts
- self-describing basis envelopes
- basis support and discovery
- basis inventories
- basis compatibility and debt registries
- basis operation lanes
- observation basis
- mutation-preparation basis
- replay basis
- inspection basis
- materialization basis
- subscription-declaration basis
- subscription-activation basis
- preview-closeout basis
- certification basis
- current-head basis
- branch-head basis
- snapshot basis
- preview basis
- preview-derived basis
- runtime snapshot basis
- historical basis
- tenant-scoped basis
- policy-scoped basis
- diff basis
- lower-runtime basis readmission
- basis lifecycle certification

- runtime intent authoring
- intent admission
- capability eligibility
- authority-lane eligibility
- basis eligibility for intents
- invariant eligibility for intents
- routing-support eligibility
- projection-source eligibility
- admitted intent plans
- advisory decisions
- violation decisions
- admitted execution handoffs
- advisory stops
- violation stops
- decision-trace envelopes
- execution provenance chains
- intent admission certification
- intent support matrices
- mutation-entrypoint audits
- family inventories for admitted runtime-facing operations

- workflow context binding
- workflow declaration families
- lowered mutation declarations
- lowered merge declarations
- lowered writeback declarations
- authoritative outcome artifacts
- freshness and staleness bindings
- explicit rebind artifacts
- post-merge inspection
- conflict inspection

- effect intent normalization
- effect eligibility
- authority-scoped effect plans
- lowered execution plans
- relational mutation execution
- merge execution through Query-owned lowering
- bridge writeback execution through Query-owned lowering
- ordered batch execution
- execution receipts
- self-describing effect envelopes
- diagnostics materialization for effects
- effect lifecycle certification
- effect family inventories
- effect support matrices
- effect receipt transition rules

- lower-runtime capability requests
- lower-runtime capability eligibility
- lower-runtime route plans
- boundary execution receipts
- lower-runtime boundary envelopes
- lower-runtime boundary summaries
- lower-runtime routing inspections
- lower-runtime support matrices
- lower-runtime crossing inventories
- lower-runtime classification of reuse, adapter, deferred, debt, and
  forbidden lanes
- lower-runtime reconciliation reports
- lower-runtime acceptance suites
- lower-runtime proof-shape audits
- lower-runtime direct-import audits
- lower-runtime non-bypass certification
- lower-runtime closeout reports

- existing-truth assertions
- existing-truth binding evidence
- existing-truth probes
- target bindings
- target evidence
- routing preflight for existing-truth probes
- target-identity failure posture

- read-family admission
- read invariant packs
- graph-composition admission
- graph-composition invariant packs
- graph-composition denials
- graph-composition admission traces
- graph-composition lifecycle outcomes
- graph-composition lineage summaries
- graph-composition resolution maps
- graph-composition programs and steps
- domain-invariant summaries and denials
- relationship-proof denial posture

- runtime-backed live reads
- derived materialization
- live view declaration
- live view maintenance
- live change execution
- locality-aware and region-scoped live planning
- change relevance and suppression
- live replay bundles
- live delivery contracts
- live performance reports

- subscription declaration
- subscription family selection
- subscription basis binding
- subscription bridge lowering plans
- subscription runtime certification
- subscription diagnostics bundles
- subscription continuation evidence
- subscription continuation and remap
- preview subscription isolation
- preview discard and promotion handoff
- subscription maintenance-delta lowering
- subscription delivery batches
- subscription delivery windows
- subscription fanout plans
- shared consumer attachment
- subscription parity explanations
- manual bridge witnesses
- active subscription lanes
- acknowledgement frontiers
- delivery pacing and backpressure
- preview residue reporting

- bridge-backed truth-view evaluation
- bridge-backed source materialization
- bridge-backed historical path handling
- bridge-backed preview semantics
- bridge-backed writeback semantics
- bridge-backed continuity
- bridge-backed replay posture
- bridge-backed causality evidence
- bridge-backed truth-to-derived coordination
- patch-to-invalidation routing
- aspect mapping
- subscription continuity across identity evolution
- merge-aware bridge semantics
- mixed truth, preview, and replay coordination

- signal invalidation
- signal evaluation
- evaluation DAG scheduling
- changed-region propagation
- observation and delivery strategy
- replay cursors
- signal lineage
- signal provenance
- forensic availability
- branchable execution state
- deterministic versus optimized execution modes
- cost-aware scheduling
- priority-aware scheduling
- convergence and fixed-point execution
- temporal signals
- previous-value signals
- async and resource execution
- tolerance-aware derived computation
- snapshot, replay, and time-travel execution state

- truth-view receipts
- read receipts
- write receipts
- batch write receipts
- observation receipts
- materialization receipts
- inspection receipts
- basis receipts
- effect receipts
- projection-consumption receipts
- subscription receipts
- boundary receipts
- causality and provenance bundles
- certification bundles wherever they matter

- runtime-native support and discovery for what is admitted, advisory, denied,
  deferred, or unsupported
- runtime-native denial taxonomy
- runtime-native performance counters
- runtime-native slope and cost certification
- runtime-native machine-checkable artifacts for hostile verification

The planning default is therefore:

- assume the runtime already owns it
- assume Query is the ordinary public door
- assume there is already a receipt, envelope, provenance, lineage,
  aspect-aware slice, or diagnostic artifact for it
- prove otherwise before adding anything local to Worth

## 6. Appendix: Default Mapping

If you are thinking:

- what basis am I on?
  - think Query basis lifecycle
- why did this happen?
  - think Query causal inspection
- what can I trust?
  - think Query receipts, envelopes, and certification artifacts
- what facts may I consume?
  - think Query projection consumption
- how do I slice this meaning without inventing a local category?
  - think aspects first
- what installed target am I binding?
  - think existing-truth binding and target evidence
- what workflow family is this?
  - think Query workflow declaration and lowering
- how do I write or merge this?
  - think Query effect execution pipeline
- what live or maintained surface is this?
  - think Query subscription families
- how does derived behavior update?
  - think Query over bridge and signal, not Worth-local invalidation
- what invariant should enforce this?
  - think relational invariant registration, if this is truly an installed
    truth rule

If you are thinking:

- I should make a local receipt
- I should make a local diagnostics bundle
- I should make a local lineage summary
- I should make a local basis wrapper
- I should make a local watcher protocol
- I should call Bridge directly
- I should call Signal directly

stop and re-evaluate. The runtime almost certainly already owns the real
surface.
