# Forge Query And Bridge Authoritative Mutation Evidence And Causality Plan

> **Status:** Proposed cross-runtime side-quest gate
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Vision parent:** [forge_query_vision.md](./forge_query_vision.md)
>
> **Bridge parent:** [../forge-runtime-bridge/forge_runtime_bridge_roadmap.md](../forge-runtime-bridge/forge_runtime_bridge_roadmap.md)
>
> **Primary predecessors:** [aspect-api-finalization-closeout.md](./aspect-api-finalization-closeout.md), [runtime-api-public-stabilization-closeout.md](./runtime-api-public-stabilization-closeout.md), and [../forge-runtime-bridge/milestone-12.md](../forge-runtime-bridge/milestone-12.md)
>
> **Primary downstream pressure:** [../worth/forge-query-runtime-rewrite-plan.md](../worth/forge-query-runtime-rewrite-plan.md)
>
> **Primary owners:** `forge-query` and `forge-runtime-bridge`
>
> **Purpose:** harden the public mutation and receipt contract together with the bridge carry-forward contract so serious domains can express authoritative writes against new and existing truth without shadow identity glue, semantic target loss, dropped causality/provenance, or domain-local writeback runtimes.

## Goal

Freeze one cross-runtime authority-evidence contract so:

- `forge-query` exposes a public mutation/receipt/inspection surface that is
  semantically strong enough for downstream domains
- `forge-runtime-bridge` carries forward the lower-runtime causality,
  provenance, naming, continuity, and writeback evidence that Query promises to
  expose

The resulting end-to-end contract must make aspect-native `insert`, `update`,
`delete`, `batch`, preview, receipt, state, and inspection surfaces preserve
enough authored and resolved meaning for:

- direct authoritative writes against existing truth
- domain-authored edit lowering
- projected naming writeback
- lineage and continuity-aware authority crossings
- first-class causality and provenance carried by the runtime rather than
  reconstructed by domains
- future cross-domain writeback and certification work

This gate is not about inventing new domain semantics inside Query. It is about
making the generic mutation substrate honest enough that domains stop needing to
rebuild target classification, identity binding, and authority evidence above
the facade.

## Why This Plan Exists

The public runtime API is now stable enough that downstream domains are trying
to use Query as the real authority lane rather than a read helper. That is the
right pressure.

It also exposes a real gap: aspect-native writes currently preserve touched
aspect meaning, but they are still too weak around target-class evidence,
existing-truth binding, authoritative naming attachment, causality/provenance,
and continuity-aware mutation evidence.

Without this gate:

- serious domains can author a write but still need local glue to explain what
  class of thing was actually targeted
- existing-truth edits depend on domain-local identity rebinding between domain
  authority ids and Query mutation targets
- naming writeback risks becoming a second runtime because attachment/rebind
  meaning is not explicit enough in the generic receipt surface
- causality and provenance are too easy to lose or flatten into ad hoc metadata
  instead of remaining first-class runtime evidence
- lineage and continuity-sensitive domains cannot ask Query to preserve enough
  identity-transition evidence for truthful inspection and replay
- batch receipts stay too scalar-shaped for authority-heavy workflows

This is exactly the kind of substrate gap that should be solved once in
`forge-query`, not rediscovered in each downstream runtime.

## Governing Summaries

- `MENTALITY.md`: protects solving the hard structural problem first. Here, the
  hard problem is not "support one more write helper"; it is making authority
  mutation evidence explicit before more domains depend on weaker semantics.
- `arch_laws.md`: protects dual write/read contracts, authority/derivation
  separation, batch-derived summaries, self-describing envelopes, and facade
  honesty. Mutation meaning must be preserved by the runtime contract, not
  reverse-engineered by domains from patches, logs, or lower-crate artifact
  spelunking.
- `perf_laws.md`: protects semantic-delta-bounded work and boundary-cardinality
  honesty. Bulk authoritative imports and edit batches must not be fragmented
  into scalar explanations that hide breadth or lose target identity.
