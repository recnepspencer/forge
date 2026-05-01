# Forge Query Runtime Generic Graph Authoring And Identity-Preserving Existing-Truth Plan

> **Status:** Proposed upstream hardening gate
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Vision parent:** [forge_query_vision.md](./forge_query_vision.md)
>
> **Test requirements:** [test-requirements-milestone-9_3-and-runtime-gates.md](./test-requirements-milestone-9_3-and-runtime-gates.md)
>
> **Primary predecessor:** [runtime-authoritative-mutation-evidence-plan.md](./runtime-authoritative-mutation-evidence-plan.md)
>
> **Primary downstream pressure:** [../worth/forge-query-runtime-kernel-hard-break.md](../worth/forge-query-runtime-kernel-hard-break.md)
>
> **Primary owners:** `forge-query`, `forge-runtime-bridge`, and production runtime adapters in downstream domains
>
> **Prerequisite milestones and gates:**
> - [runtime-api-public-stabilization-plan.md](./runtime-api-public-stabilization-plan.md)
> - [runtime-authoritative-mutation-evidence-plan.md](./runtime-authoritative-mutation-evidence-plan.md)
>
> **Concurrent downstream programs:**
> - [../worth/forge-query-runtime-kernel-hard-break.md](../worth/forge-query-runtime-kernel-hard-break.md)
>
> **Impacted later roadmap work:**
> - `Milestone 9.4` (`Temporal Query Basis And Time-Aware Subscription Contracts`)
> - `Milestone 9.5` (`Async And Resource-Backed Query Families`)
> - `Milestone 10` (`Store-Backed Execution And Historical Parity`)
> - `Milestone 11` (`Durable Saved-Query, Cursor, And Artifact Reload Semantics`)
>
> **Primary architectural driver:** make graph-shaped mutation authoring
> physically executable through one ordinary Query runtime contract without
> allowing downstream domains to reintroduce shadow runtime semantics for
> relation rewrites, subgraph composition, or backend verification

## Goal

Harden Forge Query's public mutation/runtime surface so serious downstream
domains can use Query as the ordinary graph-authoring runtime without keeping
domain-local substitutes for:

- identity-preserving existing-target relation rewrites
- invariant-complete same-batch graph composition
- backend-verified existing-truth checks on real bridge-backed runtimes

The result must be a domain-agnostic authoring contract that lets downstream
domains express graph-shaped workflows through Query-native mutation authoring,
receipts, inspection, support metadata, and typed denial rather than through
private authority glue or mirror runtimes.

## Why This Plan Exists

The runtime authoritative mutation evidence gate solved a major part of the
problem: public receipts now preserve target evidence, batch/session authority
meaning, causality/provenance, existing-truth bindings, and admitted verified
mutation families strongly enough that downstream domains can stop rebuilding
explanation layers locally.

That was necessary. It is not yet sufficient for Query to feel like the real
runtime.

Downstream kernel pressure now exposes three remaining gaps that are still too
generic to leave inside domain-local adapters:

1. existing-target relation rewires need a true identity-preserving update lane
   rather than delete-plus-recreate disguise
2. same-batch graph authoring needs a first-class public composition surface
   rather than a fragile pile of scalar symbolic writes
3. backend-verified existing-truth checks need a stable ordinary story on real
   bridge-backed runtimes, not just on memory or compatibility slices

If these gaps are not solved in Forge Query itself:

- downstream domains will author relation rewires through local shadow
  semantics that violate existing-target meaning
- graph-shaped create-plus-attach workflows will drift into one-off builder
  tricks rather than one generic runtime story
- backend-verified checks will remain technically available in principle but
  operationally incomplete on the production runtime paths that matter
- Query will continue to look like the runtime while domains quietly keep the
  real hard mutation semantics above or beside it

This plan exists to close those generic substrate gaps once, upstream, so the
same public mutation/runtime facade can honestly serve as the daily-driver
runtime for graph-shaped domains.

## Hard Part

The hard part is not adding three more facade methods.

The hard part is keeping five things separate that a weaker runtime will blur
together the moment downstream pressure increases:

- existing-target mutation semantics that preserve target identity
- replacement workflows that only look update-shaped at the API boundary
- same-batch graph composition semantics that preserve symbolic intent,
  ordering, and resolved-target meaning
