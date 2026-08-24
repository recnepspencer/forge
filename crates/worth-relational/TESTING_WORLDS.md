# Relational certification worlds

Milestone 9.17.1 uses **Supply Chain** as its canonical semantic world. This
document describes the Phase 2 pure world/oracle, the Phase 3 causal
production adapter, and the landed Phase 4-6 branch root and exact-observation
slices. It is deliberately not a complete branch-local MVCC guide: detached
transactions, candidate preparation, publication, lifecycle reclamation, and
production history/replay certification remain later phases.

## Authority split

The world definition, semantic keys, baselines, scenario deltas, oracle, and
expected observations contain descriptive meaning only. They do not contain a
Relational runtime, commit/version/snapshot IDs, branch heads, roots, indexes,
leases, pins, authority witnesses, or private constructors. Semantic names
such as `world.ports.southpoint` are not runtime handles and are never converted
to guessed integers.

Phase 3 compiles the same immutable definition through public Relational
schema and transaction facades and binds those names to owner-issued handles.
Its observed projection is a separate type and code path from the pure
oracle. No Phase 2 test may import a production DTO, facade, canonicalizer,
query projection, digest implementation, or lowering helper.

## Phase 3 causal baseline contract

The Phase 3 production slice adds only the adapter cone needed to certify an
installed baseline. `program.rs` owns the immutable semantic-to-public schema
and seed intent; `compiler.rs` creates a fresh runtime, installs the schema
through the initial-installation authority, stages bulk entity/relation intents,
commits through the public transaction facade, and obtains an owner-issued
snapshot; `handles.rs` binds semantic names through sealed commit-result
correspondences; normal, relation-aspect, and bulk relation creation all feed
that same owner correspondence; `observation.rs` reads the public snapshot view and maps its
records back through those bindings; and `baseline_audit.rs` compares that
projection with the independent expected observation. The pure definition,
oracle, and comparator remain outside this adapter cone.

Entity and relation handles are distinct typed bindings. An entity binding is
accepted only from the owner's created-entity correspondence. A relation
binding requires the corresponding owner-issued relation result; changed-record
order, endpoint matching, query lookup, allocator arithmetic, and guessed raw
IDs are not valid substitutes. The production result API must reject missing,
duplicate, ambiguous, wrong-kind, and foreign-runtime bindings before a
`CertifiedSupplyChainBaseline` can be constructed.

The deterministic `1000`/`2000` kind ranges in the schema vocabulary are
declaration keys for the Supply Chain schema only. They are never used to
derive an entity or relation identity; all record identities still come from
the owner's sealed commit correspondences.

Phase 3 certifies empty and operating Court/Standard baselines. The returned
baseline may carry a descriptive branch envelope for traceability and an
owner-issued snapshot, but that envelope is not an admitted branch basis and
cannot authorize a transaction. The Phase 4 fork-only source token and exact
fork provenance are now exercised by the production Supply Chain target; the
Phase 3 descriptive envelope remains non-operational. General branch-basis
admission/readmission and visible-root currentness are owner-issued Phase-6
operations: `observe_branch` returns a non-operational serializable descriptor
and an admitted basis, while reads consume only its
`RelationalBranchObservation`. Transported descriptors must pass owner
`readmit_branch_basis`, and external retention is represented by an explicit
lease. Phase 4 certifies runtime-clone
affinity, singular immutable catalog identity, exact source/target fork
provenance, typed empty/duplicate/stale/foreign denials, and constant
*logical* metadata-only fork deltas at 1/64/512 fan-outs. The probe does not
yet claim physical COW bytes or all population-scan absence; those need the
later currentness and cost gate. Scale remains a semantic/cost lane until its
production installation cost is separately bounded. The production compiler
uses the public
`PublicationConfig` override of 16,384 patch records
for these causal baselines; the Standard seed emits 8,211 records, so this is
an explicit fixture budget rather than a hidden split commit or validation
bypass. The empty declaration also executes a required public no-op commit
(zero patch records) so its owner-issued commit snapshot is observed through
the same public read-view path; there is no optional-commit or hand-built empty
observation fallback.

The adapter reports declaration, schema-installation, transaction, entity
binding, relation binding, observation, oracle, and comparison failures as
distinct typed stages. A failed construction or audit is fixture evidence, not
an MVCC product outcome. The required causal cases are
`supply_chain_world_compiles_causally_through_public_facades`,
`supply_chain_named_handles_are_owner_issued_and_complete`, and
`supply_chain_baseline_matches_independent_oracle`, plus one independent
mutation for every failure stage (including budget denial, wrong-kind,
wrong-endpoint, incomplete/duplicate correspondence, foreign or missing
snapshots, and unknown production identities) and the existing Fintech/generic
preservation suite. Private runtime access, production-derived expected state,
and imports from the pure oracle remain forbidden.

