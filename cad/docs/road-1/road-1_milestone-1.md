# Platform Constitution Milestone 1: Contract Stratification

> **Status:** Draft
>
> **Purpose:** freeze the first operational constitutional split of Road 1 so
> later roads inherit one explicit map for pure meaning, Query-native
> declaration and obligation adoption, ordinary entry authority, derived
> publication, pack extension, and certification-only replay/proof.

## Goal

Milestone 1 closes the first real implementation boundary of Road 1.

By the end of this milestone:

- the repository has one official Road 1 physical topology under
  `cad/workspaces/`
- the first constitutional classes are frozen as real build and visibility
  boundaries rather than as prose-only categories
- the first Road 1 seed crates exist with narrow, predictive facade-shaped
  skeletons
- the first mechanical enforcement surfaces exist early enough that later
  milestones cannot drift on convention alone
- the platform can name and route one explicit exemplar family across the
  constitutional classes that exist now and the deferred classes that are only
  being reserved now, without inventing a pseudo-Query layer or pretending that
  deferred runtime flow is already implemented

This milestone does **not** close graph constitution, Query declaration
adoption, or ordinary retained-consumption semantics in full. Those belong to
later milestones in the roadmap. This milestone freezes the first operational
stratification that makes those later milestones buildable honestly.

## Why This Milestone Exists

The first failure mode of Road 1 is not "missing geometry."

It is this:

- two topology maps
- illegal crate births taught by the roadmap
- schema crates drifting into Query imports
- replay fence delayed until after ordinary lanes already exist
- agents inferring structure from nearby convenience instead of from a real
  constitutional skeleton

If Milestone 1 is weak, every later milestone inherits that weakness and turns
it into training data.

This milestone therefore has one job: make the first constitutional split real
in the filesystem, in Cargo topology, in facade boundaries, in naming, and in
mechanical enforcement.

## Governing Summaries

- `MENTALITY.md`
  - Protects: foundation-first sequencing under hostile conditions.
  - Strongest implication here: enforcement and topology must arrive before
    later roads depend on them.
- `arch_laws.md`
  - Protects: typed phase progression, facade ownership, and compile-visible
    boundary crossings.
  - Strongest implication here: this milestone must leave real types, facades,
    workspace boundaries, and compile-fail fences, not only architectural
    intent.
- `composition_laws.md`
  - Protects: one semantic responsibility per file and explicit semantic step
    naming.
  - Strongest implication here: the seed crates must start with narrow skeletons
    and class-specific modules, not mixed constitutional bags.
- `domain_structure_laws.md`
  - Protects: physical boundaries that preserve meaning, truth source,
    lifecycle, and ownership.
  - Strongest implication here: `cad/workspaces/` must become a real ownership
    topology, not just a future folder plan.
- `perf_laws.md`
  - Protects: carried proof, hot-path honesty, explicit locality, and
    separation between ordinary and reconstructive cost.
  - Strongest implication here: replay fence and generated context belong in
    Milestone 1, not after ordinary lanes already exist.
- `platform-constitution-roadmap.md`
  - Protects: Road 1 as a short sequence of real constitutional closures.
  - Strongest implication here: this milestone closes stratification and
    enforcement only; it must not sprawl into graph constitution or real Query
    adoption work that the roadmap sequenced later.

## Adversarial Constraint

An agent implementing the next milestone must be structurally unable to answer
the following questions by guessing:

- "where should this new constitutional crate live?"
- "is this surface allowed to import `worth-query`?"
- "is replay available here because no fence exists yet?"
- "can I just put this in a broad contracts crate until the bridge exists?"

The hostile condition is:

> several later-road changes begin in parallel immediately after this milestone
> closes, and each change tries to take the cheapest local continuation.

Milestone 1 succeeds only if the cheapest local continuation is already the
architecturally honest one.

## Product Decision Lock