- backend-verified existing-truth semantics that depend on lower-runtime truth
  authority rather than Query-local assertions
- support metadata and certification evidence that must report the same truth
  as the runtime without requiring internal-code archaeology

The design fails if:

- a relation "update" is implemented as create-plus-delete under a nicer name
- graph composition is really just scalar batch mutation plus raw symbolic
  string folklore
- bridge-backed verification is documented as ordinary while production
  runtimes still deny or degrade it silently
- support metadata says "supported" for substrate that only works on
  compatibility or memory runtimes
- downstream domains still need private authoring builders to express one
  invariant-complete graph workflow honestly

This plan therefore has to define one exact public authoring contract, one
runtime-support contract, and one proof contract that all three missing
capabilities inherit.

## Explicit Assumptions

- the Runtime API Public Stabilization Gate remains the governing public facade
  shape for workspaces, handles, inspection, and support metadata
- the Runtime Authoritative Mutation Evidence Gate remains the governing
  evidence story for target binding, causality, provenance, batch/session
  aggregation, and typed denial
- lower runtimes and bridges remain authoritative for relation identity,
  replay, merge, naming, and verification semantics
- Query may orchestrate mutation authoring and evidence, but it may not invent
  truth identity semantics that the lower runtime cannot preserve
- same-batch symbolic references already exist as substrate, but they are not
  yet sufficient on their own to count as a first-class graph composition
  surface
- downstream domains such as Worth are valid pressure tests for generic
  runtime capability, but they do not get to define the public Query contract
- production bridge-backed runtime support is the governing support bar; memory
  and compatibility runtimes may lead implementation, but they do not define
  completion

## Governing Summaries

- `MENTALITY.md`
  The most important thing it protects is solving the real substrate gap rather
  than widening pretty wrappers. This plan therefore starts from the hostile
  downstream cases that would force shadow runtimes back into existence.
- `arch_laws.md`
  The most important thing it protects here is explicit proof-bearing public
  surfaces. Existing-target update, graph composition, and verified
  existing-truth support must become typed, inspectable, and support-reportable
  runtime contracts rather than metadata conventions.
- `perf_laws.md`
  The most important thing it protects is breadth and coordination honesty.
  Graph composition and verified existing-truth support must lower once and
  execute once; they may not broaden into repeated rediscovery, implicit scans,
  or domain-local patchup passes.
- `domain_laws.md`
  The most important thing it protects is responsibility separation. Generic
  graph authoring and existing-target mutation semantics belong in Query; the
  downstream domain should still own vocabulary, invariants, and interpretation.
- `forge_query_vision.md`
  The most important thing it protects is the product promise that developers
  declare query intent once and reuse one runtime surface for reads, writes,
  subscriptions, branches, history, and explanation. This plan extends that
  promise into graph-shaped authoritative mutation.
- `forge_query_roadmap.md`
  The most important thing it protects is roadmap sequencing. This plan belongs
  under the Runtime Authoritative Mutation Evidence Gate and before downstream
  domains widen more workflow families on top of incomplete generic substrate.
- `test-requirements-milestone-9_3-and-runtime-gates.md`
  The most important thing it protects is certification-grade proof. This plan
  must end in named runtime-gate suites with hostile rows, compile-fail
  boundaries, and exact digest/counter output rather than feature demos.
- `runtime-authoritative-mutation-evidence-plan.md`
  The most important thing it protects is the existing public evidence model.
  This plan must extend that model without inventing a second mutation contract
  or weakening the already-shipped target/causality/provenance story.
- `runtime-api-public-stabilization-plan.md`
  The most important thing it protects is beautiful ordinary DX. The new
  surfaces must feel like stable public runtime authoring, not expert-only
  compatibility seams.
- `../worth/forge-query-runtime-kernel-hard-break.md`
  The most important thing it protects is the downstream kernel bar. Worth
  should not solve generic graph authoring, verified mutation, or identity-
  preserving rewires locally; if those are needed, the fix belongs here.

## Adversarial Constraint

For the same graph-shaped workflow expressed through the public Query facade,
the runtime must preserve the same target identity meaning, graph-composition
meaning, verification meaning, and receipt/inspection meaning regardless of
whether the workflow:

- mutates an existing relation in place
- creates several entities and relations in one same-batch subgraph
- verifies existing truth before mutate/delete continuation
- executes live on a production bridge-backed runtime
- is denied because the runtime cannot yet preserve the required meaning

If any admitted path:

- rewrites an existing-target relation by deleting one identity and creating a
  new one under an "update" label
- makes graph-shaped authoring depend on caller-owned ordering folklore or raw
  symbolic identity strings without one composition contract
- exposes a verified existing-truth surface in the facade but leaves production
  bridge-backed runtimes unable to admit it honestly
- forces downstream domains to distinguish "real runtime support" from
  "technically present but not production-grade" by reading internal code

then this plan has failed.

The public authoring surface must make the same canonical meanings available on
real runtime paths or deny typed and early before domains are tempted to fill
the gap themselves.

## Product Decision Lock

- Query remains a domain-agnostic mutation/runtime facade; it does not become a
  topology engine, CAD kernel, workflow author, or naming semantic authority.
- Existing-target update means identity-preserving update. Delete-plus-recreate
  may not masquerade as that surface.
- Same-batch graph authoring must be a first-class public composition story,
  not a downstream convention over scalar batch operations.
- Backend-verified existing-truth support counts as "supported" only when real
  bridge-backed runtimes can admit it honestly through the ordinary facade.
- Receipts, inspection, support metadata, and typed denials must all agree on
  the new authoring families.
- If a required capability is missing, the fix belongs in Forge Query and its
  bridge/runtime contracts, not in a domain-local wrapper that restores the old
  dual-runtime shape.

Normative consequence:

- any implementation that exposes relation update while changing authoritative
  relation identity is out of spec
- any implementation that teaches graph composition as "just use batch plus
  strings carefully" is out of spec
- any implementation that documents verification as ordinary while support
  metadata or bridge-backed runtimes still deny the same family is out of spec
- any implementation that makes downstream domains compute support posture from
  internal bridge code instead of public support artifacts is out of spec

## Scope

### In Scope

- one exact public authoring family for identity-preserving existing-target
  relation update
- one exact public authoring family for same-batch graph composition
- one exact public bridge-backed support/admission family for backend-verified
  existing-truth operations on admitted entity and relation families
- receipt, inspection, support-profile, and support-matrix widening for all of
  the above
- compile-fail, support-closeout, and hostile certification proof surfaces
- public feature docs and roadmap/closeout updates that teach the new runtime
  literally

### Explicitly Out Of Scope

- topology-specific, workflow-specific, CAD-specific, or naming-specific
  semantic helpers
- downstream invariant packs for any one domain's complete graph workflow
- lower-runtime implementations of relation identity mutation that are not
  actually supported by the underlying truth engine
- store-backed historical parity, temporal basis semantics, or async resource
  scheduling beyond the support/reporting obligations needed to keep this work
  honest

## Required Runtime Contracts And Counters

The runtime must emit machine-checkable counter bundles for this gate instead
of only human-readable summaries.

Required output bundle:

- `counter_snapshot`

Rules:

- `counter_snapshot` must be emitted by named certification suites for this
  gate
- counters attach to authoring-family work units rather than one rolled-up
  "graph mutation" total
- support-closeout docs must quote the same counter families the tests certify

### Identity-Preserving Update Contract

Identity-preserving existing-target update must preserve one authoritative
target identity before and after mutation.

Required contract surfaces:

- `DeclaredExistingTarget`
- `ResolvedExistingTarget`
- `ExistingTargetIdentityDigest`
- `IdentityPreservingUpdateSupportVerdict`
- typed denial taxonomy for unsupported identity-preserving update families

Required counters and outputs:

- existing-target update component count
- target-identity-preserved count
- target-identity-denied count
- touched-aspect breadth
- affected live/computed breadth

Rules:

- no admitted update path may change authoritative relation identity
- identity-preservation success or denial must be visible in receipt,
  inspection, and support metadata
- batch/session summaries must not collapse identity-preserving update into the
  same family meaning as create-plus-delete
- lowerings that require delete-plus-create replacement must deny before
  mutation execution begins rather than after a provisional plan is built

### Graph Composition Contract