- `domain_laws.md`: protects responsibility-shaped modules and honest naming.
  This work belongs in generic runtime mutation/inspection modules, not in a
  Worth-shaped adapter layer.
- `forge_query_vision.md`: protects Query as the daily-driver platform surface
  for asking for truth and orchestrating domain work over truth. If writes are
  part of the daily-driver story, their authority evidence, causality, and
  provenance have to be first class.
- `forge_query_roadmap.md`: protects the rule `declare query intent once,
  lower it once, execute it against canonical truth`. Mutation authoring and
  authoritative outcome evidence must obey the same rule.
- `test-requirements.md`: protects canonical, adversarial certification
  artifacts. This gate needs named suites for authoritative mutation evidence,
  not just happy-path CRUD tests.
- `forge_runtime_bridge_vision.md`: protects the bridge as a causal protocol
  boundary, not incidental glue. If Query promises lineage/provenance for free,
  the bridge must preserve that meaning through the runtime seam.
- `forge_runtime_bridge_roadmap.md`: protects bridge-owned causality transfer,
  writeback protocol meaning, and replay-safe cross-runtime artifacts. This
  gate must consume those strengths and harden any missing carry-forward seam.
- `forge-runtime-bridge/test-requirements.md`: protects canonical,
  machine-checkable bridge bundles for causality, replay, writeback, temporal,
  and async boundaries. This gate must not define a Query-side evidence story
  the bridge cannot certify.
- `forge-runtime-bridge/milestone-12.md`: protects bridge-mediated writeback,
  replay-safe provenance, idempotence, and loop prevention as first-class bridge
  protocol surfaces. This gate must reuse and extend that contract, not dilute
  it.
- `forge-runtime-bridge/milestone-13.md`: protects one canonical end-to-end
  causality bundle story across routing, evaluation, writeback, and replay.
  This gate must make Query consume that story rather than inventing a second
  public provenance model.
- `aspect-api-finalization-closeout.md`: protects aspect-native mutation as the
  ordinary public story. This plan deepens that story rather than replacing it.
- `runtime-api-public-stabilization-closeout.md`: protects the stable public
  facade and inspectable handles. This plan hardens the mutation side of that
  same facade so downstream write-heavy domains can rely on it honestly.
- `worth/forge-query-runtime-rewrite-plan.md`: protects the clean-break rule
  that Worth must harden Query instead of working around it. Worth Phase 6 is
  the forcing function, not the owner of the fix.

## Adversarial Constraint

Under direct writes, ordered batches, authoritative imports, preview-local
mutation, projected naming attachment, continuity-sensitive updates, and
domain-authored writeback lowering, the same canonical authored mutation must
produce the same target-class meaning, the same target identity evidence, the
same authority-lane explanation, the same causality/provenance bundle, and the
same typed denial behavior regardless of whether the target already existed,
was created earlier in the same batch, was addressed through a projected naming
attachment, or participated in lineage continuity.

If the runtime loses any of the following and asks a domain to recover it
locally, this gate has failed:

- what class of truth was targeted
- whether that target was declared, resolved, or rebound
- which authoritative identity the mutation was anchored to
- which upstream authority, source declaration, or writeback cause produced the
  mutation
- which provenance and lineage breadcrumbs must follow the outcome forward for
  later reads, naming, and certification
- whether naming was created, rebound, orphaned, or removed
- whether continuity/lineage meaning was preserved, denied, or ambiguous
- what batch-scoped authority evidence explains the whole mutation session

## Non-Negotiable Boundary

- `forge-query` owns mutation declaration vocabulary, target evidence
  vocabulary, receipt/inspection shaping, support gating, and typed denial.
- `forge-runtime-bridge` owns cross-runtime causality transfer, replay-safe
  provenance carry-forward, writeback-family protocol meaning, and the lowered
  bridge artifact surfaces that connect truth/runtime outcomes to Query-facing
  evidence.