- `cad/workspaces/` is the official physical landing map for new Road 1 work.
- Phase 1 freezes only the directory-routing map under `cad/workspaces/`.
- Phase 2 is the first point where those folders become real Cargo workspaces.
- The repo root `Cargo.toml` is a thin whole-world aggregator, not the primary
  ownership boundary.
- Pure meaning remains Query agnostic.
- Query imports do not appear in schema crates.
- Replay and reconstruction remain cert-only even before the rich cert suite is
  built.
- Every crate born in this milestone must already be legal under `NAMING.md` or
  must land in the same change as an explicit naming amendment.
- This milestone births only the smallest honest set of crates:
  - `worth-schema-core`
  - `worth-pack-registry`
- This milestone creates Road 1 workspaces for `worth-entry` and
  `worth-derived`, but does not yet populate them with full bridge or retained
  authority crates.
- No platform-tier `entry`, `derived`, or `cert` crate may be taught as born in
  Phase 1 through placeholder wording, speculative examples, or routing prose.
- The repo root is an orchestration surface, not a Cargo workspace owner for
  Road 1 packages. Whole-world commands run against sub-workspaces through
  explicit `--manifest-path` routing or an equivalent orchestrator.

## Class Inventory

This table is part of the closure surface. It names which constitutional
classes exist, which are legal first births in Milestone 1, and which remain
reserved for later milestones.

| Class | Band | Tier | M1 status | Legal crate in M1 | Owning later milestone if deferred | Allowed imports | Forbidden imports |
|---|---|---|---|---|---|---|---|
| Pure meaning | `schema` | `worth` | legal M1 birth set | `worth-schema-core` | Milestone 2 widens with `worth-schema-graph` | none in-tree | `worth-query`, replay, product-tier crates |
| Query-native declaration/adoption | `entry` | `worth` | deferred and reserved-only | none born in M1 | Milestone 3 | `schema`, `worth-query` | product-tier crates, replay |
| Derived/publication posture | `derived` | `worth` | deferred and reserved-only | none born in M1 | Milestone 4 | `schema`, math-only solver surfaces when later justified | `worth-query` as source authority, replay on ordinary path |
| Pack seam | `pack` | `worth` | legal M1 birth set | `worth-pack-registry` | widened in Milestone 5 | public seam contracts only | Query imports, runtime adapters, source-authority minting |
| Cert-only replay/proof | `cert` | `worth` | deferred crate birth; enforcement starts now through tools | none born in M1 | Milestone 5 for first cert crate birth | broad by later design | ordinary crates must not depend back |

## Phase Plan

### Phase 1: Naming And Contract-Class Inventory

This phase freezes the first legal, implementation-bearing constitutional
inventory.

It answers:

- which workspace routes exist now
- which crates are legal to birth now
- which constitutional classes are being made operational in this milestone
- which classes are only named now and implemented later

**Relevant subsystems**
- Road 1 naming
- Road 1 workspace routing
- constitutional class inventory

**Relevant APIs**
- `_docs/worthy/NAMING.md`
- `_docs/worthy/ROAD.md`
- `_docs/worthy/platform-constitution-roadmap.md`
- Road 1 naming amendment entries for any new legal births

**Directory skeleton**

This phase does not create Rust source trees yet. It creates the naming and
routing skeleton the later phases will materialize:

```text
cad/
  workspaces/
    worth-contracts/
    worth-entry/
    worth-derived/
    worth-packs/
    worth-certification/
```

The only on-disk shape this phase may create is the empty directory-routing
skeleton above. No `Cargo.toml`, `crates/`, or placeholder Rust modules belong
to Phase 1.

**Warnings**
- Do not teach a crate name in this milestone unless it is already legal under
  `NAMING.md` or is amended in the same closure.
- Do not invent a pseudo-band like `bridge`.
- Do not let the inventory imply that schema crates may import Query later.

**Test requirements**
- Adversarial equivalence test: the same exemplar family must be routable across
  pure meaning, entry adoption, derived publication, pack extension, and cert
  proof without inventing a sixth constitutional class.