Graph composition must preserve symbolic authoring intent as one explicit
runtime program rather than one caller-owned ordering convention.

Required contract surfaces:

- `GraphCompositionProgram`
- `GraphCompositionSymbol`
- `GraphCompositionResolutionMap`
- `GraphCompositionSupportVerdict`
- typed denial taxonomy for unresolved, illegal, or unsupported composition
  edges

Required counters and outputs:

- symbolic entity count
- symbolic relation count
- existing-target edge count
- symbolic-resolution count
- graph breadth and component-order count

Rules:

- symbolic references must be typed handles, not raw public strings
- composition must lower once into canonical mutation commands
- receipts and inspection must expose symbolic-to-resolved mapping explicitly
- mixed existing-target and symbolic edges are part of first-ship completion;
  "symbolic entity creation only" does not count as generic graph composition
- composition lowering may not rediscover graph ordering or target shape by
  rereading workspace state during execution

### Bridge-Backed Verification Contract

Bridge-backed verification support must remain ordinary only when the real
runtime can execute it with the same public meaning as the facade advertises.

Required contract surfaces:

- `BridgeBackedVerificationSupportVerdict`
- per-family verification support rows
- typed denial taxonomy for unsupported verification substrate

Required counters and outputs:

- verified-assertion family count
- verified-update family count
- verified-delete family count
- verification-denial count by family

Rules:

- support must be reported per family, not as one vague verification bool
- production bridge-backed runtimes are the completion bar
- compatibility or memory support may not be reported as ordinary production
  support
- verification denials must distinguish unsupported bridge substrate from
  target-shape or collection mismatch so downstream callers can react honestly

## Compile-Time Boundary Rule

The public API must make the proof-bearing runtime boundary mechanically
enforceable instead of convention-based.

Required compile-fail boundaries:

- external callers cannot mint graph-composition symbols directly
- external callers cannot mint graph-composition resolution maps directly
- external callers cannot mint identity-preserving update support verdicts
  directly
- external callers cannot mint bridge-backed verification support verdicts
  directly
- external callers cannot submit raw symbolic string identities where typed
  graph-composition handles are required
- external callers cannot fabricate proof-bearing inspection/evidence handles
  for identity-preserving update or graph-composition families

Required implementation consequence:

- proof-bearing authoring artifacts are constructed only by Query-owned
  builders, lowering, execution, and inspection paths
- facade-visible types should make the illegal states above unrepresentable or
  uncompilable rather than merely documented as forbidden

## Required Public Authoring Surfaces

The resulting runtime surface must make these authoring shapes ordinary,
typed, and inspectable.

### Identity-Preserving Existing-Target Relation Update

```rust
let binding = workspace.bind_existing_relation(
    ForgeQueryExistingRelationTarget::new(
        format!("{relation_id:?}"),
        live_query_identity.clone(),
    )?
    .in_target_collection("WorthTopologyRelation")?,
)?;

let receipt = workspace.update_existing_verified(
    binding,
    |assertion| {
        assertion
            .aspect("topology.kind", "worth.half_edge_next")
            .aspect("topology.source_identity", old_source_identity.clone())
            .aspect("topology.target_identity", old_target_identity.clone())
    },
    |update| {
        update
            .aspect("topology.target_identity", new_target_identity.clone())
            .touches(["topology.boundary", "diagnostics.decisions"])
            .metadata("domain.family", "RewireLoopSuccessor")
    },
)?;
```

### First-Class Same-Batch Graph Composition

```rust
let receipt = workspace.compose_graph(|graph| {
    let inner_loop = graph.insert_entity("WorthTopologyEntity", |insert| {
        insert
            .aspect("topology.kind", "worth.loop")
            .aspect("topology.structure", "cube.face.inner_loop")
            .aspect("naming.persistent_name", "cube.face.inner_loop")
    })?;

    graph.insert_relation("WorthTopologyRelation", |insert| {
        insert
            .aspect("topology.kind", "worth.face_inner_loop")
            .source_existing(face_identity)
            .target_symbolic(inner_loop.identity())
            .touches(["topology.boundary", "diagnostics.decisions"])
    })?;

    Ok(())
})?;
```

### Bridge-Backed Backend-Verified Existing-Truth Check

