# Platform Constitution Milestone 1B: Query Constitution Enforcement

> **Status:** Draft
>
> **Purpose:** harden the Milestone 1 enforcement substrate until there is
> exactly one legal way to consume Worth Query from constitutional code, and
> every illegal way fails mechanically — at compile time where possible, at
> edit time otherwise, and always with a diagnostic that names the legal path.

## Goal

Milestone 1 froze *where* constitutional classes live and *which crates* may
depend on which. That fence is real but coarse: it operates on Cargo metadata
only, it fires in CI rather than in the authoring loop, and it says nothing
about *how* Query authority is exercised inside a legally-placed crate.

By the end of this milestone:

- Query consumption is physically split by audience, so the crate DAG itself
  carries the law instead of one blunt "imports worth-query" bit
- authority to progress Query work is proof-carrying and unforgeable, using
  `worth-proof` typestate and witnesses with a sealed concrete-authority law
- illegal continuations fail inside the agent's edit loop, not minutes later
  in CI
- every widening of a constitutional surface — a new public export, a new
  dependency edge — is a visible reviewable diff, never a silent drift
- every denial diagnostic teaches the legal alternative
- the first certification crate exists and owns the hostile compile-fail
  corpus that proves all of the above under attack

This milestone does **not** implement the Query declaration bridge, obligation
adoption semantics, or ordinary entry lanes. Those remain Milestone 3 and 4
work. This milestone builds the fence those milestones will be forced through.

## Why This Milestone Exists

The platform is being built primarily by AI agents, and the framework is not
in any model's training data. An agent under pressure does not consult
folklore; it takes the cheapest local continuation that compiles. Milestone 1
made the cheapest *placement* the honest one. This milestone makes the
cheapest *usage* the honest one.

The specific failure shapes this milestone exists to kill:

- an entry crate legally imports Query, then re-exports or returns Query types
  from its facade, so downstream crates consume Query without ever depending
  on it ("type laundering")
- an agent blocked by a missing authority invents a parallel one
  (`struct MyAuth; impl AuthorityMarker for MyAuth {}`) and the open
  `worth-proof` substrate waves it through
- an agent widens a facade "for convenience" and nothing forces the widening
  to be seen
- enforcement fires in CI after the agent has already built three things on
  top of the mistake, so the correction costs a full rework instead of one
  in-loop retry
- a denial tells the agent "no" without telling it "here instead", so the
  agent routes around the fence instead of through the gate

## Governing Summaries

- `MENTALITY.md`
  - Protects: foundation-first sequencing under hostile conditions.
  - Strongest implication here: the Query fence must exist before Milestones
    2-4 generate the code that would otherwise define it by accident.
- `arch_laws.md`
  - Protects: typed phase progression, contractual facades, compiler-visible
    authority.
  - Strongest implication here: Query authority must be carried as types and
    witnesses, not as convention or review vigilance.
- `composition_laws.md`
  - Protects: named semantic responsibilities.
  - Strongest implication here: each audience facade owns one consumption
    posture; the rule engine gains named rule families, not one grab-bag pass.
- `domain_structure_laws.md`
  - Protects: physical boundaries that preserve authority and truth source.
  - Strongest implication here: the audience split must be real crates with
    real DAG edges, not doc-comment guidance over one wide crate.
- `perf_laws.md`
  - Protects: hot-path honesty and carried proof.
  - Strongest implication here: all proof machinery must be zero-sized and
    compile-time; enforcement may not tax the ordinary runtime path.
- `road-1.md` and `road-1_milestone-1.md`
  - Protect: Road 1 as a short sequence of real constitutional closures, with
    mechanical enforcement in the first closure surface.
  - Strongest implication here: this milestone deepens the Milestone 1
    enforcement mandate; it does not open graph, bridge, or lane semantics.

## Adversarial Constraint

An agent implementing Milestone 2, 3, or 4 work must be structurally unable
to succeed at any of the following, and every failure must name the legal
continuation:

- depend on the `worth-query` engine crate directly from governed code
- consume a Query surface from a band that is not that surface's audience
- mint, forge, or substitute an authority the platform did not issue
- move a value past a progression stage without the stage's ceremony
- export a Query type or re-export a Query item through a governed facade
- widen any governed public surface or dependency graph without producing a
  reviewable snapshot diff in the same change
- reference the retired `forge-*` naming in governed code

The hostile condition is:

> multiple agents build Milestones 2-4 in parallel, each starved for context,
> each taking the cheapest continuation that compiles, and each treating any
> compiling continuation as approval.

This milestone succeeds only if the single continuation that compiles — and
survives the edit-time check — is the constitutional one.

## Product Decision Lock

- The Query engine keeps its facade-only posture, but governed code never
  consumes it directly. Consumption flows only through **audience facade
  crates**:
  - `worth-query-decl` — declaration nouns and handles; entry-band audience
  - `worth-query-host` — admission, lowering, execution authority; entry-band
    audience
  - `worth-query-replay` — replay and reconstruction; cert-band audience
- Audience facades are thin re-export crates over the engine. Splitting the
  engine's internals is explicitly out of scope; the facades freeze the
  consumption grammar so the engine can be reorganized later behind them.
- Audience facades are platform framework crates. They live in the root
  `crates/` workspace beside the engine, outside the
  `{tier}-{band}-{domain}` birth grammar, and are recorded in `NAMING.md`
  as a framework-family exemption in the same change that births them.
- `worth-proof` is the only blessed compile-time law substrate. It is legal
  in **every** band and tier, including schema, and this is recorded in the
  machine config rather than discovered by argument.
- The authority sealing law: public items on governed crates must demand
  **concrete** platform authority types
  (`AuthorityWitness<EntryAdmission>`, `Proof<Fact, EntryAdmission>`).
  Generic `AuthorityMarker` / `CapabilityMarker` / `AuthorityProves` bounds
  are forbidden on governed public surfaces. `worth-proof` stays an open
  substrate; the *domain* crates close it.
- Enforcement gains a source-level (AST) pass. Cargo metadata remains the
  DAG truth; the AST pass owns import-shape, signature-leak, re-export, and
  rename-drift rules that metadata cannot see.
- All governed surface inventories become committed **snapshots** (crate DAG,
  facade manifests, legacy-reference allowlist). Snapshots are the interchange
  artifact between `boundary-check` (verifies) and `agent-context` (renders).
  Widening without a snapshot diff in the same change is a CI failure.
- Enforcement runs in the agent edit loop via hooks, and CI runs the exact
  same entrypoint. Hook and CI may never diverge in what they check.
- Every diagnostic carries a required legal-home pointer. A denial without a
  taught alternative is itself a rule-engine test failure.
- **Amendment to Milestone 1:** the first cert crate birth moves from
  Milestone 5 to this milestone. `worth-cert-adoption` (already reserved in
  `NAMING.md` as the Query adoption proof harness home) is born here and owns
  the hostile compile-fail corpus. Milestone 5 still owns the broad cert
  suite, parity proof, and pack-seam specimens.
- Milestone 1 residue is cleaned here, not carried: the seed-crate-local
  deep-import proof relocates to the corpus, and the seed skeleton additions
  that landed beyond the Phase 3 skeleton are recorded as reviewed amendments.
- The `forge-*` to `worth-*` rename is complete in the tree; this milestone
  adds the ratchet that keeps it complete. Finishing any residual rename work
  is not itself in scope.

## Query Audience Matrix

This table is part of the closure surface. It is mirrored in machine config;
the config is canonical.

| Surface | Kind | Audience (may depend on it) | Everyone else |
|---|---|---|---|
| `worth-query` (engine) | framework engine | audience facades only | denied |
| `worth-query-decl` | audience facade | `entry` band (worth + worthy), cert | denied |
| `worth-query-host` | audience facade | `entry` band (worth + worthy), cert | denied |
| `worth-query-replay` | audience facade | `cert` band only | denied |
| `worth-proof` | law substrate | every band, every tier | — |

Deliberately absent:

- a derived-band Query audience. Milestone 4 decides whether derived
  projection consumption enters through `worth-query-decl` or a new facade;
  it must amend this matrix visibly rather than inherit an accidental edge.

## Phase Plan

### Phase 1: Milestone 1 Residue And Rename Ratchet

This phase leaves Milestone 1 with no unrecorded deviations and freezes the
completed rename so it cannot silently regress.

**Relevant subsystems**
- seed crate skeletons
- boundary-check source rules (rename drift)
- Milestone 1 amendment record

**Relevant APIs**
- `cad/workspaces/worth-contracts/crates/worth-schema-core/tests/`
- `cad/workspaces/worth-packs/crates/worth-pack-registry/src/registration/`
- `tools/boundary-check/config/road1.toml` seed skeleton allowlists
- `tools/boundary-check/snapshots/legacy-references.toml`

**Directory skeleton**

- `worth-schema-core` loses its top-level `tests/` tree. The deep-import
  denial proof moves to the `worth-cert-adoption` corpus (Phase 8); the
  `compile_fail` doctest in `lib.rs` remains as the crate-local statement of
  the same law. The seed skeleton allowlist shrinks to match.
- `PackRegistration` composes its `ContributionDescriptor` instead of
  duplicating its fields. No public surface change.
- The Phase 3 skeleton additions that already landed (`identity_name.rs`,
  `contribution_descriptor.rs`, `pack_name.rs`) are recorded in this spec as
  reviewed amendments to the Milestone 1 skeleton, closing the gap between
  the M1 acceptance wording and the tree.
- A legacy-reference ratchet is added: governed surfaces
  (`cad/workspaces/`, `tools/`, `crates/worth-proof/`) may not gain new
  `forge_` / `forge-` references. Any grandfathered references are pinned in
  a committed allowlist snapshot that may only shrink.

**Warnings**
- Do not "improve" the seed crates beyond the named cleanups; this phase
  closes residue, it does not open redesign.
- Do not let the rename ratchet block `_docs/` history or non-governed legacy
  code; it governs the constitutional surfaces only.

**Test requirements**
- Adversarial equivalence test: the deep-import law proves identically before
  and after the proof relocation (doctest denial in-place, corpus specimen in
  Phase 8).
- Adversarial denial test: introducing a new `forge_query`-shaped reference
  under a governed surface fails with a pointer to the worth-name and the
  allowlist snapshot.

**Engineering decisions**
- Cleanup lands before fence-widening so later phases diff against a clean
  substrate.
- Amendments are recorded in the spec and the machine config in the same
  change — the M1 pattern for visible acts.

**Open questions**
- None.

### Phase 2: Query Audience Facade Topology

This phase makes the audience split physically real.

**Relevant subsystems**
- Query engine facade
- audience facade crates
- root workspace membership

**Relevant APIs**
- `crates/worth-query/src/facade.rs`
- `crates/worth-query-decl/src/lib.rs`
- `crates/worth-query-host/src/lib.rs`
- `crates/worth-query-replay/src/lib.rs`
- `cad/docs/worthy-foundations/NAMING.md` framework-family amendment

**Directory skeleton**

Each audience facade starts as:

```text
crates/worth-query-<audience>/
  Cargo.toml
  AGENT_CONTEXT.md
  src/
    lib.rs        # doc header: audience, law, blessed example
    facade.rs     # re-exports from worth-query only; no behavior
```

Facade crate rules:

- depends on `worth-query` and nothing else (plus `worth-proof` where the
  authority vocabulary requires it in later milestones)
- `facade.rs` contains re-exports only; a facade that needs behavior is
  evidence the engine facade is missing a surface, and the engine grows it
- every re-exported item carries a doc comment with at least one runnable
  example; the facade doctests are the canonical usage corpus for agents
- `worth-query-replay` re-exports nothing in this milestone beyond the
  replay-surface markers boundary-check already fences; it exists so the
  cert-only edge is real in the DAG from birth

**Warnings**
- Do not split the engine internals; the facades freeze grammar, not
  implementation.
