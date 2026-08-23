# Milestone 9.17.1 Closure Ledger

This ledger is the durable handoff for the phase gates in
`milestone-9.17.1.md`. A phase remains open until its row has production
evidence, focused proof, the required independent QA reviews, and a final Sol review.
Evidence is scoped to the implementation surface; a green unrelated suite does
not close a row.

## Phase status

| Phase | Scope | Status | Required next gate |
| --- | --- | --- | --- |
| 1 | Foundational exact branch-reference grammar and candidate-vocabulary migration | Closed — Luna and Sol certified | Phase 2 plan and two plan critics |
| 2 | Supply Chain semantic world and independent oracle | Closed — Luna and Sol certified | Phase 3 plan and two plan critics |
| 3 | Production-backed Supply Chain compiler and baseline audit | Closed — Luna and Sol certified | Phase 4 plan and two plan critics |
| 4 | Relational immutable commit/reference split and branch-local MVCC foundation | Closed — independent qa-loop, qa-tests, and code-quality-qa certified `7cbeb3a8645809890143b28117b5e6fc87aeb3cc` | Hand off to Phase 5/6 without widening the compatibility inventory |
| 5 | Immutable branch roots, persistent COW, branch-qualified inspection/traversal, and sharing/cost evidence | Closed — corrected packet `20452f29...`; fresh Sol-high qa-loop, qa-tests, and code-quality trio CLEAN; separate final Sol-high gate CLEAN | Phase 6 Sol-high plan-implementation review |
| 6 | Exact owner observations, descriptor readmission, repeatable reads, explicit external retention, and exact-root read cutover | Open — first Sol-high trio reopened exact-root joins, registry cost proof, Bridge binding races, and composition; corrected packet and local evidence green | New fresh separate qa-loop, qa-tests, and code-quality-qa critics, then a separate Sol-high final gate |

## Phase 1 evidence ledger

| Claim | Implementation surface | Proof artifact | Expected counter/result | Mutation that must fail | Status |
| --- | --- | --- | --- | --- | --- |
| Candidate and exact-reference meanings are distinct | `worth-foundational/src/transitions/branches/{candidate,identity,local_state,reference,fork,comparison}.rs`; candidate consumers and facade exports | `tests/certification/transitions/branch_local.rs`; `tests/ui/transitions/branch_reference/candidate_basis_cannot_be_exact_fork.rs` | candidate artifact remains descriptive; exact fork requires a complete observation | restore an alias or constructor accepting an epoch-shaped candidate basis | Evidence green; independent review pending |
| Empty and basis targets are explicit | `reference.rs` `FoundationalBranchTarget` | `branch_reference::target_keeps_empty_and_basis_as_explicit_distinct_variants` | `Empty != Basis`; no missing lookup represents empty | replace `Empty` with `Option<T>` or default target | Evidence green; independent review pending |
| Exact observations compare all structural axes | `reference.rs` observation and mismatch types | `branch_reference::mismatch_reports_all_structural_axes_in_deterministic_order` | branch, target, generation axes are retained in order | compare only commit/digest or return one generic stale flag | Evidence green; independent review pending |
| Canonical encoding is deterministic and variant-tagged | `reference.rs` target encoding and observation encoder | `branch_reference::canonical_encoding_is_versioned_variant_tagged_and_round_trips` plus hand-authored Empty/Basis vectors | hand-authored Empty and Basis encodings match; serde round-trip preserves descriptive value | derive expected bytes from the same production encoder or omit the variant/generation | Evidence green; independent review pending |
| Descriptive transport cannot bypass structural constructors | manual `Deserialize` for `FoundationalBranchId` and `FoundationalBranchTargetEncoding` | `branch_reference::malformed_transport_cannot_bypass_reference_validation` | empty/whitespace branch ids, empty domains, zero schema versions, and malformed nested observation payloads are rejected after decode | derive deserialization directly over private fields and admit malformed values | Evidence green; independent review pending |
| Generation cannot wrap | `reference.rs` generation and checked advance denial | `branch_reference::generation_advancement_is_checked_and_never_wraps` | zero initial value; `u64::MAX` returns typed overflow | wrapping, saturating, or reusing a generation | Evidence green; independent review pending |
| Foundational stays descriptive and non-authoritative | exact-reference modules contain no runtime tables, leases, proof carriers, or minting APIs | `tests/ui/transitions/branch_reference/{equivalence_id_cannot_be_target_basis,exact_reference_has_no_authority_mint}.rs`; boundary/context/line-cap checks | raw equivalence ids cannot implement target basis; authority mint call is absent | add a generic authority marker or public owner-proof constructor | Evidence green; independent review pending |
| Owner adapters use the shared grammar | `worth-relational/src/branch/{reference,target}.rs`; `worth-signal/src/branch/{reference,target}.rs`; owner facades | deterministic `branch_reference_contract.rs` mapping suites in both owner crates | owner-shaped descriptors lower to runtime/graph-affine Foundational observations; Empty remains owner-affine; exact owner target bytes are pinned | keep local tuples as a peer authority lane or compare raw ordinals across runtimes | Structural evidence green; causal production lowering deferred to Phase 3/11 |
| Relational targets carry complete immutable identity | `worth-relational/src/branch/target.rs` `RelationalBranchRootDescriptor` | relational target canonical-vector and root-axis contract tests | runtime, commit, version, ordered parents, truth root, and schema root all affect target identity and bytes | omit a root or let commits with different roots compare equal | Evidence green; independent review pending |
| Signal owner identity is injective | `worth-signal/src/branch/reference.rs` length-prefixed owner components | Signal collision/blank-component contract test | slash-containing graph and branch components remain distinct; owner-issued branch id is encoded; blank components are denied | concatenate components with an ambiguous delimiter, drop branch id, or admit blank ownership | Evidence green; independent review pending |
| Owner admission has concrete Proof carriers and sealed markers | owner branch authority modules and curated branch facades | `branch_reference_compile_time.rs` trybuild suites, generic readmission denial, and owner mint unit tests | concrete `AuthorityWitness<...Marker>` doors reject forged and generic witnesses; currentness/readmission is not claimed from a synthetic descriptor | forge a marker, use a generic `Auth: AuthorityMarker` door, or cross owner authority types | Type-level evidence green; runtime-issued admission deferred to owner phases |
| No parallel Signal branch authority lane remains | Signal branch basis module and runtime reexports | source/topology audit plus branch compile-fail suite | one concrete Signal basis authority/proof path is present; obsolete generic readmission marker is absent | retain a second owner-local `AuthorityMarker` witness or reexport it through runtime layers | Evidence green; independent review pending |
| Public topology is singular and documented | `branches/mod.rs -> transitions/mod.rs -> facade.rs`; branch-reference docs | full Foundational test/UI suite; `docs/branching-merging-and-commit-vocabulary/branch-references.md` | one curated export path; no legacy ambiguous names remain | retain a parallel `vocabulary.rs` export or stale constructor docs | Evidence green; independent review pending |

## Phase 1 command evidence

- `cargo check -p worth-foundational` — green.
- `cargo test -p worth-foundational` — 418 certification tests, 67 UI-boundary
  tests, and 3 doc tests green.
- `cargo clippy -p worth-foundational --all-targets -- -D warnings` — green.
- `cargo fmt --all -- --check` — green.
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .` —
  `Road 1 Cargo topology is valid`.
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check` — green.
- `scripts/ci/check_workspace_rust_line_caps.sh dirty` via the installed Git
  Bash — green; the only allowlisted over-cap file is the pre-existing facade
  aggregation.
- `cargo test -p worth-relational --test branch_reference_contract` — 4
  structural owner-lowering/root-axis tests with exact bytes and cross-runtime
  mismatch assertions green; this is not production-world evidence.
- `cargo test -p worth-signal --test branch_reference_contract` — 5 structural
  owner-lowering/transport tests with exact bytes, cross-graph mismatch,
  injective owner identity, and blank-component rejection assertions green;
  this is not production-world evidence.
- `cargo test -p worth-relational --test branch_reference_compile_time` and
  `cargo test -p worth-signal --test branch_reference_compile_time` — forged,
  generic-authority, and (Signal) generic-readmission UI cases green; no
  test-only Relational authority mint is counted as admission evidence.
- `cargo test -p worth-signal branch_basis --lib` — existing Signal basis
  authority/readmission tests green after authority reuse.

The reviewer verdicts and any corrective evidence are appended to this ledger
before Phase 1 is marked closed.

## Fresh independent Luna review pass

- QA loop (`phase1_qa_loop_luna4`) — **clear**. The reviewer found no
  remaining blocker. It independently checked the seven Foundational reference
  proofs, four Relational adapter proofs, five Signal adapter proofs, concrete
  Signal proof/readmission doors, and the shared facade topology. It confirmed
  that production-issued Relational currentness, causal Supply Chain lowering,
  and Signal live-owner cutover remain correctly deferred.
- QA tests (`phase1_qa_tests_luna4`) — **clear**. The reviewer confirmed the
  focused counts (Foundational 7, Relational 4, Signal adapter 5, Signal basis
  4, and eight trybuild cases), independent canonical vectors, malformed and
  blank-input cases, owner-affinity/collision cases, and absence of stale
  goldens. Its note about a test-only Relational authority mint was corrected
  by removing that mint and its smoke test; no test-only issuance lane is now
  counted as evidence.
- Code quality (`phase1_code_quality_luna4`) — **clear**. The reviewer found
  no topology, tier-direction, facade, authority, line-cap, or decomposition
  blocker. It confirmed that the only over-cap Rust file is the pre-existing
  allowlisted Foundational facade and that Signal warnings are unrelated
  repository debt.

All three Luna reviews are clear. Phase 1 remains open only for the required
fresh Sol review; no Phase 2 work may start before that review approves this
ledger.

## Final independent Sol review

- Final Sol (`phase1_final_sol`) — **approved** Phase 1 with no required
  correction. It independently confirmed the candidate/exact-reference split,
  explicit Empty/Basis and checked generation semantics, full mismatch axes,
  deterministic transport validation, singular facade topology, complete
  Relational root identity and runtime affinity, injective Signal owner
  identity, concrete `worth-proof` authority/proof carriers, and the absence
  of the obsolete Signal readmission authority lane.
- Sol also reran or verified the focused suites, full Foundational tests and
  certification, strict Foundational clippy, boundary/context checks, dirty
  line caps, formatting, and clean diff checks. It confirmed that all product
  world prose uses **Supply Chain**, and that production Supply Chain lowering
  and Signal live-owner currentness remain later-phase claims.

Phase 1 is closed. Phase 2 may begin with a separately critiqued plan.

## Phase 2 implementation plan (revised after two independent critics)

Phase 2 is a pure semantic deliverable. It must not compile a Relational
runtime, call a public schema/transaction/fork/read facade, mint or imitate an
owner-issued handle, or import Relational runtime, schema, transaction, MVCC,
query, index, history, replay, digest, or visibility behavior. Production
facade/compiler work is Phase 3; Relational MVCC, retention, publication, and
replay/history certification are later phases. The one intentional integration target is
`crates/worth-relational/tests/relational_certification.rs`; its descendants
are semantic certification code and remain one compile/link unit. No
`helpers`, `common`, `support`, or universal fixture module is introduced, and
every Rust file remains under the 400-line cap.

### Destination topology

- `tests/relational_certification.rs` — declares the one certification target
  and semantic test modules; it contains no production-facade imports.
- `tests/relational_certification/world/supply_chain/definition.rs` — an
  immutable, validated `SupplyChainWorldDefinition` with seed, schema version,
  typed records, typed relations, and declared invariants.
- `.../schema.rs` — the eight entity contracts (`Port`, `Terminal`, `Berth`,
  `Vessel`, `Voyage`, `PortCall`, `CargoLot`, `Inspection`), ten relation
  contracts, endpoint kinds, cardinality/uniqueness/symmetry rules, route
  ordering and acyclicity, cross-partition legality, and hazardous-cargo
  schema/meaning versions.
- `.../scale.rs` — `SupplyChainScale::{Court,Standard,Scale}` with exact
  deterministic counts, seed coordinates, cost-lane labels, and named-anchor
  preservation; profile construction is not arbitrary row multiplication.
- `.../semantic_key.rs` — distinct typed identity for each entity kind,
  relation, field, branch label, absence/deletion marker, and canonical path.
  Relation identity is independent of endpoint pairs so port-call rewiring
  preserves call identity.
- `.../scenarios/{empty,operating,contested,retention_pressure,version_boundary}.rs`
  — immutable baseline declarations derived as a dependency graph
  `Empty -> Operating -> Contested -> RetentionPressure` and
  `Operating -> VersionBoundary`; Version Boundary is deliberately not a
  child of Contested Planning. `Empty` contains only profile/schema
  declarations and no records; `Operating` contains the complete accepted
  topology and baseline oracle state; `Contested` contains validated Storm,
  Maintenance, Customs, and Rewire branch-creation intents with no branch delta
  or reference applied; `RetentionPressure` contains semantic obligation
  descriptors at named ancestors for snapshot, observation, transaction,
  candidate, and external-basis retention without runtime pins, leases, or
  authority; and `VersionBoundary` contains Operating plus the pre-upgrade
  hazardous-cargo schema descriptor. These are never five rebuilt mutable
  fixtures and never runtime snapshots, commits, leases, or authority.
- `.../delta.rs` — the eight named typed deltas: Storm Reroute Aurora,
  Maintain Atlas Berth, Hold Medical Cargo, Expand Southpoint Capacity,
  Competing Aurora Arrival, Retire Atlas While Inspecting Aurora, Rewire Aurora
  Port Call, and Adopt Hazard Classification V2. Each variant carries typed
  preconditions, postconditions, semantic write footprint, invariant posture,
  identity basis, and schema meaning; no generic string/op bag or merge
  classifier is allowed.
- `.../oracle/state.rs` — immutable ordered semantic maps, sets, ordered call
  lists, absence markers, and schema identity. It owns semantic facts only;
  `oracle/ancestry.rs` owns lineage and accepted-delta history, and a typed
  branch value composes the two. Failed application leaves the input state
  unchanged.
- `.../oracle/application.rs` — pure delta interpretation and validation only;
  no `head`, commit/version/snapshot, owner target, production digest, query,
  classifier, or runtime selection.
- `.../oracle/ancestry.rs` — semantic branch labels, parent observations, and
  ordered accepted-delta ancestry; wrong ancestry is checked independently of
  equal domain state.
- `.../expected_observation.rs` and `.../expected_digest.rs` — separate
  `ExpectedSupplyChainObservation` and versioned, independently implemented
  canonical bytes/digest. Digest is a report, never authority. Hand-authored
  vectors cover the Court baseline and every delta class, including explicit
  absence, unchanged facts, ordered calls, relation sets, schema identity,
  ancestry, and duplicate detection; permutation tests prove map ordering is
  semantic rather than iterator-dependent.
- `.../comparison.rs` — a named comparator and concrete mismatch matrix for
  missing entity/relation write, sibling fact leak, floating/latest-head
  selection, wrong ancestry, duplicate relation, illegal endpoint, schema
  meaning drift, and explicit absence/unchanged-fact differences. Expected
  observations, mutated observed vectors, and comparison failures are distinct
  types; digest equality is never the only comparison.
- `.../trace.rs` — versioned deterministic semantic trace with profile, seed,
  baseline, branch/fork labels, ordered delta IDs, typed `MutationId` and
  mutation operation, canonical mutated observation input, and first typed
  divergence. Replay is independent of map iteration and reproduces the first
  failure exactly. Pause schedules and production failure traces are deferred
  to Phase 3; MVCC/reclamation/publication replay belongs to later phases.
- `.../schema_contracts.rs`, `.../baseline_layers.rs`,
  `.../profile_contracts.rs`, `.../delta_contracts.rs`,
  `.../oracle_application.rs`, `.../oracle_ancestry.rs`,
  `.../observation_contracts.rs`, `.../comparison_contracts.rs`, and
  `.../trace_replay.rs` — responsibility-specific evidence within the single
  integration target; each file has an explicit sub-400-line budget and no
  implementation-surveillance getter tests.
- `crates/worth-relational/TESTING_WORLDS.md` — documents the Supply Chain
  semantic contract, profile/baseline/delta portfolio, authority split, trace
  format, cost lanes, mutation matrix, and explicit Phase 2/Phase 3 boundary.

### Required typed contracts and evidence

Definition, profile, schema, baseline, delta, oracle, expected-observation,
comparison, and trace-replay failures are distinct typed families retaining
semantic path, rule, step, branch, or axis context. Required negative controls
assert the specific class for invalid endpoint, duplicate relation, illegal
schema transition, missing write, sibling fact leak, floating/latest-head
selection, and wrong ancestry. The wrong-ancestry control keeps domain state
identical and changes only lineage. No boolean, empty result, panic, string
mismatch, or one generic `Err` can close a row.

Focused proof must establish the exact Court/Standard/Scale counts and stable
anchors; all five baseline derivations (including a test that Version Boundary
does not depend on Contested Planning); every delta's footprint, precondition,
postcondition, unchanged-region and invariant behavior; field-level units,
precision, statuses, capacities, minute/timestamp domains, hazard meaning, and
invariant verification posture; route/cardinality/symmetry/endpoint/schema
rules; branch isolation and ancestry; hand-authored canonical vectors; typed
failure classification; permutation invariance; and profile/seed/delta trace
replay. Mutation-sensitive positive/negative twins
must turn red for missing writes, sibling leaks, floating branch selection,
wrong ancestry, duplicate relations, and illegal endpoints. Oracle setup and
oracle execution costs are reported separately by cost lane.

A scoped source/dependency audit must mechanically prove that the entire Phase
2 pure subtree — definition, schema, scale, semantic keys, all scenarios,
delta, oracle, expected observation/digest, comparison, trace, and their tests
- has no production imports or calls. The fence has an explicit future
allowlist for Phase 3 adapter files only; it rejects production DTOs, facade
calls, canonicalizers, production digest implementations or reuse, query
projections, and shared lowering/comparison helpers. The independent pure
`expected_digest.rs` implementation is explicitly allowed and must not import
a production canonicalizer. Sharing the declarative delta type is permitted;
sharing lowering, normalization, comparison, branch selection, ancestry
resolution, or canonicalization logic is not. `SupplyChainSemanticHandles`,
`CompiledSupplyChainProgram`,
`CertifiedSupplyChainBaseline`, `observation.rs`, `compiler.rs`, and
`runtime_driver.rs` are Phase 3 production-install artifacts and must not be
introduced as Phase 2 authority aliases.

### Phase 2 exit gate