- lower runtimes remain authoritative for truth mutation execution, continuity
  semantics, persistent naming semantics, writeback protocol semantics, and
  lineage truth.
- `forge-relational`, `forge-signal`, and the runtime bridge must be treated as
  the source of causality/provenance truth below the Query facade. If Query
  cannot carry that meaning forward without loss, the bridge/runtime seam is
  incomplete and must be hardened before downstream domains continue.
- domains may declare target meaning, naming meaning, and continuity meaning,
  but they must hand that meaning to Query through generic public contracts
  rather than preserving shadow runtimes.
- unsupported identity-binding, naming-writeback, or continuity families must
  fail typed and early instead of degrading into "best effort" target recovery.

This is one end-to-end contract, not a Query promise stapled onto a separate
bridge promise:

- Query may not overclaim evidence the bridge cannot preserve
- the bridge may not preserve evidence in a way Query cannot expose honestly
- downstream domains should not need to know where inside the cross-runtime
  path one breadcrumb was minted in order to trust the public receipt

## Concrete Artifact Boundary

This gate must not stay abstract. It needs a closed family of proof-bearing
runtime artifacts that a downstream engineer could point to in code and tests.

At minimum, the final public surface should converge on materially equivalent
artifacts to:

- `ForgeQueryMutationTargetEvidence`
- `ForgeQueryMutationCausalityEvidence`
- `ForgeQueryMutationProvenanceEvidence`
- `ForgeQueryExistingTruthBinding`
- `ForgeQueryBatchMutationEvidence`
- `ForgeQueryNamingMutationEvidence`
- `ForgeQueryContinuityMutationEvidence`
- `ForgeQueryWriteReceiptInspection`
- `ForgeQueryBatchWriteComponentInspection`

At minimum, the bridge-side carry-forward surface should converge on materially
equivalent artifacts to:

- `BridgeMutationCausalityBundle`
- `BridgeMutationProvenanceBundle`
- `BridgeExistingTruthBindingArtifact`
- `BridgeNamingWritebackEvidence`
- `BridgeContinuityMutationEvidence`
- `BridgeWritebackOutcomeProvenance`

The exact Rust names may differ, but the artifact responsibilities may not blur.
If one type tries to carry target classification, causality, provenance,
naming, continuity, denial, and batch aggregation all at once, the spec has
collapsed too many ontologies into one bag.

## Compile-Time Enforcement Policy

This gate must make the wrong thing hard or impossible, not merely discouraged.

`Unrepresentable` in public types:

- a mutation receipt or inspection artifact that exposes touched aspects but not
  whether the target was declared, resolved, or denied
- a naming-aware or continuity-aware mutation family encoded as open-ended
  strings, bools, or generic metadata bags instead of closed evidence families
- a batch/session summary that can exist without a canonical relationship to
  component receipts
- a causality/provenance surface that can omit source declaration identity,
  authority-lane transition identity, or denial class while still claiming to
  be complete authority evidence

`Uncompilable` through privacy and compile-fail enforcement:

- external construction of proof-bearing mutation evidence artifacts without the
  runtime-owned lowering path
- public APIs that accept raw lower-runtime mutation provenance blobs as though
  they were admitted public Query evidence
- public mutation APIs that let callers override resolved target class,
  continuity class, or naming outcome after runtime lowering
- public shortcut paths that encode existing-truth identity binding as ad hoc
  `String` reuse with no admitted binding artifact

`Construction-time rejection`:

- unsupported existing-truth binding families
- unsupported symbolic same-batch reference classes
- unsupported naming-writeback families
- unsupported continuity-sensitive families
- invalid target-class pairings between authored mutation family and resolved
  lower-runtime target
- attempts to cross from ordinary mutation into naming/continuity semantics
  without the explicit admitted family

Compile-fail coverage must prove at minimum:

- no public forging of mutation evidence artifacts
- no bool-driven "is continuity-aware" or "is naming-aware" shortcuts
- no public consumption of raw lower-runtime provenance bags
- no public target-class override after lowering

## Phases

