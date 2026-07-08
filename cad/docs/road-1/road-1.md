# Platform Constitution Roadmap

**Status:** Draft
**Road:** `ROAD.md` -> Road 1: Platform Constitution

## Purpose

This document sequences Road 1, `Platform Constitution`.

It is not a milestone spec.
It is not the architecture thesis.
It is not a migration ledger.

Its job is to name the real closure boundaries inside the constitutional road
so later work can build on frozen substrate instead of discovering the
substrate ad hoc while working on topology, geometry, policy, packs, or UI.

## Goal

Freeze the smallest honest sequence that gives `Worthy` a permanent substrate
for:

- pure domain meaning
- graph constitution
- Query-native declaration bridge
- ordinary entry authority
- derived publication
- pack contribution
- certification-only replay and proof posture

without turning Road 1 into a giant pre-committed crate census or a second
copy of the thesis docs.

## Governing Summaries

**`MENTALITY.md`**
- Protects: foundation-first build order under adversarial pressure.
- Strongest implication here: constitutional boundaries must be frozen before
  later roads start using them opportunistically.

**`arch_laws.md`**
- Protects: typed phase progression, contractual facades, and compiler-visible
  authority boundaries.
- Strongest implication here: the roadmap must sequence real proof-bearing
  transitions, not conceptual categories.

**`composition_laws.md`**
- Protects: named semantic responsibilities instead of broad bags.
- Strongest implication here: each milestone should close one structural
  boundary, not gather vaguely related "platform" work.

**`domain_structure_laws.md`**
- Protects: physical boundaries that preserve meaning, authority, truth source,
  and replacement surface.
- Strongest implication here: the roadmap must separate pure meaning, Query
  bridge, entry authority, derived publication, and certification posture into
  distinct closure steps.

**`perf_laws.md`**
- Protects: hot-path honesty, carried proof, explicit locality, and separation
  between ordinary and reconstructive cost.
- Strongest implication here: ordinary retained consumption and cold replay
  posture must be frozen as constitutional substrate, not left for later roads
  to patch.

**`ROAD.md`**
- Protects: Road 1 as the first permanent substrate every later road consumes.
- Strongest implication here: this roadmap should stay short, structural, and
  dependency-ordered.

## Adversarial Constraint

Later roads must be able to add real domain meaning in parallel without
re-litigating where meaning lives, how Query consumes it, how ordinary runtime
admits it, how derived artifacts publish it, or when replay is allowed.

The hostile failure shape is:

> topology, geometry, policy, motion, and pack work all begin landing while
> Road 1 is still vague, and the codebase responds by rebuilding a hybrid
> schema/runtime layer whose boundaries are only recoverable by folklore.

If Road 2 or Road 3 engineers can still honestly ask:

- "is this pure meaning or a Query declaration?"
- "can this ordinary consumer reopen replay?"
- "should this pack add a runtime lane directly?"
- "is this publication authoritative or rebuildable?"

and the tree does not answer decisively, Road 1 is not closed.

## Product Decision Lock

- Road 1 is a real child roadmap, not one milestone and not an endless
  umbrella.
- The road stays small on purpose; it should close constitutional substrate,
  not absorb domain feature work.
- The roadmap sequences boundary closures in dependency order:
  stratification first, then graph constitution, then Query bridge, then
  ordinary entry/publication, then pack and certification proof.
- Query ownership is frozen in Road 1, not discovered later:
  - pure meaning stays Query agnostic
  - Query imports, admission, declaration lowering, obligation selection
    adoption, and contribution orchestration live on `entry` surfaces
  - projection consumption and retained/publication grammar live on `derived`
    surfaces
  - replay, reconstruction, and Consumer Kit hostile proof stay in `cert`
- Each milestone in this roadmap must leave a usable substrate for later roads,
  not a conceptual placeholder.
- This roadmap is allowed to create new crates and workspaces where the
  boundaries demand them.
