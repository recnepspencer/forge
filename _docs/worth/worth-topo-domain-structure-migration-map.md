# Worth Topology Domain Structure Migration Map

> **Status:** Closed; topology-domain skeleton landed and mechanically enforced
>
> **Parent gate:** [worth-topo-domain-structure-gate.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-gate.md)
>
> **Closeout:** [worth-topo-domain-structure-closeout.md](/Users/Esther/Documents/Programming/forge_workspace/worktree_2/_docs/worth/worth-topo-domain-structure-closeout.md)
>
> **Scope:** `crates/worth-topo`

## Coverage Rule

Rows ending in `/**` classify every current file underneath that path unless a
more specific exception row appears later in the table.

This map is proof-carrying: every row states the current role, target
responsibility class, public API posture, migration kind, affected tests, main
risk, and any owner decision needed before movement.

Allowed `Responsibility class` values are:

- authoritative topology truth
- derived topology
- validation
- topology operators
- projection
- certification
- test support
- public facade

Allowed `Move type` values are:

- `move_only`
- `split`
- `merge`
- `delete`
- `public_contract_preserve`
- `public_contract_break`

## Closeout Status Discipline

The table above remains the proof-carrying lowering map. Closeout status is
tracked below so the map does not pretend classification alone equals closure.

Allowed `Closeout status` values are:

- `landed_enforced`: the move or deletion has landed and is protected by a
  structural guard, compile-fail contract, line-cap guard, or focused test.
- `landed_manual`: the move has landed, but enforcement is still human-reviewed
  through this map and surrounding tests.
- `pending`: the row remains intentionally unfinished.
- `blocked_owner_decision`: the row needs an owner decision before movement.

## Migration Map