## Semantic world

The world has eight typed entity kinds: `Port`, `Terminal`, `Berth`, `Vessel`,
`Voyage`, `PortCall`, `CargoLot`, and `Inspection`. Their fields have explicit
units and meanings: capacities are units, berth depth is metres, cargo mass is
metric tons, and departure/arrival/inspection times are minutes from the
deterministic seed epoch. Status, posture, booking, inspection, vessel class,
region, and hazard values are closed enums. Hazard schema V1 and V2 are
different schema/meaning identities, not a renamed field.

The ten relation contracts are `TerminalAtPort`, `BerthAtTerminal`,
`VesselAssignedToBerth`, `VoyageUsesVessel`, `VoyageHasCall`, `CallAtPort`,
`CallPrecedes`, `CargoBookedOnVoyage`, `InspectionCoversVessel`, and
`SharesPilotageZone`. `CargoLot` also carries a deterministic,
customer-neutral synthetic code; that code is semantic data, not a customer or
runtime handle. Contracts pin endpoint kinds, minimum and maximum
cardinality, uniqueness, symmetry, ordered route edges, cross-partition
legality, and route acyclicity. Complete-world validation enforces minimum
cardinality; partial observation validation intentionally does not. Symmetric
pilotage edges are stored in both directions, `VoyageHasCall` follows
port-call sequence, and `CallPrecedes` links consecutive calls in one voyage.
Complete validation also rejects duplicate call ownership, orphan or missing
predecessor links, sequence gaps, and V1 cargo carrying the V2-only hazardous
meaning. Cross-region routes remain legal when their endpoint contracts are
valid. The semantic field vocabulary is exhaustive: port codes, vessel call
signs, inspection minutes, cargo customer codes, and schema meaning each have
distinct keys rather than aliases. Relation identity is a typed key independent
of its endpoints, so port-call rewiring preserves call identity.

The semantic owners are physically separated: `definition_entities.rs` and
`definition_relations.rs` declare the two topology families;
`scenario_delta_vocabulary.rs` owns the shared delta/precondition vocabulary;
and `comparison_state.rs` plus named ancestry/entity/schema/relation comparison
modules feed a small comparison orchestrator. These boundaries prevent read,
delta, and comparison modules from becoming mutual dependency authorities.

## Profiles and baselines

`SupplyChainScale` has three semantic profiles with the same meaning:

| Profile | Regions | Ports | Terminals | Berths | Vessels | Voyages | Calls | Cargo |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Court | 2 | 4 | 8 | 16 | 12 | 16 | 48 | 128 |
| Standard | 4 | 16 | 32 | 64 | 64 | 128 | 384 | 4,096 |
| Scale | 8 | 64 | 128 | 256 | 256 | 512 | 1,536 | 65,536 |

The complete-world totals are 244 entities/247 relations for Court, 4,848/
3,363 for Standard, and 68,544/38,019 for Scale. Every entity and relation
kind is checked against its profile formula, every port is checked against its
seeded region, and repeated construction with the same profile/seed is equal.

Named Court anchors remain present in every profile. Generated members use a
seed-derived name/value mix and represent every declared region (including
generated regions beyond North/South); they do not alter the Court facts.

The immutable baseline dependency graph is:

```text
EmptyInstallation -> Operating -> ContestedPlanning -> RetentionPressure
                  \-> VersionBoundary
```

- `EmptyInstallation` contains only profile/schema declarations and no
  records.
- `Operating` contains the complete accepted topology and baseline oracle
  state.
- `ContestedPlanning` adds validated Storm, Maintenance, Customs, and Rewire
  branch-creation intents without applying a branch delta or reference movement.
- `RetentionPressure` adds semantic descriptions of snapshot, observation,
  transaction, candidate, and external-basis obligations at named ancestors;
  each target must be a declared branch, every path is rooted at Operating,
  and every adjacent ancestor pair must be a declared branch-creation intent.
  These are not runtime pins or leases.
- `VersionBoundary` derives directly from Operating and names the pre-upgrade
  and post-upgrade hazardous-cargo schema descriptors. It does not depend on
  ContestedPlanning.

## Scenario deltas

The reusable typed `SupplyChainScenarioDelta` vocabulary contains exactly:

1. Storm Reroute Aurora;
2. Maintain Atlas Berth;
3. Hold Medical Cargo;
4. Expand Southpoint Capacity;
5. Competing Aurora Arrival;
6. Retire Atlas While Inspecting Aurora;
7. Rewire Aurora Port Call; and
8. Adopt Hazard Classification V2.

Each delta declares typed preconditions, a dedicated entity/relation/field/
schema/branch/history read footprint, exact entity/relation/field write
footprint, identity basis, invariant posture, and schema meaning. Target and
current relation endpoints are explicit read entities; unrelated source facts
are not smuggled into the footprint. Every V1 delta requires the V1 schema and rejects
reapplication on the same branch. The vocabulary is not a generic operation
bag and does not classify merges. A failed application returns a typed error
and leaves its input branch unchanged.

## Independent oracle and observations

`OracleState` is an ordered semantic map/set model of facts, relations,
absence, and schema identity. `OracleAncestry` separately owns recursive semantic
branch lineage, common-ancestor calculation, and accepted-delta order;
`OracleBranch` composes the two. Accepted history events retain their branch
owner and flattened order across forks. An explicit ordered-history fixture records
multiple accepted delta identities without pretending those branch-specific
domain mutations were successfully replayed on one branch. The oracle
does not use Relational MVCC, roots, indexes, queries, history classifiers,
production encoders, production digests, or latest-head selection.

`ExpectedSupplyChainObservation` is distinct from oracle state and the
`ObservedSupplyChainState` comparator carrier. It carries entity/relation
absence and full ancestry, while the observed carrier also preserves a raw
relation vector so duplicate-edge mutations cannot disappear into a map. The
comparator proves vector/map key and endpoint parity, rejects vector-only
omissions, replacements, and extras, and still accepts insertion permutations.
Its canonical bytes and digest are versioned and independently implemented.
Hand-authored vectors pin the Court baseline and every delta class, including
unchanged facts, explicit absence, ordered calls, relation sources and
targets, schema identity, and accepted-delta ancestry. Digest equality is only
a report; the comparator checks semantic fields and paths. The pure observed
carrier is structural evidence; the Phase 3 adapter separately certifies
causal production observation through the public snapshot/read-view facades.

Comparison failures distinguish missing writes, entity/relation absence,
sibling fact leaks, floating/latest-head selection, wrong parent and accepted
history, relation source/target changes, duplicate relations, illegal
endpoints, schema meaning drift, and unexpected facts. A wrong-ancestry probe
keeps domain facts equal and changes only lineage.

## Reproduction, mutation, and cost lanes

Every semantic trace records format version, profile, seed, baseline, branch,
ordered delta IDs, a typed mutation ID/operation, canonical pre-mutation input,
the raw duplicate-preserving relation-vector bytes, and the first typed
divergence. `SemanticTrace::replay` verifies the profile
format version and profile seed, reconstructs the baseline, applies the ordered deltas, executes the
typed mutation, reruns comparison, and returns the actual canonical mutated
input and divergence. The returned trace is itself replayable without replacing
its recorded pre-mutation input. A caller-supplied false first divergence is
rejected, typed delta-application errors retain their step and source, and
repeated replay is deterministic across Court, Standard, Scale, and the
non-empty named baselines. The replay fingerprint is supplemental evidence and
does not replace execution. Runtime pause schedules and production failure
traces are deferred to later phases.

The Court ordinary lane permits one immutable definition, eight accepted delta
steps, 128 trace steps, 512 observations, 128 cargo lots, and 512 setup
entities/relations. Standard constructs its 4,848-entity/3,363-relation
definition with 8,192 observations and setup entities, 4,096 setup relations,
4,096 cargo lots, and 1,024-step oracle/trace ceilings. Scale is scheduled and
constructs its exact density under a 65,536-cargo, 70,000-entity, and 40,000-
relation ceiling. Every one of the seven dimensions is present in a typed,
machine-readable report bound to the profile and deterministic seed, and has an
independent over-budget denial; these are deterministic semantic budgets, not
timeout-only assertions.

The mutation matrix includes missing write, explicit entity/relation absence,
entity field value, relation source/target, sibling fact, floating branch,
wrong parent/history, duplicate relation, illegal endpoint, schema drift, and
deletion markers. Each has a valid positive state and a one-axis negative twin;
the six named trace controls are executable and the remaining controls are
covered by direct comparator/application courts. Production mutations such as
eager fork cloning, latest-root reads, global publication locks, and partial
root swaps are later MVCC release-court obligations, not Phase 2 claims.

## Phase 5 ordinary and scheduled evidence commands

The ordinary Phase 5 certification command is:

```text
cargo test -p worth-relational --test relational_certification --no-fail-fast
```