- The roadmap must overbuild where overbuilding is structural, especially on
  authority boundaries and hot/cold posture.
- Mechanical enforcement is part of the first closure surface, not a later
  polish step.

## Physical Placement Lock

All new Worthy-first physical work created under this roadmap should land under
`cad/workspaces/`.

That is a repository-shape decision, not a naming decision.

The architecture grammar remains `{tier}-{band}-{domain}`. The physical
workspace placement rule is:

```text
/cad
  /workspaces
    /worth-contracts
    /worth-entry
    /worth-derived
    /worth-packs
    /worth-certification
    /worthy-contracts
    /worthy-entry
    /worthy-derived
    /worthy-packs
    /worthy-certification
```

Rules:

- workspace folders are packaging and compile/test isolation surfaces
- each folder under `cad/workspaces/` is intended to be a real Cargo workspace
- crate names remain grammar-shaped and do not inherit folder names as part of
  their public identity
- new constitutional work should prefer `cad/workspaces/` even while legacy
  Worth code still lives elsewhere
- migration of legacy code is not required for this roadmap unless a later
  milestone explicitly makes it part of the closure surface
- a thin repo-level aggregation layer may still exist above the sub-workspaces
  for whole-platform CI, shared policy, and cross-workspace commands; that
  layer does not replace the sub-workspaces as the primary ownership boundary

This roadmap therefore plans both:

- the constitutional crate topology
- the initial `cad/workspaces/` landing map that later roads will inherit

## Initial Workspace Map

Road 1 does not need every future workspace, but it should establish the
minimum honest workspace skeleton that later roads can extend without moving the
constitutional cuts.

The intended initial map is:

```text
cad/workspaces/
  worth-contracts/       # worth-schema-*
  worth-entry/           # worth-entry-*
  worth-derived/         # worth-derived-*
  worth-packs/           # worth-pack-*
  worth-certification/   # worth-cert-*

  worthy-contracts/      # worthy-schema-*
  worthy-entry/          # worthy-entry-*
  worthy-derived/        # worthy-derived-*
  worthy-packs/          # worthy-pack-*
  worthy-certification/  # worthy-cert-*
```

Phase 1 of Milestone 1 uses only the five platform-tier paths above as a
directory-routing skeleton:

```text
cad/workspaces/
  worth-contracts/
  worth-entry/
  worth-derived/
  worth-packs/
  worth-certification/
```

That Phase 1 routing freeze is intentionally narrower than the full Road 1
workspace topology. It teaches where constitutional platform work will live
before Cargo manifests or seed crates are born. The real Cargo workspace
ownership arrives in Milestone 1 Phase 2. The `worthy-*` workspace rows above
remain roadmap-level topology in this phase, not Phase 1 implementation scope.

Deliberately absent in Road 1:

- `worthy-resolvers`
- `worthy-solvers`
- `worthy-ui`
- `worthy-dsl`

Those are real later workspaces, but Road 1 does not need to birth them just to
look complete. They should appear when their first constitutional milestone or
later road needs them for real.

The Road 1 skeleton is intentionally biased toward:

- contract classes
- entry/publication posture
- pack seam
- certification fences

That is the smallest physical map that still teaches the intended architecture.

The intended Cargo topology is:

- many real local workspaces under `cad/workspaces/`
- one thin top-level aggregator at repo root for whole-world commands

That split is intentional:

- sub-workspaces make constitutional boundaries real in build topology
- sub-workspaces keep local compile/test loops small as the system widens
- the top-level aggregator preserves one place for global CI, shared policy,
  and end-to-end orchestration

Road 1 should therefore avoid two failure shapes:

- one giant workspace that makes every constitutional cut soft
- fully isolated workspaces with no higher aggregation story for cross-workspace
  automation

## Initial Crate Candidates

Road 1 should be opinionated about likely first crates even before every crate
is born. This is not a pre-commitment to create them all immediately; it is the
initial routing map.

