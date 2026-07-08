# Worthy Architecture Thesis

**Status:** Permanent thesis, v2. Supersedes the v1 Query-Native Architecture Thesis.
**Companions:** `worthy_vision.md` (why), `BOUNDARIES.md` (routing table), `GLOSSARY.md` (vocabulary), `NAMING.md` (frozen grammar + reserved names).

---

## Purpose

This document freezes the permanent architecture for the platform. It is not a
roadmap, not a milestone spec, and not a migration checklist.

It answers four questions:

1. **Where does code live?** (the two-axis grammar)
2. **What shape is the graph?** (the graph constitution)
3. **What is mechanically enforced?** (the fences)
4. **Why will AI agents build it correctly by default?** (the agent contract)

v1 answered only the first question, and answered it by pre-committing ~100
crates across band-shaped workspaces. This version keeps v1's dependency laws
and Query-native rules, and replaces its crate inventory with a discovery
discipline. The three questions v1 never answered - graph shape, agent
defaults, and what is enforced versus advised - are now the load-bearing
sections.

---

## The Thesis In One Sentence

> The platform scales because the graph only holds meaning - components,
> constraints, references, and promoted identity - while geometry stays
> derived, solvers stay extracted, domains reconcile through layered
> projections, agents inherit correctness from the nearest neighboring file,
> and certification proves that edit cost never grows with the size of the
> world.

Everything below exists to keep that sentence true.

---

## Part I - Tiers, Bands, and Domains: The Grammar Is the Architecture

### The three-tier stack

```text
forge-*     runtime substrate     (forge-query, forge-runtime-bridge,
                                   forge-relational, forge-signal)
worth-*     engineering platform  (shared truth grammar, graph constitution,
                                   pack seams, entry patterns, proof harnesses -
                                   everything that survives a pivot from
                                   buildings to turbines)
worthy-*    CAD/BIM product tier  (topology, geometry, BREP, BIM resolvers,
                                   jurisdiction packs, the app)
```

**Plain English:** Forge is the runtime. Worth is the engineering platform
built on it. Worthy is the first product built on the platform.

The tier test for any crate: *would aerospace need this crate unchanged?*
Yes -> `worth-`. No -> `worthy-`.

> **Known hazard (accepted, not resolved):** `worth` and `worthy` differ by
> one letter and will be misread in review, prompts, and conversation. The
> tier *structure* is frozen; the *spellings* remain swappable until the first
> commit. If the collision proves too expensive in practice, rename a tier -
> do not collapse the tiers.

### The two axes

Every crate has a coordinate on two axes:

- **Band** (the authority axis): what *kind* of authority the code holds.
  Fixed set: `schema`, `dsl`, `entry`, `resolver`, `solver`, `derived`,
  `pack`, `app`/`ui`, `cert`.
- **Domain** (the meaning axis): what the code is *about*. Open set, grown by
  explicit extension of the reserved-names list: `topology`, `geometry`,
  `structure`, `physics`, `assumption`, `jurisdiction`, `route`, `cost`, ...

Crate names are `{tier}-{band}-{domain}`. Examples:
`worth-schema-core`, `worthy-solver-curve`, `worthy-entry-boolean`,
`worthy-derived-brep`, `worthy-cert-replay`.

**The naming grammar - not the folder tree - is the architecture.** Smart
agents navigate by grep, naming, and routing docs, not by walking
directories. The grammar gives a *computable home function*: identify the
domain noun of a task, identify the band of the change, and the crate name is
determined. Workspace folders are packaging for compile-time and test-time
isolation; the grammar is what carries the laws.

### Band vocabulary (plain English, one line each)