Named cost budgets are part of the exit evidence and are recorded in
`TESTING_WORLDS.md`: Court permits one immutable definition, at most eight
accepted semantic delta steps and 128 trace steps per test, with at most 512
semantic branch observations; Standard permits one immutable definition, at
most 128 accepted delta steps and 1,024 trace steps, and records profile
construction separately from oracle execution; Scale is a scheduled lane with
the exact declared density profile, a 65,536-cargo ceiling, and separate setup
versus oracle allocation/time reports. These are semantic/cost limits rather
than generous timeout assertions and are checked on named profiles, not
inferred from test count.

Phase 2 closes only when the two plan critics approve this revised plan, the
semantic target compiles as one integration target, every required pure-world
test and mutation twin is green, the whole-subtree source fence is green,
deterministic trace replay reproduces first divergence, the documentation
names the same authority/topology, and the closure ledger records that no
causal production baseline or owner-issued handle has yet been claimed. The
required verification commands are `cargo fmt --all -- --check`,
`cargo clippy -p worth-relational --all-targets -- -D warnings`, the focused
`cargo test -p worth-relational --test relational_certification`, the scoped
source-fence audit, `cargo run --manifest-path tools/boundary-check/Cargo.toml
-- --root .`, `cargo run --manifest-path tools/agent-context/Cargo.toml --
check`, and `scripts/ci/check_workspace_rust_line_caps.sh dirty`. Public-facade
causal compilation and owner-issued-handle evidence are explicitly deferred
to Phase 3; Relational MVCC, retention, publication, and broad replay/history
evidence are later-phase gates.

### Phase 2 plan-critic approval

- World/topology critic (`phase2_plan_critic_world`) — **unconditional
  approval** after the final amendments. It confirmed the baseline contents and
  dependency graph, field-level contracts, singular state/ancestry ownership,
  whole-subtree source fence, responsibility-specific modules, concrete cost
  budgets, and Phase 3/later-phase boundaries.
- Oracle/evidence critic (`phase2_plan_critic_oracle`) — **unconditional
  approval** after the final amendments. It confirmed typed mismatch and
  mutation-aware trace contracts, canonical vectors, independent digest
  exception, strict Clippy evidence, and no remaining oracle-independence or
  phase-leakage blocker.

The Phase 2 plan is approved. Implementation may proceed within the pure
semantic subtree; no production causal baseline is claimed until Phase 3.

## Phase 2 implementation evidence ledger

Phase 2 correction implementation is complete and ready for the independent QA gates. The
deliverable is intentionally a test-owned, production-independent semantic
world under `crates/worth-relational/tests/relational_certification/world`.
It does not import the Relational production crate, construct a runtime
facade, mint an owner-issued handle, or claim MVCC/publication behavior.

| Requirement | Evidence | Status |
| --- | --- | --- |
| Typed Supply Chain schema | `schema.rs` plus `schema_validation.rs` define versioned entity/relation vocabulary, the customer-neutral cargo code, endpoint/cardinality checks, duplicate detection, minimum underflow, symmetric reverse edges, ordered calls, and multi-node route-cycle rejection. | Green |
| Named scale profiles | `scale.rs` and `profile_contracts.rs` prove Court 244/247, Standard 4,848/3,363, and Scale 68,544/38,019 density, seeded regions/anchors, deterministic reconstruction, and all seven typed budget dimensions. | Green |
| Honest baselines | `scenarios/` defines Empty, Operating, Contested, RetentionPressure, and VersionBoundary. Contested intent sets are validated and executable through `OracleBranch::fork`; VersionBoundary branches directly from Operating. | Green |
| Required delta vocabulary | `delta.rs` defines all eight typed semantic deltas, separate read/write footprints, relation-source preservation, identity basis, branch/schema/reapplication preconditions, complete postconditions, invariant posture, and hazard-schema transition. | Green |
| Independent oracle | `oracle/state.rs`, `oracle/application.rs`, and `oracle/ancestry.rs` own state, mutation application, and ancestry separately from observations and digests. Every successful application validates the complete post-state; failed applications leave the input branch unchanged. | Green |
| Expected/observed semantic comparison | `expected_observation.rs` and `comparison.rs` carry entity/relation absence, full ancestry, duplicate-preserving relation vectors, vector/map parity, complete source/target edges, and typed absence, sibling-leak, floating-branch, endpoint, schema, vector, and unexpected-fact mismatches. The pure observed carrier is structural; causal production observation is deferred to Phase 3. | Green |
| Canonical vectors/digest | Hand-authored vectors cover every named delta and the Court Operating baseline; `expected_digest.rs` has an independent tagged encoding and pinned Court digest. | Green |
| Mutation-aware trace replay | `trace.rs` seed-checks and reconstructs the named baseline, applies ordered deltas, executes each typed mutation, rejects a forged first divergence, reruns comparison, and returns actual canonical mutated input/first divergence; the fingerprint is supplemental. | Green |
| Whole-subtree purity | `scripts/ci/check_supply_chain_phase2_purity.sh` fences the complete Phase 2 subtree against production compiler/runtime/handle imports and Phase 3 residue. | Green |
| Documentation handoff | `crates/worth-relational/TESTING_WORLDS.md` names the same Supply Chain topology, authority split, profiles, budgets, oracle boundary, and deferred production gates. | Green |

Focused evidence is green: `cargo test -p worth-relational --test
relational_certification` reports **56 passed, 0 failed**; the Supply Chain
purity fence passes; `cargo fmt --all -- --check`, the boundary checker, the
agent-context check, `git diff --check`, and the dirty Rust line-cap guard
pass. The repository's strict Relational Clippy command still reports
pre-existing warnings in untouched Relational/Signal production code; no new
warning was introduced by the certification target, and this debt is recorded
without weakening the Phase 2 purity or test evidence.

The focused run now constructs and validates Court, Standard, and Scale worlds,
proves every entity/relation density and seeded region, emits deterministic
seven-dimension cost reports with independent over-budget denials, proves a
valid positive comparison before vector/map mutation twins, checks
permutation-invariant canonical bytes, proves exact delta effects and complete
post-state schema validation, validates contested intents and sibling isolation,
and exercises explicit entity/relation deletion markers. The Scale case remains
scheduled by profile budget, but its exact 65,536-cargo topology is constructed
and budget-checked in the target rather than asserted as constants.

The Phase 2 exit gate still requires three fresh independent Luna reviews
(QA-loop, QA-tests, and code-quality), followed by a fresh Sol certification.
Only those reviews may close this phase. Production-backed Supply Chain
lowering, owner-issued handles, branch-local MVCC, retention/publication, and
merge/history evidence remain later-phase gates, with MVCC explicitly the
principal Phase 4 concern.

### Phase 2 independent QA disposition

The first fresh QA pass found no composition/topology blocker, but both the
QA-loop and QA-tests reviewers independently **blocked** semantic closure. The
focused target and mechanical checks are green; the blockers are proof-quality
gaps, not build failures:

- schema contracts were declared but minimum cardinality, symmetry, ordered
  routes, and multi-node acyclicity were not enforced by definition validation;
- several delta footprints/postconditions did not describe every field or edge
  mutated by their oracle application;
- relation sources, entity/relation absence, and accepted-delta ancestry were
  not represented in the observed comparison state;
- trace evidence recorded a fingerprint rather than executing deterministic
  replay and mutation-specific first divergence;
- Standard/Scale profiles and named budgets were constants without construction
  or setup-versus-oracle cost evidence; and
- positive comparator, permutation-invariance, complete unchanged-region, and
  deletion/retention mutation proofs were missing or overstated.

The code-quality reviewer was **clear**: module topology, naming, purity fence,
line caps, and production-boundary placement are sound. Phase 2 therefore
remains open. The correction slice must close every semantic blocker above,
re-run the focused and mechanical evidence, and obtain a fresh QA-loop,
QA-tests, code-quality, and Sol review. No production compiler or Relational
MVCC work may begin until that closure.

### Phase 2 correction plan after independent QA

1. Move schema validation into a responsibility-specific module and enforce
   endpoint kinds, duplicate keys, maximum and minimum cardinality, symmetric
   reverse edges, ordered call sequences, and full directed-cycle detection;
   make `SupplyChainWorldDefinition::validate` invoke the complete validator.
   `SharesPilotageZone` stores both directed edges; `VoyageHasCall` follows
   port-call sequence; `CallPrecedes` links consecutive calls in one voyage;
   partial-vector validation remains separate from complete-world underflow.
2. Expand the semantic delta contract vocabulary and table-driven assertions so
   every delta's preconditions, changed fields/edges/schema, invariant posture,
   and unchanged-region claim agree with the independent oracle. The contract
   separates read/precondition facts from exact write footprints, requires V1
   schema on every V1 delta, rejects reapplication, adds Southpoint terminal and
   berth anchors, and fixes the maintenance-delay voyage effects.
3. Carry explicit entity/relation absence and full ancestry (branch, parent,
   accepted deltas) through expected and observed states. Compare complete
   relation edges, not only targets, and add a valid positive observation plus
   one-axis mutation twins.
4. Give `SemanticTrace` a deterministic executor that reconstructs the named
   baseline, applies its deltas, executes each typed mutation, and records the
   actual first comparison divergence. It verifies the profile seed, branch and
   delta sequence, stores the canonical mutated input produced by replay, and
   rejects a caller-supplied false mismatch. Keep the replay fingerprint as an
   additional canonical artifact, not as replay proof.
5. Make profile seeds drive generated identity/value naming, represent all
   declared region counts, construct Court/Standard/Scale worlds (Scale in its
   scheduled lane), and enforce typed setup/oracle budgets with a
   machine-readable cost report. The report uses deterministic record/edge and
   oracle-step counts, not wall-clock timing.
6. Add permutation-invariance, exact unchanged-region, relation/entity absence,
   retention/deletion, and mutation-specific negative tests. Revise any ledger
   row whose wording still claims production or independent observation before
   the fresh review gate.

The two correction-plan critics both found the initial outline **blocked** until
these details were explicit. Their required amendments are now incorporated:
the observed relation representation must preserve duplicate vectors in
addition to keyed maps; replay must be executable and seed-checked; profile
construction and cost reports must be real; and every mutation twin must have
an independently valid positive counterpart. This remains a pure-world phase;
production compilation, owner-issued handles, MVCC, retention/publication, and
merge evidence stay deferred.

### Phase 2 correction implementation evidence

The correction slice closes the fresh QA proof gaps without widening the phase
boundary:

- `CargoCode` is a typed semantic field on every `CargoLot`, with deterministic
  anchor and seeded values and independent canonical-digest encoding.
- `delta_contracts.rs` now computes independent field-level, entity-key,
  relation-edge, schema, absence, and ancestry diffs for all eight deltas and
  requires exact equality with the declared footprint/postconditions. Relation
  source preservation is explicit, and every non-written field is checked
  unchanged.
- `oracle/application.rs` validates every successful post-state through the
  complete schema/cardinality/route validator and returns a typed
  `InvalidPostState` denial. Negative applications snapshot the deliberately
  broken input, and V1/V2 precedence is covered for every V1 delta.
- `comparison.rs` compares the expected relation set, keyed observed map, and
  duplicate-preserving raw vector as one semantic relation set. Vector-only
  omissions, replacements, extras, duplicates, illegal endpoints, and valid
  insertion permutations each have focused evidence.
- `scale.rs` reports and enforces delta steps, trace steps, observations, cargo
  lots, setup entities, setup relations, and oracle steps. Court/Standard/Scale
  tests construct exact per-kind and total density, seeded region coverage,
  stable anchors, deterministic reconstruction, and one independent overage per
  dimension.
- Contested branch intents have typed legality errors and are executable through
  `OracleBranch::fork`; the sibling mutation court proves shared immutable
  ancestry, unchanged Operating state, branch-local writes, and independent
  accepted histories.
- `SemanticTrace::replay` rejects a forged recorded divergence and covers the
  non-empty baseline/profile matrix with byte-identical repeated replay. Empty
  installation rejects domain deltas instead of fabricating a successful trace.

The focused target now reports **56 passed, 0 failed**. The purity fence,
formatting, boundary/context checks, diff check, and dirty line-cap guard are
green. Strict Relational Clippy still reports pre-existing warnings in
untouched Relational/Signal production code; that repository debt remains
separate from this pure certification evidence. Fresh QA-loop, QA-tests,
code-quality, and Sol reviews are still required before Phase 2 closes.

### Phase 2 replay/schema/ancestry correction evidence

The second correction slice closes the structural blockers found by the fresh
QA reviewers while preserving the pure-world boundary:

- `SemanticTrace` keeps canonical pre-mutation input immutable, carries the
  duplicate-preserving raw relation vector in the mutated artifact, and makes
  the returned trace self-replayable. Delta failures retain typed step, delta,
  and source context instead of being stringified.
- `OracleAncestry` now carries recursive root-to-branch lineage, rejects reuse
  of any ancestor label, computes common ancestors, and records branch-owned
  accepted history events with an explicit
  ordered-history fixture for multi-delta ancestry without fabricating a
  successful branch-domain replay. Expected and observed comparison state and
  canonical bytes include lineage.
- Complete schema validation rejects duplicate voyage-call ownership, missing or
  orphan `CallPrecedes` links, sequence gaps, and V1/V2 hazard-meaning drift.
  A positive cross-region route proves that lawful partition crossings remain
  legal. The field vocabulary has distinct `PortCode`, `CallSign`, and
  `InspectionMinute` keys with an exhaustive record-field court.
- Delta read facts are a dedicated entity/relation/field/schema/branch/history
  footprint. Storm and Rewire explicitly read their current and target port
  endpoints; unexplained source reads are not retained. Every contract is
  checked against an independent hand-authored read table and mutation court;
  the history axis is distinct from branch-local accepted-delta lists.
- Cost reports are bound to both profile and deterministic seed, and trace
  steps/observation counts are measured from executable artifacts. Retention
  obligations use named ancestor paths rooted at Operating and have typed
  legality checks for every adjacent declared branch-intent pair; they remain
  descriptive rather than runtime leases.

Focused evidence now reports **56 passed, 0 failed**. The purity fence,
formatting, boundary/context checks, diff check, and dirty line-cap guard are
green. Strict Relational Clippy continues to expose only pre-existing errors in
untouched Signal/Relational production code; no certification-target warning is
counted as evidence. Fresh QA-loop, QA-tests, code-quality, and Sol reviews are
still required before Phase 2 closes. Production compilation, owner-issued
handles, Relational branch-local MVCC, retention/publication mechanics, and
merge/history behavior remain later-phase gates.

### Phase 2 topology correction evidence

The fresh topology review found and the correction slice removed two semantic
module SCCs and two collapsed declaration owners. `scenario_delta_vocabulary.rs`
now owns the shared delta/precondition vocabulary below both the delta contract
and read-footprint modules. `comparison_state.rs` owns the observed carrier and
typed mismatch families; named ancestry, entity, schema, and relation comparison
modules feed a thin comparison orchestrator. `definition_entities.rs` and
`definition_relations.rs` own entity and relation declaration generation while
`definition.rs` only coordinates construction and validation. Cost construction
now takes a named `SupplyChainCostInputs` record. The focused target remains
**56 passed, 0 failed**; the only strict-Clippy failures are the 36 pre-existing
untouched `worth-signal` diagnostics, and the scoped Signal basis diagnostic was
removed.

### Phase 2 final proof correction evidence

The latest proof-quality correction closes the remaining independent test-review
gaps without widening the pure-world boundary:

- retention obligations now validate both declared target branches and every
  rooted, adjacent ancestor path; the mutation court rejects target-only
  forgeries in addition to malformed paths;
- parent application requires an exact root-to-child lineage with a valid root,
  unique labels, and the expected terminal child. Suffix, duplicate-label, and
  wrong-final-label lineage twins are rejected before domain application, and
  malformed-parent root and duplicate-label twins prove those guards
  independently;
- ordered history fixtures carry explicit branch owners. Unavailable owners are
  denied, comparison detects an owner-only history mutation, and canonical
  digest bytes change when only the history owner changes;
- cost evidence asserts a successful comparison before recording one
  observation, so failed comparison values cannot inflate certification counts;
- semantic traces reject unsupported versions, record and replay the canonical
  pre-mutation relation-vector bytes, expose those bytes in the replay result,
  fingerprint them, and reject forged vector input; and
- the duplicate `CallPrecedes` route test asserts the exact offending relation
  key rather than accepting any orphan-route error.

The focused target now reports **59 passed, 0 failed**. Purity, formatting,
boundary/context, diff, and dirty line-cap checks are green. Strict Relational
Clippy remains blocked only by the known 36 diagnostics in untouched
`worth-signal` production code; no warning originates in the certification
target. Fresh QA-loop, QA-tests, code-quality, and Sol reviews remain required
before the Phase 2 status row can close.

### Phase 2 fresh Luna review pass

- QA-tests (`phase2_qa_tests_luna_postfix`) — **clear**. It independently
  confirmed the 59-test target, causally strong malformed-parent root and
  uniqueness twins, typed retention target/path denials, branch-owned history
  and digest sensitivity, successful-only observation accounting, trace
  version/raw-vector validation, exact route-key rejection, and all mechanical
  gates. It found no certification-target Clippy warning; the remaining 36
  diagnostics are untouched Signal debt.
- QA-loop (`phase2_qa_loop_luna_postfix`) — **clear**. It confirmed the latest
  spec, ledger, and `TESTING_WORLDS.md` agree on the pure boundary, four
  branch intents, retention semantics, and current evidence, with no material
  requirement or proof blocker.
- Code-quality (`phase2_qa_loop_luna_final`) — **clear**. It found an acyclic
  pure-world topology, singular Foundational vocabulary and owner adapters,
  no decomposition or line-cap blocker, and no phase leakage in the latest
  correction.

All three fresh Luna reviews are clear. Phase 2 remains open only for the
required independent Sol certification.

### Phase 2 final independent Sol review

Final Sol (`phase2_final_sol_postfix`) — **approved** Phase 2 with no required
correction. It independently verified the 59-test Supply Chain certification,
all six final proof corrections, the Foundational/Relational/Signal branch
reference suites, purity and boundary gates, the exact 36-diagnostic untouched
Signal Clippy debt, and the production boundary. Source and documentation
agree that Phase 2 is production-independent; compiler/owner admission is
Phase 3, while runtime MVCC/publication/retention mechanics are Phase 4 and
later. Sol recorded reviewed HEAD `28253488e991b2e22308324b2db15aac8998fdff` and
Phase 2 fingerprint
`a3f117f92c092d7981be927f1eb72e9debae7540f827802d98710333e80f116a`.

Phase 2 is closed. Phase 3 may begin with a separately critiqued plan.

## Phase 3 implementation plan (after two independent critics)

Phase 3 is the causal production-baseline gate. It is deliberately narrower
than the later branch-local MVCC court: it proves that the immutable Supply
Chain definition can be lowered through the real Relational schema and
transaction facades, that every semantic name is bound to an owner-produced
identity, and that a public read can be compared with the independent oracle.
It must not claim fork isolation, exact admitted branch currentness, structural
sharing, retention, publication, cancellation, or MVCC readiness. Those claims
remain Phase 4 and later release-court obligations.