Likely first platform-tier crates:

- `worth-schema-core`
  - small shared pure-meaning home for foundational identity, naming, units,
    tolerance, and measure grammar that is not yet graph-specific
- `worth-schema-graph`
  - pure graph constitution: layers, edge classes, promotion grammar, spine,
    and aspect rules
- `worth-pack-registry`
  - pack seam and contribution registry

Additional likely births, but **not pre-named here unless reserved in
`NAMING.md`**:

- one reviewed `worth-entry-*` crate for Query-native declaration lowering and
  obligation/contribution adoption if those surfaces do not fit honestly inside
  existing reserved homes
- one reviewed `worth-derived-*` crate for retained/publication grammar if
  Road 1 proves that surface should not live entirely inside product-tier
  derived crates
- one reviewed `worth-cert-*` scale-proof home if Road 1 needs a platform-tier
  scale harness before later roads widen the proof; this is intentionally
  deferred rather than born in Milestone 1

For Milestone 1 Phase 1 specifically, none of those additional platform-tier
entry, derived, or cert births are legal yet. Phase 1 freezes only the
reserved names and owning later milestones so later phases cannot smuggle them
through placeholder language.

Likely first product-tier crates:

- `worthy-schema-topology`
  - first product meaning specimen consuming graph constitution
- `worthy-schema-geometry`
  - first product geometry meaning specimen
- `worthy-entry-construct`
  - first ordinary product-facing entry lane
- `worthy-derived-brep`
  - first real derived publication and retained-consumption specimen
- `worthy-cert-replay`
  - explicit cold replay/reconstruction fence proof on the product tier

Additional likely births, but **not pre-named here unless reserved in
`NAMING.md`**:

- one first-party `worthy-pack-*` specimen chosen through the normal naming
  path; the leading candidate is the corrugated wall from `BOUNDARIES.md`

Open design note:

- the roadmap intentionally freezes **authority placement** before it freezes
  every crate name
- any crate not already legal under `NAMING.md` must be born through an
  explicit Road 1 naming amendment rather than by roadmap implication

## Road 1 Directory Skeleton Standard

Each Road 1 workspace should start with a predictable, narrow skeleton.

Workspace-level default:

```text
cad/workspaces/<workspace>/
  Cargo.toml
  README.md
  crates/
    <crate-a>/
    <crate-b>/
```

Workspace-level meaning:

- `README.md` is the workspace charter and routing note, not marketing copy
- `Cargo.toml` owns workspace membership and local build topology
- `crates/` is the only place Road 1 Rust packages may be born inside that
  workspace
- ad hoc sibling folders such as `helpers/`, `misc/`, `scratch/`, or
  `temporary/` are not allowed as quiet ownership boundaries
- proof tools, runner machinery, and other cross-workspace enforcement do not
  live under a Road 1 workspace; they stay under dedicated repo-root tool
  surfaces such as `tools/` or `automation/`

Sub-workspace `Cargo.toml` posture:

- owns its local member list
- owns its local default members where useful
- may inherit shared dependency/version policy from the repo-level layer, but
  still remains a real standalone Cargo workspace
- should be runnable directly for local build/test loops without requiring a
  whole-repo command

Repo-level aggregator posture:

```text
/
  Cargo.toml                # thin aggregation layer
  cad/
    workspaces/
      ...
```

The repo-level `Cargo.toml` should:

- orchestrate explicit sub-workspace commands for whole-platform CI
- centralize only the policy that truly benefits from being global
- avoid becoming the practical day-to-day ownership boundary for Road 1 work

Crate-level default:

```text
<crate>/
  Cargo.toml
  AGENT_CONTEXT.md
  src/
    lib.rs
    facade.rs
```

Crate-level meaning:

- `AGENT_CONTEXT.md` is machine-generated local orientation, not hand-authored
  prose
