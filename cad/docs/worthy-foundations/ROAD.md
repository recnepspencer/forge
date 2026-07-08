# Worthy Road

**Status:** Draft
**Companions:** `ARCHITECTURE.md`, `BOUNDARIES.md`, `GLOSSARY.md`, `NAMING.md`, `worthy_vision.md`

## Purpose

This document defines the top-level planning structure for `Worthy`.

It is not a milestone spec.
It is not a domain roadmap.
It is not a migration checklist.

Its job is to define the enduring roads of the platform and the order in which
those roads should be opened so the system can grow toward a real BIM model
without collapsing back into one giant milestone ladder.

## Goal

Replace the old monolithic roadmap shape with a higher-level planning document
that:

- names the durable roads of the platform
- orders them by structural dependency and product consequence
- makes room for roads that were previously easy to lose, especially motion,
  interchange, policy, and AI-facing surfaces
- establishes the planning frame that child roadmap documents will live under

## Why This Doc Exists

The old roadmap shape tried to make one document sequence everything:

- platform substrate
- geometry kernel growth
- booleans
- curved work
- blends
- history
- interaction
- certification

That shape no longer matches the architecture.

`Worthy` is now explicitly organized as a multi-road platform with distinct
authority bands, graph laws, pack seams, and product surfaces. Planning must
reflect that same structure or future roadmap work will quietly retrain the
tree toward milestone-shaped filing instead of responsibility-shaped growth.

## Governing Summaries

**`MENTALITY.md`**
- Protects: adversarial-constraint-first planning and foundation-first build
  order.
- Strongest implication here: the road order must be driven by what would fail
  catastrophically if built late, not by what is easiest to demo.

**`arch_laws.md`**
- Protects: explicit authority boundaries, proof-bearing progression, and
  facade-owned subsystem decomposition.
- Strongest implication here: roads must separate entry, resolver, solver,
  derived, pack, and certification concerns where those distinctions carry real
  authority.

**`composition_laws.md`**
- Protects: one semantic unit per file and named structure instead of giant
  bags of mixed responsibility.
- Strongest implication here: one roadmap doc should own one coherent road;
  the top-level road doc should not become a second monolithic roadmap.

**`domain_structure_laws.md`**
- Protects: recursive responsibility structure and physical boundaries that
  preserve truth source, ownership, and replacement surfaces.
- Strongest implication here: planning must become recursive too; road, roadmap,
  and milestone spec are different structural levels and should stay different.

**`perf_laws.md`**
- Protects: bounded execution breadth, locality-native execution, explicit hot
  paths, and separation between ordinary and reconstructive cost surfaces.
- Strongest implication here: motion, booleans, interchange, policy, and
  certification all deserve explicit roads because they carry different
  locality and proof laws.

**Old `worth_roadmap.md`**
- Protects: the seriousness of the ambition, especially around replay, naming,
  validation, and certification.
- Strongest implication here: keep the ambition and hostile bar, but stop
  forcing every future domain into one numbered chain.

**`milestone-7-roadmap.md`**
- Protects: the idea that a hard domain earns its own roadmap-sized sequencing
  surface.
- Strongest implication here: that pattern should become normal for future
  roads rather than remaining a one-off exception for booleans.

## Adversarial Constraint

`Worthy` must grow from a kernel foundation into a real BIM system with
topology, geometry, motion, booleans, imports, features, packs, assumptions,
physics, approval, DSL, UI, and AI collaboration without forcing those domains
back into one global milestone ladder whose numbering becomes the only map.

If the planning shape is wrong, the failure modes are predictable:

- motion gets buried under generic geometry work
- interchange gets treated like a parser side quest instead of a first-class
  model-admission problem
- policy and configuration truth smear across unrelated roads until no one can
  tell what actually governs model meaning
- packs, UI, DSL, and AI collaboration get bundled because they all "feel
  product-y"
- child roadmap docs never form, so the top-level plan becomes a provenance
  archive instead of an operational planning tool