| Band | Owns | One-line law |
|---|---|---|
| `schema` | shared truth grammar, contract nouns | Defines meaning; imports nothing in the tree |
| `dsl` | language, parsing, lowering to declarations | Speaks intent; never executes it |
| `entry` | Query-native runtime entry & orchestration | The only door into the runtime for ordinary work |
| `resolver` | domain semantic decisions | Decides *what should be true*; may call solvers |
| `solver` | pure computation kernels | Computes *what is true*; never touches the graph |
| `derived` | published derived artifacts & products | Always rebuildable; never mints authority |
| `pack` | distributable domain knowledge bundles | Extends through declared seams only |
| `app`/`ui` | human- and AI-facing surfaces | Consumes facades; never reaches around them |
| `cert` | hostile, adoption, performance, and scale proof | Depends on everything; nothing depends on it |

Two renames from v1, both to kill permanent confusion:

- Runtime crates are `*-entry-*`, **not** `*-query-*`. A `worthy-query-*`
  crate that is not forge-query would be misread forever. "Entry" also
  teaches the correct mental model: this is where declared work enters Query.
- Derived-artifact crates are `*-derived-*`, **not** `*-products-*`.
  "Product" stays reserved for the thing we sell.

### The domain axis lives in packs

v1 treated packs as an extensibility feature. This thesis promotes them:
**packs are the primary organizing principle for engineering knowledge,
including first-party knowledge.**

The platform substrate - entry lanes, contract grammar, solver kernels,
publication lanes - is band-organized, because the invariant laws are
band-shaped. But engineering *content* - components, assemblies, jurisdiction
rules, physics models, policy bundles - is domain-shaped, grows
domain-by-domain, and ships domain-by-domain. A corrugated steel wall is not
smeared across six workspaces; it is a pack with one home, crossing bands
through the declared seams (contract contribution, resolver-backed component
registration, invariant registration, projection consumption).

First-party content is built as packs from day one. This dogfoods the
extension seam continuously, which is the only way the seam stays honest.

The first routing question for any new capability is therefore:

> Does this change require **new runtime capability** (band grid), or only
> **declared knowledge over admitted seams** (pack)?

Many of v1's pre-listed resolver crates (envelope, interiors, chunks of MEP)
are expected to turn out to be pack content, not platform crates.

---

## Part II - Repository Shape

```text
/
  _docs/
    worthy/
      worthy_vision.md
      ARCHITECTURE.md
      BOUNDARIES.md
      GLOSSARY.md
      NAMING.md

  Cargo.toml               # thin whole-repo aggregator

  cad/
    workspaces/
      worth-contracts/     # platform-tier shared grammar      worth-schema-*
      worth-entry/         # platform-tier Query-owned entry   worth-entry-*
      worth-derived/       # platform-tier derived grammar     worth-derived-*
      worth-packs/         # pack seam & registry              worth-pack-*
      worth-certification/ # platform proof harnesses          worth-cert-*

      worthy-contracts/    # CAD-tier grammar                  worthy-schema-*
      worthy-entry/        # Query entry                       worthy-entry-*
      worthy-dsl/          # language                          worthy-dsl-*
      worthy-resolvers/    # semantic decisions                worthy-resolver-*
      worthy-solvers/      # pure computation                  worthy-solver-*
      worthy-derived/      # published derived artifacts       worthy-derived-*
      worthy-packs/        # CAD-tier knowledge bundles        worthy-pack-*
      worthy-ui/           # surfaces                          worthy-app-*, worthy-ui-*
      worthy-certification/# CAD-tier proof                    worthy-cert-*

  tools/
    boundary-check/        # the fences (Part V)
    agent-context/         # per-crate orientation generator (Part VI)
```

The folders under `cad/workspaces/` are real Cargo workspaces. They are not
just grouping directories. They exist to make architectural boundaries real in
build topology, compile/test loops, ownership, and agent context.

The repo-root `Cargo.toml` is intentionally thin. Its job is whole-world CI,
shared policy, and cross-workspace orchestration. It is **not** the primary
ownership boundary for ordinary work.

Workspaces are created **now**, most nearly empty. Empty workspaces are cheap.
Empty *crates* are the thing to avoid.