- Adversarial denial test: a naming-proof test or CI rule must reject any crate
  birth under `cad/workspaces/` whose name is not legal under the grammar and
  reserved list.

**Engineering decisions**
- Freeze the operational class split used by this milestone:
  - pure meaning -> `worth-schema-*`
  - Query-native declaration/adoption -> future `worth-entry-*`
  - derived/publication posture -> future `worth-derived-*`
  - pack seam -> `worth-pack-*`
  - cert-only replay/proof -> future `worth-cert-*`
- Freeze the legal births in this milestone to:
  - `worth-schema-core`
  - `worth-pack-registry`
- Record any additional Road 1-targeted names as reserved-only, not born.
- Record that no platform-tier `entry`, `derived`, or `cert` crate may be born
  in Phase 1 without an explicit naming amendment and roadmap/spec revision.

**Open questions**
- None. Milestone 1 is not allowed to leave its own legal births ambiguous.

### Phase 2: Cargo Workspace Topology

This phase makes the Road 1 physical map real as Cargo topology.

**Relevant subsystems**
- repo-root aggregation
- sub-workspace Cargo ownership
- workspace member routing

**Relevant APIs**
- repo-root `Cargo.toml`
- `cad/workspaces/worth-contracts/Cargo.toml`
- `cad/workspaces/worth-entry/Cargo.toml`
- `cad/workspaces/worth-derived/Cargo.toml`
- `cad/workspaces/worth-packs/Cargo.toml`
- `cad/workspaces/worth-certification/Cargo.toml`

**Directory skeleton**

This phase must leave this exact filesystem shape behind:

```text
/
  Cargo.toml
  cad/
    workspaces/
      worth-contracts/
        Cargo.toml
        README.md
        crates/
      worth-entry/
        Cargo.toml
        README.md
        crates/
      worth-derived/
        Cargo.toml
        README.md
        crates/
      worth-packs/
        Cargo.toml
        README.md
        crates/
      worth-certification/
        Cargo.toml
        README.md
        crates/
```

The sub-workspace `Cargo.toml` files must:

- declare a real Cargo workspace
- own their local `members`
- be runnable directly for local build/test loops

The repo-root `Cargo.toml` must:

- stay thin
- avoid owning Road 1 packages as workspace members
- host only orchestration helpers, shared policy, or xtask-like whole-world
  commands
- invoke sub-workspace commands through explicit `--manifest-path` routing or an
  equivalent orchestrator surface

Sub-workspaces must:

- own their local `members`
- own local `default-members` when those help local iteration
- remain runnable directly without going through the repo-root manifest
- contain a small `README.md` workspace charter that states:
  - what constitutional class or classes may live there
  - what may not live there
  - where cross-workspace proof and tooling belongs instead

**Warnings**
- Do not create one giant Road 1 workspace instead of real sub-workspaces.
- Do not create fake workspaces that are only folders with no Cargo ownership.
- Do not centralize member ownership at repo root.
- Do not let a workspace grow ad hoc sibling buckets such as `helpers/`,
  `misc/`, `scratch/`, or `temporary/` as shadow ownership boundaries.

**Test requirements**
- Adversarial equivalence test: running Cargo commands from a sub-workspace and
  from the repo-root orchestrator surface should both target the same born crates
  through explicit manifest routing.
- Adversarial denial test: a workspace-topology check must fail if a new Road 1
  crate is added outside the declared `cad/workspaces/` workspaces or if the
  repo root becomes the actual owner workspace for a Road 1 crate.

**Engineering decisions**
- Make sub-workspaces the real ownership boundary.
- Make repo root an orchestration layer only, not a package-owning aggregator.
- Create empty Road 1 workspaces now even where this milestone does not yet
  birth member crates.

**Open questions**
- None. Cargo topology is part of the closure surface.

### Phase 3: Seed Crate Births

This phase births the first two legal Road 1 crates and freezes only their
directory and module skeletons.

**Relevant subsystems**
- pure meaning seed
- pack seam seed