This document exists to make those failures structurally harder.

## Product Decision Lock

- `ROAD.md` is the top-level planning document for `Worthy`.
- A road is a durable strategic lane of the platform, not a milestone bucket.
- A road earns a child roadmap when its internal closure surface is too large
  for one milestone sequence to remain honest.
- A milestone spec closes one capability boundary inside one roadmap.
- The old "one roadmap for everything" shape is not the future planning model.
- `milestone-7-roadmap.md` is the exemplar child-roadmap pattern, not a weird
  special case.
- Motion, interchange, policy/configuration truth, packs, UI, DSL, and AI
  collaboration must be visible roads so they cannot disappear into geometry
  residue.

## Planning Grammar

The planning hierarchy is:

1. **Road**: a durable strategic lane with its own authority, proof, or product
   gravity
2. **Roadmap**: the real implementation sequence inside one road
3. **Milestone spec**: one closure surface inside one roadmap

The key rule is:

> the top-level road names where the platform is going; child roadmaps name how
> one lane gets there; milestone specs close one hard boundary at a time

The top-level document should therefore stay strategic and structural. It names
roads, their ordering, and why each one exists. It should not become a long
internal split plan for booleans, motion, UI, or any other single road.

## The Roads

The current intended road order is:

1. Platform Constitution
2. Core B-rep Truth
3. Policy, Regimes, And Configuration Truth
4. Motion
5. B-rep Boolean Program
6. EMBER Boolean Program
7. Feature Construction
8. Edge And Surface Modification
9. Interchange And Import
10. Components And Assemblies
11. Packs And Extension Ecosystem
12. Building Systems
13. Assumptions
14. Physics
15. Jurisdiction And Approval
16. Derived Products
17. DSL
18. UI
19. AI Collaboration
20. Certification And Scale

The sections below explain why each road exists and why it appears where it
does.

### Road 1: Platform Constitution

This road owns the permanent substrate that every later road consumes:

- shared contracts grammar
- graph constitution
- naming and promotion grammar
- Query-native entry laws
- pack seam
- replay fence
- scale-ladder and enforcement skeleton

This road comes first because every later road depends on it for authority
boundaries, graph shape, and enforcement posture.

Its child roadmap is `platform-constitution-roadmap.md`: sequence the
constitutional substrate work before later roads start filing real domain
content.

### Road 2: Core B-rep Truth

This road turns the platform from architecture into a real modeling kernel.

It owns:

- topology truth
- geometry truth
- primitive topology
- primitive construction
- canonical derived B-rep artifact shape
- identity and continuity foundations at B-rep scale

This road comes before motion, booleans, and imports because those roads need a
real model substrate to operate on.

### Road 3: Policy, Regimes, And Configuration Truth

This road exists so the platform has an explicit home for what governs model
meaning and runtime admission before those concerns leak into random roads.

It owns:

- policy grammar
- regime grammar
- tolerance and measure posture
- modeling-policy vocabulary
- admission-time regime binding
- reusable policy packs
- policy applicability and policy-visible diagnostics

This road is intentionally separate from assumptions, physics, jurisdiction,
and UI settings.

If changing a setting changes engineering meaning, this road should usually own
the grammar or regime that expresses it. If changing a setting changes only UI
presentation, this road does not own it.

### Road 4: Motion

Motion is not a minor operator family.

It owns:

- transform grammar
- move / rotate / align / reframe semantics
- motion entry lanes
- snapping and admissibility rules
- continuity, naming, and locality consequences under motion
- motion-specific replay and complexity proof

This road appears early because motion changes continuity, edit locality,
selection consequences, UI behavior, and hot-path execution laws. Treating it
as residue under generic geometry work would be dishonest.

### Road 5: B-rep Boolean Program

This road owns B-rep boolean execution and certification in the ordinary kernel
lane.

It owns:

- planar and later broader B-rep boolean sequencing
- deterministic split / classify / assemble closure
- legality and cleanup closure
- replay and diagnostics for the B-rep lane
- canonical derived-artifact handoff into later roads