**Plain English:** the folders exist on day one so the map is stable; the
sub-workspaces are real; the crates are born only when real code needs them.

Road 1 does not need to seed every workspace immediately. The workspace map is
the permanent topology. The seeded subset is a roadmap decision.

### Seeded crates (the minimum honest set, ~13)

A crate is born only when real code needs it. These are needed on the first
day of the geometry port:

| Crate | Tier rationale |
|---|---|
| `worth-schema-core` | identity, naming, units, tolerance, measure vocabulary - platform, size-fenced |
| `worth-schema-graph` | **the graph constitution** (Part IV): layers, edge classes, spine grammar, aspect rules, promotion grammar |
| `worth-pack-registry` | the pack seam itself, so the first component is *forced* to arrive as a pack |
| `worth-cert-adoption` | Query adoption proof harness (Consumer Kit backed) |
| `worthy-schema-topology` | CAD-tier topology grammar |
| `worthy-schema-geometry` | CAD-tier geometry grammar |
| `worthy-solver-curve` | first pure kernel |
| `worthy-solver-intersection` | second pure kernel |
| `worthy-entry-construct` | first operation-family entry lane |
| `worthy-entry-boolean` | second operation-family entry lane |
| `worthy-derived-brep` | the shaped, validated output downstream stages consume - the structural answer to the three-minute test |
| `worthy-cert-replay` | home of the replay fence proofs |
| `worthy-cert-scale` | home of the scale ladder (Part V, fence 4) |

Deliberately absent: `worthy-solver-surface`, `-boolean`, and most of v1's
crate lists. The curve/surface/intersection cut is unproven (the fillet
ambiguity proved it). Start with two solver crates; when the first blend
operation lands, the code decides whether it is a third crate or a
restructure. That decision is cheap at two crates and brutal at ten.

Everything else from v1's crate lists moves to `NAMING.md` as **reserved
names**. The grammar guarantees what a crate will be called; CI rejects
crates whose names are neither reserved nor a reviewed extension of the
reserved list; nothing exists until code needs it.

**Plain English:** freeze the grammar now, birth crates lazily. Bands are
frozen; domains are discovered.

---

## Part III - Query-Native Core Rules (Retained From v1, Condensed)

These are unchanged in substance and restated in plain English. The runtime
orientation doc (`AI_README`) owns the details.

1. **Ordinary domain work starts at Query.** Lower Forge layers are for
   understanding semantics, never for bypassing lanes.
2. **Declare once, lower once, execute through canonical artifacts.** No
   local wrappers, status enums, recovery folklore, or pseudo-Query layers.
3. **Query owns:** operating world, admission, readiness, binding, grouped
   authoring, read planning, publication, projection consumption,
   continuation, recovery, obligation selection, and consumer proof.
4. **Worthy owns:** what the domain *means* - truth grammar, resolver
   semantics, solver behavior, component grammar, derived-product semantics,
   packs, UI.
5. **Typed artifacts over strings.** Where Query ships a typed identity,
   label, receipt, or outcome, raw strings and copied digests are forbidden
   on the ordinary path.
6. **Published derived facts are consumed through projection consumption** -
   never by spelunking materialization rows or bridge helpers.
7. **Downstream proof goes through the Consumer Kit** - never local report
   structs, digest folklore, or fabricated receipts.
8. **Replay and reconstruction are explicit, quarantined modes** (see fence 2).

Constitutional split, made explicit:

- `schema` crates stay Query agnostic. They define meaning, graph axes, touch
  vocabulary, and domain nouns; they do not import `forge-query`.
- `entry` crates are the last pre-runtime home for Query imports. Admission,
  declaration lowering, graph-touch obligation adoption, and contribution
  orchestration live here.
- `derived` crates own rebuildable publication, projection-consumption-facing
  artifacts, and ordinary retained-consumption posture. They do not mint source
  authority.
