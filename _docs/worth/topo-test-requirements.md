# Worth Topology Certification Requirements

## Purpose

This document defines the permanent topology certification bar for Worth.

It is not a milestone checklist.
It is not a polite inventory of expected tests.
It is the standing topology war plan.

Its job is to answer this question:

`what must Worth topology survive before we are allowed to trust it as the substrate for primitive construction, booleans, curved surfaces, and eventually aerospace-grade workflows?`

This document exists because topology fails in production in ways that demo
coverage will never catch:

- local rewires that look legal until replay diverges
- branch-local edits that preserve counts but shred meaning
- non-manifold pinch points that trap traversals in loops
- primitive construction flows that work on cubes but collapse on generic
  families
- cancellation chains that return to the same counts but not the same truth
- validator gaps that stay invisible because nobody named them

This document is the contract that says we will go looking for those failures
on purpose.

It should be read alongside:

- [worth_roadmap.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/worth_roadmap.md)
- [test-requirements.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements.md)
- [test-requirements_pt2.md](/Users/Esther/Documents/Programming/forge_workspace/forge/_docs/worth/test-requirements_pt2.md)

## Adversarial Constraint

Worth topology must survive this hostile condition:

> Arbitrary admitted topology histories, including high-cardinality shell
> construction, non-manifold radial stress, degenerate local rewires, long edit
> chains, cancellation storms, branch-local divergence pressure, and deliberate
> corruption attempts, must either converge to the same deterministic topology
> truth and the same machine-checkable certification artifacts or fail with
> exact, localized, replay-safe diagnostics.

More concretely:

- replay of the same admitted history must produce the same topology truth,
  digests, validator conclusions, and diagnostics
- branch-local and mainline reads of the same admitted truth basis must agree
  unless the system produces an explicit typed divergence
- no topology category may be "proven" by one tetrahedron, one cube, or one
  hand-authored shell
- no hostile topology may crash, hang, spin forever, or silently corrupt state
- no known validator gap may remain unnamed
- no geometry semantics may leak into `worth-topo`

If a naive topology implementation would break under a condition we can name,
that condition belongs in this document.

## Topology Purity Rule

All certification in this document assumes:

- `worth-topo` remains geometry-free apart from explicitly permitted opaque
  handles and topology-safe identifiers
- geometry binding, spatial classification, and intersection semantics belong
  outside `worth-topo`
- kernel construction programs may generate topology workloads, but topology
  legality, replay, determinism, and hostility proof still belong to the
  topology program

If a test requires geometry numerics to determine whether topology passed, that
test belongs in `worth-spatial` or `worth-kernel`, not here.

## Acceptable Outcome Rule

Every hostile topology certification workload has exactly two acceptable
outcomes:

- exact success with machine-checkable proof artifacts
- exact structured failure with localized rejection evidence

Never acceptable:

- crash
- hang
- infinite traversal
- silent topology corruption
- silent validator widening
- silent replay drift
- silent structural-identity drift
- "works on this shape" with no family closure

## Brutality Tier Rule

The topology program must maintain explicit hostility tiers.

### Tier 0: Foundation Integrity

This tier proves the substrate is not lying:

- storage integrity
- transaction integrity
- naming integrity
- replay integrity
- deterministic digests

### Tier 1: Admitted Workflow Closure

This tier proves the claimed topology workflows actually work generically:

- primitive families
- primitive-body topology classes
- operator families
- query/traversal families
- branch-local and replay parity

### Tier 2: Pathological But Admitted

This tier attacks the admitted surface at its sharpest edges:

- high valence
- high cardinality
- thin local structures
- cancellation chains
- bowties and pinch-adjacent structures
- ambiguous local rewires
- large branch-local histories

### Tier 3: Corruption And Boundary Assault

This tier proves the system fails honestly:

- intentionally corrupted internal states
- out-of-class family members
- broken radial rings
- impossible membership states
- illegal loop/path structures
- invalid construction sequences

If a topology milestone widens the admitted surface, it must widen the relevant
tiers too. No new surface gets a free pass.

## Required Topology Torture Categories