### Independent plan review

- `phase3_plan_api_critic_luna` — **clear with required API work**. It verified
  that the current 59-test target is pure semantic/oracle evidence and found
  no production compiler, owner-issued semantic binding, public observation
  adapter, or certified-baseline artifact. It identified the missing
  owner-issued relation correspondence, the absence of a live branch/basis
  issuance path, the combined `CommitReference`, and the optional/ambient
  transaction branch fields as boundaries that must not be smuggled into the
  Phase 3 proof.
- `phase3_plan_boundary_luna` — **clear with the same hard boundary**. It
  confirmed the public schema-install, transaction, snapshot, and read-view
  doors; required a fresh runtime and a public-facade-only compiler; and
  rejected relation-ID reconstruction from `changed_records`, guessed branch
  IDs, fabricated roots, or copied oracle state as evidence.

Both critics agree that the relation binding API is a prerequisite rather than
an implementation detail. A Phase 3 compiler cannot be honest until the owner
can return a sealed `CreatedRelationRef`-style correspondence from a commit.
The current branch/reference adapters remain structural. Exact owner-issued
branch-basis observation is intentionally staged with the Phase 4/6 reference
and admission work; Phase 3 may carry a descriptive baseline branch envelope
and an owner-issued snapshot, but it must not call that envelope an admitted
operational basis.

### Causal topology and dependency order

1. **Freeze the boundary and purity fence.** Keep `definition`, `schema`,
   `scale`, `scenarios`, `delta`, `oracle`, `expected`, `comparison`, and
   `trace` production-independent. Change the Phase 2 purity script from a
   whole-tree ban to an explicit adapter allowlist containing only
   `program.rs`, `handles.rs`, `compiler.rs`, `baseline_audit.rs`, and
   `observation.rs`; reserve `runtime_driver.rs` for the later named-delta
   slice or make it an explicit typed unsupported boundary. The fence must
   continue to reject production imports and runtime/MVCC/query/digest/residue
   in the pure modules.
2. **Add the owner result correspondence.** Introduce the smallest public
   Relational result API that maps an owner-created relation intent to its
   owner-issued `RelationId`, analogous to `CommitResult::created_entity`.
   The correspondence must be sealed/minted by the commit owner, preserve
   semantic intent identity through bulk staging, reject duplicate or missing
   bindings, and expose no raw allocator slots. Matching by changed-record
   position, kind/endpoints, query results, or integer arithmetic is forbidden.
   Keep the new production files below the 400-line cap and add focused unit,
   integration, and compile-fail coverage for forgery and ambiguity.
3. **Define the immutable production program.** Add
   `world/supply_chain/program.rs` with a canonical, reusable sequence of
   schema-install and baseline-create operations derived from the pure
   `SupplyChainWorldDefinition`. It owns semantic-to-public lowering but no
   runtime, storage, branch-head, root, version, retention, authority, or raw
   ID construction. Court and Standard are the causal lanes; Scale remains a
   semantic/cost lane until a separate installation-cost decision is made.
4. **Bind only owner-issued handles.** Add `handles.rs` with distinct entity-
   kind and relation-key maps, owner/runtime/schema identity, and typed missing,
   duplicate, foreign-runtime, wrong-kind, and incomplete-binding failures.
   Entity names resolve only through `CommitResult::created_entity`; relation
   names resolve only through the new sealed relation correspondence. A raw
   `EntityId`, `RelationId`, `BranchId`, or guessed slot cannot satisfy a
   semantic handle. Branch metadata in this phase is descriptive; no admitted
   basis is fabricated.
5. **Compile through public facades.** Add `compiler.rs` to create a fresh
   `RelationalRuntime` namespace, install the canonical schema through
   `prepare_initial_schema_installation().install(...)`, lower entity and
   relation bulk intents with `EntityReference::Created` endpoints, commit
   through the public transaction facade, retain the owner commit result, and
   mint an owner-issued snapshot through the public visibility authority.
   Runtime construction, schema installation, transaction execution, and
   handle binding must be separately typed and separately observable.
6. **Observe and audit independently.** Add `observation.rs` to read only the
   public `SnapshotHandle`/`RelationalReadView` records, decode declared aspect
   values, and map IDs back through the owner-issued handle table. It must not
   call branch-head lookup, private roots, canonical digests, query planners,
   MVCC classifiers, or the oracle's expected-state constructor. Add
   `baseline_audit.rs` to compare the observed carrier with the pure expected
   baseline and construct `CertifiedSupplyChainBaseline` only after all prior
   stages succeed.
7. **Keep failure classes honest.** The public report must distinguish
   definition/declaration, schema-installation, transaction/runtime,
   entity-binding, relation-binding, observation, oracle, and comparison
   failures. A construction/audit failure is not an MVCC product failure, and
   no `String`, boolean, `Option`, or production-derived expected state may
   erase that distinction.

The adapter schema cone is physically one-way: `program_schema.rs` owns
registration assembly and `schema_vocabulary.rs` owns deterministic schema
declarations; neither imports the compiled-program error or production-world
carrier. `production_world.rs` owns only the runtime carrier, while
`baseline_audit.rs` owns oracle/observation/comparison orchestration.

### Required Phase 3 proof

Add the named cases to the existing integration target while retaining all 59
pure-world tests:

- `supply_chain_world_compiles_causally_through_public_facades` (fresh Court
  and Standard runtimes, real schema installation receipt, real commit, and
  public snapshot/read);
- `supply_chain_named_handles_are_owner_issued_and_complete` (every declared
  entity and relation has exactly one sealed owner binding; raw-ID and
  changed-record reconstruction twins fail);
- focused owner-correspondence proofs cover normal, relation-aspect, and bulk
  relation creation paths, each resolving the exact semantic client key and
  endpoint references from the sealed commit result;
- `supply_chain_baseline_matches_independent_oracle` (public observation and
  pure expected state agree on schema, lifecycle, fields, endpoints, route
  order, and absence);
- typed declaration, installation, transaction, binding, observation, oracle,
  and comparison failure matrices with one independent mutation per class;
- wrong kind, missing/duplicate binding, foreign runtime, missing snapshot,
  endpoint rewiring, pinned-snapshot/latest-head selection, and
  production-observation sabotage controls. Sibling crossover is explicitly
  N/A for Phase 3 because no admitted fork/basis surface exists yet; it is a
  Phase 4/6 MVCC gate, not a baseline compiler claim; and
- the existing Fintech (20 focused tests) and generic preservation suites.

Compile-fail/UI evidence must deny private runtime access, raw semantic-ID
reconstruction, caller-created relation bindings, forged owner authority,
generic `AuthorityMarker` doors, and production imports from the pure oracle.

### Phase 3 exit and deferred work

The exit row can close only after the causal compiler, owner correspondence,
public observation, independent comparison, typed failure matrix, purity and
compile-fail fences, preservation suites, fresh Luna QA-loop/QA-tests/
code-quality reviews, and final independent Sol review are green. The ledger
must record exact test counts and all mechanical gates. Phase 4 begins only
after this row closes and owns the immutable commit/reference split plus the
fork-only source basis; general admitted branch bases, detached branch-bound
transactions, visible-root/fork sharing, retention, publication, and the MVCC
courtroom remain later gates.

## Phase 3 implementation evidence

The causal implementation is complete and stays within the Phase 3 boundary:

- `cargo test -p worth-relational --test relational_certification --no-fail-fast`
  — 77 passed, including Court/Standard/empty public-facade compilation,
  public observation, independent comparison, and the typed production failure
  matrix.
- `cargo test -p worth-relational --lib same_commit_relation_endpoints --no-fail-fast`
  — 4 passed: normal, relation-aspect, and bulk owner correspondence plus
  same-commit endpoint denial.
- `cargo test -p worth-relational --lib transactions::data::outcomes::created_relation_bindings --no-fail-fast`
  — 1 passed: exact owner correspondence is stable and intent-specific.
- `cargo test -p worth-relational --lib tests::domains --no-fail-fast`
  — 22 passed across the Fintech, CAD, and Chip preservation worlds.
- Relational branch-reference contract and compile-fail targets — 4 and 1
  passed; these remain structural owner-vocabulary evidence, not MVCC proof.
- `cargo fmt --all -- --check`, the Supply Chain purity fence, dirty Rust
  line-cap check, boundary checker, generated agent-context check, and
  `git diff --check` — green. The only over-cap Rust file remains the
  pre-existing allowlisted Foundational facade.
- Strict Relational Clippy was attempted; it remains blocked only by the
  unrelated pre-existing dependency warning set (36 Signal diagnostics).
  A `--no-deps` diagnostic also reports untouched Relational lint debt; no
  warning originates in the new Phase-4 branch/catalog files. These are
  recorded as repository debt and are outside this scoped change.

The final independent Luna review (`phase3_final_luna6`) approved the current
source after the required-commit, relation-aspect, causal missing-binding, and
durable-documentation corrections. The final independent Sol review
(`phase3_final_sol4`) approved the same current source with no required
correction. Phase 3 is closed. Phase 4 implementation has begun with the
immutable catalog, branch-cell, fork-only source, clone-affinity, and
metadata-cost vertical slice. The transaction/public-residue cutover and
later MVCC claims remain open.

## Phase 4 implementation plan (approved; implementation in progress)

Phase 4 is the currentness-authority cutover, not the detached-transaction or
copy-on-write implementation. It separates immutable commit facts from mutable
branch-reference facts, introduces branch-local truth version and checked
reference generation, and adds a concrete Proof-backed **fork-only** source
basis. General admitted read bases, boundary readmission, repeatable reads,
immutable visible roots, detached transactions, prepared candidates, atomic
publication, external retention, and reclamation remain their named later
phases.

### Authority and topology contract

- `branch/identity.rs` owns runtime-affine branch identity and Foundational
  lowering; raw public tuple ids are retired.
- `history/commit/{identity,parentage,artifact,catalog}.rs` own immutable commit
  identity, ordered parents/fork provenance, a sealed canonical artifact, and
  append-only lookup. The catalog has no branch-head mutation. Canonical
  artifacts are complete before insertion; `Arc::make_mut`,
  `publish_metadata_only_commit`, and post-catalog `append_index_generations`
  authority are removed or moved to a named non-authoritative sidecar.
- `branch/reference.rs` owns one reference cell containing the Foundational
  exact observation (branch identity, explicit Empty/Basis target, generation)
  plus owner-local truth version and the minimal head-retention obligation
  needed to keep a fork source available. The target/root descriptor is not a
  second currentness source: it is inert identity until Phase 5 installs the
  visible `RelationalBranchRoot`.
- `branch/fork.rs`, `lifecycle.rs`, and `coordination.rs` issue a fresh
  runtime-affine target reference from an exact source observation, start the
  target local truth version at zero, preserve one canonical source artifact,
  and record exact source observation as provenance. Authoring provenance is
  never target authority. `RelationalRuntime::fork` is an operational clone:
  it receives a fresh runtime id and freshly rebound branch cells; source
  observations deny as foreign in the clone.
- `RelationalForkSourceDescriptor` and
  `AdmittedRelationalForkSourceBasis` are the only Phase-4 basis artifacts.
  They are privately minted, non-serializable operational tokens consumed only
  by `fork_branch`. They cannot open reads, transactions, publication, or
  general retention. `AdmittedRelationalBranchBasis`,
  `RelationalBranchObservation`, readmission, and retention APIs remain
  phase-gated later ports.
- The legacy executor may retain its broad runtime borrow, but public branch
  authority changes now. `RelationalLegacyBranchBinding` is a private,
  runtime-affine, non-defaultable/non-serializable owner binding accepted only
  by a private adapter. `TransactionOptions` loses optional/ambient branch
  routing, `ExpectedBranchHead`, `Default`, and serde construction. Merge
  parent selectors and other branch-bearing inputs are private owner-resolved
  bindings or provenance-only data; raw public `BranchId` cannot resolve a
  current head.
- Public combined `CommitReference`, `BranchHead`/`VersionNode` authority,
  `latest_published_commit_ref`, and every broad publication door are removed.
  A catalog-latest diagnostic may report commit identity only and cannot feed
  currentness, visibility, validation, or admission. Visibility/snapshot and
  other later-root consumers may use a private immutable commit-selection
  projection during migration, never a read-root or currentness authority.

### Dependency-ordered slices

1. Freeze exported-surface and operational-path residue checks and compile-fail
   cases before source edits.
2. Introduce private identity/artifact/parentage/catalog types and sealed
   Proof markers; migrate commit construction and remove post-catalog mutation.
3. Install branch-reference cells, local-version/generation laws, minimal
   head retention, runtime-clone rebinding, and fork/lifecycle transitions.
4. Migrate immutable-history readers and legacy commit plumbing; replace public
   transaction branch inputs with the required owner binding without changing
   detached-transaction semantics.
5. Remove combined/head/public broad-authority residue, update checkpoints and
   preservation call sites, and keep visibility/root migration phase-gated.
6. Add causal Supply Chain fork proofs, mandatory metadata-cost probe, docs,
   and closure evidence.

### Phase 4 proof and test matrix

The existing `relational_certification` target gains responsibility-specific
fork tests using a fresh public-facade Supply Chain baseline. Required cases:

- immutable artifact and ordered-parent immutability;
- one canonical source artifact for Court/Standard forks, distinct reference
  identities/generation lines, and target local truth version zero;
- provenance non-authority and non-convertibility of the fork-only basis;
- checked truth-version/generation movement and overflow denial;
- typed stale-generation, foreign-runtime/equal-ordinal, duplicate-target,
  empty-source, malformed-target, and missing-basis denials with zero residue;
- runtime clone/rebind affinity and source-observation foreign denial;
- compile-fail proof that the fork-only basis cannot open reads, transactions,
  publication, general retention, or Phase-6 readmission; and
- semantic/exported-surface residue checks for retired combined references,
  optional/ambient routing, partial expected heads, latest-publication
  currentness, broad publication aliases, generic authority, and raw target
  constructors.

`phase4_reference_cost_probe` is mandatory at fan-outs 1, 64, and 512. Fixture
setup is reported separately; per-fork catalog lookup, artifact clone,
reference allocation, and branch-cell contact counters must remain constant
with no branch-population scan. This is metadata-only scaling evidence;
physical bytes, COW, visible-root atomicity, semantic sibling reads,
readmission, external retention, cancellation, and reclamation are not Phase-4
claims.

### Independent plan review

- `phase4_plan_authority_luna` — **approved** after phase-gating the fork-only
  basis, quarantining all branch-bearing transaction inputs, removing default/
  serde construction, and auditing metadata-only publication/index paths.
- `phase4_plan_mvcc_luna` — **approved** after the same corrections; it
  confirmed the MVCC boundary, runtime clone contract, mandatory 1/64/512
  counter probe, and test-honesty posture.

The first implementation slice is present, but Phase 4 is not yet closed.
The production and focused evidence for this slice, and its explicit MVCC
non-claims, are recorded below; the Phase-4 exit claim remains unmade.

## Phase 4 first-slice evidence and open review (historical snapshot)

The following section records the first-slice review state before the corrective
owner-currentness and recovery work. It is retained for audit history and is
superseded by the corrective closure record at the end of this ledger.

The status remains **Implementation in progress**. The first vertical slice
now has causal evidence, but it is deliberately not being counted as the
Phase-4 exit proof. The landed slice consists of:

- an append-only immutable commit catalog whose sealed artifacts retain
  commit identity, ordered parentage, descriptive truth/schema roots, and one
  shared canonical envelope;
- runtime-affine branch-reference cells with Foundational exact observations,
  checked local truth versions, checked observation generations, source-head
  retention, clone rebinding, and exact fork provenance;
- a Proof-backed, privately minted, non-Clone/non-Serialize fork-only basis
  consumed only by `fork_branch`;
- metadata-only publication/recovery that advances reference generation but
  does not advance branch truth or the legacy diagnostic head; and
- a private legacy transaction binding whose catalog lookup is rejected when
  branch identity, exact observation, or local truth version has drifted.

Focused evidence currently green:

- `cargo test -p worth-relational --test relational_certification
  --no-fail-fast` — **88 passed** (including the Supply Chain Phase-4 fork
  and owner-binding cases and the 1/64/512 reference-cost probe);
- `cargo test -p worth-relational --test branch_reference_contract
  --no-fail-fast` — **2 passed**; and
- `cargo test -p worth-relational --test branch_reference_compile_time
  --no-fail-fast` — **5 compile-fail cases passed**, covering forged concrete
  authority, generic authority substitution, raw target construction, and
  fork-basis Clone/Serialize attempts.

The reference-cost probe reports constant *logical* per-fork deltas for the
current path (two branch-cell lookups, one catalog lookup, one reference
allocation, three cell contacts, and zero deep-artifact clones). That is
metadata-work evidence only. It does not yet establish physical byte/COW
cost, prove absence of every population scan, or certify visible-root
atomicity, sibling reads, retention, cancellation, or reclamation.

### Blocking findings from independent QA reviews

Fresh QA-loop, QA-tests, and code-quality reviews agree that this slice cannot
close Phase 4 yet:

1. `TransactionOptions` still exposes optional raw `BranchId` routing,
   `ExpectedBranchHead`, `Default`, and serde construction. The required
   owner-issued runtime-affine binding is only a private compatibility path;
   validation and history still have legacy fallback routes.
2. Public/operational legacy maps and APIs remain (`branch_heads`, combined
   `CommitReference`/`BranchHead`/`VersionNode` projections, public
   `HistoryAuthority::create_branch`, raw target serialization/diagnostic
   constructors, ancestry/merge inspection over legacy heads, and diagnostic
   latest/head helpers). The descriptive `RelationalBranchObservation` type
   remains intentionally non-operational, but no public readmission door may
   be restored before its later phase. These legacy surfaces must be
   quarantined or removed before the currentness authority is singular and
   general readmission is opened in its later phase.
3. The required denial/lifecycle matrix is incomplete: generation/truth
   overflow, malformed/missing basis, retention failure, and zero-residue
   assertions still need causal cases. Compile-fail coverage must also deny
   fork-basis reads, transactions, publication, general retention, and later
   readmission doors.
4. The Supply Chain fixture still uses explicit `BranchId` and
   `TransactionOptions::default()` in the stale-advance case. It therefore
   proves the fork transition and its denials, but not the final owner-issued
   transaction admission contract.
5. Artifact/parentage assertions and cost instrumentation need an independent
   oracle and a setup-versus-operation report. The zero clone count is an
   intentional no-copy result, not proof of physical constant space.

The next implementation slice is the binding/currentness cutover: remove the
public optional/ambient transaction routes, migrate branch creation and
history readers to the cell/catalog authority, add the missing denial and
compile-fail matrix, and install semantic/exported-surface residue checks.
Only after that slice is independently reviewed can Phase 4 claim its MVCC
currentness gate. Merge, visible-root, readmission, retention, and reclamation
certification remain later milestone phases.