- `cert` crates own replay, reconstruction, Consumer Kit hostile proof,
  adoption residue proof, and scale proof. Ordinary crates do not depend back
  on them.

**The boundary in one line:** Query decides how declared work becomes runtime
work; Worthy decides what the work means.

---

## Part IV - The Graph Constitution

The workspace map says where code lives. This section says what is allowed
to become graph truth. It is the more important half of the architecture,
because everything Query provides - touched-graph obligation selection,
aspect-narrowed invalidation, region-scoped live maintenance, grouped
neighborhoods, budget-honest denial - keys off *graph shape*.

**The governing property:** the consequence set of any touch must be
derivable from the schema, without discovery traversal. If consequences can
only be found by walking, nothing downstream saves us.

What is frozen now is not entity inventories. It is the **axes**: layers,
edge classes, the partition spine, aspect discipline, and promotion. Domains
fill in nouns forever; the axes are what keep touched-graph work bounded as
they do. All of this vocabulary lives in `worth-schema-graph` and is enforced
in CI with the same status as the crate DAG.

### Axis 1 - Layers

The graph has its own vertical bands, parallel to the crate bands:

```text
L0  CONTEXT     jurisdiction, assumption regimes, policy, tolerance regimes, site
L1  INTENT      components, assemblies, systems, declared parameters,
                relationships, constraints, manual refinements, promoted identity
L2  RESOLUTION  resolver outputs: placements, connections, applicability
                decisions, routing solutions, advisory state
L3  GEOMETRY    derived BREP/mesh/route artifacts - the graph holds references
                and identity anchors; bulk payload lives in artifacts
L4  PRODUCTS    cost, compliance, physics reports, scenes, fabrication,
                downstream impact
```

**Laws:**