**Relevant APIs**
- `cad/workspaces/worth-contracts/crates/worth-schema-core/Cargo.toml`
- `cad/workspaces/worth-contracts/crates/worth-schema-core/src/lib.rs`
- `cad/workspaces/worth-contracts/crates/worth-schema-core/src/facade.rs`
- `cad/workspaces/worth-packs/crates/worth-pack-registry/Cargo.toml`
- `cad/workspaces/worth-packs/crates/worth-pack-registry/src/lib.rs`
- `cad/workspaces/worth-packs/crates/worth-pack-registry/src/facade.rs`

**Directory skeleton**

`worth-schema-core` must start as:

```text
worth-schema-core/
  Cargo.toml
  AGENT_CONTEXT.md
  src/
    lib.rs
    facade.rs
    identity.rs
    naming.rs
    units.rs
    tolerance.rs
```

`worth-pack-registry` must start as:

```text
worth-pack-registry/
  Cargo.toml
  AGENT_CONTEXT.md
  src/
    lib.rs
    facade.rs
    registration/
      mod.rs
    contribution_kinds/
      mod.rs
```

Top-level crate skeleton rules for both born crates:

- no top-level `tests/`, `fixtures/`, `proof/`, or `compile_fail/` tree is born
  inside the seed crates in Milestone 1 unless the phase explicitly introduces
  real crate-owned behavior that requires it
- cross-crate denial proof, replay-fence proof, and freshness proof remain in
  `tools/` during Milestone 1 rather than being copied into each seed crate
- `AGENT_CONTEXT.md` is a generated artifact placeholder only at this phase; it
  becomes machine-owned in Phase 6
- every extra file or folder under `src/` must correspond to one named
  responsibility from the spec, not a convenience bucket

**Warnings**
- Do not create broad `mod.rs` bags outside honest child-responsibility
  folders.
- Do not place future Query adoption logic into `worth-schema-core` because the
  entry crate does not exist yet.
- Do not create decorative test trees, fixture bundles, or compile-fail folders
  inside the seed crates just to imply future seriousness.

**Test requirements**
- Adversarial equivalence test: each seed crate must be reviewable as one idea
  from its top-level directory skeleton alone.
- Adversarial denial test: a structure QA check must fail if a seed crate gains
  a mixed-class module such as `query_bridge.rs`, `runtime.rs`, `helpers.rs`,
  or `common.rs`.

**Engineering decisions**
- `worth-schema-core` is allowed to define only shared pure-meaning nouns that
  are not yet graph-specific.
- `worth-pack-registry` is allowed to define only pack seam and contribution
  category registration surfaces. It does not own Query contribution lowering,
  runtime contribution orchestration, or pack-executed capability admission.

**Open questions**
- None. The two seed crates are intentionally narrow.

### Phase 4: Facade And Visibility Contract

This phase freezes how the born seed crates may be consumed publicly.

It separates "the crate exists" from "the crate exports authority honestly."

**Relevant subsystems**
- crate facade topology
- public import shape
- internal module visibility

**Relevant APIs**
- `cad/workspaces/worth-contracts/crates/worth-schema-core/src/lib.rs`
- `cad/workspaces/worth-contracts/crates/worth-schema-core/src/facade.rs`
- `cad/workspaces/worth-packs/crates/worth-pack-registry/src/lib.rs`
- `cad/workspaces/worth-packs/crates/worth-pack-registry/src/facade.rs`

**Directory skeleton**

Visibility contract for all born seed crates:

- `src/lib.rs` exports only the crate facade
- internal modules are private or `pub(crate)`
- `facade.rs` aggregates public exports only
- `facade.rs` may not contain behavior-bearing implementation
- downstream crates must not be able to deep-import internal modules

**Warnings**
- Do not let `facade.rs` implement behavior; it may only aggregate.
- Do not publish convenience reexports that bypass the class boundary.
- Do not let public imports reveal unfinished future class surfaces.