| Current path | Current role | Target path | Responsibility class | Public API impact | Move type | Tests affected | Risk | Owner decision |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `crates/worth-topo/Cargo.toml` | Crate metadata and test harness wiring | `crates/worth-topo/Cargo.toml` | public facade | Preserve crate identity and test declarations | `move_only` | All crate tests | Accidental test target loss during file moves | None |
| `crates/worth-topo/src/lib.rs` | Public module root | `crates/worth-topo/src/lib.rs` | public facade | Preserve public facade exports from permanent target paths | `public_contract_preserve` | Public API and compile-fail tests | Deep modules may leak during re-export repair | None |
| `crates/worth-topo/src/facade.rs` | Public topology facade | `crates/worth-topo/src/facade.rs` | public facade | Preserve public semantics unless explicitly approved | `public_contract_preserve` | `tests/public_api_contract.rs`, compile-fail tests | Facade may mirror old internal topology instead of owning final public contracts | None |
| `crates/worth-topo/src/read_stage.rs` | Read-stage proof helper | `crates/worth-topo/src/projection/runtime_boundary/read_stage.rs` | projection | Internal shape only | `move_only` | Read-stage compile-fail and runtime proof tests | Projection could absorb interpretation if not kept thin | None |
| `crates/worth-topo/src/data/topology_view/**` | Topology entity and relation view records | `crates/worth-topo/src/brep/topology_graph/**` | authoritative topology truth | Internal shape only | `split` | Materialization, validators, certification closeout | Some files may be derived views rather than truth vocabulary | Confirm per-entity target names during move |
| `crates/worth-topo/src/data/mod.rs` | Old data module root | Delete after `brep` rehome | authoritative topology truth | Internal shape only | `delete` | Module compilation | Root module can preserve the old story if left behind | Delete once imports are retargeted; do not keep a duplicate root |
| `crates/worth-topo/src/materialization/**` | Builds materialized topology view from truth rows | `crates/worth-topo/src/derived_topology/materialized_graph/**` | derived topology | Internal shape only | `move_only` | Materialization tests, Milestone 2 closeout | Relation wiring may reveal authoritative vocabulary mixed in | Split authoritative vocabulary back to `brep` if found |
| `crates/worth-topo/src/interpretation/boundary/**` | Boundary interpretation over materialized topology | `crates/worth-topo/src/derived_topology/traversal_views/boundary/**` | derived topology | Internal shape only | `move_only` | Interpretation tests, derived validation | Boundary logic may hide validation decisions | None |
| `crates/worth-topo/src/interpretation/radial/**` | Radial interpretation over topology view | `crates/worth-topo/src/derived_topology/radial_rings/**` | derived topology | Internal shape only | `move_only` | Radial interpretation and NMT certification | Radial validation may be mixed with interpretation | Split validators to `validation/radial_rings` |
| `crates/worth-topo/src/interpretation/shells/**` | Shell interpretation over topology view | `crates/worth-topo/src/derived_topology/shell_views/**` | derived topology | Internal shape only | `move_only` | Shell interpretation and certification | Shell closure checks may be mixed with view building | Split validators to `validation/shell_closure` |
| `crates/worth-topo/src/interpretation/vertex_branching/**` | Vertex branching interpretation | `crates/worth-topo/src/derived_topology/vertex_disks/**` | derived topology | Internal shape only | `move_only` | Vertex branching and NMT tests | Vertex-disk topology may be undernamed | Confirm final vertex-disk vocabulary |
| `crates/worth-topo/src/interpretation/wires/**` | Wire interpretation over topology view | `crates/worth-topo/src/derived_topology/wire_views/**` | derived topology | Internal shape only | `move_only` | Wire interpretation and certification | Wire validation may be mixed with interpretation | Split validators if found |
| `crates/worth-topo/src/interpretation/types.rs` | Derived interpretation types | `crates/worth-topo/src/derived_topology/traversal_views/types.rs` | derived topology | Internal shape only | `split` | Interpretation and certification | Type names may still be too generic | Split by view family if needed |
| `crates/worth-topo/src/interpretation/facade.rs` | Derived interpretation facade | `crates/worth-topo/src/derived_topology/facade.rs` | derived topology | Internal shape only | `move_only` | Interpretation callers | May expose too much internal view topology | None |
| `crates/worth-topo/src/interpretation/mod.rs` | Old interpretation module root | `crates/worth-topo/src/derived_topology/mod.rs` | derived topology | Internal shape only | `move_only` | Module compilation | Root name changes can cascade broadly | None |
| `crates/worth-topo/src/interpretation/tests.rs` | Broad derived topology tests | `crates/worth-topo/src/certification/derived_topology_closeout/interpretation_coverage.rs` | certification | Internal test shape only | `split` | Derived topology tests | 939-line god test can preserve fog if moved intact | Split by shell/wire/radial/vertex responsibility |
| `crates/worth-topo/src/validators/**` | Domain validator families and validator tests | `crates/worth-topo/src/validation/**` | validation | Internal shape only | `split` | Validator tests, certification closeouts | `validators/tests.rs` is a 1476-line proof sink | Split tests into certification/validation responsibility suites |
| `crates/worth-topo/src/runtime_invariants/**` | Runtime-facing invariant families parallel to validators | `crates/worth-topo/src/validation/**` | validation | Internal shape only | `merge` | Runtime invariant tests, closeout suites | Duplicate validator/runtime invariant concepts may drift | Merge only where failure topology is identical |
| `crates/worth-topo/src/diagnostics/**` | Diagnostic vocabulary | `crates/worth-topo/src/projection/diagnostic_surfaces/**` | projection | Internal shape only | `move_only` | Diagnostic assertions in certification | Diagnostics could begin deciding domain meaning | Keep as presentation of produced evidence only |
| `crates/worth-topo/src/bridge/**` | Runtime bridge mapping and tests | `crates/worth-topo/src/projection/runtime_boundary/bridge/**` | projection | Internal shape only | `move_only` | Bridge tests and Milestone 1/2 closeouts | Bridge code could own causality instead of expose it | None |
| `crates/worth-topo/src/query/assembly/**` | Query row assembly and authority/snapshot decoding | `crates/worth-topo/src/projection/runtime_boundary/query_assembly/**` | projection | Internal shape only | `split` | Query assembly tests, read-composition closeout | Assembly may contain truth vocabulary or row helpers that should die | Mark obsolete row helpers as `delete` in implementation |
| `crates/worth-topo/src/query/domain/views/models.rs` | Decoded topology domain read-view data models | `crates/worth-topo/src/projection/read_views/models.rs` | projection | Preserve public view semantics through facade | `move_only` | Domain read parity and compile-fail tests | View models could become authority-shaped if fields open | Keep fields sealed and accessors read-only |
| `crates/worth-topo/src/query/domain/views/surface.rs` | Read-only accessors for decoded topology domain read views | `crates/worth-topo/src/projection/read_views/surface.rs` | projection | Preserve public view semantics through facade | `move_only` | Domain read parity and compile-fail tests | Accessors could infer missing domain meaning | Accessors only expose already-decoded evidence |
| `crates/worth-topo/src/query/domain/views/adjacency.rs` | Query-backed adjacency/radial read decoding | `crates/worth-topo/src/projection/runtime_boundary/read_decoding/adjacency.rs` | projection | Preserve public view semantics through facade | `split` | Domain read parity and runtime tests | Decoding depends on private Query execution internals | Move after runtime boundary exists; do not put interpretation in `read_views` |
| `crates/worth-topo/src/query/domain/views/local_rewire.rs` | Query-backed local-rewire read decoding | `crates/worth-topo/src/projection/runtime_boundary/read_decoding/local_rewire.rs` | projection | Preserve public view semantics through facade | `split` | Domain read parity and runtime tests | Decoding depends on private Query execution internals | Move after runtime boundary exists; do not put interpretation in `read_views` |
| `crates/worth-topo/src/query/domain/views/loop_cycle.rs` | Query-backed loop-cycle read decoding | `crates/worth-topo/src/projection/runtime_boundary/read_decoding/loop_cycle.rs` | projection | Preserve public view semantics through facade | `split` | Domain read parity and runtime tests | Decoding depends on private Query execution internals | Move after runtime boundary exists; do not put interpretation in `read_views` |
| `crates/worth-topo/src/query/domain/execution/**` | Domain read-family execution and basis contexts | `crates/worth-topo/src/projection/runtime_boundary/read_execution/**` | projection | Internal shape only | `move_only` | Domain query execution and historical basis tests | Basis context could become authority-shaped | None |
| `crates/worth-topo/src/query/domain/lowering/**` | Domain request lowering to Query read graph | `crates/worth-topo/src/projection/runtime_boundary/read_lowering/**` | projection | Internal shape only | `move_only` | Domain query lowering tests | Lowering may hide fallback policy | Keep fallback evidence in diagnostic surfaces |
| `crates/worth-topo/src/query/domain/proof/**` | Domain read proof, parity, fallback, closeout reports | `crates/worth-topo/src/projection/diagnostic_surfaces/read_proof/**` | projection | Preserve proof rows through facade | `move_only` | No-N+1 and domain read closeout tests | Proof rows could drift from certification aggregation | Certification must consume, not house, these rows |
| `crates/worth-topo/src/query/domain/*.rs` | Domain read request, topology, and errors | `crates/worth-topo/src/projection/read_views/` and `projection/runtime_boundary/` | projection | Preserve public domain read facade | `split` | Domain read docs and compile-fail tests | Request types may belong at facade boundary | Confirm per file during move |
| `crates/worth-topo/src/query/runtime/**` | Query runtime support, adapters, support posture, and tests | `crates/worth-topo/src/projection/runtime_boundary/query_runtime/**` | projection | Preserve runtime support facade | `split` | Runtime support docs, runtime tests, compile-fail tests | Runtime boundary could become a junk drawer | Split diagnostic proof out of runtime boundary |
| `crates/worth-topo/src/query/tests/**` | Query/domain read tests | `crates/worth-topo/src/certification/projection_closeout/**` | certification | Internal test shape only | `split` | Query/domain read tests | Tests may preserve query-shaped names | Rename by projected surface/proof responsibility |
| `crates/worth-topo/src/query/*.rs` | Root query helpers, row lookup, naming, diagnostics, derived views | `crates/worth-topo/src/projection/**` | projection | Internal shape only, facade preserved | `split` | Domain read and runtime tests | Root `query` can survive as renamed junk drawer if split is shallow | Delete root `query` after submodules move |
| `crates/worth-topo/src/topology_operators/types/**` | Operator contracts, constructors, naming, vocabulary | `crates/worth-topo/src/topology_operators/contracts/**` | topology operators | Preserve edit facade semantics | `move_only` | Operator contract tests and public facade tests | Constructors file is near line cap and may hide phases | Split constructors by contract family |
| `crates/worth-topo/src/topology_operators/facade.rs` | Operator public contract builders | `crates/worth-topo/src/topology_operators/facade.rs` | topology operators | Preserve final public operator semantics | `public_contract_preserve` | Operator public contract tests | Public naming may still expose old edit shape | None |
| `crates/worth-topo/src/topology_operators/proof.rs` | Operator proof types | `crates/worth-topo/src/topology_operators/rejection_locality/proof.rs` and `topology_operators/replay/proof.rs` | topology operators | Preserve facade semantics | `split` | Operator proof and Milestone 3 certification | Proof type ownership may be ambiguous | Split by proof responsibility |
| `crates/worth-topo/src/topology_operators/contract_tests.rs` | Operator contract tests | `crates/worth-topo/src/certification/topology_operator_closeout/contract_tests.rs` | certification | Internal test shape only | `move_only` | Operator contract tests | Tests could stay too close to implementation helpers | None |
| `crates/worth-topo/src/topology_operators/mod.rs` | Topology operator module root | `crates/worth-topo/src/topology_operators/mod.rs` | topology operators | Retarget facade directly to permanent operator exports | `public_contract_preserve` | Module compilation | Root can become too broad if application/contracts/proof are not split | Continue family split; do not recreate `edit` root |
| `crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/membership_programs/**` and `local_rewrites/boundary_wiring/composed_successor_program.rs` | Landed composed membership/successor operator execution, formerly hidden behind graph-named application artifacts | Same as current path | topology operators | Internal shape only | `split` | Operator execution and relation update tests | Program orchestration can still hide operator-family responsibilities if it grows | Landed: keep composed programs inside the owning local rewrite family; do not recreate graph-named application modules |
| `crates/worth-topo/src/topology_operators/application/relation_successor*` | Successor relation operator execution and support | `crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/**` | topology operators | Internal shape only | `split` | Relation update successor tests | Support and execution may be mixed | Separate contracts/support/application |
| `crates/worth-topo/src/topology_operators/application/relation_wire_rehome*` | Wire membership operator execution and support | `crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/wire_rehome/**` | topology operators | Internal shape only | `split` | Wire operator execution tests | Membership semantics may be too operation-specific | None |
| `crates/worth-topo/src/topology_operators/application/relation_shell*` | Shell membership and shell-face operator execution | `crates/worth-topo/src/topology_operators/local_rewrites/sheet_wire_laminar/shell_membership/**` | topology operators | Internal shape only | `split` | Shell operator execution tests | Shell split may belong to a separate operator family | Confirm before move |
| `crates/worth-topo/src/topology_operators/application/relation_boundary.rs` | Boundary relation operator execution | `crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/relation_boundary.rs` | topology operators | Internal shape only | `move_only` | Relation update tests | None | None |
| `crates/worth-topo/src/topology_operators/application/relation_create.rs` | Relation creation operator execution | `crates/worth-topo/src/topology_operators/local_rewrites/entity_lifecycle/relation_create.rs` | topology operators | Internal shape only | `move_only` | Relation create tests | Creation may span lifecycle and membership | Confirm exact family |
| `crates/worth-topo/src/topology_operators/application/relation_update.rs` | Relation update orchestration | `crates/worth-topo/src/topology_operators/local_rewrites/boundary_wiring/relation_update.rs` | topology operators | Internal shape only | `split` | Relation update tests | Landed under boundary-wiring operator ownership; future radial or vertex-disk expansion must earn its own local rewrite family instead of growing this file | Closed: no application-root relation-update shim remains |
| `crates/worth-topo/src/topology_operators/application/*.rs` | Operator admission, bindings, existing truth, errors | `crates/worth-topo/src/topology_operators/application/` and `topology_operators/contracts/` | topology operators | Internal shape only | `split` | Operator execution tests | Application can become a tool bucket if family split stalls | Continue splitting by admitted operator family |
| `crates/worth-topo/src/fixtures/**` | Shared authored/derived/validated topology fixtures | `crates/worth-topo/src/test_support/**` | test support | Internal test shape only | `split` | Certification, validators, interpretation tests | Fixture convenience may hide responsibility | Split into brep builders, primitive corpus, branch histories |
| `crates/worth-topo/src/parity/**` | Parity proof helpers | `crates/worth-topo/src/certification/support/parity/**` | certification | Internal shape only | `move_only` | Parity tests and closeouts | Parity helper may be too generic | Keep under certification support only |
| `crates/worth-topo/src/certification/closeout/**` | Closeout aggregate helpers | `crates/worth-topo/src/certification/support/closeout_aggregation/**` | certification | Preserve closeout report semantics | `move_only` | Closeout tests | Support folder may become helper bucket | Keep aggregation-specific |
| `crates/worth-topo/src/certification/corpus/**` | Certification primitive corpus | `crates/worth-topo/src/certification/primitive_corpus/**` | certification | Preserve corpus semantics | `move_only` | Primitive corpus tests | Corpus and test support builders may blur | Keep proof corpus distinct from builders |
| `crates/worth-topo/src/certification/milestone_three/**` | Milestone 3 topology-operator hostile and closeout proof | `crates/worth-topo/src/certification/topology_operator_closeout/` and `certification/hostile_topology_operators/` | certification | Preserve final closeout report semantics through permanent facade exports | `split` | Milestone 3 hostile tests, closeout tests | Provenance names may survive in internals | Rename internal folders in place; do not create old-name exports |
| `crates/worth-topo/src/certification/milestone_two.rs` | Milestone 2 derived closeout proof | `crates/worth-topo/src/certification/derived_topology_closeout/mod.rs` | certification | Preserve report semantics | `split` | Milestone 2 closeout tests | 1157-line file hides closeout families | Split by report family before closeout |
| `crates/worth-topo/src/certification/read_view.rs` | Read-view certification proof | `crates/worth-topo/src/certification/derived_topology_closeout/read_views.rs` and `projection_closeout/read_views.rs` | certification | Preserve report semantics | `split` | Read-view certification tests | Derived and projection proof may be collapsed | Split by proof target |
| `crates/worth-topo/src/certification/report.rs` | Certification report aggregation | `crates/worth-topo/src/certification/support/reporting/**` | certification | Preserve report semantics | `split` | Closeout and report tests | 603-line aggregation sink | Split by report responsibility |
| `crates/worth-topo/src/certification/bridge.rs` | Bridge proof certification | `crates/worth-topo/src/certification/projection_closeout/bridge.rs` | certification | Preserve report semantics | `move_only` | Bridge proof tests | None | None |
| `crates/worth-topo/src/certification/rejections.rs` | Rejection proof surfaces | `crates/worth-topo/src/certification/support/rejection_reports.rs` | certification | Preserve report semantics | `move_only` | Rejection closeout tests | May overlap validation error taxonomy | Keep reports separate from validators |
| `crates/worth-topo/src/certification/tests/milestone_three/**` | Milestone 3 hostile test modules | `crates/worth-topo/src/certification/hostile_topology_operators/tests/**` | certification | Internal test shape only | `split` | Milestone 3 hostile tests | Provenance test folder survives if only moved | Rename by hostile scenario family |
| `crates/worth-topo/src/certification/tests/**` | Certification closeout/support tests | `crates/worth-topo/src/certification/**/tests` or `test_support/certification_assertions/**` | certification | Internal test shape only | `split` | All certification tests | Broad test folder hides failure responsibility | Split by closeout/proof target |
| `crates/worth-topo/src/certification/*.rs` | Certification root, facade, core, errors, requirements, shared | `crates/worth-topo/src/certification/**` | certification | Preserve facade semantics | `split` | All certification closeouts | Root files may be aggregation buckets | Split by proof responsibility |
| `crates/worth-topo/src/certification/public_facade_contracts/contracts/public_api.rs` | Crate-level public API contract, landed from former root integration tests | Same as current path | certification | Preserve public API expectations through explicit Cargo test target | `move_only` | Public API contract | Cargo target path can drift from certification ownership | Landed: root test directory removed; Cargo points directly at certification-owned contract files |
| `crates/worth-topo/src/certification/public_facade_contracts/contracts/topology_operator_closeout.rs` | Crate-level topology-operator closeout public contract, landed from former side-quest-named test | Same as current path | certification | Preserve public report semantics, rename test file away from side-quest provenance | `public_contract_preserve` | Public contract tests | Public type names still include milestone terms by API contract | Landed without duplicate legacy target |
| `crates/worth-topo/src/certification/public_facade_contracts/compile_fail_contracts.rs` | Trybuild compile-fail runner, landed from former root integration tests | Same as current path | certification | Preserve compile-fail runner through explicit Cargo test target | `move_only` | Compile-fail tests | Fixture paths must stay honest after move | Landed: runner resolves certification-owned fixture paths |
| `crates/worth-topo/src/certification/public_facade_contracts/compile_fail/**` | Compile-fail facade/privacy fixtures, landed from former root integration tests | Same as current path | certification | Preserve facade/privacy semantics | `move_only` | Trybuild tests | Stderr fixture paths can silently preserve old root-test wording | Landed: stderr paths updated to certification-owned fixture paths |
| `crates/worth-topo/docs/domain-reads.md` | Developer docs for topology domain reads | `crates/worth-topo/docs/domain-reads.md` | projection | Preserve user-facing docs, update terminology | `move_only` | Doc freshness tests | Docs may retain stale query wording | Update during projection move |
| `crates/worth-topo/docs/runtime-support.md` | Developer docs for runtime support | `crates/worth-topo/docs/runtime-support.md` | projection | Preserve user-facing docs, update terminology | `move_only` | Doc freshness tests | Docs may describe old support rows | Update during projection move |