- L0-L1 are **authoritative**. Humans and AI write here, and only here.
- L2-L4 are **derived**: destroyable, rebuildable, consumed via projection.
- **Derivation edges point downward only. Invalidation flows only along
  derivation edges.** No layer skips ("cost reads faces directly, just this
  once") - every skip is an invalidation edge that bypasses the pyramid and
  makes consequence sets undiscoverable.
- **Upward influence is not an edge.** A product that wants to change intent
  ("over budget - swap the material") authors a new L1 declaration through
  the entry lane, like any other edit. Feedback loops are conversations
  through the runtime, never graph edges. One product-writes-intent edge
  creates a cycle and ends the touched-graph story.

**Plain English:** the graph is a shallow pyramid. Truth at the top, derived
consequences below, invalidation only downhill, feedback only through the
front door.

### Axis 2 - The edge taxonomy (five propagation classes)

Relation kinds are typed by **propagation semantics**, not domain meaning.
Obligation selection and invalidation narrowing key on these classes, so they
are writable today and stay correct when the entity is a wing spar instead of
a wall.

| Class | Meaning | Propagation |
|---|---|---|
| `CONTAINS` | structural ownership (building->floor->zone->component) | child touch implicates parent rollups; parent delete cascades - the spine invalidation climbs |
| `ATTACHES` | physical adjacency/connection (wall->wall, duct->vent) | geometric consequence bounded to the attachment neighborhood - what "local edit" means, mechanically |
| `REFERENCES` | durable pointer to promoted identity (dimension->face, refinement->edge) | lineage/continuity obligations only; no geometric fan-out |
| `PARTICIPATES` | membership in a cross-cutting system (wall in structural frame, in fire zone, in cost package) | touch implicates the *system's* derived products, not sibling members |
| `DERIVES` | the layer-crossing edge (intent->resolution->geometry->product) | invalidation, downward only, aspect-filtered |

**Law:** domains add entity kinds forever; they almost never add propagation
classes. A proposed sixth class carries the burden of proof "this propagates
differently than all five," not "this means something different."

### Axis 3 - The partition spine

Locality must be graph-native. If locality is computed at read time (bounding
boxes, spatial hashes), every locality question is a scan.

- The containment hierarchy - **site -> building -> storey -> zone/bay ->
  component** - is infrastructure, present from day one.
- Every L1 entity `CONTAINS`-chains to the spine.
- Every L2/L3 derived artifact records which spine cell(s) it derives from.
- L0 regimes (assumptions, jurisdiction, tolerance) bind to spine cells;
  "which regime governs this component" is a shallow `CONTAINS` walk, never a
  search.
- **Corridors are first-class spine entities.** Work with legitimately wide
  extent (MEP runs, structural load paths) is declared as a grouped
  neighborhood on a corridor entity holding `PARTICIPATES` edges from every
  crossed cell. Wide, but *declared* wide: budgetable, groupable, one grouped
  publication. A route must never exist as per-cell fragments that a consumer
  has to reassemble - that reproduces the hot-path/cold-path entanglement
  inside the graph itself.

### Axis 4 - Aspect discipline

Aspects are the third granularity dial (after entity-vs-payload and
promotion). A wall whose geometry parameters, structural participation, cost
drivers, finish language, and fire rating are one blob is an invalidation
bomb: any edit wakes every domain.

**Law:** aspects on L1 entities are **partitioned by which downstream domain
consumes them**, and every `DERIVES` edge declares which aspects it consumes.
A crown-molding tweak flows only along finish-language edges; the structural
solver never wakes.

### Axis 5 - Promotion on reference

The authoritative graph holds semantic truth (order 10^4-10^5 entities for a
serious building). Dense geometry - tessellations, BREP micro-topology,
route polylines - is derived artifact payload, **not** graph entities.

But manual refinement requires referencing fine geometry ("pull *this* edge
tighter"). The rule:

- A subelement **acquires graph identity at the moment something durable
  refers to it, and not before.** The unreferenced 99.9% of faces stay
  artifact-resident.
- Promoted identity is an L1 entity, lineage-bound to the derived artifact
  that carries its geometry, kept alive across regeneration by resolver-owned
  continuity (backed by Query's structural-identity remapping and
  lineage-aware continuity).
- `worth-schema-graph` owns the promotion grammar: which reference kinds
  force promotion, and how promoted identity binds to carrying artifacts.

This is the persistent-naming problem - the thing that has crippled CAD
systems for decades - answered by substrate continuity applied to a
deliberately sparse identity population. It is the single most important
contract family in the platform.

### Proof the axes compose: the fillet

"Fillet this edge; what does it do to physics, cost, and approval?"

1. The edge gets promoted identity (it is now referenced) - L1, lineage-bound
   to its BREP artifact.
2. The fillet is an L1 feature entity: `REFERENCES` the promoted edge,
   `CONTAINS`-anchored to the owning component's spine cell, touching only
   the component's geometry-parameters aspect.
3. Touch shape = {feature birth, one `REFERENCES` edge, one component, one
   aspect, one cell}. Obligation selection reads that shape from schema -
   geometric validity, continuity of other references into the region, and a
   structural check *only if* a `PARTICIPATES` edge into a structural system
   consumes the touched aspect. Zero traversal.
4. Invalidation runs down `DERIVES`: resolution -> that component's BREP ->
   quantity facts -> cost. Physics only if the aspect-filtered edge says
   section properties moved. The crown-molding fillet never touches the frame
   solver; the spar fillet does - because its edges say so, not because
   anything scanned.
5. Approval consumes the decision trace as product-layer facts.

Every step bounded; every bound derivable from schema. That property is what
`worthy-cert-scale` certifies (fence 4).

### What domain schemas are allowed to do

A domain schema family (`worthy-schema-structure`, `worthy-schema-physics`,
`worthy-schema-jurisdiction`, ...) answers exactly four questions:

1. Which entities, at which layers?
2. Which aspects, partitioned by consumer?
3. Which edges, **of the five classes**?
4. Which spine bindings?

A schema-authoring agent is not designing graph topology; it is filling in a
fixed form, and CI rejects a sixth edge class or an upward derivation edge.
`BOUNDARIES.md` rows carry the graph columns: layer, edge classes, aspects
touched, spine scope.

---

## Part V - The Fences (What Is Mechanically Enforced)

The tree is the promise; the fences are the proof. All four live in
`tools/boundary-check` and CI from day one. Everything not listed here is
advice; everything listed here is law.

### Fence 1 - Band dependency law (name-pattern, layout-independent)

```text
schema-*     -> nothing in the tree
dsl-*        -> schema-*
solver-*     -> schema-*                          # NEVER forge-query
resolver-*   -> schema-*, solver-*
derived-*    -> schema-*, solver-* (math only)
entry-*      -> schema-*, resolver-*, derived-*, forge-query
pack-*       -> public seams only
app-*/ui-*   -> entry-*, derived-*, dsl-*
cert-*       -> anything (proof depends broadly)
```

Enforced by crate-name pattern, so it holds regardless of folder layout.
`worth-*` crates may not depend on `worthy-*` crates (platform never depends
on product).

**Plain English:** grammar imports nothing; solvers never see the runtime;
nothing reaches around a facade; proof looks at everything and nothing looks
at proof.

### Fence 2 - The replay fence

Replay, reconstruction, and reconstructive-proof entry points live in
dedicated crates (or behind features) that **only `*-cert-*` crates may
depend on**. An `entry` or `derived` crate importing a replay surface is a
compile/CI failure, not a review comment.

**Plain English:** the ordinary lane consumes shaped truth; proving the truth
is a quarantined mode. The three-minute-test failure becomes structurally
unwritable.

### Fence 3 - Grammar enforcement

Every crate name must parse as `{tier}-{band}-{domain}` with band in the
fixed set and domain in the reserved list - or the PR explicitly extends the
reserved list, which is a visible, reviewable act. `common`, `utils`,
`helpers`, `logic`, `core` overflow buckets are unrepresentable (the single
exception, `worth-schema-core`, is size-fenced).

### Fence 4 - The scale ladder

`worthy-cert-scale` benchmarks the same canonical edits at 10^3, 10^5, and
10^7 entities and asserts that **touch-shape size and invalidation fan-out stay
flat as the world grows**. That certified curve *is* the graph-scaling claim.
If it bends, the granularity discipline broke somewhere, and it is found at
10^5 in CI instead of in the first customer building.

Alongside the fences, two standing contracts:

- **Touch envelopes per operation.** Every `entry` operation ships a named
  complexity contract ("touches O(component neighborhood)", "grouped
  O(corridor)"), asserted against Query's budget receipts.
- **Budget denials are design feedback.** When Query returns
  `BudgetExceeded` or a required-capability posture, the correct response is
  never a retry wrapper or a raised budget; it is "the declared shape is too
  wide - narrow the touch, add the index, or split the neighborhood." This
  expectation is written into the per-crate agent docs, because agents
  otherwise treat denials as errors to route around, and routing around them
  is how pseudo-Query folklore is reborn.

---

## Part VI - The Agent Contract (Why Correctness Is the Default)

forge-query is not in any model's training prior. Every agent arrives
pre-trained on conventional Rust - HashMaps, local status enums, string IDs,
hand-rolled validation - and regresses to that mean the moment attention
drifts. The architecture must make Query-nativeness survive the transition
from *enforced by attention* to *enforced by structure*.

The governing fact: **an agent's effective prior is training data plus
context, and the repo is the context.** Every file an agent opens is a
few-shot prompt. Agents are pattern amplifiers: the repo scales whichever
pattern is locally dominant. Five mechanisms keep Query dominant.

### 1. Wrong patterns fail to compile, and the error routes

Compile errors are the one feedback signal that reliably reaches an agent
that has forgotten everything else. Fence failures therefore say *what to do
instead*:

> `worthy-solver-* may not depend on forge-query. Solvers take extracted
> inputs; the resolver that calls you owns graph contact. See
> worthy-resolver-structure for the shape.`

Fence error messages are reviewed with the same care as public API docs -
for an agent, they *are* the API docs.

### 2. The honest path is the shortest path, measured in tokens

An agent under generation pressure takes the locally cheapest continuation.
If the Query lane costs 40 lines and a HashMap costs 6, no fence or audit
holds - folklore has been made *rational*. Therefore:

- `entry` facades absorb ceremony once, in reviewed code, rather than
  distributing it across ten thousand agent call sites.
- **Ergonomic parity is a standing metric:** for each `BOUNDARIES.md` task
  row, the canonical implementation's token count versus the naive one.
  Anywhere canon loses by more than ~2x, that is a facade bug, not an
  agent-discipline problem.
- Consequence for sequencing: the construct/boolean entry lanes must be
  genuinely pleasant *before* agents are unleashed on domain breadth, because
  they are the pattern every subsequent domain imitates.

### 3. Exemplars: seed crates are pedagogy

Seed crates are the curriculum; they will be imitated thousands of times.
They are written to be imitated - aggressively idiomatic, no residue, with
comments at decision points naming the lane ("projection consumption here -
consuming derived facts, not reopening authority").

One crate per band is formally designated the **reference implementation**
(a field in `NAMING.md` and the crate manifest). `BOUNDARIES.md` rows point
at exemplars. When a pattern improves, the exemplar is updated and the change
propagates through future imitation instead of through migration.

**Corollary - folklore never merges.** In an agent-authored codebase, merged
folklore is training data. The Consumer Kit residue audits are quarantine,
not hygiene.

### 4. Per-crate context, generated, small

A ~30-line agent orientation file in every crate: which band, what it may
import, the public entrypoint, the exemplar to imitate, the top two mistakes
for this band. **Generated by `tools/agent-context` from the naming grammar
and fence config** - hand-written per-crate docs rot in a hundred-crate tree,
and a rotted doc teaches folklore with authority.

This inverts the context problem: instead of fitting a 10k-token orientation
doc into every subagent window, every crate carries exactly the slice that
applies to it, provably current. The full runtime orientation doc remains the
map for cross-cutting work.

### 5. The working set is bounded structurally

An agent must be able to do correct work in one crate with only that crate,
the contracts it names, and one entry facade in context. If correct work
requires six workspaces in the window, agents fill the gaps from their prior -
which is exactly how folklore is invented. Every dependency edge is also a
context edge; the narrow-import laws of Fence 1 are, from the agent's side,
the guarantee that the relevant world fits in the window.

---

## Part VII - Docs Are Part of the Skeleton

Written **before the first crate**, in one sitting:

- **`BOUNDARIES.md`** - the routing table. Each row: task sentence -> pack or
  platform? -> domain noun -> band -> tier -> crate name -> layer / edge classes /
  aspects / spine scope -> exemplar -> one-line reason. Seeded with the
  adversarial cases: fillet, corrugated wall, duct reroute, cost delta, a
  permit rule, undo of a manual tweak, "make the entry more dramatic," and
  the 7.5->7.6 handoff restated as *stage N+1 consumes `worthy-derived-brep`
  through projection consumption; touching replay from stage N+1 is a compile
  error*. A row that cannot be filled is a taxonomy gap found while it costs
  nothing.
- **`NAMING.md`** - the frozen grammar, the reserved-names list, exemplar
  designations.
- **`GLOSSARY.md`** - the tribal vocabulary made public. Every law in this
  document has a plain-English restatement; the glossary hammers the pairs
  outsiders will not infer, starting with: *resolvers decide what should be
  true; solvers compute what is true.*

Test topology, so it does not accrete by convenience: unit tests per crate;
ordinary cross-crate integration tests live in the highest workspace they
exercise, built on projection-consumption fixtures - never on replay;
hostile, adoption, performance, and scale proof lives in `cert`.

---

## Part VIII - Sequence and Acceptance

1. `BOUNDARIES.md` + `NAMING.md` + `GLOSSARY.md` - adversarial examples first.
2. Workspaces + `tools/boundary-check` (all four fences) + `tools/agent-context`.
3. Seed the ~13 crates, written as exemplars.
4. Port the planar boolean pipeline: construct/boolean entry,
   curve/intersection solvers, `worthy-derived-brep` output.
5. **Acceptance test 1:** the 7.6-equivalent test, rebuilt as a consumer of
   `worthy-derived-brep`. It passes fast *by construction*, and the replay
   fence makes the slow version unwritable.
6. First component - the corrugated steel wall - built **as a pack**. Forces
   resolver birth, pack-seam honesty, promotion grammar in anger, and the
   first real test of the domain axis, all on one artifact.
7. **Acceptance test 2:** the scale ladder goes green at three orders of
   magnitude on the canonical edits.

The skeleton's success criterion is not completeness. It is that when step 6
surprises us - it will - the surprise lands as "add a reserved name" or
"split one crate," never as "rethink the topology." **Bands frozen, domains
discovered, axes constitutional.**

---

## Hard Prohibitions

Code:

- No new primary capability in legacy `worth-topo` / `worth-spatial` /
  `worth-kernel` style buckets. Legacy code is a research corpus and
  migration reference; reused concepts are rehomed, not copied.
- No `common` / `utils` / `helpers` / `logic` / `core` overflow crates
  (single size-fenced exception: `worth-schema-core`).
- No solver crate touching forge-query or any entry surface.
- No resolver minting operating-world, publication, or projection folklore
  Query owns.
- No derived crate minting source authority.
- No pseudo-Query layers, second admission paths, string-smuggled identity,
  flattened outcomes, or local status enums for states Query represents.
- No replay or reconstruction imports outside `cert` (Fence 2).

Graph:

- No authoritative writes below L1. No derivation edge pointing upward. No
  layer skips. No product writing intent except through the entry lane.
- No sixth edge class without a propagation-semantics proof.
- No L1 entity off the spine. No dense geometry as graph entities; payload
  lives in artifacts, identity is promoted on reference only.
- No blob aspects: L1 aspects are partitioned by consuming domain.
- No route or corridor represented as per-cell fragments a consumer must
  reassemble.

Process:

- No folklore merges - residue audits are quarantine.
- No hand-written per-crate agent docs - they are generated or absent.
- No pre-created speculative crates - reserve the name instead.
- No treating a Query budget denial as an error to route around.
- No first-party component built outside the pack seam.

---

## Open Decisions (Tracked, Not Blocking)

1. **Tier spellings.** The Worth/Worthy one-letter collision is accepted for
   now; revisit after the first month of agent work if misrouting shows up in
   practice. The tier structure itself is not open.
2. **Solver domain cut.** curve/intersection seeded; surface/boolean/blend
   boundaries decided by the first blend operation.
3. **Zone vs bay subdivision of the spine.** Grammar supports both; decided
   by the first MEP corridor.
4. **Which v1 resolver families are packs.** Presumption: envelope,
   interiors, and most of MEP are pack content; decided at step 6.

---

## Final Claim

Worthy is not a renamed CAD platform. It is a Query-native engineering
runtime with a world-class geometry and approval stack built on top of it -
and, just as deliberately, **a codebase engineered so that the agents
building it produce the canonical pattern by default**: the grammar routes
them, the fences catch them, the exemplars teach them, the facades make
honesty cheap, and the graph constitution keeps every edit's cost
proportional to its consequence rather than to the size of the world.

The tree is the promise. The fences, the exemplars, the routing table, and
the scale ladder are the proof.