**Test requirements**
- Adversarial equivalence test: opening only `lib.rs` and `facade.rs` must be
  enough for a reviewer to understand the public surface of each born crate.
- Adversarial denial test: a visibility proof must fail if downstream code can
  deep-import an internal module past the facade.
- Adversarial denial test: a structure check must fail if `facade.rs` gains
  behavior-bearing implementation rather than pure aggregation.

**Engineering decisions**
- Facade discipline is its own closure step, not an incidental property of seed
  crate creation.
- The first public import contract must be explicit before boundary-check begins
  enforcing cross-crate rules.

**Open questions**
- None. Public authority shape is part of Milestone 1, not later cleanup.

### Phase 5: Boundary Rule Engine

This phase turns the constitutional claims into machine-enforced failure.

**Relevant subsystems**
- boundary-check
- replay fence
- naming grammar
- tier/band dependency proof

**Relevant APIs**
- `tools/boundary-check/`
- repo-root CI entrypoints that invoke boundary-check
- `tools/boundary-check/Cargo.toml`
- `tools/boundary-check/src/main.rs` or one explicitly named equivalent
- `tools/boundary-check/config/road1.toml` or one explicitly named equivalent
- `tools/boundary-check/tests/fixtures/`

The required enforcement surfaces are:

- naming grammar enforcement
- band dependency enforcement
- tier direction enforcement (`worth-*` may not depend on `worthy-*`)
- replay fence compile-fail checks

`boundary-check` must have an explicit machine contract:

- inputs:
  - Road 1 naming grammar / reserved-name machine config
  - Cargo manifests and `cargo metadata` output for each Road 1 sub-workspace
  - Road 1 replay-surface marker config
  - negative fixture crates or fixture modules under
    `tools/boundary-check/tests/fixtures/`
- computation:
  - parse crate names into `{tier, band, domain}`
  - build the crate DAG from Cargo metadata
  - classify each crate by constitutional class
  - apply naming, band, tier, and replay rules
- outputs:
  - named diagnostic codes for each failed rule family
  - nonzero exit on any rule violation
  - machine-readable and human-readable failure summaries

**Directory skeleton**

This phase must leave an explicit enforcement skeleton:

```text
tools/
  boundary-check/
    Cargo.toml
    src/
      main.rs
    config/
      road1.toml
    tests/
      fixtures/
```

**Warnings**
- Do not use doc comments where a compile-fail or CI-fail is possible.
- Do not mix generated-orientation responsibilities into the rule engine
  implementation.
- Do not delay rule enforcement until Milestone 5.

**Test requirements**
- Adversarial equivalence test: the same illegal import or illegal replay use
  must fail the same way whether introduced by a human edit or an agent edit.
- Adversarial denial test: CI must fail on:
  - an illegal crate name
  - a schema crate importing Query
  - an ordinary crate importing replay
  - a `worth-*` crate importing a `worthy-*` crate
  - a root-owned Road 1 package

**Engineering decisions**
- Boundary enforcement is a milestone-local deliverable, not later infra debt.
- Replay fence exists now even before the richer replay proof crates land.
- Compile-fail and denial proofs that need ordinary-lane misuse specimens live
  under `tools/boundary-check/tests/fixtures/`; they do not require Road 1 to
  birth real ordinary product crates early.

**Open questions**
- None. This is a hard closure requirement.

### Phase 6: Generated Crate Context

This phase freezes how Road 1 crate-local orientation is produced and kept
fresh.

It separates "the rule engine knows the boundary" from "each crate carries the
same boundary locally in a machine-derived form."

**Relevant subsystems**
- agent-context generation
- generated orientation freshness
- crate-local boundary briefing

**Relevant APIs**
- `tools/agent-context/Cargo.toml`
- `tools/agent-context/src/main.rs` or one explicitly named equivalent
- `tools/agent-context/templates/`
- generated per-crate orientation files for:
  - `worth-schema-core`
  - `worth-pack-registry`

**Directory skeleton**