- Do not let a facade re-export another facade; each is a leaf audience.
- Do not populate `worth-query-decl`/`-host` with speculative bridge nouns;
  Milestone 3 owns what they eventually re-export. Seed them with the
  narrowest honest surface the engine facade already exposes for their
  audience, even if that is initially very small.

**Test requirements**
- Adversarial equivalence test: an entry-band consumer compiled against
  `worth-query-decl` + `worth-query-host` observes identical types to the
  engine facade (re-export identity, no wrapper drift).
- Adversarial denial test: a governed crate adding a direct `worth-query`
  dependency fails BC-family diagnostics naming the audience facade to use
  instead.
- Adversarial denial test: a derived-band or schema-band crate depending on
  any audience facade fails with the audience matrix quoted.

**Engineering decisions**
- Audience facades are framework crates in the root workspace, exempt from
  the band grammar, recorded in `NAMING.md` in the same change.
- The engine package name lives in exactly one machine-config key so the
  fence survives any future engine reorganization.

**Open questions**
- None. The audience matrix above is the decision.

### Phase 3: Authority Sealing Law

This phase closes the forgeability seam in the `worth-proof` substrate at the
domain boundary.

**Relevant subsystems**
- worth-proof consumption posture
- governed public signatures
- boundary-check signature rules

**Relevant APIs**
- `crates/worth-proof/src/proof/witnesses.rs` (consumed, not modified)
- `tools/boundary-check/src/source_rules.rs` (new, shared with Phase 5)
- `tools/boundary-check/config/road1.toml` `[law_substrates]` entries

**The law**

`worth-proof` stays an open substrate: anyone may define markers for their own
domain. What is sealed is the **platform's** vocabulary:

- public items on governed crates must name concrete platform authority and
  capability types; generic bounds over `AuthorityMarker`,
  `CapabilityMarker`, `AuthorityProves`, or `ProofSetAuthorizedBy` are
  forbidden on governed public surfaces
- platform authority types are value-gated: private field, no `Default`, no
  public constructor; the only mint is the owning crate's ceremony function
- a forged authority may therefore satisfy `worth-proof`'s generic machinery,
  but it can never satisfy a governed signature, so it opens no doors

**Directory skeleton**

No new crates. `worth-proof` is added to the allowed-everywhere substrate
list in machine config. The signature rule lands in the boundary-check AST
pass.

**Warnings**
- Do not seal `worth-proof` itself; product tiers and packs need the open
  substrate for their own domains.
- Do not allow "temporary" generic authority bounds on governed surfaces to
  ease Milestone 3 bring-up; the entire point is that bring-up happens inside
  the sealed grammar.

**Test requirements**
- Adversarial equivalence test: a legal governed signature demanding
  `AuthorityWitness<ConcreteAuthority>` passes the rule identically whether
  the type is named directly or via a re-export within the same crate.
- Adversarial denial test: a governed public fn generic over
  `Auth: AuthorityMarker` fails with the sealing law quoted and the concrete
  pattern shown.
- Adversarial denial test (corpus, Phase 8): the forged-authority attack —
  local marker type plus `AuthorityProves` impl — compiles against
  `worth-proof` alone but fails to type-check against any governed ceremony
  signature.

**Engineering decisions**
- Sealing is enforced at the signature level by the rule engine, not by
  modifying `worth-proof`; the substrate's openness is a feature.
- The concrete-authority pattern is documented in the governed facades'
  doctests so the legal spelling is the discoverable one.

**Open questions**
- None.

### Phase 4: Band Guard Macro

This phase gives compile-time law a way to see crate identity, closing the
gap between "the type system enforces ceremony" and "the DAG decides who may
hold the ceremony".

**Relevant subsystems**
- worth-proof law machinery
- future declaration macro surfaces

**Relevant APIs**
- `crates/worth-proof/src/band.rs` (new)
- `worth_proof::band_guard!` (new exported macro)

**Directory skeleton**

```text
crates/worth-proof/src/
  band.rs        # const prefix checker + band_guard! macro
```