## Owner Decisions Closed Or Deferred

- Public Milestone 3 report names remain permanent public audit contract names.
  Internal modules use `topology_operator_closeout` and operator-family proof
  names rather than milestone-provenance folders.
- Vertex-disk and radial-ring derived topology vocabulary is landed as
  `derived_topology/vertex_disks` and `derived_topology/radial_rings`.
- Current shell and wire membership execution lives under
  `topology_operators/local_rewrites/sheet_wire_laminar`. Future shell-face
  split pressure may earn a distinct local rewrite family only when real code
  and proof obligations justify it; no empty folder is created for ceremony.
- Crate-level public contract tests were rehomed under
  `certification/public_facade_contracts` without legacy duplicate targets,
  facade aliases, or old-name export shims.

## Immediate Risk Rows

- Landed/enforced: `crates/worth-topo/src/certification/structure_guard.rs`
  now rejects forbidden permanent folders, root integration-test drift,
  dependency-direction inversions, read-view interpretation/certification
  leakage, validation/certification responsibility blur, and missing closeout
  discipline in this migration map.
- Landed/enforced: `scripts/ci/check_worth_topo_domain_structure.sh` is now the
  CI-discoverable gate for this skeleton. It runs formatting, `worth-topo`
  check, structure guards, facade/privacy trybuild contracts, the full
  `worth-topo` suite, and a worth-topo-specific Rust line-cap scan.