This phase must leave an explicit generated-context skeleton:

```text
tools/
  agent-context/
    Cargo.toml
    src/
      main.rs
    templates/
```

And generated crate-local orientation artifacts for each born seed crate at one
explicit path shape:

```text
<born-crate>/
  AGENT_CONTEXT.md
```

**Warnings**
- Do not hand-write per-crate orientation docs.
- Do not let crate-local context drift away from the machine rule source.
- Do not treat generated context as optional just because the born crates are
  still small.
- Do not let `AGENT_CONTEXT.md` become a substitute for the workspace charter;
  workspace and crate routing documents serve different boundaries.

**Test requirements**
- Adversarial equivalence test: regenerating crate-local context twice from the
  same inputs must produce stable outputs for the same born crates.
- Adversarial denial test: a stale hand-edited generated context file must fail
  freshness validation.
- Adversarial denial test: a crate with a changed constitutional class or public
  surface must fail until its generated context is refreshed.

**Engineering decisions**
- Generated crate context is part of the constitutional substrate.
- Agent guidance is produced from the same machine-owned boundary model that the
  rule engine enforces.
- Crate-local orientation is a separate deliverable from the rule engine so its
  failure mode stays visible.

**Open questions**
- None. Generated orientation is a first-class closure surface.

### Phase 7: Exemplar Routing And Deferred Public Proof

This phase proves the milestone did not only build empty boxes.

It does that by routing one exemplar family across the classes that exist now
and the classes that are intentionally deferred.

**Relevant subsystems**
- exemplar routing
- legal placeholder posture for later classes
- public proof surface for later milestones

The exemplar for this milestone is explicit:

- the foundational identity / naming / tolerance family rooted in
  `worth-schema-core`
- routed through the Road 1 naming and workspace topology as a pure-meaning
  specimen
- paired with one pack-seam descriptor specimen in `worth-pack-registry` that
  proves category registration shape only, not real pack contribution
- covered by generated crate context and `boundary-check` proof surfaces

**Relevant APIs**
- `BOUNDARIES.md` exemplar row updates for the Road 1 specimen
- `NAMING.md` reserved-name amendments for deferred-but-named later crates
- public facades of:
  - `worth-schema-core`
  - `worth-pack-registry`
- generated crate context outputs for the born seed crates

This phase must explicitly name the deferred follow-on APIs that later
milestones will own, without implementing them early:

- future `worth-entry-*` declaration/adoption facade
- future `worth-derived-*` retained/publication facade
- future `worthy-derived-brep` consumer-facing retained artifact path

**Directory skeleton**

No new born crate is required here. The directory obligation is instead that
the born crates expose stable facade-only surfaces and that deferred follow-on
surfaces are recorded in routing/naming artifacts rather than in speculative
code.

**Warnings**
- Do not birth placeholder crates just to make the exemplar look complete.
- Do not leave the exemplar as a verbal promise with no routing artifacts.
- Do not let the cert proof crate invent public ordinary APIs in order to fake
  completeness.

**Test requirements**
- Adversarial equivalence test: the exemplar family must be locatable from
  `BOUNDARIES.md`, `NAMING.md`, and the generated crate contexts without
  opening implementation folklore.
- Adversarial denial test: the milestone must prove that no deferred class is
  being smuggled through one of the two born seed crates as a compatibility
  shortcut.

**Engineering decisions**
- The exemplar for this milestone is the foundational identity / naming /
  tolerance family plus one pack-seam descriptor specimen. It is a routing and
  ownership proof only, not yet a real pack-admission proof and not yet the
  real BREP pressure specimen of Milestone 4.
- Deferred later classes must be recorded through naming reservations and
  routing entries, not through empty speculative crates.
- The public proof surface of this milestone is:
  - legal names
  - legal workspace topology
  - born seed facades
  - generated crate context
  - boundary-check failure on illegal continuation

**Open questions**
- None. The exemplar is intentionally narrow at this milestone.

## Must Ship