The mechanism: `band_guard!("worth-entry-", "worthy-entry-")` expands to a
`const` assertion over `env!("CARGO_PKG_NAME")`, which resolves in the
**expanding** crate. Expansion in a crate outside the listed prefixes is a
compile error whose message names the legal bands and points at
`BOUNDARIES.md`. The checker is a dependency-free `const fn` over bytes.

Adoption law (recorded now, exercised in Milestone 3): every public macro on
a Query audience facade embeds a band guard for its audience. Declaration
macros that lower into Query handles are the first mandatory adopters.

**Warnings**
- Do not encode the band list inside `worth-proof`; the guard takes prefixes
  as arguments — `worth-proof` supplies mechanism, the facades supply law.
- Do not treat the guard as a substitute for DAG rules; it is the backstop
  for surfaces that travel through macro expansion where the DAG is blind.

**Test requirements**
- Adversarial equivalence test: the guard accepts every legal prefix spelling
  it is given and is a zero-cost `const` in the expansion.
- Adversarial denial test (corpus, Phase 8): expanding a guarded macro inside
  a wrong-band fixture crate fails with the band list in the error text.

**Engineering decisions**
- Mechanism lives in `worth-proof` because it is dependency-free compile-time
  law machinery legal in every band.
- The guard message format is part of the prescriptive-diagnostics law
  (Phase 7): it must name the legal bands.

**Open questions**
- None.

### Phase 5: Source-Level Import Law

This phase gives boundary-check eyes below Cargo metadata.

**Relevant subsystems**
- boundary-check AST pass
- type-laundering fence
- re-export fence

**Relevant APIs**
- `tools/boundary-check/src/source_rules.rs`
- `tools/boundary-check/config/road1.toml` `[query_fence]` block

**Rule families**

- **import shape**: no `worth_query`-rooted (or engine-rooted) paths in the
  source of governed crates outside the audience matrix — catches
  dev-dependencies, `cfg`-gated imports, and fully-qualified paths that never
  touch the manifest
- **signature leak**: no Query-rooted types in the public signatures of
  governed crates; entry crates consume Query, they do not emit it —
  detection is name-based over the facade-manifest item set plus crate-root
  path segments, and this limitation is stated in the rule's docs
- **re-export fence**: no `pub use` of any Query item from a governed crate;
  the audience facades themselves are the only legal re-exporters