Worth topology certification must maintain named suites for all of the
following categories.

### 1. Storage, schema, and snapshot integrity

The topology runtime must continuously prove:

- entity schema completeness
- handle / generation safety
- slot reuse safety
- sidecar parity
- hierarchy-chain integrity
- serialization and snapshot round-tripping
- stale-handle rejection

This category exists because operator correctness is meaningless if the arena
can already lie.

### 2. Mutation pipeline integrity

The topology runtime must continuously prove:

- immutable snapshot to mutable draft transition correctness
- drop-as-rollback behavior
- commit-time validation and versioning
- same-commit graph mutation correctness
- per-operation contract enforcement
- no bypass around the canonical mutation runner
- exact delta accounting where the runtime declares deltas

### 3. Lineage, replay, and naming integrity

The topology runtime must continuously prove:

- lineage stamping on admitted mutations
- replay determinism
- persistent naming attachment legality
- naming resolution determinism
- identity separation between topology, naming, lineage, and later spatial
  meaning

### 4. Primitive topology family closure

The topology program must maintain family proof for:

- `WireOpen(n)`
- `WireClosed(n)`
- `WireBranch(k)`
- `SheetDisk(n)`
- `SheetAnnulus(n, h...)`
- `SheetPatch(f)`
- `SolidShell(f)`
- `SolidWithVoid(f_outer, f_inner...)`
- `MultiLumpBody(l)`
- `NmtEdgeFan(k)`
- `NmtVertexPinch(d)` once admitted

For each admitted family:

- smallest admitted member
- generic admitted member
- hostile admitted member
- explicit out-of-class member

A single showcase body does not count as family proof.

### 5. Primitive-body topology closure

Topology must also certify the topology classes produced by construction
programs such as:

- simplex bodies
- orthotope / box bodies
- prisms
- pyramids
- shells with holes
- multi-lump bodies
- wire bodies

These may be authored by `worth-kernel`, but topology still has to certify the
result generically and adversarially.

### 6. Operator brutality

Topology must keep category-shaped operator proof for:

- entity lifecycle operators
- loop / boundary wiring operators
- shell / region / body membership operators
- radial splice and radial repair operators
- construction-sequence operators
- cancellation / inverse-like workflows where admitted

Every operator family must be attacked with:

- legal admitted cases
- hostile admitted cases
- out-of-class cases
- replay parity checks
- localization checks

### 7. Query and traversal brutality

Topology must continuously prove:

- read-only traversal correctness
- deterministic iteration order
- no infinite loop in admitted pathological structures
- no duplicate traversal artifacts where the query contract forbids them
- stale-handle safety in query paths
- topology-only classification purity with no geometry leakage

### 8. Non-manifold and radial brutality

Topology must maintain explicit proof for:

- admitted radial fans
- high-valence vertices
- branch vertices
- bowtie and bowtie-adjacent structures
- disconnected radial rings
- repeated-edge usage
- repeated-vertex local pathologies
- pinch-like multi-disk topologies when admitted

This category must remain separate. Non-manifold hostility is not a side effect
of manifold tests.

### 9. Degeneracy and corruption-localization

Topology must maintain explicit proof for:

- degenerate loops
- repeated-vertex loops
- collapsed local structures
- impossible membership states
- broken radial splice states
- intentionally corrupted internal states caught by validators

If a degeneracy or corruption case is known but not yet enforced, the missing
validator must be named explicitly.
It may not hide as:

- a vague ignored test
- a silent `TODO`
- a suite nobody wired up
- a comment saying "future work"

### 10. Determinism and order assault

Topology must maintain explicit proof for:

- replay parity for accepted workflows
- replay parity for rejected workflows
- branch-local parity
- commutative / order-invariant workflows where the contract requires it
- structural identity stability under legal ordering noise
- stable digests for the same admitted topology history

This category should include dedicated determinism fuzzers, not just a couple
of replay smoke tests.

### 11. Diagnostics and failure taxonomy

Topology must maintain explicit proof for:

- exact validator-family localization
- exact operator-step localization
- exact rejection-class taxonomy
- exact changed-scope reporting
- exact fallback and rebuild breadth counters where applicable
- exact corruption trigger localization

If failure is not localizable, it is not certified.

### 12. Scale, depth, and sustained pressure

Topology must maintain explicit proof for:

- high-cardinality loops
- high-face-count shells
- long edit chains
- cancellation and return-to-prior-state workflows
- large branch-local histories
- scale-separated admitted workloads
- mixed admitted shell / wire / NMT histories

The topology program should explicitly expect a large and growing hostile test
surface. If Worth is aiming for aerospace-grade topology, the proof body must
grow into a serious certification program rather than staying a small
convenience suite.

## Required Hostile Scenario Families

The category list above is still not enough by itself.
Worth topology must also maintain named hostile scenario families.

At minimum, this document expects suites analogous in spirit to:

- bowtie / pinch survivability
- commutative edit-order fuzzers
- sliver-adjacent topology survival
- ambiguous local rewire selection
- pole / fan / star high-valence constructions
- cancellation chains
- shell-with-hole and nested-shell stress
- multi-lump split / merge pressure
- repeated split / collapse churn
- broken-loop and broken-radial corruption cases

These scenario families should grow over time, not shrink.

## Required Proof Artifacts

At minimum, topology certification should preserve direct machine-checkable
artifact families for:

- truth digests
- validation digests
- validator coverage reports
- family coverage matrices
- family parity matrices
- replay parity reports
- branch-local parity reports
- rejection-class reports
- failure-locality reports
- counter reports
- corruption-localization reports
- explicit blocked-validator reports when known gaps remain

These artifacts should be emitted directly rather than reconstructed only from
nested per-case trees.

## Category Progression Rule

The categories in this document are cumulative.

That means:

- early milestones close only the subset they explicitly admit
- later milestones inherit earlier topology obligations
- widening the admitted topology surface widens the required category coverage
- no milestone may silently drop a previously required topology category

## Gate Rule

This document defines two especially important gates:

### Before booleans

Worth should not claim boolean readiness until topology has closed:

- primitive topology family closure
- primitive-body topology closure
- operator brutality for the admitted edit and construction surface
- non-manifold and degeneracy hostility
- replay and branch-local parity

### Before curves

Worth should not move on to curved surfaces until the boolean brutality gate in
the roadmap is closed and the topology program continues to satisfy the
categories above under the widened boolean pressure.

## Architecture And CI Rule

Topology certification must stay structural and enforceable:

- `worth-topo` purity should be CI-audited
- topology certification suites should be registered explicitly, not left as
  loose tests
- blocked invariants should be tracked as named certification debt
- topology test categories should be referenced from milestone closeout
  registries so missing classes cannot disappear silently
- architecture checks should fail when forbidden geometry dependencies enter
  `worth-topo`

## Legacy Reference Bar

The old `forge-topo` test body is not the target architecture, but it is a
serious reference bar for topology seriousness.

Important legacy signals this document intentionally inherits:

- the QA checklist in
  [forge_topo_qa.md](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-topo/forge_topo_qa.md)
  treated storage, transactions, provenance, validators, naming, operators,
  and queries as separate topology proof categories
- the stress suite in
  [topology_stress.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-topo/src/tests/topology_stress.rs)
  used named category families instead of one generic stress bucket
- the brutality suite in
  [brutality.rs](/Users/Esther/Documents/Programming/forge_workspace/forge/crates/forge-topo/src/tests/brutality.rs)
  explicitly tested bowties, commutative determinism, sliver-adjacent topology
  survival, and ambiguous local operations

Worth should meet or exceed that seriousness while preserving the newer Forge
runtime boundaries and crate-purity rules.

## Closeout Use

This document is a standing topology-certification contract.

It should be used in two ways:

1. milestone specs and milestone closeouts should reference the specific
   topology categories and hostile tiers they admit or depend on
2. topology implementation work should use this document as the checklist for
   what must eventually become explicit test suites and proof artifacts

If a topology milestone claims progress without strengthening or at least
preserving the relevant categories here, that milestone is not topology-complete.