```rust
let binding = workspace.bind_existing_entity(
    ForgeQueryExistingEntityTarget::new(
        format!("{entity_id:?}"),
        query_identity.clone(),
    )?
    .in_target_collection("WorthTopologyEntity")?,
)?;

let receipt = workspace.verify_existing(binding, |assertion| {
    assertion
        .aspect("topology.kind", "worth.vertex")
        .aspect("naming.persistent_name", "cube.vertex.1")
})?;
```

These are required output standards, not suggestive examples. Equivalent names
are acceptable only if the semantic contract remains equally explicit.

## First-Ship Scope Rule

The first ship may be conservative. It may not be vague.

Required first-ship topology:

- one public workspace-owned graph composition entry surface
- one typed graph-composition builder context
- one typed symbolic-handle family returned from that builder context
- one canonical lowering artifact for composed graph programs
- one support verdict family for identity-preserving update support
- one support verdict family for bridge-backed verification support
- one receipt/inspection evidence path that reuses the same public mutation
  evidence story as ordinary scalar mutation

Required first-ship posture:

- identity-preserving relation update support must be declared per admitted
  target family
- graph composition support must be declared per admitted edge/target family
- bridge-backed verification support must be declared per admitted verification
  family
- denial posture must be emitted through the same support artifacts and
  inspection vocabulary as success posture

Explicit first-ship debt:

- relation families that still require lower-runtime identity support before
  admission
- graph workflows that still require downstream invariant-complete subgraph
  widening
- verification families that remain unavailable on production bridge-backed
  runtimes even if memory or compatibility runtimes can prove them earlier

Allowed first-ship conservatism:

- entity-family support may land before relation-family support where the lower
  runtime genuinely differs
- one graph-composition facade may ship with a narrower admitted workflow set
  than scalar batch mutation
- bridge-backed verification may ship per family instead of universally

Required first-ship honesty:

- every admitted family must be named explicitly in support metadata
- every denied neighbor must be named explicitly in support metadata
- docs and examples must use only admitted first-ship families unless a denial
  example is the point
- first-ship debt must be recorded as explicit blocked families, not implied by
  silence

Not allowed as first-ship debt:

- relation replacement disguised as identity-preserving update
- raw string symbolic references as the public composition contract
- bridge-backed verification documented as ordinary when only compatibility or
  memory runtimes support it
- support artifacts that say "generic graph authoring supported" while only one
  narrow builder path actually works

## Phases

### Phase 1: Freeze The Public Authoring Vocabulary

Lock one coherent public vocabulary for the three missing substrate families
before implementation spreads the wrong names.

Must ship:

- one public authoring family for identity-preserving existing-target relation
  update
- one public authoring family for same-batch graph composition
- one public support/admission family for bridge-backed backend-verified
  existing-truth checks
- typed receipt and inspection accessors that preserve the same evidence story
  already required by the authoritative mutation evidence gate
- support-matrix rows that distinguish:
  - stable public runtime surfaces
  - admitted but bridge-backed-runtime-incomplete surfaces
  - unsupported neighbors
- compile-fail boundaries for proof-bearing support verdicts and graph
  composition evidence artifacts

Must preserve:

- aspect-native insert/update/delete/batch remain the ordinary vocabulary
- new graph-oriented authoring surfaces compose with existing target evidence
  rather than replacing it
- public names remain domain-neutral and cost-honest

This phase is complete only when a downstream engineer can tell, from public
types and support metadata alone, which graph-authoring/runtime surfaces are
stable, denied, or still deferred.

### Phase 2: Identity-Preserving Existing-Target Relation Updates

Add a real existing-target relation update lane whose semantics preserve target
identity rather than hiding a replacement workflow behind update-shaped names.

Must ship:

- a public relation binding path for existing-target updates on ordinary
  bridge-backed runtimes
- `update_existing(...)` and `update_existing_verified(...)` support for
  relation targets where the authoritative relation identity remains the same
  before and after mutation
- explicit typed denial for:
  - unsupported identity-preserving relation update families
  - collection mismatch
  - target-shape mismatch
  - backend verification unsupported
- receipt and inspection evidence that preserve:
  - binding family
  - declared target
  - resolved target
  - target identity digest
  - causality/provenance digest
  - touched-aspect fallout