The complete Scale admission court and maximum 4,096-fork slope are scheduled
tests so the ordinary loop stays responsive. Run them explicitly with:

```text
cargo test -p worth-relational --test relational_certification \
  scale_invariant_admission::large_runtime_keeps_global_enforcement_and_filters_graph_planning \
  -- --ignored --exact --nocapture --test-threads=1

cargo test -p worth-relational --test relational_certification \
  root::sharing::fork::phase5_standard_fork_copy_slope_is_flat_through_4096_forks \
  -- --ignored --exact --nocapture --test-threads=1
```

Scale is scheduled because its production installation is the complete
106,563-record causal world, not a reduced substitute. The scheduled test
retains the independent definition/live-snapshot count, Global commit and
baseline-publication ceilings, direct GraphComposition `Touched` result and
one-call counters, ordinary publication lowering, ordinary graph exclusion,
and duplicate-rejection behavior.

## Phase 2 and Phase 3 evidence commands

The shared target is `cargo test -p worth-relational --test
relational_certification --no-fail-fast`; the Phase 3 baseline closed with 77
tests, while the current target (including the Phase 4 fork/currentness slice)
passes 91 tests. The focused owner-correspondence unit target passes four
tests (normal, relation-aspect, bulk, and missing-endpoint denial). The
owner-correspondence unit target
`transactions::data::outcomes::created_relation_bindings` passes one test, and
the Fintech/CAD/Chip preservation target `cargo test -p worth-relational --lib
tests::domains --no-fail-fast` passes 22 tests. The phases also require
formatting, the scoped whole-subtree source/dependency fence,
boundary-check, generated agent-context validation, and dirty Rust line-cap
checks. Strict Relational Clippy is a required command; existing unrelated
repository Clippy debt is reported separately and cannot be cited as
semantic-world evidence. Phase 3 additionally runs the branch-reference
contract/compile-fail suites and receives independent review of the final diff
and direct results.

## Phase 4 currentness and compatibility-court requirements

The canonical Phase-4 court remains Supply Chain; no phase-specific substitute
is permitted. The court must exercise a real owner-admitted branch basis and
branch-bound transaction, then compare production observations with the pure
Supply Chain oracle. Its fork/currentness matrix
must include:

- one shared immutable catalog artifact with distinct runtime-affine target
  branch cells;
- source advancement that cannot alter a forked target, and unrelated branch
  progress that cannot stale another binding;
- stale generation, stale truth version, foreign-runtime identity, duplicate
  target, empty source, malformed target, exact-parent drift, and missing-local
  basis denials with zero branch/catalog residue;
- metadata-only movement that increments generation without changing local
  truth, and truth movement that increments the local truth version; recovery
  must restore the exact cell or fail closed;
- merge-parent order and replay continuity proofs that use owner bindings and
  canonical artifacts, not raw branch ids or diagnostic heads; and
- setup-separated 1/64/512 fan-out counters for branch-cell lookup, catalog
  lookup, artifact construction, reference allocation, and cell contact. The
  per-fork deltas must be constant and no population scan may occur in the
  operation path. These are logical metadata-cost claims; physical COW bytes
  remain a later release-court obligation.

The independent oracle is mandatory for every positive and negative semantic
case. Production-derived expected state, catalog-latest reads, branch-head
maps, and guessed ids are not valid oracles. Every failure must be typed and
must compare branch-cell checkpoints, catalog length, and artifact identity
before and after the attempted operation.

The Phase-4 target does not call the then-existing historical-read, Bridge,
application-commit, or replay compatibility surfaces. Direct behavior and
compiler-boundary tests keep those surfaces outside the transaction path. The
application-commit compatibility case additionally confirms that exact lease
admission does not move branch currentness. Phase 6 has now removed the
consumer-facing current/latest adapters: snapshot, history, merge-basis, and
Bridge reads require an admitted exact observation, while replay remains a
separate cert-only lane.

The current Phase-4 evidence commands are:

```text
cargo test -p worth-relational --lib
cargo test -p worth-relational --test relational_certification --no-fail-fast
cargo test -p worth-relational --lib merge_replay_continuity
cargo test -p worth-relational --test branch_reference_contract --quiet
cargo test -p worth-relational --test phase_boundaries_compile_fail --quiet
cargo test -p worth-relational --test ui --quiet
cargo clippy -p worth-relational --lib --no-deps -- -D warnings
```

The repository-wide Clippy command still reports known unrelated
`worth-signal` production warnings; those are recorded as debt and do not
replace the Relational `--no-deps` gate above.