- Landed/enforced: dense direct-file clusters are now mechanically reviewed by
  `structure_guard.rs`; any new folder with more than eight direct Rust files
  must either split or be added to the explicit reviewed cluster list.
- Landed/enforced: `worth-topo` Cargo/source geometry purity is now checked so
  topology-to-geometry binding cannot be smuggled into this crate.
- Landed: the former validator god test is split into focused validation tests
  plus named hostile-neighborhood test support.
- Landed: the former Milestone 2 derived closeout sink is split under
  `certification/derived_topology_closeout/` by read-basis tracing, derived
  corpus assembly, closeout orchestration, aggregate report building, and
  closeout assertions.
- Landed: the former derived topology interpretation god test is split into
  focused tests plus named hostile-neighborhood test support.
- Landed: the former read-view certification proof sink is split under
  `certification/authority_closeout/read_view/` by read-basis tracing,
  query-evidence accounting, and localization reporting.
- Landed: certification report aggregation is split by authority, primitive
  corpus, and derived-topology report families.
- Landed: no Rust source/test/support file under `crates/worth-topo/src`
  exceeds the default 400-line cap after this QA pass.
- Landed/enforced: relation-update execution no longer lives at the
  topology-operator application root. Boundary-wiring relation update ownership
  is now under `topology_operators/local_rewrites/boundary_wiring/`, with
  family growth guarded by direct-file-count and dependency-direction checks.