## Independent review corrections

The first QA-test review found that the owner mapping suites were synthetic,
that canonical vectors and cross-owner affinity assertions were incomplete,
and that the ledger overstated production admission. The suites now pin
hand-authored Empty/Basis and owner target bytes, compare foreign runtime/graph
observations across all affected identity axes, and label the mapping tests as
structural. Production-backed Supply Chain lowering is a Phase 3 gate and
Signal live-owner cutover is a Phase 11 gate; Phase 1 does not use these
fixtures as a causal baseline.

The follow-up QA-loop review found that Signal's public branch artifact erased
its owner witness through `NoProofs`, and that its trust-boundary artifact
exposed Proof's generic readmission method. The current artifact carries a
concrete Signal Proof carrier, the boundary wrapper exposes only a concrete
Signal witness door, and a compile-fail fixture proves a generic witness cannot
readmit it. The same review found Relational's optional commit/version fields
created a second Basis-shaped empty; the owner target now requires complete
committed ids, leaving Foundational `Empty` as the only empty target variant.

The second independent review pass found adapter-consistency and vocabulary
gaps. Relational targets now carry explicit truth/schema root descriptors and
reject a target from another runtime. Signal owner components use
length-prefixed identity encoding, include the owner-issued branch id, reject
foreign graph targets and blank components, and expose a typed conversion path
instead of panicking. The obsolete generic Signal branch readmission marker was
removed, public owner names now use `*BranchObservation` and
`Admitted*BranchBasis`, and both renamed trybuild goldens were regenerated.

Mutation probes, production-world compilation, and valid externally observed
owner admission remain release-court evidence for Phases 3, 6, 11, and 12.
Phase 1 records the sabotage controls and their intended red outcomes but does
not claim that those later mutation artifacts already exist.

## Fresh Phase 4 QA-loop review

The fresh independent QA-loop pass (`phase4_qa_loop_fresh_luna`) remains
**blocked/open** for Phase 4 closure. It reproduced two currentness-authority
leaks and corrected their root paths: foreign or stale owner-issued bindings
now fail at validation, execution admission, and immutable-history resolution
before publication; and mutation/strategy delete allowance now derives its
branch from the owner binding before any legacy raw target fallback. The pass
also corrected clone provenance rebinding so a cloned fork preserves the
owner branch name instead of nesting an already encoded Foundational id.

The focused owner-binding suite (6), branch-reference unit suite (5),
commit-strategy suite (55), causal clone-fork proof, full Relational
certification (88), `cargo check -p worth-relational --lib`, boundary-check,
agent-context validation, dirty Rust line-cap check, and `git diff --check`
are green. Signal warnings remain unrelated repository debt.

Phase 4 remains open because public raw/default transaction routing and
`ExpectedBranchHead`/merge-parent selectors remain; legacy branch-head maps,
combined history DTOs, latest/head APIs, and raw public branch creation remain
operational; recovery still synthesizes branch cells from legacy heads; and
the required denial/compile-fail matrix plus independent artifact/cost oracle
has not yet certified zero-residue currentness. No visible-root, sibling-read,
general readmission, merge, retention, COW, or full MVCC claim is made here.

## Final independent Phase 4 review record

- Fresh QA-loop (`phase4_qa_loop_fresh_luna`) — **blocked/open**. It confirmed
  the owner-binding currentness and clone-provenance corrections, then stopped
  the gate on the remaining public transaction/history lanes, legacy-head
  recovery synthesis, incomplete denial/UI coverage, and the absence of an
  independent physical-sharing oracle. It explicitly made no sibling-read,
  merge, retention, replay, COW, or complete-MVCC claim.
- Fresh QA-tests (`phase4_qa_tests_fresh_luna`) — **conditional pass**. It
  found and corrected a real cross-runtime owner-binding admission bug, and
  strengthened fork, stale-advance, unknown-branch, forged-target, and
  unrelated-branch proofs. It still requires the public residue and
  owner-issued transaction cutover before closure.
- Fresh code-quality (`phase4_code_quality_fresh_luna`) — **blocked; no
  edits**. It identified the mixed-level fork orchestrator, the combined
  branch-cell/legacy history authority, broad publication sequencing,
  transitional `TransactionOptions`, duplicated history resolution, and
  uncompiled branch scaffolds as composition/topology blockers. The two
  uncompiled `branch/coordination.rs` and `branch/lifecycle.rs` scaffolds were
  removed from the working tree; their later-phase responsibilities remain
  documented in the milestone topology and are not claimed by Phase 4.

Final enforcement after that cleanup is green: formatting, diff check,
boundary checker, generated agent-context validation, and dirty Rust line caps
(only the pre-existing allowlisted Foundational facade exceeds 400 lines).
The Phase 4 row remains open pending the owner-authority/public-residue slice
and a fresh independent Sol review.

## Final independent Sol review

- Final Sol (`phase4_final_sol_recheck`) — **BLOCKED**. The green 88/2/5
  focused suites and mechanical checks do not satisfy the Phase-4 exit
  contract. Public `TransactionOptions`/`ExpectedBranchHead`/raw merge and
  ambient-main lanes remain operational; `HistorySubsystem` and publication
  still maintain parallel branch-cell and legacy-head authorities; recovery
  reconstructs cells from legacy heads with zero root descriptors and reset
  local progression; and the five UI cases do not deny fork-basis capability
  misuse or retired raw/default entry points. Sol also confirmed that the
  current cost probe counts logical increments rather than independently
  proving artifact sharing, no population scan, or physical no-copy behavior.
- Required next slice: complete owner-binding-only transaction entry; remove or
  quarantine public/raw branch and combined-history authority; make recovery
  fail closed or restore exact branch-cell state; migrate history, merge,
  visibility, replay, and publication consumers; complete the denial/UI and
  zero-residue matrix plus independent artifact/cost oracle; then repeat the
  three fresh Luna reviews and final Sol review.

Phase 4 therefore remains **Implementation in progress**. No complete MVCC,
visible-root, sibling-read, merge, retention, COW, replay/recovery, or
reclamation certification is claimed.

## Phase 4 corrective closure record

This record supersedes the historical first-slice findings above. The current
tree has completed the owner-authority/currentness slice; the only remaining
milestone gate is an independent final Sol certification. It does not promote
the later visible-root, readmission, merge, retention, COW, replay, or
reclamation obligations into Phase 4.

### Corrected authority and recovery guarantees

- `TransactionOptions` requires one privately minted,
  runtime-affine `RelationalLegacyBranchBinding`; there is no public
  `Default`, serde, ambient-main, `ExpectedBranchHead`, or raw branch selector
  construction lane.
- The append-only catalog is the one immutable artifact source. Branch cells
  carry the exact Foundational observation, checked generation, and local truth
  version. Production history, validation, merge preparation, and execution
  resolve currentness through the owner binding and exact cell; raw branch
  ancestry helpers are test-only.
- Merge parent order is resolved from exact owner bindings. Stale secondary
  parents, foreign-runtime identities, empty local bases, and mismatched
  observations deny before mutation.
- Recovery restores an exact checkpoint into the recovering runtime before
  idempotence checks. Existing checkpoint artifacts fail closed on mismatch;
  only an explicitly checkpoint-free tail may be reconstructed. Metadata-only
  replay rebinds the restored checkpoint to the recovering runtime before
  generation comparison.
- Runtime cloning rebinds every branch cell to the new runtime identity, and a
  foreign-runtime branch identity cannot admit execution basis. Empty local
  branches use local zero truth rather than an unrelated main-branch fallback.
- `pin_snapshot` and owner-bound execution-basis admission on
  `VisibilityAuthority` are crate-private. The residue guard rejects retired
  branch authority names, public raw doors, branch-population scans outside the
  named boundary, duplicate artifact materialization, and Phase-4 certification
  imports of later compatibility authorities.

### Bounded compatibility inventory

The milestone specification now explicitly records the pre-existing public
historical-read, Bridge, application-commit, replay, and retention adapters
that existing Query consumers still use. They are compatibility-only: exact
runtime/owner validation is required, they cannot move a branch cell or enter
Phase-4 fork/transaction/publication admission, and the Supply Chain Phase-4
target does not import them. The application-commit proof additionally
compares branch-cell checkpoints before and after lease admission. Later
read-basis/Bridge/retention phases own their eventual removal or exact-basis
cutover; no new raw selector or lease constructor may be added under this
exception.

### Fresh evidence

- `cargo test -p worth-relational --lib` — **1,034 passed, 0 failed, 25
  ignored** (including the seeded CDC rewrite-storm and thousand-step resume
  cases; 334.70 seconds).
- `cargo test -p worth-relational --test relational_certification
  --no-fail-fast` — **91 passed, 0 failed**. The Supply Chain target includes
  the fork/currentness proofs, independent oracle mutations, and the 1/64/512
  setup-separated cost probe.
- Merge replay continuity — **4 passed**; foreign-runtime owner admission —
  **1 passed**; empty local basis — **1 passed**; branch-reference contract —
  **2 passed**; Bridge boundary — **1 passed**; later-phase compile-fail
  boundaries — **3 passed**; UI harness — **1 passed**.
- The independent QA-tests review is **APPROVE**. The fresh QA-loop review
  finds no implementation blocker after the lease-door quarantine and marks
  the stale ledger as the remaining closure-evidence issue. The independent
  plan review conditionally approves the core currentness/merge slice; its
  historical-read/replay concerns are resolved by the bounded compatibility
  inventory above and the Phase-4 certification residue guard.

### Mechanical gates

`cargo fmt --all -- --check`, Relational library Clippy with
`cargo clippy -p worth-relational --lib --no-deps -- -D warnings`, boundary
check, generated agent-context validation, the dirty Rust line-cap guard, the
residue checker, and `git diff --check` are green. The full workspace Clippy
command still reports the known 29-warning `worth-signal` production debt;
those warnings are outside the Relational slice and are not used as Phase-4
semantic evidence.

### Remaining gate and non-claims

The Phase-4 row remains pending the final independent Sol review. Until that
review records approval, this ledger claims implementation/evidence
completion, not formal phase closure. Phase 4 still makes no claim for a
visible immutable root, sibling-read isolation, exact external readmission,
merge selection/publication, retention/reclamation, physical COW proof, or
complete product MVCC. Those obligations remain in the later phases named by
the milestone specification.

## Final independent certifications (7cbeb3a86)

GPT-5.6 was unavailable in this harness. Each skill used a fresh, read-only,
worktree-isolated Grok 4.6 critic that did not inherit implementer
conclusions. Earlier critics withheld on `3d29c4fa9` and `d4338e317`; those
supported findings were fixed before this pass.

| Skill | Critic identity | Revision | Verdict |
| --- | --- | --- | --- |
| qa-loop | independent Grok qa-loop reviewer, model grok-4.6, read-only | `7cbeb3a8645809890143b28117b5e6fc87aeb3cc` | CERTIFY |
| qa-tests | independent Grok qa-tests reviewer, model grok-4.6, read-only | `7cbeb3a8645809890143b28117b5e6fc87aeb3cc` | CERTIFY |
| code-quality-qa | independent Grok code-quality-qa reviewer, model grok-4.6, read-only | `7cbeb3a8645809890143b28117b5e6fc87aeb3cc` | CERTIFY |

Phase 4 is closed for the currentness cutover only. This record does not claim
visible immutable roots, repeatable-read bases, detached transactions,
prepared candidates, compare-and-publish, physical copy-on-write, external
retention/reclamation, or complete product MVCC.

## Current dirty-tree corrective revalidation (Phase 4 review)

This section supersedes the historical evidence counts above for the current
working tree. The original Grok Phase-4 result was not accepted without an
independent pass. That pass found and corrected two functional gaps: replay of
an empty-intent merge could advance branch currentness twice, and the
retention-overflow proof bypassed the real fork transition. The corrected
replay path restores catalog/index sidecars with currentness disabled and lets
the single MVCC publication perform the one branch-cell advance. The new
causal fork denial drives `RelationalRuntime::fork_branch` and proves typed
retention overflow leaves the source cell, registry, catalog, durable envelope
set, and target absence unchanged. Contradictory source comments were also
corrected before clearance.

### Independent clearance

The fresh read-only Sol-high review returned **CLEAR** on the current dirty
tree. It verified Foundational exact comparison, concrete `worth-proof` fork
and legacy-binding doors, paired fork-provenance checkpoint validation,
publication identity preflight, lineage durable-append ordering, staged
recovery, MVCC/history publication topology, corrected merge replay, and the
causal denial matrix. The review explicitly preserves the narrow Phase-4
boundary and does not promote later-phase obligations into this milestone.

### Current evidence

- `cargo test -p worth-relational --lib --no-fail-fast` — **1,047 passed,
  0 failed, 25 ignored** (including the full CDC resume stress cases).
- `cargo test -p worth-relational --test relational_certification
  --no-fail-fast` — **95 passed, 0 failed**.
- Merge replay continuity — **5 passed**; branch denial matrix — **3
  passed**; branch-reference compile-fail harness — **20/20 cases passed**;
  Supply Chain retention proof — **1 passed**.
- Mechanical gates are green: formatting, Relational Clippy with `-D
  warnings`, boundary topology, generated agent context, dirty Rust line cap,
  Phase-4 residue guard, and `git diff --check`. The known `worth-signal`
  warnings remain unrelated workspace debt.

This current-tree review certifies the Phase-4 currentness cutover, canonical
artifact singularity, fork-only admission, owner-bound legacy transaction
routing, and exact currentness/recovery behavior. It does not certify visible
immutable roots, repeatable-read bases, detached transactions, prepared
candidates, compare-and-publish, physical copy-on-write, external
retention/reclamation, broad merge publication, crash atomicity, or complete
product MVCC.

## Phase 5 implementation and evidence record (current dirty tree)

Phase 5 is the immutable-root and structural-sharing slice. The implementation
uses owner-issued branch identities and exact root-selected `PartitionAccess`
through the Relational production facade. It does not promote the fork-only
Phase-4 basis into general readmission, retention, publication, or recovery
authority.

| Claim | Implementation surface | Proof artifact | Expected counter/result | Observed result | Mutation that must fail | Mutation result | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Fork retains one immutable root and one canonical ancestor envelope | `branch/root.rs`, `branch/fork.rs`, `runtime/state/subsystems/history_root_capture.rs` | `root/sharing/{fork,observation}.rs` | Court/Standard forks report one shared root, one canonical artifact, zero copied truth/envelopes, and distinct coordination cells | 1 shared root/artifact; copied truth/envelopes/entities/relations/bytes all 0; cells distinct | eager per-fork truth/envelope clone or one global coordination cell | eager-clone and global-cell mutants are rejected by independent byte/cell assertions | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Persistent COW replaces only the declared touched regions | `branch/root_capture.rs`, `branch/root_regions.rs`, `storage/overlay/partition.rs` | `root/copy_on_write/{branch,region_reuse,named_delta}.rs` | declared entity/relation footprint matches touched regions; untouched region locators and sibling roots remain exact | Court delta touched/reused counts and locator sets match the declared footprint; sibling/ancestor roots unchanged | rebuild the complete world or omit a touched partition | whole-world and omitted-partition mutants produce mismatched bytes/locators | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Persistent radix path retention is observable and no-residue | `branch/root_regions.rs`, owner allocation inspection | `root/accounting/persistent_path.rs` | pre/post locator sets show one touched region and 33 new/retired path nodes, exact untouched-node reuse, and no-op identity preservation | 1 touched region; 33 new and 33 retired path nodes; all other locators intersect exactly; no-op set identical | drop one retained ancestor path node or rebuild all nodes | dropped-ancestor, wrong-path-count, and no-op-rebuild mutants fail locator assertions | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Schema axis is canonical owner schema authority, including relation admission/delete policies | `schema/data/authority_snapshot.rs`, `schema/data/mod.rs`, `branch/root.rs`, `branch/root_readmission.rs` | root schema mutation/readmission unit proofs, V2 length-boundary/policy digest proofs, and content-binding contracts | root schema digest equals the versioned, length-prefixed `schema_authority_snapshot_digest_bytes` over all entity/relation fields, including `cross_context_policy` and `cascade_delete_policy`, and is recomputed on completeness/readmission | 4/4 schema digest/root mutation proofs pass; variable-boundary and both policy mutations produce distinct digests; corrupted digest denies | mutate a schema name/registry boundary or either relation policy while retaining the old digest | boundary, cross-context, cascade-policy, schema mutation/readmission, and content-binding mutants deny before admission | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Visibility is a consumed typed commitment, not a label | `branch/root_visibility.rs`, root completeness/readmission, sharing inspection facade | root visibility tuple-mutation/corruption proofs and `root/inspection/boundaries.rs` | commitment binds storage, schema, correctness posture, canonical identity, branch context, and patch position; incomplete roots deny inspection | 3/3 root commitment tests pass; sharing exposes nonzero commitment and rejects incomplete root | mutate any commitment axis or report an incomplete root | tuple mutation, corrupted commitment, and incomplete-root mutants are rejected | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Branch-qualified reads and traversal use the selected root | snapshot/query root selection, `query_traversal.rs`, `storage/partition/adjacency_queries.rs` | `root/selection/{branch_isolation,traversal}.rs`, independent Supply Chain oracle | child retains ancestor entity/relation and traversal edges after main rewiring; sibling-only edges are absent | adversarial traversal 1/0; focused planned-query/traversal 7/0; full certification 130/0/1 | route traversal through runtime-global adjacency | old-global adjacency sabotage fails retained-child assertion | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Owner allocation accounting is complete and cache lanes stay excluded | `inspection/mvcc/{sharing,cost}.rs`, owner allocation ledger, derived-index accounting | `root/accounting/{authoritative,derived_cache}.rs`, `root/inspection/boundaries.rs` | independently summed authoritative locators/bytes match production; optional cache growth is recursive and excluded from authority | full accounting, recursive cache, and inspection evidence green; cache growth matches owner ledger while authoritative bytes remain unchanged | omit root/reachability/envelope allocation or promote diagnostics/cache bytes | omitted-allocation and cache-promotion mutants fail independent sums | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Inspection is read-only and cannot mint authority | `facade/inspection.rs`, private observation fields/accessors | `root/inspection/boundaries.rs`, `tests/ui/branch_reference/inspection_artifacts_cannot_open_authority.rs` | foreign/rootless/duplicate identities deny; all 21 branch-reference UI cases remain green | foreign/rootless/duplicate denials pass; UI authority suite 21/21 green | use sharing/allocation/cost/visibility observations as fork, transaction, publication, or retention authority | all four forged-use call sites fail type checking with typed argument mismatches | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |
| Commit-boundary and graph invariants consume the selected branch root and version | `validation/invariant_access/execution.rs`, `validation/engine/{context.rs,request}`, invariant packet planning/worker, `custom_rule/scope_planner.rs`, `invariant_access/metadata.rs` | `root/selection/branch_invariant.rs`, `invariants/admission/standard_graph_composition.rs`, `invariants/admission/graph_selected_state_probe.rs`; custom scope-planner owner proof; relation-integrity replay regression | after main advances, child planning and evaluation retain child state/version; stale branch bases deny before execution | Supply Chain child commit succeeds; the Court graph probe violates on main-only state and passes on the pre-divergence child; custom planner preserves selected version; focused and full certification green | route invariant observation, planner, context, or metadata through runtime-global currentness or retain an ambient test seam | selected-state, custom-version, stale-basis, and seam searches fail the mutant | Current evidence green on packet `20452f29...`; Sol-high trio CLEAN; final Sol-high gate CLEAN |