- **sealing law** (Phase 3's rule, same pass): no generic authority bounds on
  governed public items

**Directory skeleton**

```text
tools/boundary-check/src/
  source_rules.rs      # syn-based pass, one named rule family per fn
tools/boundary-check/tests/fixtures/
  query_type_laundering/
  query_pub_use_reexport/
  query_source_import_bypass/
  generic_authority_bound/
```

**Warnings**
- Do not attempt full type resolution; name-based detection plus the Phase 6
  manifest freeze covers the alias gap honestly. State what the pass cannot
  see rather than implying it sees everything.
- **Reviewed enforcement amendment:** authority sealing is definition-resolved
  across the compiled local module graph and path-dependency graph. This is a
  deliberately stronger closure for proof-carrying authority ceremonies, not
  general Rust type resolution and not Query-type inference. Query signature
  detection remains name-based over the committed facade vocabulary. The
  stronger authority closure is required because aliases, re-exports, blanket
  bounds, and exported macros otherwise create caller-mintable constitutional
  bypasses while still looking concrete at the governed surface.
- Do not merge the AST pass into the manifest pass; they are separate rule
  families with separate failure modes.

**Test requirements**
- Adversarial equivalence test: the same laundering shape fails identically
  whether introduced as `pub use`, `pub fn -> QueryType`, or a public struct
  field.
- Adversarial denial test: each new fixture fails with its named diagnostic
  code and a legal-home pointer.
- Adversarial denial test: a governed crate importing the engine through a
  renamed dependency (`[dependencies] q = { package = "worth-query" }`)
  is still caught by the metadata pass, proving the two passes overlap rather
  than gap.

**Engineering decisions**
- The AST pass runs only over governed crates, keeping the check fast enough
  for the edit loop.
- Rule families get distinct diagnostic codes so corpus tests can assert
  exact failures.

**Open questions**
- None.

### Phase 6: Surface And DAG Ratchets

This phase makes every widening a visible diff.

**Relevant subsystems**
- crate DAG snapshot
- facade manifest snapshot
- generated-context unification

**Relevant APIs**
- `tools/boundary-check/snapshots/crate-dag.toml`
- `tools/boundary-check/snapshots/facades.toml`
- `tools/boundary-check/src/snapshots.rs` (new)
- `tools/agent-context/src/boundary_model.rs` (consumes snapshots)

**Directory skeleton**

```text
tools/boundary-check/
  snapshots/
    crate-dag.toml        # every governed dependency edge, exact set
    facades.toml          # every governed public facade item, per crate
    legacy-references.toml # Phase 1 ratchet allowlist
```

Ratchet semantics:

- the computed value must **exactly equal** the snapshot — additions and
  removals both require a snapshot update in the same change
- `boundary-check --update-snapshots` is the only writer; CI runs without the
  flag and fails on drift
- facade manifests are extracted from `facade.rs` files by the AST pass, so
  the manifest, the visibility contract, and the generated context all derive
  from one extraction
- `agent-context` renders its `Facade exports` lines from `facades.toml`
  instead of re-deriving them; snapshots are the interchange artifact between
  the two tools — no shared library crate is introduced for this

**Warnings**
- Do not let snapshots become approximate ("at least these edges"); exactness
  is what makes the diff reviewable.
- Do not hand-edit snapshots; the updater writes them, review reads them.
- Do not snapshot non-governed crates; the ratchet governs the constitution,
  not the whole repository.

**Test requirements**
- Adversarial equivalence test: regenerating snapshots twice from the same
  tree is byte-identical (stable ordering).
- Adversarial denial test: a new dependency edge on a governed crate fails
  until `crate-dag.toml` is regenerated in the same change.
- Adversarial denial test: a new `pub use` in a governed `facade.rs` fails
  until `facades.toml` is regenerated, and the regenerated file makes the
  widening visible as a one-line diff.

**Engineering decisions**
- Exact-set equality over subset checks; ratchets that only catch additions
  rot silently.
- Snapshot regeneration is explicit and flag-gated so an agent cannot satisfy
  the ratchet by accident.

**Open questions**
- None.

### Phase 7: Edit-Time Enforcement And Prescriptive Diagnostics

This phase moves enforcement into the authoring loop and makes every denial
teach.

**Relevant subsystems**
- agent hook integration
- whole-world check entrypoint
- diagnostic contract

**Relevant APIs**
- `scripts/check-constitution.ps1` (single entrypoint: boundary-check +
  agent-context check)
- `.claude/settings.json` PostToolUse hook invoking the entrypoint with
  `--format json` when edited paths fall under governed surfaces
- `.github/workflows/ci.yml` constitution job invoking the same entrypoint
- `tools/boundary-check/src/diagnostics.rs` `legal_home` field

**Directory skeleton**

No new crates. One script, one hook entry, one CI job — all three invoke the
identical command so hook, CI, and a human terminal can never disagree.

Diagnostic contract:

- every `Diagnostic` carries a required, non-empty `legal_home` pointer:
  where the denied thing belongs, or which snapshot/config to amend and how
- human rendering prints it as a `belongs:` line; JSON rendering carries it
  as a field agents can act on
- a diagnostic emitted without a legal-home pointer is itself a rule-engine
  unit-test failure

**Warnings**
- Do not let the hook check a different rule set, scope, or config than CI;
  divergence here recreates the exact folklore problem this road exists to
  kill.
- Do not make the hook so slow that it gets disabled; it runs the prebuilt
  rule engine over governed paths only, with a stated time budget.
- Do not point diagnostics at prose alone; every pointer names a machine
  artifact (config key, snapshot file, facade) first and a doc second.

**Test requirements**
- Adversarial equivalence test: the entrypoint invoked by hook, by CI, and by
  hand produces identical diagnostics for the same tree state.
- Adversarial denial test: a unit test walks every diagnostic constructor and
  fails on any empty or missing `legal_home`.
- Adversarial denial test: an edit introducing an illegal Query import under
  `cad/workspaces/` produces a JSON diagnostic naming the correct audience
  facade, demonstrated in a hook-shaped integration test.

**Engineering decisions**
- One entrypoint, three invokers. The entrypoint is the contract; the
  invokers are transport.
- Diagnostics are steering inputs for agents; the legal-home field is the
  mechanism that converts a failed attempt into a correct second attempt.

**Open questions**
- None.

### Phase 8: Cert Corpus Birth

This phase births `worth-cert-adoption` and gives the hostile corpus a
permanent constitutional home.

**Relevant subsystems**
- first cert crate birth (amended forward from Milestone 5)
- trybuild compile-fail corpus
- corpus growth law

**Relevant APIs**
- `cad/workspaces/worth-certification/crates/worth-cert-adoption/`
- `cad/docs/worthy-foundations/NAMING.md` birth record (name already
  reserved for the Query adoption proof harness)

**Directory skeleton**

```text
worth-cert-adoption/
  Cargo.toml
  AGENT_CONTEXT.md
  src/
    lib.rs
    facade.rs
  tests/
    compile_fail.rs          # trybuild driver
    specimens/
      forged_authority.rs
      deep_import_past_facade.rs      # relocated from worth-schema-core
      replay_facade_in_ordinary_band.rs
      band_guard_wrong_band.rs
      generic_authority_bound_public_surface.rs
```

Corpus law:

- cert crates may depend broadly (`worth-proof`, audience facades, seed
  crates); nothing ordinary depends back
- every new public item on a Query audience facade must land with at least
  one corpus specimen proving its illegal twin fails; the pairing is checked
  against `facades.toml` so the corpus cannot silently lag the surface
- specimens assert on diagnostic codes and stable message fragments, not on
  full compiler prose, so toolchain updates do not rot the corpus

**Warnings**
- Do not let the corpus invent public ordinary APIs to fake coverage; it
  proves denial, not features.
- Do not grow a parity/scale/regression tree here; Milestone 5 owns those.

**Test requirements**
- Adversarial equivalence test: every named hostile subcase in this spec's
  closeout maps to exactly one specimen or rule-engine fixture, enumerated in
  one place.
- Adversarial denial test: deleting any specimen fails the facade-pairing
  check, proving the corpus is load-bearing rather than decorative.

**Engineering decisions**
- The corpus lives in cert, not `tools/`, because it proves Rust-level law
  (typestate, sealing, guards) rather than repository-shape law; the two
  proof homes stay separate on purpose.
- `worth-cert-adoption`'s early birth is the explicit Milestone 1/5 amendment
  this spec records; it fills the empty `worth-certification` workspace with
  its intended first tenant.

**Open questions**
- None.

## Must Ship

- three audience facade crates with doctest-bearing, snapshot-frozen surfaces
- machine-config audience matrix with `worth-proof` blessed in every band
- authority sealing law enforced by the AST pass
- `band_guard!` mechanism in `worth-proof` with its adoption law recorded
- source-level rule families: import shape, signature leak, re-export fence,
  sealing law, rename ratchet
- exact-set snapshots: crate DAG, facade manifests, legacy references, with
  flag-gated regeneration
- one enforcement entrypoint wired identically into hook, CI, and terminal
- required legal-home pointers on every diagnostic
- `worth-cert-adoption` born with the hostile compile-fail corpus and the
  facade-pairing check
- Milestone 1 residue closed: relocated deep-import proof, composed
  registration, recorded skeleton amendments

## Must Preserve

- pure meaning remains Query agnostic; no schema surface gains any Query
  audience
- the engine's facade-only posture; audience facades add no behavior
- `worth-proof` remains open, dependency-free, and zero-sized at runtime
- Milestone 3 and 4 still own bridge semantics, obligation adoption, entry
  lanes, and derived publication; this milestone ships fences, not lanes
- Milestone 5 still owns the broad cert suite and pack-seam specimens
- the repo root remains a thin orchestrator; no governed package moves to
  root ownership

## Acceptance Evidence

- the audience matrix is enforced: entry fixtures compile against decl/host,
  every other band is denied with the matrix quoted
- direct engine dependency from governed code is denied at both the metadata
  and source layers
- the forged-authority attack compiles against `worth-proof` alone and fails
  against every governed ceremony signature
- a guarded macro expansion fails in a wrong-band fixture with the band list
  in the error
- type laundering, `pub use` re-export, and generic-authority-bound fixtures
  each fail with distinct named diagnostic codes
- snapshot drift in DAG, facades, or legacy references fails CI and the
  regenerated snapshot shows the widening as a reviewable diff
- hook, CI, and terminal invocations of the entrypoint produce identical
  diagnostics
- every diagnostic constructor is proven to carry a legal-home pointer
- `worth-cert-adoption` exists at its reserved name with the corpus and the
  facade-pairing check active
- closeout hostile proof:
  `platform_constitution_m1b_query_fence_refuses_bypass`
  - named hostile subcases:
    - `direct_engine_dependency_is_rejected`
    - `wrong_band_audience_import_is_rejected`
    - `replay_facade_in_ordinary_band_is_rejected`
    - `forged_authority_is_rejected`
    - `generic_authority_bound_on_governed_surface_is_rejected`
    - `band_guard_wrong_band_expansion_is_rejected`
    - `query_type_laundering_is_rejected`
    - `query_pub_use_reexport_is_rejected`
    - `renamed_dependency_engine_import_is_rejected`
    - `facade_widening_without_manifest_is_rejected`
    - `new_dag_edge_without_snapshot_is_rejected`
    - `new_forge_reference_is_rejected`
    - `diagnostic_without_legal_home_is_rejected`
    - `hook_and_ci_diagnostics_are_identical`
    - `corpus_specimen_deletion_is_rejected`

## Sequencing Notes

- Milestone 2 (Graph Constitution) may not begin until this milestone's
  entrypoint is green in CI and active as an edit-time hook.
- Milestone 3 consumes the audience facades, the sealing law, and the band
  guard as its starting grammar; it defines the platform authority vocabulary
  (`EntryAdmission`-class types, fact markers, obligation adoption proofs)
  inside fences that already exist.
- Milestone 4's derived-consumption decision amends the audience matrix
  visibly if derived surfaces need a Query audience.
- Milestone 5 inherits `worth-cert-adoption` as an existing tenant and adds
  the broad suite beside it rather than founding cert posture from scratch.

## Required Self-Check

- Does the milestone solve a real structural problem or just package work
  cosmetically? Yes: it converts Query discipline from convention plus coarse
  DAG rules into layered mechanical law — DAG, source, type system, macro
  expansion, snapshots, and the edit loop.
- Is the adversarial constraint precise and load-bearing? Yes: parallel
  context-starved agents treating "it compiles" as approval, with the
  requirement that exactly one continuation compiles.
- Does the roadmap justify this milestone now? Yes: Milestones 2-4 generate
  the code this fence must already contain; retrofitting it would re-litigate
  every surface.
- Does the spec preserve crate authority boundaries? Yes: facades re-export,
  the engine keeps implementation, cert proves, tools enforce, and no
  governed crate gains mixed authority.
- Are the phases carrying most of the real design information? Yes.
- Is each phase centered on one conceptual detail or boundary? Yes: residue,
  audience topology, sealing, expansion guard, source law, ratchets, edit
  loop, corpus.
- Does each phase contain at least 2 adversarial tests by default? Yes.
- Could a competent engineer map this spec into honest types, modules, and
  tests? Yes.
- Does the milestone belong in this roadmap sequence, or is it out of order?
  It belongs between Milestones 1 and 2: it deepens Milestone 1's enforcement
  mandate and must exist before parallel domain work begins.