- one legal Road 1 naming inventory for the crates actually born
- one real `cad/workspaces/` Cargo topology with:
  - sub-workspace manifests
  - a thin repo-root aggregator
- two born seed crates with predictive facade skeletons:
  - `worth-schema-core`
  - `worth-pack-registry`
- one active mechanical enforcement set:
  - naming grammar
  - band DAG
  - tier direction
  - replay fence
  - generated crate context
- one exemplar routing/proof surface showing how deferred later classes will be
  reached without speculative placeholder crates

## Must Preserve

- pure meaning remains Query agnostic
- the repo root remains a thin aggregator, not the ordinary ownership surface
- no illegal crate names are taught by the spec or the implementation
- no born seed crate owns mixed constitutional authority
- replay remains cert-only from the first implementation cut onward
- later milestones still own:
  - graph constitution
  - Query-native declaration and obligation adoption
  - ordinary retained publication and real BREP downstream proof

## Acceptance Evidence

- `cad/workspaces/` exists with five real Road 1 sub-workspace `Cargo.toml`
  files
- repo-root `Cargo.toml` acts as thin whole-world aggregation only
- born seed crates exist at the exact filesystem locations declared in Phase 3
- each born seed crate contains `src/lib.rs` and `src/facade.rs`
- `worth-schema-core` contains only pure-meaning modules from the allowed
  Phase 3 skeleton
- `worth-pack-registry` contains only pack-seam modules from the allowed
  Phase 3 skeleton
- each born seed crate obeys the facade visibility contract:
  - `lib.rs` exports only the facade
  - no public deep import path exists into internal modules
- boundary-check fails on:
  - illegal Road 1 crate birth
  - schema crate importing Query
  - ordinary crate importing replay
  - `worth-*` crate importing `worthy-*`
- generated crate context exists for both born seed crates
- `BOUNDARIES.md` and `NAMING.md` record the exemplar routing and deferred
  follow-on legal names
- repo-root aggregation posture is explicit and machine-checked:
  - repo root orchestrates sub-workspace commands for whole-world CI
  - sub-workspaces own local `members`
  - sub-workspaces own local `default-members` when needed for local loops
- closeout hostile proof:
  `platform_constitution_m1_contract_stratification_refuses_hybridization`
  - named hostile subcases:
    - `illegal_crate_name_is_rejected`
    - `schema_query_import_is_rejected`
    - `ordinary_replay_import_is_rejected`
    - `mixed_class_seed_module_is_rejected`
    - `repo_root_does_not_become_workspace_owner`
    - `deferred_follow_on_surface_is_named_not_smuggled`
    - `query_bridge_module_in_schema_is_rejected`
    - `runtime_adapter_in_pack_registry_is_rejected`
    - `stale_hand_edited_agent_context_is_rejected`
    - `placeholder_worth_entry_birth_is_rejected`
    - `public_deep_import_past_facade_is_rejected`

## Sequencing Notes

- Milestone 2 may not begin until Milestone 1 enforcement is real in CI or the
  equivalent whole-world check path.
- Milestone 2 should consume `worth-schema-core` and the legal Cargo topology;
  it should not reopen where Road 1 code lives.
- Milestone 3 should consume the empty-but-real `worth-entry` workspace and the
  replay fence from this milestone.
- Milestone 4 should be the first place a real CAD/BREP retained-consumption
  specimen lands.
- Milestone 5 should be the first place the corrugated wall or equivalent
  `BOUNDARIES.md`-grade first-party pack specimen lands.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it makes the first constitutional split real in naming,
  Cargo topology, crate skeletons, and enforcement.
- Is the adversarial constraint precise and load-bearing? Yes: it is about
  later agents taking the cheapest local continuation and still landing in the
  honest path.
- Does the roadmap justify this milestone now? Yes: later Road 1 milestones
  need legal topology and enforcement first.
- Does the spec preserve crate authority boundaries? Yes.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs first because it closes the legal and physical substrate later
  milestones require.