### Phase 5 command evidence

- `cargo test -p worth-relational --test relational_certification
  --no-fail-fast --quiet` — final corrective run reported **130 passed, 0
  failed, 1 ignored** in **50.93 seconds**; the ignored case is the explicitly
  scheduled true-Scale profile.
- Scheduled true-Scale profile — **1 passed, 0 failed** in **505.42 seconds**
  via the live `scale_invariant_admission::large_runtime...` route.
- Focused root, content-binding, persistent-path, inspection, density, and
  traversal proofs are green; the final adversarial branch traversal test is
  **1 passed**, the native and custom branch-root invariant selection proofs
  are **2 passed**, and the focused planned-query/traversal lane is **7
  passed**.
- `cargo test -p worth-relational --lib
  custom_scope_planner_preserves_owner_selected_current_version -- --nocapture`
  — **1 passed**; the custom planning lane preserves the selected branch
  current version independently of the runtime-global version.
- `cargo test -p worth-relational --test branch_reference_compile_time
  --no-fail-fast` — **21/21 UI cases passed**, including inspection artifacts
  passed to fork, transaction, bridge publication, and retention doors.
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .` —
  `Road 1 Cargo topology is valid`.
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check` — green.
- `cargo fmt --all -- --check` and `git diff --check` — green.
- `cargo clippy -p worth-relational --lib --no-deps -- -D warnings` — exit 0
  after removing the redundant closure in the owner allocation accounting
  path; the command still prints unrelated pre-existing `worth-signal`
  warnings because that dependency is compiled in the same lane.
- `cargo test -p worth-relational --lib durability --no-fail-fast` — **60
  passed, 14 failed, 1 ignored**. The failures are the inherited historical
  snapshot-root, retention/replay-pin, visibility-cache, and schema-transition
  recovery family. They are explicitly outside the Phase-5 claim rows and are
  not promoted as Phase-5 evidence; Phase 6–10 observation/readmission and
  lifecycle/recovery work owns their closure.
- Current dirty-source fingerprint for this evidence record: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700`; sorted dirty-Rust path SHA-256
  `20452f29a59457d1aa074a2c3f0ae7f82992a4503d892634fd8c8f8ce08d4422`;
  **284 dirty Rust files**, **42,219 nonblank lines**, maximum **400** lines
  (`tests/relational_certification/invariants/uniqueness/global.rs`), zero
  over-cap files. Reviewers must cite
  this fingerprint or a newer one rather than an unqualified inventory count.
- Dirty Rust line-cap guard — all 284 dirty Rust files are at or below 400
  lines; the repository Bash script passes through the installed Git Bash
  runtime.

### Phase 5 independent review history

- `phase5_final_luna_qaloop` — **BLOCKED, historical first pass; later source
  review clear but ledger freshness blocked**. Reviewer: fresh Codex agent,
  model `gpt-5.6-luna` (max), read-only. Source fingerprint: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700` plus the then-current dirty
  Phase-5 source. Scope/prompt: audit Phase-5 implementation, root-qualified
  traversal, COW, allocation/cost evidence, and closure-ledger completeness
  against the milestone requirements. Complete findings: source and cited
  evidence were clear; closure was withheld because the Phase-5 row was absent
  at first and then lacked per-row observed/mutation outcomes and reviewer
  metadata. Those ledger defects are retained as history and corrected in the
  current table/record; a fresh recheck is required.
- `phase5_final_luna_qatests` — **PASS** for the declared Phase-5 scope.
  Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max), read-only. Source
  fingerprint: `HEAD 08a79b079499602a374a2c09986bbd50e62f1700` plus the dirty
  Phase-5 source. Scope/prompt: apply `qa-tests` to the Supply Chain public
  world, independent oracle, root-qualified reads/traversal, COW/accounting,
  and authority-denial proofs, looking for tests that pass for the wrong
  reason. Complete findings/disposition: no supported test-world, oracle,
  boundary, adversarial, or cost defect; the reviewer explicitly did not
  promote true Scale, fresh-process/PostgreSQL, retention, merge, or Phase-9
  claims.
- `phase5_final_luna_codequality` — **PASS** for the declared Phase-5 scope.
  Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max), read-only. Source
  fingerprint: `HEAD 08a79b079499602a374a2c09986bbd50e62f1700` plus the dirty
  Phase-5 source before this ledger-only metadata repair. Scope/prompt: apply
  `code-quality-qa` to the complete dirty set, branch/root/schema/visibility/
  traversal/facade topology, dependency direction, and the 400-line cap.
  Complete findings/disposition: 187 dirty paths (181 Rust), maximum 399
  lines, and 89 advisory candidates were inspected; no structural blocker,
  dishonest facade, misplaced authority, or boundary violation was supported.
  Boundary/context/format/diff checks were green.
- `phase5_ledger_luna_qaloop_recheck` — **BLOCKED, historical metadata-only
  pass**. Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max), read-only,
  source fingerprint `HEAD 08a79b079499602a374a2c09986bbd50e62f1700`. Scope/
  prompt: verify the Phase-5 closure table against the governing requirement
  for expected, observed, and mutation results and verify independent-review
  identity/scope metadata. Complete findings: source/traversal/root/COW/
  accounting evidence was clear, but the table and review history were missing
  the required observed/mutation columns and reviewer metadata. The current
  record adds those columns and this expanded history; the finding is closed
  by re-review below.
- `phase5_ledger_luna_quick` — **BLOCKED, historical pre-repair pass**.
  Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max), read-only. Source
  fingerprint: `HEAD 08a79b079499602a374a2c09986bbd50e62f1700` before this
  metadata repair. Scope/prompt: inspect each Phase-5 row and cited proof
  artifact, then check review-history identity/model/revision/scope/prompt/
  findings. Complete findings: all nine rows and cited source/tests supported
  their claims, but closure metadata was incomplete. This is the final
  pre-repair finding; a new reviewer must audit the repaired record.
- `phase5_ledger_luna_closure` — **BLOCKED, historical self-reference
  placeholder**. Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max),
  read-only. Source fingerprint: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700` plus the dirty Phase-5 source.
  Scope/prompt: concise QA-loop closure check of the repaired Phase-5 table,
  cited artifacts, and review-history metadata. Complete findings: all nine
  rows had the required eight columns and cited artifacts supported their
  claims; the only blocker was the still-present `phase5_ledger_luna_final`
  pending placeholder at the moment of review. No source or evidence defect
   was found. The placeholder is replaced by this historical record and a new
   fresh closure pass below.
- `phase5_ledger_luna_clean_pass` — **PASS**, fresh final ledger closure.
  Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max), read-only. Source
  fingerprint: `HEAD 08a79b079499602a374a2c09986bbd50e62f1700` plus the dirty
  Phase-5 source and repaired ledger. Scope/prompt: verify the nine current
  Phase-5 rows, cited artifacts, required expected/observed/mutation fields,
  command evidence, explicit nonclaims, and complete independent-review
  metadata; treat historical BLOCK entries as history and reject only current
  open or unsupported claims. Complete findings/disposition: 9/9 rows had
  all required fields and cited artifacts; sharing, COW/path accounting,
  schema/visibility bindings, selected-root traversal, accounting/cache
  exclusion, inspection boundaries, and cost claims were supported; evidence
  inventory matched 119/0/1 certification and 21/21 UI cases; no Phase-5
  evidence defect remained. The sole remaining gate was the final Sol review.
- `phase5_final_sol_certification` — **BLOCKED**, fresh Sol 5.6 high
  read-only audit. Source fingerprint: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700` plus the pre-correction dirty
  Phase-5 source. Scope/prompt: certify the exact Phase-5 root/COW/schema/
  visibility/traversal/accounting/cost claims, cited commands, nonclaims, and
  all Luna metadata without promoting Phases 6–9 or true Scale. Complete
  findings/disposition: the reviewer reproduced a variable-length schema
  digest preimage ambiguity and found that synchronous finalization retention
  enumerated all branch heads while the cost row hid that work in a flat
  invocation counter. Both findings were accepted; the source/test/ledger
  corrections are recorded in the corrective reopening below, and all affected
  rows are reopened for fresh closure critics.
- `phase5_finalcorrected_luna_qaloop` — **PASS**, fresh final-source
  qa-loop closure. Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max),
  read-only. Source fingerprint: current `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700` plus corrected dirty source and
  ledger. Scope/prompt: attack the complete Phase-5 requirement/evidence
  ledger, especially injective schema framing with both relation policies and
  the explicit retention-maintenance cost nonclaim. Complete findings:
  schema V2 framing and registry projection include all authoritative fields;
  completeness/readmission recompute the digest; policy/boundary tests pass;
  selected tuple and separate maintenance scan are honestly scoped; all other
  Phase-5 rows and nonclaims remain supported. Disposition: no current
  OPEN/DEFECT or unsupported claim; PASS, with only final Sol pending.
- `phase5_finalcorrected_luna_qatests` — **PASS**, fresh final-source
  qa-tests closure. Reviewer: fresh Codex agent, model `gpt-5.6-luna` (max),
  read-only. Source fingerprint: current `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700` plus corrected dirty source and
  tests. Scope/prompt: falsify the schema adversary, relation-policy mutation,
  root/readmission denial, Supply Chain public world/oracle, selected-root
  COW/traversal/accounting/authority, and maintenance-lane cost assertions.
  Complete findings/disposition: adversarial former-collision fixture is
  distinguished by V2 framing; both policy mutations change the digest and
  production snapshots carry them; root denial, independent oracle, selected
  traversal/COW/accounting, authority UI, and cost-lane evidence are causal
  and honest. True Scale, fresh process/PostgreSQL, retention/reclamation,
  merge, and later-phase claims remain excluded. PASS.
- `phase5_finalcorrected_luna_codequality` — **PASS**, fresh final-source
  code-quality closure. Reviewer: fresh Codex agent, model `gpt-5.6-luna`
  (max), read-only. Source fingerprint: current `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700` plus corrected dirty source.
  Scope/prompt: enumerate every dirty path/function against composition/domain
  laws, schema digest ownership, relation-policy placement, maintenance cost
  separation, branch/root/traversal topology, authority direction, future
  insertion, and line caps. Complete findings/disposition: 188 paths (182
  Rust), 89 advisory candidates, max 399 lines, no structural blocker;
  schema digest has one owner, cost maintenance is separate, facades remain
  aggregation-only, authority direction and Phase 6–9 insertion are sound.
  Boundary/context/format checks are green. PASS.

### Phase 5 explicit nonclaims

The 65,536-cargo Scale profile remains ignored only in the ordinary lane and
is instead proved by its separately scheduled `1/0` resource run. Phase 5 does
not claim an independently measured Scale fork slope or process-memory bound,
fresh-process/PostgreSQL durability, retention/reclamation, detached
transactions/readmission, compare-and-publish concurrency, merge behavior,
Phase-9 facade topology, or broad legacy version-only adjacency cutover. Those
remain later-phase gates and are not silently promoted by the bounded
Court/Standard evidence or the scheduled semantic Scale court.

Phase 5 also does not claim that the existing synchronous retention
`branch_head_versions()` enumeration is O(1) or branch-population-flat. The
cost facade reports that global maintenance lane separately from the selected
branch tuple; Phase 10 owns its lifecycle/retention cutover and must either
make acquisition local or certify the scan as maintenance with its own slope.

### Phase 5 corrective reopening (post-Sol audit)

The first final Sol review blocked closure on two supported findings. First,
the schema authority digest concatenated variable-length names and identifiers
without lengths or collection counts; distinct registry snapshots could share
the same preimage. `authority_snapshot.rs` now uses a versioned,
length-prefixed encoding for every variable field plus explicit option and
collection boundaries. The adversarial boundary test constructs the previously
ambiguous byte shape and proves distinct digests.

Second, finalization synchronously invokes retention, whose branch-head
enumeration is population-dependent. The selected Phase-5 tuple is now
explicitly limited to ordinary selected-branch counters; `branch_population_scans`
is reported as a separate maintenance lane and is not represented as O(1)
ordinary branch work. The cost certification asserts that separation, and the
ledger records the retention scan as a Phase-10-owned nonclaim rather than
silently calling it flat.

Correction evidence:

- schema digest boundary and relation-policy unit proofs — **2 passed**;
- relational certification after correction — **119 passed, 0 failed, 1
  ignored** (the Scale case remained intentionally ignored in that ordinary
  run);
- the cost-scope assertions prove the selected tuple remains stable while
  synchronous maintenance is exposed separately;
- formatting, boundary/context, UI authority, and dirty-file mechanical gates
  remain required closure checks below.

The Sol blocker is retained as audit history. The affected schema and cost rows
are reopened until fresh independent Luna qa-loop, qa-tests, and
code-quality-qa critics, followed by a new Sol-high reviewer, certify this
corrected source and record.

### Phase 5 corrective reopening (branch-local invariant proof)

The fresh post-correction qa-tests pass found a proof gap in the commit
boundary: the branch-specific routing existed, but the production
certification suite did not independently force a divergent main root and
child root to produce different invariant outcomes. It also identified that
the invariant execution context still reported the runtime-global current
version. The finding was accepted and the Phase-5 rows were reopened.

Correction evidence:

- `phase5_branch_invariant_selection.rs` now builds a production Supply Chain
  world with a source-cardinality contract, forks `storm`, advances main with
  an assignment absent from the child root, and proves the child accepts a
  distinct assignment. A mutation routing invariant evaluation back to the
  global main root rejects the child commit.
- `InvariantWorkPacket` carries `current_version_id`; native and custom
  execution contexts consume it; custom scope planning receives the same
  selected version; skipped/preparation-failure metadata records it rather
  than reading runtime-global currentness.
- `custom_scope_planner_preserves_owner_selected_current_version` — **1
  passed**; Supply Chain native and production custom branch-root selection —
  **2 passed**; relation-integrity filter — **48 passed, 0 failed**; full
  relational certification after the production custom-rule correction —
  **121 passed, 0 failed, 1 ignored**.
- UI authority suite — **21/21**; boundary checker, generated context,
  formatting, diff check, and dirty Rust line-cap equivalent remain green
  (`206` dirty Rust files, maximum `399` lines; 146 tracked and 60
  untracked Rust files in the current dirty scope).

The production custom-rule correction closes the last proof gap in this row:
`compile_supply_chain_baseline_with_custom_invariant` installs a real custom
registration through the Supply Chain production builder, and the child commit
asserts both `InvariantExecutionMetadata.current_version_id` and
`CustomInvariantProvenance.current_version_id` against the fork basis after
main advances. A custom packet/context or metadata fallback to runtime-global
currentness therefore fails the branch certification rather than only a unit
planner test.

Review history for this reopening:

- `phase5_qatests_source_verdict_luna` — **BLOCKED**, fresh Luna-max
  qa-tests. It passed fixture/oracle, COW/traversal/accounting, authority
  denial, and ledger metadata, but required the divergent-root Supply Chain
  invariant proof and a selected current-version assertion. No source edits
  were made by the reviewer.
- `phase5_final_qatests_luna_fast` — **BLOCKED**, fresh Luna-max qa-tests
  after the native correction. It confirmed the native child-root proof and
  packet propagation, then found custom planner and skipped/preparation
  metadata still reading runtime-global currentness. The finding is closed by
  the correction evidence above; a new Luna trio is required.
- `phase5_custom_version_luna_qatests` — **PASS**, fresh Luna-max qa-tests
  after custom-version propagation. It confirmed source threading through
  branch execution, packets, native/custom contexts, and metadata, but its
  review predates the production custom-rule proof added below; it is retained
  as historical evidence, not the current gate.
- `phase5_ledgerfresh_luna_codequality` — **PASS**, fresh Luna-max
  code-quality-qa after the ledger inventory correction. It inspected the
  current dirty scope of 206 Rust files (146 tracked, 60 untracked), measured
  a 368-line maximum, and found no topology, authority, decomposition, or
  line-cap blocker across the Phase-5 source and nine claim rows.
- `phase5_ledgerfresh_luna_qatests` — **BLOCKED**, fresh Luna-max qa-tests
  after the ledger inventory correction. It found the branch-local native
  proof and source propagation sound, but required a production-facade custom
  registration with metadata/provenance assertions; the custom-rule proof
  correction above closes that finding and requires a new fresh trio.

Current final-source Phase-5 gate reviews:

- `phase5_ledgercorrected_qaloop_luna` — **PASS**, fresh independent Luna-max
  qa-loop. Read-only review of the current dirty source and ledger verified
  all nine rows, the production custom-invariant selected-version proof,
  121/0/1 certification, UI 21/21, boundary/context/format/diff, clippy exit
  0, and the 206-file/399-line cap. The reviewer accepted the explicit
  Phase-10 durability nonclaim and found no current Phase-5 defect.
- `phase5_ledgercorrected_qatests_luna` — **PASS**, fresh independent
  Luna-max qa-tests. Read-only review found the Supply Chain production
  builder, owner-issued handles, independent oracle/comparator, mutation
  probes, selected-root/COW/accounting/authority/schema/custom-version tests,
  and the 121/0/1 and 21/21 evidence causal. No test-world or evidence
  blocker was supported; true Scale, retention/reclamation, fresh-process,
  CAS/merge, and later-phase claims remain excluded.
- `phase5_final_custom_codequality_luna_retry` — **PASS**, fresh independent
  Luna-max code-quality-qa. It found no topology, authority, facade,
  decomposition, dependency-direction, or line-cap defect in the current
  dirty set; the production custom test is correctly placed and the current
  ledger evidence is coherent.
- `phase5_final_sol_high` — **BLOCKED, historical metadata-only review**.
  Fresh Sol 5.6-high reproduced the implementation and all Phase-5 commands:
  121/0/1 certification, UI 21/21, boundary/context/fmt/diff/clippy, and the
  60/14/1 durability nonclaim. Its only finding was that its first inventory
  sample reported 200 files (140 tracked + 60 untracked), while the
  deterministic current-tree inventory above is 206 files (146 tracked + 60
  untracked). No implementation or evidence claim was rejected; the ledger
  now records a path fingerprint and the fresh Luna trio is being rerun
  against it before Sol re-certification.