- exact denial taxonomy rows for:
  - identity-not-preservable on this runtime
  - unsupported relation family
  - target mismatch
  - verification unavailable on this runtime

Must preserve:

- Query does not invent relation identity semantics; lower truth runtimes still
  decide whether a relation can be updated in place
- if the lower runtime cannot preserve identity, Query denies typed and early
- batch/session aggregate evidence remains honest about this mutation family

This phase is complete only when an existing-target relation rewrite can be
expressed through the ordinary public facade without delete-plus-recreate
disguise and without domain-local target recovery.

### Phase 3: First-Class Composed Graph Authoring

Make same-batch graph construction an explicit public runtime capability rather
than a downstream convention over scalar batch operations.

Must ship:

- one public composition surface for same-batch graph authoring, such as
  `compose_graph(...)` or an equally explicit family
- explicit symbolic entity/relation handles produced within the composition
  block and reused through typed identity references rather than raw strings
- composition-level receipts and inspection that preserve:
  - component ordering
  - symbolic-to-resolved target mapping
  - graph breadth counters
  - affected live/computed breadth
  - typed denial for unresolved or illegal composition edges
- support for mixed existing-target and same-batch symbolic references inside
  one composition block
- compile-fail boundaries that prevent public fallback to raw symbolic strings

Must preserve:

- composition lowers once into canonical mutation plans; execution does not
  rediscover the graph shape at the hot path
- composition does not hide domain invariants; unsupported or incomplete graph
  workflows still deny typed and early
- scalar batch APIs remain available for non-graph workflows

This phase is complete only when downstream domains can author one admitted
multi-entity, multi-relation subgraph through one public composition surface
without stitching together raw symbolic identity folklore themselves.

### Phase 4: Bridge-Backed Backend-Verified Existing-Truth Execution

Turn backend-verified existing-truth checks into a real ordinary runtime
capability on production bridge-backed runtimes.

Must ship:

- bridge-backed runtime support for:
  - `verify_existing(...)`
  - `probe_existing(...)`
  - `update_existing_verified(...)`
  - `delete_existing_verified(...)`
  on admitted entity and relation families
- public support metadata that reports bridge-backed verification posture
  honestly by family
- typed denial for bridge-backed runtimes that lack required verification
  substrate
- verification receipts and inspection bundles that preserve the same target
  evidence, causality, provenance, and mode distinction already frozen by the
  mutation-evidence gate
- explicit support rows that distinguish:
  - admitted on production bridge-backed runtimes
  - admitted only on non-production runtimes
  - denied everywhere

Must preserve:

- verification support remains lower-runtime-authority-dependent rather than
  fabricated by Query
- unsupported families fail closed rather than degrading into retained local
  assertions with the same public shape
- production runtimes and memory/compatibility runtimes remain phase-typed in
  support posture

This phase is complete only when backend-verified existing-truth support is
ordinary and support-reportable on real bridge-backed runtimes, or denied typed
and early with no ambiguity.

### Phase 5: Support, Documentation, And Certification Closeout

Close the gate with machine-checkable proof, frozen support metadata, and
developer-facing documentation that teaches the new runtime honestly.

Must ship:

- support-matrix rows and support-profile tests for all newly admitted or
  denied authoring families
- compile-fail boundaries preventing external minting of proof-bearing support,
  closeout, or graph-composition evidence artifacts where appropriate
- feature docs that show:
  - identity-preserving relation update authoring
  - graph composition authoring
  - bridge-backed verification authoring
  - typed denial and support-report reading
- roadmap and closeout documents updated so downstream domains can cite one
  stable upstream contract instead of oral tradition

Must preserve:

- the public docs teach only admitted stable or explicitly denied/deferred
  surfaces
- compatibility or deprecated mutation seams remain documented as such and are
  not mixed into the new authoring story

This phase is complete only when public docs, roadmap placement, support
metadata, and certification suites all tell the same story.

## Must Ship

- one public identity-preserving existing-target relation update family
- one public composed graph authoring family
- one honest bridge-backed backend-verified existing-truth support story
- support metadata, receipts, and inspection for all of the above
- hostile certification suites and compile-fail boundaries for the new
  families