### Phase 1: Freeze Target Evidence Vocabulary

Define the shared Query-facing and bridge-facing vocabulary for mutation
targets, causality, provenance, and authority evidence.

Must ship:

- distinct declared-versus-resolved target evidence in public mutation receipts
- target collection or target class evidence for insert, update, delete, and
  batch components
- explicit target-entity identity evidence where the mutation family addresses
  one concrete target
- explicit causality and provenance sections in receipts and inspection so a
  caller gets lineage/provenance for free from the runtime once the mutation
  crosses the public authority lane
- bridge-side causality/provenance bundles that the Query receipt and
  inspection surfaces can consume without reclassification or host-local repair
- one closed naming rule set for target evidence so "declared target",
  "resolved target", and "authoritative target binding" do not blur together
- inspection accessors and batch inspection sections that expose the same
  evidence without domains reaching into raw deltas

Must preserve:

- touched aspects remain part of the contract; target evidence does not replace
  fallout meaning
- target evidence is generic runtime vocabulary, not domain-specific nouns
- preview receipts and authoritative receipts use the same conceptual model

This phase is complete only when one engineer can point at one concrete public
type for target evidence and another engineer cannot accidentally mint a weaker
"just use metadata" substitute in ordinary downstream code.

### Phase 2: Make Batch And Session Evidence Honest

Treat authoritative mutation sessions as bulk authority artifacts rather than
scalar last-write shadows.

Must ship:

- batch receipts and inspection that preserve per-component and aggregate
  target evidence
- batch/session causality bundles that preserve source declaration identity,
  authority-transition identity, and aggregate provenance rather than leaving
  downstream code to summarize the session itself
- explicit counters and summaries for target-class breadth, resolved-target
  breadth, and authored-metadata breadth
- public aggregation rules for mixed insert/update/delete sessions
- authoritative import/session helpers that preserve one inspectable batch
  artifact rather than teaching domains to inspect only the final component
- bridge-side aggregation rules so multi-write carry-forward evidence does not
  fragment into per-component-only provenance that Query must restitch

Must preserve:

- batch aggregation remains a summary of canonical component receipts rather
  than a second mutation truth source
- bulk import lanes stay explicit about target breadth and fallout breadth

This phase is complete only when bulk authoritative import, ordered write batch,
and single-write mutation each produce inspection artifacts that are different
only where the semantic boundary is actually different.

### Phase 3: Existing-Truth Identity Binding

Add a generic admitted path for mutations that target already-existing
authoritative truth.

Must ship:

- a public existing-truth target binding contract that can carry authoritative
  identity without collapsing it into ad hoc string reuse
- causality and provenance rules for existing-truth binding so the runtime can
  explain why a preexisting target was selected, denied, or rebound
- typed denial for unresolved, mismatched, or unsupported existing-truth target
  bindings
- batch-safe rules for symbolic same-batch references and existing-truth
  references living in one mutation session
- bridge-carried existing-truth binding artifacts that preserve how lower
  authority resolved, denied, or rebound the target
- inspection evidence that distinguishes:
  - newly created target
  - existing authoritative target
  - same-batch symbolic target
  - denied or unresolved target binding

Must preserve:

- Query does not become the owner of truth identity semantics
- lower runtimes still decide whether a supplied authoritative identity binding
  is valid
- domains do not need to mint a parallel target-id registry above Query

This phase is complete only when an existing-truth mutation can fail typed and
early with an explicit binding artifact instead of later surfacing as a vague
"target not found" or "wrong type" runtime surprise.

### Phase 4: Naming-Aware Authority Evidence

Make projected naming writeback and authoritative naming attachment a first-
class generic contract neighbor.

Must ship:

- a public mutation evidence family that can preserve naming attachment intent
  and outcome class without requiring domains to inspect opaque side channels
- provenance fields that let later reads and certification know which naming
  attachment or rebinding path produced the current outcome
- enough target-binding structure to say whether naming was:
  - attached to a new target
  - attached to an existing target
  - rebound from one target to another
  - removed
  - denied as ambiguous or unsupported