- `src/lib.rs` is the public crate entry
- `src/facade.rs` is aggregation only
- any additional top-level file or folder under `src/` must correspond to one
  named responsibility the workspace charter and milestone can justify
- broad catch-all files or folders such as `helpers.rs`, `common.rs`,
  `shared.rs`, or `util.rs` are not part of the default Road 1 grammar

Proof and test placement defaults:

- ordinary production behavior proof stays close to the owning crate and may
  live in crate-local tests only when the proof belongs to that crate's public
  or crate-private law surface
- cross-crate boundary denial, compile-fail, replay-fence, and generated-context
  freshness proof live in dedicated tool or certification surfaces rather than
  being scattered through ordinary crates
- fixture bundles may exist only where the owning phase names them; they are
  not an automatic sibling of every crate
- if a crate has no real behavior yet, it should not invent a decorative test
  tree just to look complete

Additional crate-level rules by constitutional class:

- pure meaning crates
  - may add topic modules such as `identity.rs`, `naming.rs`, `graph/`,
    `promotion.rs`
  - must not add runtime orchestration folders
- Query declaration bridge crates
  - may add `collections.rs`, `basis.rs`, `handles.rs`, `projection/`
  - must not add execution or replay folders
- entry crates
  - may add `admission/`, `lowering/`, `plan/`, `denial.rs`
  - must not add derived artifact storage or certification replay modules
- derived crates
  - may add `artifact/`, `receipt/`, `ordinary_consumption/`
  - must not add source-authority constructors
- pack crates
  - may add `registration/`, `contribution/`, `specimens/`
  - must not add hidden runtime adapters
- certification crates
  - may add `compile_fail/`, `parity/`, `scale/`, `regression/`
  - may depend broadly; nothing ordinary should depend back on them

The point is not rigid aesthetics. The point is that a future agent opening a
Road 1 crate can predict what belongs there before reading it.

## Roadmap Sequence

### Milestone 1: Contract Stratification

This milestone freezes the first constitutional split:

- pure meaning
- Query declaration bridge
- ordinary entry authority
- derived publication
- certification-only reconstruction

It closes the question "what classes of contract exist here at all?" before
later milestones decide how those classes are physically embodied.

It must ship:

- one frozen contract-class inventory
- one dependency-direction law between those classes
- one initial visibility/facade posture for class boundaries
- one exemplar family named across all five classes
- the first mechanical enforcement set:
  - naming grammar enforcement
  - band DAG enforcement
  - tier-direction enforcement
  - replay-fence enforcement
  - generated per-crate agent context

Primary physical work:

- create the initial `cad/workspaces/` skeleton for the Road 1 workspaces
- birth only the smallest honest first constitutional crates
- likely touch:
  - `cad/workspaces/worth-contracts/`
  - `cad/workspaces/worth-entry/`
  - `cad/workspaces/worth-derived/`
  - `cad/workspaces/worth-packs/`
  - `cad/workspaces/worth-certification/`

Likely crate births:

- `worth-schema-core`
- `worth-pack-registry`
- no cert crate birth yet; Milestone 1 only reserves cert posture and begins
  enforcement through tools

Likely directory skeleton focus:

- `src/lib.rs`
- `src/facade.rs`
- one module per constitutional class-specific responsibility
- no mixed-class catch-all modules

It must not try to:

- finish graph constitution
- define every future declaration handle
- build rich runtime behavior
- widen into domain roads

Why it comes first:
Without this milestone, every later milestone would be allowed to quietly
invent its own contract classes.

This milestone is also where `tools/boundary-check` and `tools/agent-context`
must become real enough that the next milestone cannot proceed on convention
alone.

### Milestone 2: Graph Constitution

This milestone freezes the first pure-meaning specimen of the platform:

- graph layers
- edge classes
- promotion grammar
- authority-vs-derived identity distinctions

It closes the question "what meaning does the graph itself carry?" before Query
bridge and runtime lanes are allowed to bind to it.

It must ship:

- one pure graph-constitution contract family
- one promoted-identity specimen
- one explicit distinction between authoritative identity and derived
  references
- one Query-adoption contract proving that graph axes are not just nouns:
  they must become touch descriptors, obligation selectors, invariant
  registrations, budget-honest receipts, and bypass-audit surfaces

Primary physical work:

- deepen `cad/workspaces/worth-contracts/`
- birth the graph-specific pure-meaning crate if it did not land in Milestone 1

Likely crate births:

- `worth-schema-graph`

Likely directory skeleton focus:

```text
worth-schema-graph/
  src/
    lib.rs
    facade.rs
    layers.rs
    edge_class.rs
    spine.rs
    aspect_rules.rs
    promotion.rs
    identity.rs
```

This milestone should make the graph constitution visible as structure, not as
one oversized module.

It must not try to:

- implement domain operators
- widen into full B-rep truth
- collapse graph meaning into Query-specific declarations

Why it comes second:
The bridge and entry lanes need a stable thing to point at. Graph constitution
is that thing.

### Milestone 3: Query-Native Declaration And Obligation Adoption

This milestone freezes the explicit bridge between pure meaning and
Query-native runtime declaration and obligation adoption.

It closes four questions at once:

- how does declared meaning become stable Query handles?
- where do graph touch obligations get adopted?
- where do domain capability contributions get lowered?
- where is the last place Query imports are allowed before ordinary work enters
  the runtime?

It must ship:

- stable declaration-handle grammar for the initial exemplar family
- collection/basis/path or equivalent bridge nouns where required
- typed declaration-lowering surfaces
- graph touch obligation adoption surfaces
- domain capability contribution adoption surfaces
- a support/admission matrix for which Query-native surfaces Road 1 is adopting
  now versus later

Primary physical work:

- deepen `cad/workspaces/worth-entry/`
- leave `cad/workspaces/worth-contracts/` Query agnostic

Likely crate births:

- one reviewed `worth-entry-*` bridge/adoption crate if the existing reserved
  entry homes are not enough

Likely directory skeleton focus:

```text
<query-bridge-crate>/
  src/
    lib.rs
    facade.rs
    handles.rs
    collections.rs
    basis.rs
    projection/
      mod.rs
```

This is the milestone that should settle whether the bridge lives as a
strictly entry-adjacent platform surface. Road 1 now locks that it must **not**
live in a schema crate.

It must not try to:

- execute runtime work
- own replay or certification
- become a second meaning layer
- leave Query surface commitments implicit

Why it comes third:
Entry and publication lanes should consume bridge surfaces, not copied strings
or private folklore.

### Milestone 4: Entry And Publication Lanes

This milestone freezes the ordinary hot path:

- admitted entry into runtime-owned work
- lowered ordinary plans or equivalent entry artifacts
- rebuildable derived publication
- explicit ordinary retained consumption
- explicit cold fence against replay-shaped ordinary use

It closes the question "what is the normal way work enters and later gets
consumed?"

It must ship:

- one ordinary entry lane for the exemplar family
- one derived publication lane for the exemplar family
- one retained ordinary-consumption path that does not reopen replay
- one explicit cold certification path that may reopen replay
- one real CAD pressure specimen proving the split under geometry pressure:
  the `worthy-derived-brep` path plus one projection-consumption-backed
  downstream acceptance test in the spirit of the 7.6 retained-artifact seam

Primary physical work:

- deepen:
  - `cad/workspaces/worth-entry/`
  - `cad/workspaces/worth-derived/`
  - `cad/workspaces/worth-certification/`
  - `cad/workspaces/worthy-entry/`
  - `cad/workspaces/worthy-derived/`

Likely crate births:

- one platform-tier ordinary entry substrate crate
- `worthy-entry-construct`
- `worthy-derived-brep`
- `worthy-cert-replay`

Likely directory skeleton focus:

```text
entry crate:
  src/
    lib.rs
    facade.rs
    admission/
    lowering/
    denial.rs

derived crate:
  src/
    lib.rs
    facade.rs
    artifact/
    receipt/
    ordinary_consumption/
```

This milestone should be the first place where hot-path ordinary consumption
and cold replay posture are mechanically visible in the tree.

It must not try to:

- solve large domain workflows
- add broad certification suites beyond what the hot/cold fence needs
- treat replay as ordinary infrastructure

Why it comes fourth:
By this point the platform knows what things mean and how Query names them; now
it can freeze how ordinary work flows through them.

### Milestone 5: Pack Seam And Certification Skeleton

This milestone closes the first extension and proof posture of the platform.

It freezes:

- pack contribution through declared seams
- first-party and third-party seam parity
- dependency-direction proof
- compile-fail or equivalent constitutional fences
- boundary counters and hostile constitutional closeout

It closes the question "how do we keep the new constitution honest as more
people and more roads start building on it?"

It must ship:

- one `BOUNDARIES.md`-grade first-party pack specimen using the same seam
  future packs will use; the leading candidate is the corrugated wall
- one hostile constitutional regression suite
- one dependency-direction proof surface
- one boundary-measurement skeleton for hot-path honesty

Primary physical work:

- deepen:
  - `cad/workspaces/worth-packs/`
  - `cad/workspaces/worth-certification/`
  - `cad/workspaces/worthy-packs/`
  - `cad/workspaces/worthy-certification/`

Likely crate births:

- one reviewed `worth-cert-*` scale home if that proof still belongs on the
  platform tier once Milestones 1-4 are real

Likely directory skeleton focus:

```text
pack crate:
  src/
    lib.rs
    facade.rs
    registration/
    contribution/
    specimens/

cert crate:
  src/
    lib.rs
    facade.rs
    compile_fail/
    regression/
    scale/
```

This milestone should make the extension seam and the hostile proof seam both
real enough that later roads cannot bypass them casually.

It must not try to:

- finish all certification for the entire platform
- absorb later road proof suites
- turn pack work into a giant plugin product road prematurely

Why it comes fifth:
Once the substrate exists, the system immediately needs proof that it cannot
quietly decay and a seam that future domain knowledge can actually use.

## What This Roadmap Must Preserve

- Road 1 stays small, substrate-oriented, and dependency-ordered.
- The thesis remains the constitutional explanation; this roadmap remains the
  implementation sequence.
- Pure meaning remains Forge Query agnostic by default.
- Query declaration bridge remains explicit rather than hidden.
- Ordinary and reconstructive cost posture remain visibly distinct.
- Pack extension remains a first-class seam rather than a later add-on.
- Later roads inherit constitutional substrate instead of improvising it.

## What This Roadmap Must Not Become

- a second copy of `ARCHITECTURE.md`
- a giant speculative crate inventory
- a catch-all "platform chores" backlog
- a hidden domain roadmap for topology or geometry
- a migration journal for legacy Worth code
- a single milestone pretending to be a roadmap

## Acceptance Evidence

This roadmap is successful when:

- an outsider can tell why Road 1 needs multiple milestones instead of one
- each milestone closes a real constitutional boundary rather than a category
- the order reads as dependency logic, not author enthusiasm
- later roadmap authors can tell where topology, policy, motion, packs, and
  certification work begin depending on substrate rather than redefining it
- the first milestone spec derived from this roadmap is obviously narrower than
  the roadmap itself
- the roadmap yields hard milestone gates rather than rhetorical closeout
  language, including:
  - boundary-check and agent-context activation in Milestone 1
  - `NAMING.md` reservation or naming-amendment proof for every born crate
  - crate DAG snapshots and replay-fence compile-fail proof
  - graph-touch obligation adoption proof
  - projection-consumption and Consumer Kit proof where Road 1 claims them
  - one real CAD/BREP retained-consumption specimen before Road 1 claims to
    have frozen ordinary-vs-cold posture
