# Milestone 9.17.1 Closure Ledger

This ledger is the durable handoff for the phase gates in
`milestone-9.17.1.md`. A phase remains open until its row has production
evidence, focused proof, an independent Luna review, and a final Sol review.
Evidence is scoped to the implementation surface; a green unrelated suite does
not close a row.

## Phase status

| Phase | Scope | Status | Required next gate |
| --- | --- | --- | --- |
| 1 | Foundational exact branch-reference grammar and candidate-vocabulary migration | Closed — Luna and Sol certified | Phase 2 plan and two plan critics |
| 2 | Supply Chain semantic world and independent oracle | Closed — Luna and Sol certified | Phase 3 plan and two plan critics |
| 3 | Production-backed Supply Chain compiler and baseline audit | Closed — Luna and Sol certified | Phase 4 plan and two plan critics |
| 4 | Relational immutable commit/reference split and branch-local MVCC foundation | Closed — independent qa-loop, qa-tests, and code-quality-qa certified `7cbeb3a8645809890143b28117b5e6fc87aeb3cc` | Hand off to Phase 5/6 without widening the compatibility inventory |

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