- Reviewed: `projection/runtime_boundary/query_runtime/edit_support.rs` remains
  runtime-boundary support posture, not topology meaning. It is below the
  default line cap, and projection/read-view guards block it from becoming the
  domain read model.

## Landed Enforcement Inventory

- `crates/worth-topo/src/certification/structure_guard.rs`:
  `landed_enforced`
  - verifies the permanent root skeleton
  - rejects forbidden folder names
  - rejects dependency inversions for `brep`, `derived_topology`, `validation`,
    `topology_operators`, and `projection/read_views`
  - rejects projection read-view interpretation, validation, repair,
    certification, and operator execution leakage
  - rejects validation/certification contract blur
  - verifies this map continues to carry closeout status discipline
- `scripts/ci/check_worth_topo_domain_structure.sh`:
  `landed_enforced`
  - runs the topology domain-structure gate in CI
  - runs `cargo fmt --package worth-topo --check`
  - runs `cargo check -p worth-topo --quiet`
  - runs focused structure guards and public facade trybuild contracts
  - runs the full `worth-topo` package test suite
  - fails on any over-cap Rust file under `crates/worth-topo`
- `scripts/ci/check_worth_topo_domain_structure.ps1`:
  `landed_enforced`
  - runs the same topology domain-structure gate locally on Windows
  - tolerates deleted tracked files in move-heavy worktrees while preserving the
    clean-checkout CI line-cap behavior in the bash gate
- `worth-topo-domain-structure-closeout.md`:
  `landed_enforced`
  - records the final QA evidence, intentional deviations, and dense cluster
    reviews required to close the gate without relying on memory