- `phase5_code_quality_luna6` — **BLOCKED, historical review**. It identified
  a redundant allocation-accounting closure and the inherited durability
  failure family. The closure was removed and the focused relational clippy
  command now exits 0; the 60/14/1 durability result is recorded above as a
  Phase-10-owned nonclaim. A fresh code-quality retry on the corrected tree
  independently returned PASS, so neither finding remains a current
  Phase-5 blocker.

The previous Luna qa-loop, qa-tests, and code-quality gates were PASS, but the
Sol inventory reconciliation reopened the current-source metadata gate. A
fresh Luna trio must now cite the deterministic fingerprint above; final Sol
5.6-high certification remains closed until that trio is green again.

Fingerprint-reconciliation rechecks:

- `phase5_fingerprint_qaloop_luna` — **PASS**, fresh independent Luna-max
  qa-loop. It cited `HEAD 08a79b079499602a374a2c09986bbd50e62f1700`, sorted
  dirty-Rust path SHA-256
  `dcc5a8098d5fa7f30ad88e75f311cdee8989766e599ff316fa1e19d92bb57c38`,
  146 tracked + 60 untracked = 206, and max 399. All nine rows, 121/0/1,
  UI 21/21, production custom provenance, mechanical checks, and the
  durability nonclaim were verified with no finding.
- `phase5_fingerprint_qatests_luna` — **PASS**, fresh independent Luna-max
  qa-tests. It cited the same HEAD, path fingerprint, 206-file inventory, and
  max 399; confirmed the production Supply Chain builder/oracle boundary,
  native and custom selected-version mutation probes, 121/0/1, UI 21/21, and
  all mechanical gates. No test or evidence blocker was supported.
- `phase5_fingerprint_codequality_luna` — **PASS**, fresh independent
  Luna-max code-quality-qa. It cited the same HEAD, sorted dirty-Rust path
  fingerprint, 206-file inventory, and max 399; scrutinized 206 files with
  109 advisory candidates and zero scan errors. Topology, authority direction,
  facade honesty, decomposition, schema ownership, maintenance-cost
  separation, custom test placement, and mechanical gates were all clear.

### Phase 5 corrective reopening (selected-root structural adjacency)

The first fresh Luna trio after the branch-local invariant correction found a
supported P1 in the custom invariant structural path. `StructuralRelationView`
and touched-scope expansion combined selected-root metadata with adjacency
enumerated from `runtime.storage_access()`. A forked child could therefore
observe main or sibling edges under child metadata after a main-branch rewire.
The finding reopened the Phase 5 structural-selection row and is corrected
before the current review gate.

Correction evidence:

- `storage/partition/adjacency_queries.rs` now exposes selected-state candidate
  enumeration over `PartitionAccess`; `InvariantStateView` owns the
  direction-filtered, metadata-validated, canonical outgoing/incoming/all
  relation views in `validation/engine/state_view/structural_adjacency.rs`.
- `StructuralRelationView` and `collect_touched_structural_set` consume only
  the selected `InvariantStateView`. Runtime authority remains available to
  the custom path only as `PerformanceAccess` for traversal counters and
  execution cost; it no longer supplies structural storage truth.
- The production Supply Chain Court proof
  `phase5_custom_invariant_structural_selection.rs` forks a child, rewires a
  relation on main, then commits an entity-only child action through the public
  facade. Its independent oracle and custom probe assert selected metadata,
  touched relation/endpoints, directional adjacency, bounded traversal,
  selected version, provenance, and exclusion of main-only endpoints.
- Full relational certification — **122 passed, 0 failed, 1 ignored**;
  focused custom structural proof — **1 passed**; custom-rule unit lane —
  **8 passed**; state-view unit lane — **2 passed**; native/custom branch
  invariant selection — **2 passed**; branch traversal isolation — **1
  passed**; UI authority suite remains **21/21**.
- Causal mutations are sensitive and restored: routing
  `StructuralRelationView` adjacency through global storage fails at the
  retained-child outgoing-edge assertion; routing touched-scope expansion
  through global storage fails at the retained relation-ID assertion. The
  restored source passes the focused and full certification proofs.
- `cargo check -p worth-relational --all-targets`, owner-scoped library
  Clippy with `-D warnings`, boundary topology, generated context,
  formatting, diff check, Git-Bash dirty line-cap guard, and the Rust
  function-structure scraper pass. The all-target test Clippy lane still
  reports 160 pre-existing lints in unrelated historical/support test files;
  that repository debt is not promoted as Phase 5 evidence.
- The broad relational library lane is recorded honestly as **1000 passed,
  86 failed, 25 ignored**. Its failures are the inherited historical,
  recovery, retention, merge, query, and later-phase owner-root family; this
  phase claims neither those later lanes nor their closure.
- Current deterministic inventory: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700`; path-sorted
  `path<TAB>sha256(raw bytes)` fingerprint, with no trailing newline,
  `d0ef80b353499caa4f3fb1d6f7067082e7dc0de61bff46bfb0015876eb07a363`;
  **152 tracked + 62 untracked = 214 dirty Rust files**, maximum **399
  physical lines** (**368 nonblank**) at
  `crates/worth-relational/src/branch/reference.rs`.

The corrected source remains open until a new, separate Luna-max qa-loop,
qa-tests, and code-quality-qa trio independently clears this exact packet;
only then may a fresh Sol-high gate certify Phase 5. Historical reviewer
verdicts above are not reused for this correction.

### Phase 5 evidence reconciliation (current packet)

The historical counts and inventories in the earlier reopening sections are
retained as audit history, but they are not the active gate. In particular,
the earlier `121 passed`, `119 passed`, `206`-file, `dcc5`, and older inventory
rows predate the selected-root adjacency correction or its current proof
updates. The active packet for the next fresh reviewer trio is:

- relational certification — **122 passed, 0 failed, 1 ignored**;
- focused selected-root structural proof — **1 passed**;
- custom-rule unit lane — **8 passed**; state-view lane — **2 passed**;
- native/custom branch-invariant selection — **2 passed**; branch traversal
  isolation — **1 passed**; UI authority suite — **21/21**;
- current dirty Rust inventory — **152 tracked + 62 untracked = 214**;
  path-sorted `path<TAB>sha256(raw bytes)` fingerprint with no trailing
  newline —
  `1d23f8d3799dee0d4532fd046258613646555d3f104153a6422be05702d684f`;
  dirty maximum — **399 physical lines** at
  `crates/worth-relational/src/branch/reference.rs` (**368 nonblank**);
- boundary checker, generated-context check, formatting, dirty line-cap
  guard, and Rust function scraper — **PASS** (`214` files, `114` advisory
  candidates, `0` scan errors); owner-scoped library Clippy remains **PASS**.

The broad relational library lane remains a deliberately separate debt
report: **1000 passed, 86 failed, 25 ignored**, with failures in inherited
historical, recovery, retention, merge, query, and later-phase owner-root
families. All-target test Clippy still reports the unrelated legacy/support
test-module lint debt. Neither lane is silently promoted into the Phase-5
claim.

The two evidence gaps from the fresh qa-tests reviewer are now closed in the
production certification tests. The structural probe asserts the exact
sorted visible entity set `[source, target]`, the exact selected relation,
the exact deduplicated touched-partition set, and negative membership for the
main-only moved endpoints. The branch invariant proof records native
cardinality execution on the first main commit, rejects the second main
assignment while asserting the main head is unchanged, and records native
cardinality execution on the child commit that succeeds from the child root.
The structural test also records the selected version and provenance; its
bounded traversal assertion checks that both directional walks charge the
selected relation without over-specifying incident relation fan-out.

#### Retained mutation evidence

The mutation probes were run against the exact current-source correction and
reverted before the current packet was measured. Their patches and outputs
are retained here so the evidence is reproducible rather than only narrated.

Mutation A — restore global adjacency in `StructuralRelationView`:

```diff
 struct StructuralRelationView<'runtime> {
+    runtime: &'runtime RelationalRuntime,
     state_view: InvariantStateView<'runtime>,
 }