- inspection and receipt evidence that preserves naming attachment outcome
  alongside ordinary target evidence
- typed denial for unsupported naming-writeback families
- bridge-carried naming/writeback provenance strong enough that Query does not
  have to synthesize naming outcome meaning after the fact

Must preserve:

- Query does not invent persistent naming semantics; it carries and exposes the
  authority evidence lower runtimes and domains already own
- ordinary CRUD remains domain-neutral and does not become naming-shaped by
  default

This phase is complete only when naming attachment, rebind, removal, and denial
can each be named concretely in receipts and inspection without reading
domain-local glue code.

### Phase 5: Continuity And Lineage-Aware Authority Evidence

Make room for full-caliber continuity-sensitive mutation without requiring a
second explanation runtime above Query.

Must ship:

- a public evidence extension for continuity-sensitive mutations where the
  authoritative outcome may preserve, deny, or ambiguously classify identity
  continuity
- first-class provenance and causality evidence for continuity-sensitive
  outcomes so lineage-aware domains do not need a second runtime just to follow
  identity transition breadcrumbs
- typed distinction between ordinary target mutation and continuity-aware
  mutation evidence
- inspection bundles that can expose continuity class, denial class, and basis
  identity without forcing domains to rediscover lineage meaning from raw lower
  artifacts
- support/admission rows that keep unimplemented continuity families fail-closed
- bridge-side continuity carry-forward rules that preserve authoritative
  lineage/continuity outcomes into one Query-facing evidence model

Must preserve:

- `forge-relational` remains authoritative for lineage and continuity truth
- Query carries continuity evidence; it does not decide lineage semantics
- unsupported continuity cases deny explicitly rather than widening into plain
  updates that lose identity meaning

This phase is complete only when continuity-sensitive mutation can either carry
one explicit runtime-owned provenance chain forward or stop with a typed denial
before semantic drift occurs.

### Phase 6: Certification And Dependency Contract Closeout

Close the gate with certification-grade proof and an explicit downstream
dependency contract.

Must ship:

- a closeout doc naming what mutation evidence is safe to build on now
- named certification suites in `test-requirements.md`
- named certification suites in `../forge-runtime-bridge/test-requirements.md`
- runtime API stabilization and support-matrix tests extended for the new
  mutation evidence surfaces
- bridge replay/causality tests extended for the same end-to-end evidence story
- migration guidance for downstream domains that currently carry local
  identity-binding or naming-writeback glue

Must preserve:

- the public mutation surface stays aspect-native
- deferred temporal/async/store/durable neighbors remain deferred
- domains do not learn expert compatibility seams as the ordinary path

## Representative Scenario Matrix

This gate must certify concrete authority lanes, not just abstract categories.

At minimum, the closeout matrix should include:

- `new-target-insert-with-causality`:
  one ordinary insert carries declared target class, resolved target class,
  source declaration identity, and provenance bundle
- `existing-target-update-with-binding`:
  one update lowers through admitted existing-truth binding and preserves why
  the authoritative target was selected
- `existing-target-delete-with-clear-fallout`:
  one delete preserves target evidence, touched-aspect fallout, and delete
  causality together
- `mixed-batch-symbolic-and-existing-targets`:
  one ordered batch mixes same-batch symbolic references and existing-truth
  bindings without collapsing their evidence into one scalar receipt
- `authoritative-import-session-aggregate`:
  one import/session proves aggregate authority evidence is not just the final
  component re-labeled
- `naming-attachment-rebind`:
  one admitted naming-aware mutation preserves attach-vs-rebind outcome and
  provenance
- `continuity-aware-rebind-or-denial`:
  one continuity-sensitive mutation either preserves a continuity evidence chain
  or yields one typed denial with explicit reason
- `unsupported-existing-target-family-denied`:
  one hostile lane requests an unsupported binding family and fails before
  execution
- `unsupported-naming-family-denied`:
  one hostile lane requests naming-aware behavior on an unadmitted family and
  fails typed and early