- public documentation updates that teach the new runtime authoring surfaces
  directly

## Must Preserve

- lower runtimes remain authoritative for truth identity, replay, naming, and
  verification semantics
- Query remains domain-agnostic and does not absorb topology-specific concepts
- unsupported or incomplete graph workflows fail typed and early
- existing target evidence, causality, provenance, and batch/session authority
  evidence remain one coherent public story
- public API stabilization vocabulary remains the governing facade shape

## Required Documentation Updates

This plan is not closed until documentation reflects the new support literally.

Must update:

- [forge_query_roadmap.md](./forge_query_roadmap.md)
  - place this gate intentionally under the runtime authoritative mutation
    evidence family
- [runtime-authoritative-mutation-evidence-plan.md](./runtime-authoritative-mutation-evidence-plan.md)
  - widen the gate narrative so the new authoring families are part of the
    same public evidence contract rather than a side appendix
- [test-requirements-milestone-9_3-and-runtime-gates.md](./test-requirements-milestone-9_3-and-runtime-gates.md)
  - add named certification suite rows for the new authoring families
- public feature docs for:
  - runtime mutation authoring
  - support matrix / admission reading
  - graph composition authoring
  - existing-truth verification authoring
- closeout docs for the runtime authoritative mutation evidence gate once this
  plan is implemented

Documentation must explicitly distinguish:

- stable public runtime support
- denied-but-planned neighbors
- compatibility or deprecated mutation seams
- first-ship debt that remains explicit after implementation
- any fallback or workaround examples must be labeled non-ordinary and may not
  appear before the canonical authoring examples in feature docs

## Acceptance Evidence

- one named certification suite:
  `Runtime Identity-Preserving Existing-Target Relation Update Test`
  proving identity-preserving relation updates remain identity-preserving in
  receipts, inspection, aggregate evidence, and support metadata
- one named certification suite:
  `Runtime Generic Graph Composition Test`
  proving graph composition preserves symbolic references, resolved target
  mapping, typed ordering meaning, graph breadth counters, and typed denial
  under hostile incomplete-subgraph conditions
- one named certification suite:
  `Runtime Bridge-Backed Verified Existing-Truth Support Test`
  proving verified existing-truth surfaces are admitted or denied honestly on
  bridge-backed runtimes by family, not by one vague support bit
- support-matrix, support-profile, and public closeout tests updated for the
  new families
- compile-fail tests proving external callers cannot mint proof-bearing support
  or composition evidence artifacts directly
- compile-fail tests proving external callers cannot fake graph-composition
  symbols, support verdict artifacts, or identity-preserving-update evidence
  handles directly
- feature docs and examples updated to teach the new surfaces and no longer
  imply the old narrower substrate story
- exact counter assertions for:
  - identity-preserved versus denied update families
  - symbolic entity/relation breadth
  - symbolic-resolution count
  - bridge-backed verification admitted versus denied count by family
- machine-checkable output bundles from each named certification suite
  including:
  - `truth_snapshot`
  - `inspection_snapshot`
  - `support_snapshot`
  - `counter_snapshot`

## Sequencing Notes

- This plan belongs under the Runtime Authoritative Mutation Evidence Gate,
  not as a separate parallel milestone, because it extends the same public
  mutation/runtime evidence story rather than defining a new runtime family.
- It should land before downstream domains widen more ordinary graph workflows
  on top of Query, because otherwise those domains will be forced to invent
  local substitutes for one of the three missing generic surfaces.
- It should precede further Worth kernel widening for relation create/attach
  and rewire families, because those families are the concrete hostile pressure
  that proves the gap is real.

## Architectural Notes

- The "graph authoring" surface is generic runtime vocabulary, not a topology
  vocabulary. It should be usable by any domain that authors related entities
  and relations in one batch.
- The "identity-preserving update" surface is the semantic opposite of
  delete-plus-recreate disguise. If the lower runtime cannot do it honestly,
  denial is the correct behavior.
- The bridge-backed verification requirement is partly a runtime-adapter
  hardening program, but it is still a Query plan because the public facade
  and support contract must expose the result honestly.
- This plan should produce one stable upstream dependency contract that the
  Worth hard-break program can cite directly when deleting more domain-local
  mutation glue.