This road already has the correct child-roadmap shape in
`milestone-7-roadmap.md`.

### Road 6: EMBER Boolean Program

EMBER booleans are related to B-rep booleans but not identical in operational
character, proof shape, or eventual parity questions.

It owns:

- EMBER execution on the shared public boolean boundary
- EMBER-specific sequencing and hostility
- parity and divergence classification against the B-rep lane
- cross-lane replay and localization proof

This remains a sibling road rather than a later phase hidden inside the B-rep
road.

### Road 7: Feature Construction

This road owns construction operations that create new shape from declared
inputs.

It owns:

- extrusions
- revolves
- sweeps
- profile-driven construction
- other admitted feature-birth operators
- continuity and naming consequences of feature construction

This road belongs after booleans because booleans harden core topological and
spatial closure, but before blends because feature birth is a more primary
authoring act than local edge refinement.

### Road 8: Edge And Surface Modification

This road owns operations that modify existing geometry through local but
semantically difficult change surfaces.

It owns:

- fillets
- chamfers
- shelling where admitted
- offsets where admitted
- blend continuity
- junction and cascade behavior
- explicit collapse and failure taxonomy

This road is separate from feature construction because its hostility surface,
failure taxonomy, and continuity burden are distinct.

### Road 9: Interchange And Import

This road makes the platform operational in the real world rather than only
through native authoring.

It owns:

- STEP import first
- later export and interchange surfaces
- foreign-model admission and loss classification
- imported topology and geometry normalization
- imported identity seeding and continuity posture
- unsupported-case and degradation taxonomy for external models

This road is first-class. `STEP` import is not a parser footnote; it is one of
the main ways real models enter the system.

### Road 10: Components And Assemblies

This road owns the transition from raw geometry authoring to semantic
engineering objects.

It owns:

- component grammar
- assembly grammar
- placement semantics
- composition semantics
- continuity of semantic objects across model evolution

This road is distinct from packs. Components and assemblies are semantic
objects. Packs are extension and distribution surfaces for domain knowledge.

### Road 11: Packs And Extension Ecosystem

This road owns the extension model itself.

It owns:

- pack registry posture
- first-party pack dogfooding
- third-party extension seams
- pack admission and compatibility posture
- pack adoption proof

This road matters because extensibility is part of the product strategy, not a
cleanup step after first-party implementation.

### Road 12: Building Systems

This is the road where the platform becomes recognizably BIM-shaped.

It owns:

- structure as a building system
- envelope systems
- interior systems
- MEP systems
- routing across building contexts
- cross-system participation and conflict semantics

This road sits after components and packs because BIM systems are built out of
semantic objects and shared regimes, not raw operator folklore.

### Road 13: Assumptions

This road owns context truth about what the model is being assumed to sit on or
inside of.

It owns:

- assumption grammar
- assumption-set binding
- scenario posture
- reusable assumption packs
- downstream consequence surfaces driven by assumption changes

This road is separate from policy because assumptions describe believed or
declared world conditions, not the rules governing admission or interpretation.

### Road 14: Physics

This road owns computation over the model under explicit scenarios.

It owns:

- scenario assembly
- loads, boundary conditions, and solver planning
- structural or physical derived reports
- comparison and replay posture for physics results

This road is separate from assumptions because assumptions provide contextual
truth while physics provides computational consequence.

### Road 15: Jurisdiction And Approval

This road owns the outside rule systems that govern whether a model is allowed,
approvable, or stampable.

It owns:

- jurisdiction grammar
- code and permit rule packs
- rule applicability and denial shaping
- approval semantics
- approval-bearing derived products

This road is separate from both policy and physics because it owns external
regulatory and approval consequence rather than internal model governance or
computation.

### Road 16: Derived Products

This road owns the rebuildable public products that the rest of the system
publishes.

It owns:

- cost products
- compliance products
- fabrication products
- downstream-impact products
- scene and presentation-facing derived products
- measure and audit views

This road is separate from approval because approval is one governed domain
surface among many derived product families.

### Road 17: DSL

This road owns the engineering language as a first-class authoring surface.

It owns:

- syntax
- AST
- binding
- lowering
- formatting
- linting
- analysis
- editor and LSP support

This road is separate from UI because language tooling and visual interaction do
not share structural fate.

### Road 18: UI

This road owns the human-facing operational interface.

It owns:

- workbench surfaces
- scene and navigation
- inspectors and review surfaces
- dense operational modeling interaction
- approval and consequence review UX

This road is separate from AI collaboration because human-facing interface
systems and AI-mediated intent workflows do not fail or evolve in the same way.

### Road 19: AI Collaboration

This road owns AI-mediated authoring and advisory interaction.

It owns:

- AI action surfaces
- ambiguity prompts
- advisory workflows
- human takeover and refinement posture
- AI / UI / DSL parity over admitted intent classes
- consequence and explanation surfaces specific to AI-authored work

This road is not "UI with chat." It has its own failure modes, trust surfaces,
and parity obligations.

### Road 20: Certification And Scale

This road owns proof that the platform deserves trust.

It owns:

- replay closure
- scale-ladder proof
- hostility suites across roads
- pack adoption proof
- workflow parity proof
- final BIM-grade certification posture

This road is sequenced late but present from day one as a standing planning
concern.

## Road Ordering Rules

The road order is not a staffing plan and not a promise that only one road is
active at a time.

It means:

- earlier roads define foundations that later roads should not be allowed to
  fake
- later roads may begin exploratory work early, but they should not become the
  main planning center before their predecessors have frozen the boundaries they
  depend on
- when two roads are active together, the lower one still owns any missing
  blocker foundations

The key planning question is always:

> if this road surprises us, does the surprise belong inside the road, or does
> it prove an earlier road was not actually closed?

If the answer is the second one, scope should expand backward honestly instead
of documenting workaround folklore in the later road.

## When A Road Gets Its Own Roadmap

A road earns a child roadmap document when one or more of the following become
true:

- it has at least three real closure boundaries that would be dishonest if
  compressed into one milestone
- it has a distinct adversarial proof surface
- it has distinct hot-path or locality laws
- it has enough product gravity that contributors will search for it directly
- it is large enough that continuing to plan it only at the top-road level
  would hide real sequencing decisions

Under that rule:

- the boolean road already earns its own roadmap
- motion likely earns one early
- interchange and import likely earn one early because `STEP` import alone is a
  large admission and normalization program
- components/assemblies and packs will likely split into roadmaps once the
  first serious specimens land

## What This Doc Must Preserve

- the platform-level road order
- the distinction between road, roadmap, and milestone spec
- explicit homes for motion, policy/configuration truth, and interchange
- the rule that roads are born from structural coherence, not from accidental
  sequencing convenience

## What This Doc Must Not Become

- a second monolithic roadmap
- a milestone ledger
- a child-roadmap index full of sub-milestone splits
- a provenance archive of every planning change

When one road becomes large enough to need internal sequencing, that work should
move into its own roadmap document rather than being appended here.

## Acceptance Evidence

This road document is successful when:

- an outsider can tell the difference between the top-level road and a child
  roadmap
- motion, `STEP` import, policy/configuration truth, DSL, UI, and AI
  collaboration each have an obvious planning home
- future roadmap work can add a child roadmap without mutating the basic
  planning grammar
- the order reads as structural dependency rather than historical milestone
  residue

## Final Claim

`Worthy` should not be planned as one giant numbered future.

It should be planned as a stable road over a collection of child roadmaps, each
of which can carry its own adversarial constraint, sequencing logic, and proof
surface honestly.

That is the planning shape that fits the architecture, leaves room for the BIM
build to widen coherently, and makes it much harder to lose critical roads like
motion, interchange, and policy under generic geometry work.