-pub(crate) fn new(state_view: InvariantStateView<'runtime>) -> Self {
-    Self { state_view }
+pub(crate) fn new(
+    runtime: &'runtime RelationalRuntime,
+    state_view: InvariantStateView<'runtime>,
+) -> Self {
+    Self { runtime, state_view }
 }

 pub fn outgoing_relations_for_entity(&self, entity_id: EntityId) -> Vec<RelationId> {
-    self.state_view.outgoing_relations_for_entity(entity_id)
+    self.runtime.storage_access().outgoing_relations_for_entity(
+        entity_id,
+        self.state_view.version_id(),
+    )
 }
 // The incoming/all methods were changed by the same substitution.
```

The two constructor call sites in `scope_planner.rs` and
`execution_context.rs` were changed to pass `runtime`. Command and retained
failure:

```text
cargo test -p worth-relational --test relational_certification phase5_custom_invariant_structural_selection -- --nocapture
assertion failed: relations.outgoing_relations_for_entity(expected.source).contains(&expected.relation)
```

Mutation B — restore global touched-scope expansion:

```diff
-pub(crate) fn collect_touched_structural_set(
-    state_view: &InvariantStateView<'_>,
+pub(crate) fn collect_touched_structural_set(
+    runtime: &RelationalRuntime,
+    state_view: &InvariantStateView<'_>,
     merged_plan: Option<&MergedCommitPlan>,
 ) -> TouchedStructuralSet {
     // ...
-    for relation_id in state_view.all_relations_for_entity(entity_id) {
+    for relation_id in runtime.storage_access().all_relations_for_entity(
+        entity_id,
+        state_view.version_id(),
+    ) {
         visible_relations.insert(relation_id);
         // ...
     }
```

The capture and packet/test call sites were changed to pass `runtime` for the
mutant. Command and retained failure:

```text
cargo test -p worth-relational --test relational_certification phase5_custom_invariant_structural_selection -- --nocapture
assertion failed: touched.visible_relation_ids().contains(&expected.relation)
```

Both mutants were reverted; the current selected-state implementation and
the focused structural test pass. These retained mutations are causal
negative controls, not source changes or additional compatibility lanes.

The current Phase-5 row remains **open** pending a new, separate Luna-max
qa-loop, qa-tests, and code-quality-qa trio against the exact packet above.
Only after all three report clean may a fresh Sol-high final gate certify and
close Phase 5.

### Phase 5 fresh Luna trio (selected-root architecture audit)

The corrected current packet was independently reviewed by three fresh
`gpt-5.6-luna` max-reasoning critics. The qa-loop critic — `Singer`, fresh
read-only instance — returned **PASS** for the nine requirements, selected-root
structural adjacency/invariant execution, causal mutation evidence, and
explicit exclusions. The qa-tests critic — `Ampere`, fresh read-only instance
— returned **PASS** for fixture realism, production-facade boundaries,
independent oracle, exact structural scope, native/custom controls, mutation
sensitivity, and ledger causality. Both verified the active `1d23…` packet,
214-file inventory, 399/368 line counts, and 122/0/1 certification.

The code-quality critic — `Laplace`, fresh read-only instance — returned
**BLOCK** with two P1 findings requiring architectural correction before Phase
5 can close:

1. `validation/invariant_access/execution.rs:44-45` and
   `authority/commit/phases/prepare.rs:123-151` can fall back from a missing
   branch root to runtime/global validation. A branch-bound commit could then
   validate against the wrong selected root. The reviewer requires fail-closed
   typed branch-root-unavailable behavior and a missing-root negative proof.
2. `validation/engine/state_view.rs:36-46,238-246` iterates every partition,
   delegating through `branch/root.rs:355-357` and `branch/root_regions.rs:114-122`
   to materialize the complete selected root. The reviewer judged this
   incompatible with Phase-5 touched-footprint preparation/cost claims and
   requires carried touched-partition scope or indexed selected-root lookup,
   plus fixed-delta/growing-unrelated-partition cost evidence.

These are accepted as supported findings pending source verification and a
Sol-high plan-implementation review. Phase 5 remains open; no later phase is
permitted to start.

### Phase 5 corrective reopening (fail-closed root selection and bounded touched scope)

The fresh code-quality finding was independently reviewed by `Meitner`, a
fresh `gpt-5.6-sol` max-reasoning plan-implementation agent. Sol confirmed both
findings as in-scope P1 defects: branch-bound preparation could fall back to
global state when a committed root was unavailable, and touched-scope
discovery could enumerate/materialize every selected-root partition. Sol's
plan required a proof-backed selected-root resolver, typed preparation denial,
carried selected state through commit and invariant phases, and touched
partition discovery from the mutation journal rather than root enumeration.

The plan was implemented as follows:

- `branch/root_selection.rs` now resolves the exact owner-issued binding into
  a carried `SelectedRelationalBranchState`. Explicit empty branches receive a
  real zero-state root; committed branches require the binding cell, runtime,
  commit catalog artifact, root commit identity, version, truth/schema roots,
  and parentage to agree. Missing roots return
  `CommitPreparationError::SelectedBranchRootUnavailable`; mismatched axes
  return the distinct reference-mismatch reason. The resolver does not use
  completeness reconstruction as an authority shortcut.
- Commit preparation resolves this state after the existing stale-binding
  admission check and before merge planning, then uses it for structural
  summary, working-state preparation, invariant execution, and publication
  context. The previous `None => runtime.storage_access().current_state()`
  branches were removed. The stale foreign-runtime binding contract remains a
  stale-validation conflict, preserving its prior boundary behavior.
- `PartitionAccess::touched_partition_ids` carries overlay mutation-journal
  keys. `InvariantStateView::touched_visible_entity_ids` and
  `touched_visible_relation_ids` probe only those keys and their touched slots;
  they no longer call `partition_ids()` on a selected root. Full-root
  materialization remains available only to explicitly global/root-wide paths.
- `branch_root_selection_denials.rs` proves unavailable committed roots fail
  before planning/effects, mismatched roots receive a distinct typed denial,
  the catalog count is unchanged, and empty and committed selected states stay
  distinct. The state-view unit proof wraps `partition_ids()` with a panic and
  passes only because touched discovery uses journal keys.

Exact correction evidence:

- full relational certification — **122 passed, 0 failed, 1 ignored**;
- Phase 5 certification filter — **27 passed, 0 failed, 1 ignored**; selected
  root denials — **3 passed**; state-view unit lane — **2 passed**;
- `cargo check -p worth-relational --all-targets`, owner-scoped library
  Clippy with `-D warnings`, `cargo fmt --all -- --check`, boundary topology,
  generated context, dirty line-cap guard, and the Rust function scraper —
  **PASS**. The scraper reports **229 Rust files, 122 advisory candidates,
  0 scan errors**. Existing all-target test-Clippy lints and the broad
  relational-library historical failure family remain separate repository
  debt/nonclaims.
- Causal mutation 1 replaced journal-key discovery with `partition_ids()`;
  `cargo test -p worth-relational state_view::tests --quiet` failed at the
  no-world-enumeration assertion, then the mutant was restored and the lane
  passed (**2/2**).
- Causal mutation 2 replaced unavailable-root denial with an empty-root
  fallback; `cargo test -p worth-relational branch_root_selection_denials
  --quiet` failed because the transaction committed instead of returning the
  typed preparation error, then the mutant was restored and the lane passed
  (**3/3**).
- Current deterministic dirty-Rust packet: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700`; path-sorted
  `path<TAB>sha256(raw bytes)` fingerprint with no trailing newline —
  `d75b9435a20de421dfc0d91345473e607d587c5421f0c42fc1d04c4ceea9ade4`;
  **164 tracked + 65 untracked = 229 files**; maximum **399 physical lines**
  (**368 nonblank**) at `crates/worth-relational/src/branch/reference.rs`.

This correction packet is open pending a new, separate Luna-max qa-loop,
qa-tests, and code-quality-qa trio against this exact fingerprint. Only after
all three report clean may a fresh Sol-high final gate certify and close Phase
5. No later phase may start before that closure.

### Phase 5 corrective reopening (selected-state propagation, publication base, and global uniqueness)

The second fresh Luna-max trio found three additional authority defects after
the fail-closed root correction. `Mendel` (qa-loop), `Ampere2` (qa-tests), and
`Nash` (code-quality-qa) independently required selected branch state to remain
carried after preparation, through strategy lowering/admission, all invariant
observations, mutation/snapshot overlays, artifact and publication preparation;
normalization had to remain after root selection; publication had to use the
selected root as its previous/base authority; and uniqueness could not claim a
touched lookup while performing a selected-state scan. `Poincare`, a fresh
Sol-high plan-implementation agent, confirmed the findings and supplied the
correction plan.

The plan is now implemented. Selected state is carried in prepared and admitted
execution packets, strategy lowering resolves it before transaction creation,
overlay/invariant/snapshot paths require it, branch-local deletion allowance and
publication root capture use it, and uniqueness is explicitly `Global` until a
branch-qualified authoritative index exists. The uniqueness evaluator scans the
selected state and applies the pending entity delta at committed observation;
the mutation-sensitive policy admits that global scan. The committed scan's
duplicate witness selection is ordered through the authoritative comparison key.
The raw-key strategy-lowering denial proof and the prior no-side-effect root
denial proof remain active.

Fresh focused evidence after this correction is green: full Relational
certification is **122 passed, 0 failed, 1 ignored**; uniqueness complexity is
**3/3**; selected-root denials are **3/3**; the state-view lane is **2/2**; and
the branch-reference UI authority suite is **21/21**. `cargo check` all targets,
owner library Clippy with `-D warnings`, formatting, boundary topology,
generated context, dirty line caps, diff check, and Rust function scrutiny all
pass. The current scrutiny reports **244 Rust files, 131 advisory candidates,
0 scan errors**. The exact deterministic dirty-Rust packet is `HEAD
08a79b079499602a374a2c09986bbd50e62f1700`; fingerprint
`1b11d7e0e2d689fed3f6607060c01f45429687bb53c6ac52c4794c53660a3c08`; **179
tracked + 65 untracked = 244 files**; maximum **400 physical lines** (**364
nonblank**) at `crates/worth-relational/src/branch/root.rs`.

The causal mutation suite was rerun and every temporary source mutation was
reverted: moving root selection after raw-key normalization failed the
interner side-effect assertion; replacing carried selected invariant state with
an empty root failed the child structural commit; dropping the selected
previous publication root failed the copy-on-write breadth proof; and changing
the mutation-sensitive global ceiling to `Touched` skipped the uniqueness
violations and scan-cost proofs. This packet remains open pending a new,
separate Luna-max qa-loop, qa-tests, and code-quality-qa trio, followed only by
the required fresh Sol-high final gate. No later phase may start before that
review chain closes Phase 5 in this ledger.

### Phase 5 certification packet after the second corrective slice

The second corrective slice is now verified against the exact dirty source
packet. The owner-issued proposal identity has a dedicated typed ordinal
exhaustion reason and issues its proposed version from the runtime-global
history sequence. Strategy validation uses that identity rather than a
branch-local `validated_against_version + 1`. Validated mutations fail closed
against a stale same-branch reference before revalidation, proposal issuance,
or invariant work; the owner test proves the stale denial consumes no proposal
ordinal. Invariant metadata receives the exact identity on normal execution,
plan-contract skips, may-break skips, and preparation-violation results.

The custom proposed-state proof now reads both committed and proposed views
during preparation and evaluation: the committed snapshot remains `Planned`,
the proposed state is `Held`, and the published snapshot becomes `Held` only
after the commit. The native uniqueness packet includes sibling-divergence
oracle agreement, one-branch rejection residue checks, and two colliding
creates in one transaction. Those rejection proofs preserve values, branch
reference state, commit catalog count, and snapshot version. The Large proof
uses the real Scale definition and selected production snapshot, asserts the
installed live record count is the causal definition count above 100,000,
proves global commit and baseline publication ceilings, proves the ordinary
post-baseline publication transition to `Partition`, excludes the
GraphComposition probe from commit execution, and checks branch-reference
residue on a rejected duplicate. The removed ignored singleton Scale-fork
profile remains an explicit nonclaim: fork slope and memory at Scale are
deferred to the later performance phase; active Scale invariant correctness
remains claimed.

Exact current evidence:

- complete `cargo test -p worth-relational --test relational_certification
  --quiet` — **128 passed, 0 failed, 0 ignored**, including the real Large
  proof; elapsed **564.52 seconds**;
- owner invariant-access unit filter — **9 passed, 0 failed**;
- `cargo check -p worth-relational --all-targets --quiet` — **PASS**;
  owner library Clippy with `-D warnings` — **PASS** (the output contains
  the pre-existing 53 `worth-signal` warnings, with no owner failure);
- `cargo fmt --all -- --check`, `git diff --check`, boundary topology,
  generated agent context, dirty Rust line-cap equivalent, and
  `scrutinize_rust_functions.py --dirty .` — **PASS**; scrutiny covered 263
  dirty Rust files, 152 advisory candidates, and 0 scan errors;
- current dirty Rust inventory — **191 tracked + 72 untracked = 263**;
  path-sorted `path<TAB>sha256(raw bytes)` fingerprint with no trailing
  newline —
  `cbf152bf76923ac70fac3f1492ed91eb4f06ac2dd3e60470a0fc84e4edfe967b`;
  maximum — **400 physical lines** (**380 nonblank**) at
  `crates/worth-relational/tests/relational_certification/phase5_global_uniqueness.rs`;
  `crates/worth-relational/src/branch/root.rs` is also **400 physical lines**
  (**364 nonblank**).

The current packet is open pending three separate fresh Luna-max critics —
qa-loop, qa-tests, and code-quality-qa — followed, only if all three report
clean, by the required fresh Sol-high final gate. No later phase may start
before that review chain closes Phase 5 in this ledger.

### Phase 5 certification packet after GraphComposition ceiling assertion

The first fresh qa-tests critic found one remaining evidence gap: the direct
GraphComposition proof counted one preparation and one evaluation, but did not
assert the returned execution metadata's admitted cost ceiling. The correction
adds that result-level assertion. A policy widening from `Touched` to `Global`
now fails the test even if the probe still executes. This is a narrow evidence
correction; it does not change the runtime authority model or claim ordinary
commit GraphComposition execution.

Exact post-correction evidence:

- focused real-Scale GraphComposition admission proof — **1 passed, 0 failed**;
  the >100,000-record path completed in **371.62 seconds**;
- complete `cargo test -p worth-relational --test relational_certification
  --quiet` — **128 passed, 0 failed, 0 ignored**; elapsed **377.87 seconds**;
- owner invariant-access unit filter — **9 passed, 0 failed**;
- `cargo check -p worth-relational --all-targets --quiet`, owner library
  Clippy with `-D warnings`, formatting, boundary topology, generated context,
  `git diff --check`, scrutiny, and the PowerShell-equivalent dirty Rust
  line-cap guard — **PASS**. Scrutiny covered **265 dirty Rust files**, **152
  advisory candidates**, and **0 scan errors**. The line-cap check reports
  **265 files**, maximum **400** physical lines at
  `crates/worth-relational/src/branch/root.rs`, with no violation; the Bash
  wrapper remains unavailable because this Windows host has no `/bin/bash`;
- current dirty Rust inventory — **191 tracked + 74 untracked = 265**;
  path-sorted `path<TAB>sha256(raw bytes)` fingerprint with no trailing
  newline —
  `d18f9d75b9cb90c8d7b262c9de92d0104b797c8965153dd52c490fe92be48a20`;
  maximum **380 nonblank lines** at
  `crates/worth-relational/tests/relational_certification/phase5_global_uniqueness.rs`.

The preceding fresh chain therefore has one resolved substantive BLOCK
(qa-tests) and one CLEAN qa-loop result; its code-quality critic returned no
verdict before the bounded wait and was closed as a procedural non-result.
Those reports do not certify this new fingerprint. A new, separate Luna-max
qa-loop, qa-tests, and code-quality-qa trio is required now. Only after all
three report clean may a fresh Sol-high final gate certify and close Phase 5;
no later phase may start before that review chain closes the phase in this
ledger.

### Phase 5 certification packet after GraphComposition admission correction

The fresh qa-tests critic identified a proof gap in the prior Large invariant
packet: it proved only that the GraphComposition probe was absent from an
ordinary commit result, without directly invoking the public graph facade or
proving that the probe would run when the graph profile admitted it. The
correction is narrow and test-only. The Large test now registers separate
Global commit/publication probes and a Touched-cost GraphComposition probe,
constructs an owner-issued real Scale transaction plan, invokes
`runtime.validation().graph_composition_plan(&plan)`, and records exactly one
preparation and one evaluation for that direct graph admission. Baseline and
post-baseline ordinary commits still assert zero graph calls; the test makes
no claim that GraphComposition is part of ordinary commit admission. The
snapshot-observation helpers were split into a named module so the corrected
test remains within the 400-line cap.

Exact current evidence:

- focused real-Scale GraphComposition admission proof — **1 passed, 0 failed**;
  the >100,000-record path completed in **383.47 seconds**;
- complete `cargo test -p worth-relational --test relational_certification
  --quiet` — **128 passed, 0 failed, 0 ignored**, including the corrected
  Large proof; elapsed **401.25 seconds**;
- owner invariant-access unit filter — **9 passed, 0 failed**;
- `cargo check -p worth-relational --all-targets --quiet` — **PASS**;
  owner library Clippy with `-D warnings` — **PASS** (the output contains
  only the pre-existing 53 `worth-signal` warnings);
- `cargo fmt --all -- --check`, `git diff --check`, boundary topology,
  generated agent context, the PowerShell-equivalent dirty Rust line-cap
  guard, and `scrutinize_rust_functions.py --dirty .` — **PASS**; scrutiny
  covered **265 dirty Rust files**, **152 advisory candidates**, and **0 scan
  errors**. The repository Bash wrapper could not run because Windows has no
  `/bin/bash`; the equivalent checks all 265 paths and reports no violations;
- current dirty Rust inventory — **191 tracked + 74 untracked = 265**;
  path-sorted `path<TAB>sha256(raw bytes)` fingerprint with no trailing
  newline —
  `fe8064fc38d78a2c3e3cf00a59034dd1c8a80382b0f02ff4ca372a2aa18620c6`;
  maximum — **400 physical lines** at
  `crates/worth-relational/src/branch/root.rs`; maximum **380 nonblank lines**
  at `crates/worth-relational/tests/relational_certification/phase5_global_uniqueness.rs`.

The previous fresh Luna-max trio was closed after its substantive findings
were recorded: the qa-tests critic required the direct GraphComposition proof;
the code-quality critic also noted destination-topology concerns (certification
module placement, the eventual HistorySubsystem split, and the eventual
plural branch facade). Those broader destination moves belong to the later
Phase 6–9 architecture unless the final Sol gate adjudicates otherwise; no
unbounded topology rewrite is being smuggled into this Phase 5 evidence
correction. Two fresh Sol-high planning attempts were requested for this
narrow adjudication but timed out before returning a plan. This packet remains
open pending a new, separate Luna-max qa-loop, qa-tests, and code-quality-qa
trio against the exact fingerprint above. Only after all three report clean
may a fresh Sol-high final gate certify and close Phase 5. No later phase may
start before that review chain closes Phase 5 in this ledger.

### Phase 5 certification packet after scheduled Scale lane and semantic test topology

The Sol-high plan-implementation review identified the remaining test-lane
problem: the real Scale world is the correct causal evidence but is too
expensive for the ordinary certification path. The correction keeps that
world unchanged and marks only its Scale certification test ignored, adds a
mandatory scheduled CI lane that runs the exact ignored test, and adds a
small Standard-world GraphComposition court to the ordinary lane. The common
court uses the production compiler and public validation facade, proves that
ordinary commit admission does not call the GraphComposition probe, then
directly admits the owner-issued graph plan with the `Touched` ceiling and
exactly one preparation and one evaluation. The Scale proof retains the
stronger real-world assertions: more than 100,000 live records, global
commit and baseline publication enforcement, the ordinary post-baseline
`Partition` transition, direct `Touched` GraphComposition admission with
one preparation and one evaluation, ordinary graph exclusion, and duplicate
rejection residue.

The certification integration test was also moved into semantic physical
topology (`reference/`, `root/`, `invariants/`, and `preservation/` paths)
without compatibility wrappers or a second test target. The root integration
module remains the sole owner of the test target; existing module identities
are retained only to keep the test authority and names stable while their
physical files now follow domain meaning. CI and both testing documents now
state the ordinary and scheduled commands and the Scale nonclaim for fork
slope and memory.

Exact current evidence:

- ordinary certification command
  `cargo test -p worth-relational --test relational_certification
  --no-fail-fast --quiet` — **128 passed, 0 failed, 1 ignored**; elapsed
  **50.04 seconds**;
- ordinary Standard GraphComposition court — **1 passed, 0 failed**;
  elapsed **4.39 seconds**; the direct result retains `Touched` as its
  admitted maximum and records exactly one preparation and one evaluation;
- mandatory scheduled Scale command
  `cargo test -p worth-relational --test relational_certification
  phase5_large_invariant_admission::large_runtime_keeps_global_enforcement_and_filters_graph_planning
  -- --ignored --exact --nocapture --test-threads=1` — **1 passed, 0
  failed, 0 ignored**; elapsed **411.59 seconds**;
- `cargo check -p worth-relational --all-targets` — **PASS**;
  owner invariant-authority unit filter — **4 passed, 0 failed**;
  `cargo clippy -p worth-relational --lib --no-deps -- -D warnings` —
  **PASS**;
- `cargo fmt --all -- --check`, `git diff --check`, boundary topology,
  generated agent context, the PowerShell-equivalent dirty Rust line-cap
  guard, and Rust function scrutiny — **PASS**; the line-cap inventory is
  **271 dirty Rust files**, with no violation and a maximum of **400 physical
  lines** at `crates/worth-relational/src/branch/root.rs`;
- current deterministic dirty-Rust packet: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700`; path-sorted
  `path<TAB>sha256(raw bytes)` fingerprint with no trailing newline —
  `5c38faae32f7e0db118cd5ed548e85b22b968c7c97efe1c81758d0b63e3fdec8`;
  **271 files**; **40,347 nonblank lines**.

The workspace-wide all-targets check and broad Clippy command still expose
pre-existing failures in untouched `worth-signal`, `worth-math`, and
`worth-harness` code (including the unchanged `worth-signal` test import
failure and existing warning-denied lint debt). `git status` and `git show
HEAD` confirm those files are outside this Phase 5 dirty slice. They are
recorded repository debt under the governing scope rule; the affected
Relational package gates above are green.

This packet remains open pending a new, separate Luna-max qa-loop, qa-tests,
and code-quality-qa trio against this exact fingerprint. Only after all three
report clean may a fresh Sol-high final gate certify and close Phase 5. No
later phase may start before that review chain closes Phase 5 in this ledger.

### Phase 5 independent-review attempt record for the current packet

The required fresh Luna Max outside-eyes chain was attempted with separate
instances and remains procedural, not a certification:

- fresh qa-loop instance `Linnaeus` returned **BLOCK — procedural
  non-result** because the Luna reviewer was unavailable;
- fresh qa-tests instance `McClintock` returned **BLOCK — procedural
  non-result** after reproducing the pinned `HEAD`, 271-file inventory,
  40,347 nonblank lines, and exact fingerprint, but without a source verdict;
- fresh code-quality-qa instance `Sagan` returned **BLOCK — procedural
  non-result** because `luna-max` was unsupported for the account. Its
  primary checks matched the fingerprint, inventory, and 400-line maximum.

Two earlier fresh three-agent attempts also stalled without verdicts and were
closed as procedural non-results. No reviewer reported a substantive source
defect, but no reviewer reported CLEAN. Therefore this packet is not
certified, Sol-high final gating has not been requested, and Phase 5 remains
open. No later phase may start until a functioning fresh Luna Max trio
returns three substantive CLEAN verdicts followed by the required Sol-high
final gate.

### Phase 5 resumed independent-review audit

After the goal resumed, a new separate Luna Max retry was performed against
the unchanged fingerprint. Fresh qa-loop instance `Volta`, qa-tests instance
`Aristotle`, and code-quality-qa instance `Faraday` each returned a
procedural **BLOCK** because Luna Max/service was unavailable. The QA-tests
instance explicitly asserted that no substantive test finding was made; the
other two likewise did not certify the packet. No files were edited by any
reviewer. The phase remains open and the required Sol-high final gate remains
unrequested.

### Phase 5 evidence rerun and reused-Luna review result

At the user's direction, the existing Luna Max QA-loop instance `Raman` was
reused instead of launching another reviewer. Its first current-packet review
found an evidence gap because its ordinary certification run had been
interrupted at 87/129; it found no source defect. The ordinary lane was then
rerun to completion:

- `cargo test -p worth-relational --test relational_certification
  --no-fail-fast --quiet` — **128 passed, 0 failed, 1 ignored** in
  **75.56 seconds**;
- the reused Raman instance rechecked that completed result and returned
  **CLEAN** for the QA-loop closure review;
- the existing QA-tests instance confirmed that the former GraphComposition
  gap is fixed and the current Standard court passed, but returned a
  procedural BLOCK because it could not obtain an independent Luna verdict;
- Raman was asked to combine the remaining QA-tests and code-quality
  dimensions per the user's reuse instruction, but it produced no bounded
  verdict and was closed as a procedural non-result.

The ordinary evidence gap is repaired, but the required independent QA-tests
and code-quality CLEAN verdicts are still absent. Phase 5 remains open;
Sol-high final gating and all later phases remain prohibited.

The reused existing code-quality Luna instance verified the pinned fingerprint,
HEAD, 271-file inventory, and zero over-limit files, but was interrupted before
its exhaustive structural pass and returned a procedural non-result. It found
no substantive defect, but did not certify CLEAN.

### Phase 5 Sol-high corrective reopening

Fresh Sol-high qa-loop, qa-tests, and code-quality critics reviewed the frozen
pre-correction Rust packet `5c38faae32f7e0db118cd5ed548e85b22b968c7c97efe1c81758d0b63e3fdec8`.
Their supported findings reopen only these Phase 5 guarantees:

- public invariant-plan admission is branch-bound and never selects ambient
  main/global state;
- `publication_new_authoritative_bytes` covers every newly owned authoritative
  allocation family rather than partition payload alone;
- every visibility-commitment axis has mutation-sensitive evidence;
- Supply Chain observation derives schema from the owner-selected production
  snapshot, and Scale receives full independent semantic comparison;
- the scheduled Scale flow includes its required zero-copy fork proof;
- certification modules expose semantic ownership rather than phase-numbered
  aliases or generic support bags; and
- branch-sharing inspection is decomposed into named admission, inventory, and
  assembly responsibilities.

The broad `HistorySubsystem` destination split is retained as a Phase 7/9
obligation: it is real architecture work but is not causal to this Phase 5
corrective slice. Unchanged sharing, traversal, schema-digest, cache-exclusion,
and authority-denial evidence remains retained until a correction changes its
source or assumptions. The scheduled Scale result must rerun after this slice
because branch-bound graph routing, production observation, and Scale fork
evidence all change. Phase 5 remains open.

### Phase 5 Sol-high corrective implementation evidence

The reopened guarantees above are implemented and locally re-proved on one
post-correction source packet. Invariant planning now enters through a
transaction-owned branch binding and rejects stale branch bases; publication
accounting carries the complete newly owned root, reachability,
partition-state, payload, and canonical-artifact allocation delta; visibility
mutation evidence covers every committed tuple axis; Supply Chain schema
observation comes from the selected production snapshot; and the Scale court
performs a complete semantic audit plus a zero-copy fork proof. Test support
and sharing inspection now use semantic physical responsibilities. The root
identity issuer and persistent-region tests were split when the dirty line-cap
guard exposed two correction-caused overages.

Exact post-correction evidence:

- `cargo test -p worth-relational --test relational_certification
  --no-fail-fast --quiet` — **130 passed, 0 failed, 1 ignored** in **50.93
  seconds**;
- scheduled Scale command
  `cargo test -p worth-relational --test relational_certification
  scale_invariant_admission::large_runtime_keeps_global_enforcement_and_filters_graph_planning
  -- --ignored --exact --nocapture --test-threads=1` — **1 passed, 0
  failed, 0 ignored** in **505.42 seconds**;
- root/visibility owner units — **6 passed**; canonical-artifact accounting
  units — **2 passed**; invariant-access owner units — **9 passed**;
- branch-bound graph, production-snapshot schema, named-delta accounting,
  branch accounting, fork-sharing, and 4,096-fork focused proofs — **PASS**;
- `cargo check -p worth-relational --all-targets`,
  `cargo clippy -p worth-relational --lib --no-deps -- -D warnings`,
  `cargo fmt --all -- --check`, `git diff --check`, boundary topology,
  generated agent context, dirty Rust line-cap enforcement, and Rust function
  scrutiny — **PASS**;
- branch-reference compile-time authority suite — **21/21 compile-fail cases
  passed**;
- deterministic dirty-Rust packet: `HEAD
  08a79b079499602a374a2c09986bbd50e62f1700`; path-sorted
  `path<TAB>sha256(raw bytes)` fingerprint with no trailing newline —
  `351a4883a4fad12ecf526d4f3044220fb8165ebf8f50955eae39b64fb4daaf96`;
  **278 files**, **41,111 nonblank lines**, no line-cap violation, maximum
  **400 physical lines** at
  `tests/relational_certification/invariants/uniqueness/global.rs`.

An additional exploratory all-target Clippy run reports **162** existing
test/example advisory failures across the broad dirty milestone packet. This
is not the established Phase 5 production-library Clippy gate and did not
invalidate unrelated green proof lanes. Its output is retained for the
code-quality critic to classify; the required production-library Clippy gate
is clean.

This packet remains open pending three separate fresh Sol-high qa-loop,
qa-tests, and code-quality verdicts against the exact fingerprint above. A
separate fresh Sol-high final gate may run only after all three are CLEAN.

### Phase 5 Sol-high review findings and delta correction

The first substantive fresh Sol-high trio reviewed the `351a4883...` packet
and reopened three narrow surfaces. The qa-loop critic found that the Standard
child-graph proof asserted metadata and staleness but did not make its custom
GraphComposition verdict depend on selected committed state; it also found the
stale ordinary count in `TESTING_WORLDS.md` and the stale reviewer-model label
in this ledger. The qa-tests critic found test-only ambient
`InvariantAccess` entrypoints that could bypass production branch selection.
The code-quality critic found milestone chronology embedded in the public and
internal branch-sharing cost vocabulary.

The corrections are delta-scoped:

- test support now resolves the owner-issued main binding through the same
  selected-branch state path as production; the ambient invariant-plan seams
  and every owner-test use of them are removed;
- a Court-profile GraphComposition probe in the ordinary lane now reads committed aspect state and
  proves the main branch sees a main-only entity while the pre-divergence child
  does not, before independently proving stale child-plan denial;
- branch-sharing counters, accessors, recorders, and state use semantic names
  with no remaining Phase 5 chronology in production vocabulary;
- the live ordinary count, scheduled Scale route, and reviewer-model ledger
  text are corrected.

Only branch-selection test support, its Court-profile graph proof, branch-sharing
naming, and directly dependent documentation were invalidated. The completed
130-test ordinary lane and 505.42-second Scale lane remain retained: the
correction did not change production execution, Scale construction, Scale
observation, or their assumptions. Focused post-correction evidence is green:

- the state-sensitive Court child/main GraphComposition proof — **1 passed,
  0 failed**;
- invariant-access owner tests — **9 passed, 0 failed**;
- root-cost scopes, named-delta copy-on-write, branch copy-on-write, persistent
  path accounting, root-region reuse, and exact fork-sharing observations —
  **PASS**;
- `cargo check -p worth-relational --all-targets`, production-library
  warning-denied Clippy, formatting, diff integrity, boundary topology,
  generated agent context, and dirty Rust line-cap enforcement — **PASS**;
- dirty function scrutiny — **284 Rust files**, **157 advisory candidates**,
  **0 scan errors**; the correction-introduced 66-line graph test was split
  into named selected-state and stale-binding responsibilities and no longer
  appears as an advisory.

The current deterministic dirty-Rust packet is `HEAD
08a79b079499602a374a2c09986bbd50e62f1700`; path-sorted
`path<TAB>sha256(raw bytes)` fingerprint with no trailing newline —
`20452f29a59457d1aa074a2c3f0ae7f82992a4503d892634fd8c8f8ce08d4422`;
**284 files**, **42,219 nonblank lines**, no line-cap violation, maximum **400
physical lines** at
`crates/worth-relational/tests/relational_certification/invariants/uniqueness/global.rs`.

This corrected packet remains open pending a new separate fresh Sol-high
qa-loop, qa-tests, and code-quality trio. A separate Sol-high final gate may
run only after all three return CLEAN.

### Phase 5 independent Sol-high certification trio

Three separate fresh Sol-high critics reproduced the exact `20452f29...`
Rust packet and completed their independent dimensions:

- qa-loop critic `Averroes` — **CLEAN** after a documentation-only delta
  correction brought the active per-guarantee table, command evidence, and
  Court-profile labels forward to the current packet;
- qa-tests critic `Curie` — **CLEAN**; it independently reran the
  state-sensitive child/main GraphComposition proof (**1 passed**) and the
  invariant-access owner tests (**9 passed**), listed exactly 131
  certification tests with Scale as the sole ignored case, and retained the
  completed ordinary and Scale lanes;
- code-quality critic `Mencius` — **CLEAN**; it found the branch-sharing
  vocabulary semantic and consistent, the selected-state support/probe
  topology responsibility-honest, all 157 advisories classified, and no
  correction-introduced composition or line-cap defect.

The QA-loop corrections changed documentation only. By the critics' explicit
causal rulings, the Rust fingerprint, ordinary `130/0/1`, scheduled Scale
`1/0`, and the qa-tests/code-quality verdicts remained valid. At that
checkpoint Phase 5 was still open pending one separate fresh Sol-high final
gate against the ledger and frozen packet.

### Phase 5 final Sol-high certification and closure

Fresh final-gate critic `Cicero` returned **CLEAN** after independently
checking the corrected active status, guarantee rows, packet evidence, Scale
claim boundary, trio record, and complete Phase 5 closure honesty. Its two
predecessor passes found documentation-only stale statements; each prescribed
the narrow ledger correction and explicitly retained the Rust fingerprint,
ordinary `130/0/1`, scheduled Scale `1/0`, and all three independent CLEAN
verdicts. No production or test source changed during those ledger-only
corrections.

Phase 5 is therefore **CLOSED** on Rust packet
`20452f29a59457d1aa074a2c3f0ae7f82992a4503d892634fd8c8f8ce08d4422`
at `HEAD 08a79b079499602a374a2c09986bbd50e62f1700`: **284 dirty Rust files**,
**42,219 nonblank lines**, maximum **400 physical lines**, ordinary
certification **130 passed / 0 failed / 1 ignored**, separately scheduled
Scale **1 passed / 0 failed**, fresh qa-loop/qa-tests/code-quality trio
**CLEAN**, and separate final Sol-high gate **CLEAN**. Phase 6 may now begin
with its required Sol-high plan-implementation review.

## Phase 6 working closure ledger

Phase 6 was planned by a fresh Sol-high plan-implementation agent before
coding. A second Sol-high architecture pass corrected the durable root-schema
boundary when exact observations exposed that a recovered old root could not
lawfully borrow the runtime's current registry. The implementation now carries
one versioned, digest-bound root schema carrier through checkpoint and
readmission instead of reconstructing historical meaning from ambient state.

| Claim | Implementation surface | Proof artifact | Expected counter/result | Red control or mutation that must fail | Status |
| --- | --- | --- | --- | --- | --- |
| Owner observation admits one exact complete branch basis | `branch/{basis,observation,reference}.rs`; owner facade observation entrypoints | `basis_observation::admitted_supply_chain_observation_is_repeatable_after_branch_moves`; `production_failures::foreign_snapshot_observation_is_typed_and_does_not_cross_runtime` | observation remains bound to runtime, branch, generation, target, and exact immutable root | substitute a foreign runtime or resolve the moved current head | Local evidence green; independent review pending |
| Descriptors are transport values and cannot operate without owner readmission | branch descriptor/readmission authority and branch-reference UI cases | `basis_readmission::{transported_descriptor_requires_owner_readmission,unretained_descriptor_cannot_follow_a_moved_reference}`; 26-case `branch_reference_compile_time` suite | copied/serialized description opens no snapshot, retention, or publication door; snapshot handles cannot be constructed or deserialized | accept a descriptor directly or restore a public authority constructor | Local evidence green; independent review pending |
| Explicit external retention is the final readmission obligation | visibility retention/pin authority and admitted-basis lifecycle | `basis_retention::external_component_pin_is_the_last_readmission_obligation`; `reference_compatibility::admitted_observation_reads_do_not_move_branch_cells` | retained exact basis remains readable without moving its branch cell; unretained stale basis denies | remove the last external pin or let a read mutate the branch reference | Local evidence green; independent review pending |
| Current snapshots, visibility, history, presentation, indexes, and Bridge select the reference-selected root | `visibility/.../projection/basis_reads.rs`; structurally distinct exact and historical snapshot builders/read views and cache-key variants; history merge-basis access; observation-qualified index routing; presentation Bridge observation-binding index | `basis_read_cutover::{history_visibility_and_bridge_read_the_observation_selected_root,merge_history_resolves_from_two_exact_observations}`; exact branch-scoped entity-field and relation-join proofs after both heads diverge; 29 Bridge tests including replacement, collision, and barrier-ordered release | exact cache identity structurally requires a root; exact reads, bounded index verification, generation selection, and parity all project one selected immutable root; historical reads remain explicitly reconstructive; exact Bridge publication has no commit-derived fallback | construct an exact key without a root, move a sibling/current head after admission, release an overwritten branch lease, retain two observations for one commit, or erase the exact observation binding | Corrected local evidence green; new independent review pending |
| Exact root schema meaning survives schema movement and fresh-process recovery | `branch/root_schema.rs`; durable root schema image/carrier; checkpoint export and recovery readmission | hostile commit replay equivalence; four branch-root schema binding tests; seven root content-binding tests | same schema carriers deduplicate; missing, tampered, and swapped carriers deny; old contracts remain readable | delete, alter, or cross-bind the carrier, or decode old state with the live registry | Local evidence green; independent review pending |
| Root schema authority is physically and operationally accounted | root publication cost, allocation inventory, sharing accounting, and independently assembled owner allocation ledger | schema publication-cost unit; corrected authoritative accounting certification; `basis_cost::basis_and_external_retention_work_is_counted_exactly` | new authority charges bytes once, reused authority charges zero new bytes, shared roots deduplicate by runtime-issued allocation id; the certification byte sum does not reuse the sharing observation walk | omit `RootSchemaAuthority` from the independent ledger sum, reuse the production accounting iterator as the oracle, or assign identity from pointer/hash coincidence | Local evidence green; independent review pending |
| Historical visibility cannot leak future retirement state | generation-aware historical metadata and lifecycle projection | full relational unit lane plus exact historical replay/read-cutover proofs | historical creation/retirement is selected by record generation at the exact basis | use the latest retirement column for an earlier generation | Local evidence green; independent review pending |
| Exact observation remains ordinary O(1) owner lookup plus requested read work | hash-indexed branch/retention registries, owner-wide atomic registry gauge/work counters, and structurally distinct exact/historical visibility builders | `complexity_contract_visibility_scans_are_explicitly_measured`; exact index observation proof; registry lookup/cleanup proof at 1, 64, and 4,096 branches | one readmission performs one key lookup and zero registry mutations at every population; the public counter performs no branch-population scan; final lease drops remove every entry exactly once | resolve through history/current-global root, scan branch cells to report registry size, collapse exact into reconstruction, or retain dead weak-map entries | Corrected local evidence green; new independent review pending |

### Phase 6 implementation and verification packet

The current implementation evidence is:

- `cargo test -p worth-relational --lib` — **1,128 passed, 0 failed, 27
  ignored** on the final corrected source;
- `cargo test -p worth-relational --test relational_certification` — **140
  passed, 0 failed, 1 scheduled Scale test ignored** on the final corrected
  source;
- focused Phase 6 `basis_` certification — **10 passed, 0 failed**; exact
  index-observation proofs include same-branch and sibling reference movement;
  Bridge unit suite — **29 passed, 0 failed**; Foundational units — **22 passed,
  0 failed**;
- durable schema red controls — missing, tampered, swapped, and shared-carrier
  cases **4 passed**; branch-root content binding **7 passed**; hostile replay
  equivalence **passed**;
- branch-reference compile-time authority suite — **26/26 compile-fail cases
  passed**, including denial of snapshot-handle construction and
  deserialization;
- exact visibility complexity proof, all-target compilation, formatting,
  dirty Rust line cap, boundary topology, generated agent context, and diff
  whitespace integrity — **PASS**;
- `cargo clippy -p worth-relational --lib --no-deps -- -D warnings` — **PASS**.
  Warning-denied dependency-inclusive Clippy remains blocked by the inherited
  `worth-signal` advisory backlog (123 errors under the workspace lint policy);
  this does not widen Phase 6.

The critic correction pass sealed operational snapshot handles, made external
retention terminal accounting explicit, preserved merge observation denials,
split exact materialization from historical reconstruction, qualified index
proof by the admitted observation, replaced ordered registries with bounded
hash-indexed owner registries and exact drop cleanup, separated the allocation
oracle from production sharing observations, proved recovered old/new roots
execute their own schema contracts, split checkpoint recovery into fallible
prepare and infallible install phases, and decomposed the branch admission bag
into responsibility-named modules. The full unit lane found and corrected the
remaining genesis exact-root assertion before this packet was frozen.

The first frozen pre-review Rust packet was `HEAD
08a79b079499602a374a2c09986bbd50e62f1700`. It is generated by
`scripts/ci/fingerprint_dirty_rust.ps1`: repository-relative paths are slash
normalized and Unicode NFC normalized, ordered by their UTF-8 bytes, and
represented as `FILE<TAB>path<TAB>sha256(raw bytes)` or
`DELETE<TAB>path`. The manifest is UTF-8 without BOM, uses one LF between rows,
and has no trailing LF. Its SHA-256 is
`5b7bfb043ed9e164309ad67160928f585c888bd68f46308f7ef9f459e90c91b3`:
**519 entries** (**503 files**, **16 deletions**), **71,942 nonblank lines**,
maximum **400 physical lines** at `crates/worth-relational/src/branch/root.rs`.

### Phase 6 first Sol-high review findings and correction

The first three separate fresh Sol-high critics reviewed the `5b7bfb...`
packet and all returned supported reopening findings:

- qa-loop critic `phase6_qaloop_sol_fresh` found that bounded relation-join
  generation selection and verification mixed an exact snapshot with ambient
  commit-graph/current-state authority, and that the public registry-entry
  counter scanned the whole branch population without accounting for it;
- qa-tests critic `phase6_qatests_sol_fresh` found missing same-branch Bridge
  replacement, shared-commit collision, barrier-controlled release, and
  1/64/4,096 registry scale evidence; and
- code-quality critic `phase6_codequality_sol_fresh` found the exact Bridge
  adapter's commit-derived fallback, boolean/optional exact-versus-historical
  visibility assembly, mixed partition/schema recovery responsibility, the
  broad root-capture preparation function, and the expanded Bridge publication
  file responsibility/line-cap pressure.

The correction is causal and direct. Relation-join generation, candidate
verification, and certification parity now use the same exact snapshot root
and branch. Exact Bridge publication requires a retained owner observation;
historical commit-derived identity is a separately typed direct-publication
posture. Branch-head registrations use binding identities so delayed old
release cannot remove a replacement, while the commit index rejects two live
observations and resolves after one remains. Owner-wide atomic registry
metrics replace population scans and expose exact lookup/mutation work.
Visibility has distinct exact and historical builders/materializers, and an
exact basis structurally carries a root. Recovery separates schema-carrier
readmission from partition decoding. Root capture uses prepared region/schema
values, and Bridge publication outcomes and snapshot-basis selection have
separate named modules.

The post-correction evidence is:

- full Relational library — **1,127 passed, 0 failed, 27 ignored**;
- full `relational_certification` — **140 passed, 0 failed, 1 scheduled Scale
  ignored**;
- Bridge — **29 passed, 0 failed**; branch-reference UI — **26/26 compile-fail
  cases passed**;
- fixed registry work and exact cleanup at **1, 64, and 4,096 branches**;
  exact branch-scoped relation-join production/certification after main and
  sibling head movement; same-branch Bridge replacement, delayed old release,
  shared-commit collision/sole resolution, and barrier-ordered release —
  **PASS**;
- all-target compilation, production-library warning-denied Clippy,
  formatting, diff integrity, dirty Rust line-cap enforcement, boundary
  topology, and generated agent context — **PASS**;
- dirty Rust function scrutiny on the final corrected packet — **510 files**,
  **284 advisory candidates**, **0 scan errors**; the correction-introduced
  70-line registry scale helper was decomposed and no longer appears as an
  advisory.

The corrected Rust packet is `HEAD
08a79b079499602a374a2c09986bbd50e62f1700`; deterministic fingerprint
`892e5da396f24239257e6ac41272f2833a7d6ba962639bc91f3e12c0f6be6960`;
**530 entries** (**513 files**, **17 deletions**), **72,676 nonblank lines**,
maximum **399 physical lines** at
`crates/worth-relational/src/branch/reference.rs`.

### Phase 6 second Sol-high review findings and correction

The next three separate fresh Sol-high critics reviewed `892e5da...` and all
returned supported findings. The qa-loop critic found the same ambient-current
authority shortcut in bounded entity-field verification. The qa-tests critic
found that the relation-join movement proof changed only unrelated data and
therefore would not kill a current-state verification mutant. The code-quality
critic found that exact visibility cache identity was still represented as a
lane plus optional root, permitting an exact key without root identity.

The final correction makes visibility cache identity a variant-specific enum:
`Exact` requires a concrete owner root id and only `Historical` permits an
optional source root. Bounded entity-field verification and certification now
share one exact snapshot projection, with no ambient current-version or
current-state shortcut. The relation-join red control deletes a participating
relation from current main, rebuilds the current generation, proves the join is
absent there, and then proves the retained old snapshot still returns it in
Production and Certification. An equivalent entity-field main/sibling
divergence proof was added. A residue audit found and corrected the adjacent
bounded related-entity ordered lane so all bounded exact index verification and
parity use concrete exact projections. Its orchestration now carries a named
prepared lookup and typed verification contract rather than a broad argument
list.

Final local evidence on the new packet is:

- full Relational library — **1,128 passed, 0 failed, 27 ignored**;
- full `relational_certification` — **140 passed, 0 failed, 1 scheduled Scale
  ignored**;
- focused index family — **37 passed, 0 failed**; Bridge — **29 passed, 0
  failed**; branch-reference UI — **26/26 compile-fail cases passed**;
- all-target compilation, production-library warning-denied Clippy,
  formatting, diff integrity, dirty Rust line-cap enforcement, boundary
  topology, and generated agent context — **PASS**; and
- dirty Rust function scrutiny — **512 files**, **283 advisory candidates**,
  **0 scan errors**; the newly decomposed bounded entity-field and ordered
  lookup production paths contribute no advisories.

The final frozen Rust packet is `HEAD
08a79b079499602a374a2c09986bbd50e62f1700`; deterministic fingerprint
`56d1f05473e9de76874742872429c597a8f8f919f4b067f669dc4547d9c7af03`;
**532 entries** (**515 files**, **17 deletions**), **73,131 nonblank lines**,
maximum **399 physical lines** at
`crates/worth-relational/src/branch/reference.rs`.

The refreshed qa-loop/qa-tests/code-quality skills reference
`_docs/coding_guidelines/qa_review_guide.md`, which is absent from this
worktree; no substitute rules were invented.

Phase 6 remains **OPEN** pending a new trio of separate fresh Sol-high critics
for qa-loop, qa-tests, and code-quality-qa against the exact final packet above.
A separate fresh Sol-high final gate may run only after all three critics are
CLEAN; only then may this ledger close Phase 6 and permit Phase 7 planning.