- `unsupported-continuity-family-denied`:
  one hostile lane requests continuity-sensitive mutation on an unadmitted
  family and fails typed and early

## Must Ship

- generic public target evidence vocabulary for direct writes and batches
- explicit declared-versus-resolved target evidence in receipts and inspection
- first-class causality and provenance bundles on receipts, inspection, and
  batch/session summaries
- aggregate batch/session evidence for authoritative mutation sessions
- admitted existing-truth identity-binding contract with typed denials
- naming-aware mutation evidence sufficient for projected naming writeback
- continuity-aware mutation evidence sufficient for future lineage-sensitive
  domains
- support-matrix rows and certification suites covering the new surfaces
- a downstream dependency contract closeout
- bridge-side carry-forward artifacts and replay-safe causality/provenance
  guarantees sufficient for Query to expose the same meaning publicly

## Must Preserve

- aspect-native CRUD remains the ordinary public mutation story
- touched-aspect fallout remains explicit and auditable
- causality and provenance remain runtime-carried evidence rather than optional
  domain-local adornment
- lower runtimes remain authoritative for truth, naming, writeback, and
  lineage semantics
- bridge-owned causality transfer remains the only admitted cross-runtime
  carry-forward path for this evidence family
- Query remains the facade and explanation carrier, not a second mutation
  engine
- unsupported families fail closed rather than degrading into best-effort
  target recovery

## Complexity / Proof Obligations

This gate must make authority evidence cost visible and testable.

Minimum named counters should include materially equivalent versions of:

- `mutation_target_evidence_count`
- `mutation_resolved_target_count`
- `mutation_existing_truth_binding_count`
- `mutation_symbolic_target_reference_count`
- `mutation_batch_component_count`
- `mutation_batch_aggregate_evidence_count`
- `mutation_causality_edge_count`
- `mutation_provenance_record_count`
- `mutation_naming_evidence_count`
- `mutation_continuity_evidence_count`
- `mutation_binding_denial_count`
- `mutation_naming_denial_count`
- `mutation_continuity_denial_count`
- `mutation_evidence_rediscovery_count`

Rules:

- counters belong on receipts, inspection bundles, certification bundles, or
  equivalent canonical artifacts, not only on debug logs
- `mutation_evidence_rediscovery_count` must be exactly zero on admitted paths
- every denied existing-truth binding must increment
  `mutation_binding_denial_count`
- every denied naming-aware family must increment
  `mutation_naming_denial_count`
- every denied continuity-aware family must increment
  `mutation_continuity_denial_count`
- aggregate batch/session counters must be derived once at the session boundary,
  not recomputed independently by each consumer

## Acceptance Evidence

This plan is complete only when `forge-query` can prove:

- direct insert/update/delete/batch receipts preserve target evidence and
  touched-aspect evidence together
- direct insert/update/delete/batch receipts preserve causality/provenance
  alongside target evidence rather than flattening them into incidental metadata
- authoritative and preview receipts expose the same target-evidence concepts
- batch/session inspection preserves aggregate authority evidence honestly
- existing-truth binding failures deny typed and early
- naming-aware and continuity-aware mutation families either preserve explicit
  outcome evidence or deny typed and early
- lineage/provenance and authority-causality breadcrumbs survive the public
  runtime boundary without downstream reconstruction
- bridge replay and diagnostics artifacts agree with the Query-facing receipt
  and inspection story rather than exposing a second incompatible provenance
  model
- support metadata, inspection bundles, and executable admission behavior stay
  in sync
- downstream domain tests can delete local target-recovery glue rather than
  merely wrapping it

Required verification commands at closeout:

- `cargo fmt -p forge-query`
- `cargo check -p forge-query --tests`
- `cargo test --manifest-path crates/forge-query/Cargo.toml --test phase_boundaries_compile_fail`
- `cargo test -p forge-query`
- targeted mutation evidence certification tests
- targeted runtime public support tests
- `cargo fmt -p forge-runtime-bridge`
- `cargo check -p forge-runtime-bridge --tests`
- `cargo test -p forge-runtime-bridge`
- targeted bridge causality/writeback certification tests
- `git diff --check`

## Roadmap Placement

This belongs immediately after the Runtime API Public Stabilization Gate on the
Query side, and as an explicit cross-reference hardening gate over the already
claimed bridge writeback/causality surfaces before downstream domains rely on
them as one end-to-end contract.

Why here:

- the public facade is already stabilized enough for downstream domains to use
  now
- the next serious blocker is not temporal/async semantics; it is authority-
  mutation honesty for write-heavy domains
- Worth Phase 6 is already surfacing this gap in a way that other domains will
  also hit around writeback, naming, and continuity
- the bridge already claims causality-transfer and writeback strength, so the
  honest next step is to bind that bridge promise to the Query public contract
  explicitly instead of leaving a soft seam between them
- later temporal/async milestones should extend a mutation contract that is
  already semantically honest, not one that still loses target identity

## Architectural Notes

- target evidence and touched-aspect evidence are separate and both required
- causality and provenance are first-class runtime evidence, not "nice to have"
  metadata layers domains can reconstruct later
- batch/session authority evidence is the mutation analogue of batch-derived
  summaries on the read side
- existing-truth binding is a public contract because serious domains cannot
  honestly avoid it
- naming-aware and continuity-aware mutation evidence are capability families;
  they must be admitted or denied explicitly
- this gate should reduce downstream code by moving authority explanation into
  Query rather than teaching each domain to rebuild it

## Explicit Failure Taxonomy

- unsupported existing-truth binding family
- unresolved existing-truth target binding
- mismatched target-class binding
- unsupported symbolic same-batch target reference
- unsupported naming-writeback family
- ambiguous naming attachment outcome
- unsupported continuity-sensitive family
- denied continuity classification
- raw lower-runtime provenance passthrough attempt
- host override of resolved target evidence
- host reconstruction of causality/provenance after the runtime boundary
- scalarized batch/session evidence that drops component authority meaning

## Naive Traps Explicitly Rejected

- treating `collection + entity_identity` as sufficient long-term authority
  evidence for every mutation family
- packing causality or provenance into an untyped metadata map and calling the
  job done
- letting domains attach their own naming/continuity side packets beside Query
  receipts because "the runtime doesn't know enough yet"
- using the final component receipt as the meaning of a bulk import or ordered
  mutation session
- degrading unsupported existing-truth, naming, or continuity families into
  ordinary update semantics
- assuming the bridge can silently drop causality/provenance and that domains
  can rehydrate it later from lower-crate internals
- exposing proof-bearing evidence types with public constructors because it is
  "just docs/spec work for now"

## Explicit Non-Goals

- inventing new persistent naming semantics
- inventing new lineage or continuity semantics
- making Query the execution owner of writeback protocols
- implementing temporal or async mutation families
- claiming store-backed restart or durable continuation behavior
- adding domain-specific target classes to the generic runtime facade

## Self-Check

- Does this solve a real structural problem or just package work cosmetically?
  It solves the missing authority-evidence seam that write-heavy downstream
  domains are already exposing.
- Is the adversarial constraint precise and load-bearing? Yes. It names the
  exact information domains must not be forced to recover locally.
- Does the plan preserve crate authority boundaries? Yes. Query owns the
  facade contract; lower runtimes keep mutation truth, naming truth, and
  lineage truth.
- Does the plan define proof obligations, not just implementation tasks? Yes.
  Batch/session evidence, typed denial, support synchronization, and downstream
  code-deletion proof are all explicit.
- Could a competent engineer map this into honest types, modules, and tests?
  Yes. The phases map directly to mutation vocabularies, receipts, inspection,
  support rows, and certification suites.
- Does this belong in the roadmap sequence? Yes. It is the immediate runtime
  hardening gate that downstream write-authority work needs before temporal and
  async expansion.
