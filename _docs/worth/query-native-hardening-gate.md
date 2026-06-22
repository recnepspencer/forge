# Worth Query-Native Hardening Gate

## Goal

Harden `worth-kernel`, `worth-spatial`, and `worth-topo` after Forge Query
milestones 9.7 and 9.8 so Worth can enter the next workload and boolean lanes
through real Query-native support, evidence, pinning, and runtime boundaries
rather than inherited pre-Query shortcuts.

## Why This Gate Exists

Forge Query now owns consumer support snapshots, real support pinning, hard
prohibition evidence, consumer-kit closure, and explicit downstream runtime
integration. Worth must consume those facilities directly before later Worth
milestones depend on kernel, spatial, and topology proofs.

This is a gate, not a refactor. It blocks the next Worth capability lane until
the three Worth crates stop carrying synthetic proof paths that could make
future milestone closeouts look stronger than the architecture actually is.

## Governing Summaries

- `MENTALITY.md` protects adversarial, enforcement-first engineering. This
  gate must remove false completion paths mechanically rather than document
  them as conventions.
- `arch_laws.md` protects authority boundaries, proof-bearing phase chains,
  explicit facades, and measured boundary crossings. Worth must consume Query
  authority and evidence instead of reconstructing them locally.
- `composition_laws.md` protects files as semantic compilation units. The
  hardening work must split collapsed responsibilities by meaning, not merely
  reduce line counts.
- `domain_structure_laws.md` protects the tree as enforceable architecture.
  Kernel, spatial, and topology boundaries must make truth, derivation,
  diagnostics, certification, and test support spatially locatable.
- `perf_laws.md` protects bounded semantic-delta execution. Worth workload
  rails must expose named counters and must not hide broad scans behind cheap
  APIs.
- `worth_roadmap.md`, `worth_roadmap_pt2`, and `milestone-7-roadmap.md`
  protect the Worth rule that spec, topology, naming, and binding truth commit
  canonically once while everything else derives honestly. This gate exists so
  later Worth milestones cannot close with synthetic fixtures, hand-built
  evidence rows, or re-extraction replay.

## Adversarial Constraint

After Forge Query 9.7 and 9.8, Worth must not retain pre-Query shortcuts where
`worth-kernel`, `worth-spatial`, or `worth-topo` can pass tests through
synthetic receipts, hand-built evidence rows, unpinned support assumptions,
lower-authority identity reconstruction, private fixture truth, or broad
unmeasured runtime paths.

Every admitted Worth operation must enter through the correct Query-native
declaration, support, evidence, receipt, and runtime boundary lane or fail
closed with typed, localizable residue.

## Product Decision Lock

- This gate belongs after Forge Query 9.8 and before Worth `Milestone 6.5` and
  the `Milestone 7.x` planar boolean band.
- `forge-query` owns consumer support snapshots, support pinning, hard
  prohibition registry, evidence reports, and consumer-kit closure.
- `worth-kernel` consumes Query-lowered Worth contracts and orchestrates
  construction programs. It must not become a shadow runtime or evidence
  authority.
- `worth-spatial` owns spatial and topology/geometry interaction semantics,
  including workload evidence vocabulary where the evidence is spatial in
  meaning.
- `worth-topo` owns topology truth semantics, topology editing, topology
  legality, and topology-hostility proof.
- Any remaining incompleteness must be typed, pinned, localized, and owned by a
  follow-on milestone. Hidden debt is not allowed.

## Directory Skeleton

```text
_docs/worth/
  query-native-hardening-gate.md
  query-native-hardening-closeout.md

crates/worth-kernel/src/
  query_adoption/
    mod.rs
    support_pins.rs
    evidence_reports.rs
    boundary_audit.rs
    residue.rs
  workload_composition/
    ...existing workload and operator families, with synthetic proof ownership removed

crates/worth-spatial/src/
  query_adoption/
    mod.rs
    support_projection.rs
    evidence_reports.rs
    boundary_audit.rs
    residue.rs
  workload_platform/
    ...existing spatial workload families, with production evidence ownership preserved

crates/worth-topo/src/
  query_adoption/
    mod.rs
    runtime_support.rs
    evidence_reports.rs
    boundary_audit.rs
    residue.rs
  projection/runtime_boundary/
    ...existing Query runtime bridge integration, with authority-preserving identities

crates/worth-kernel/tests/
  query_adoption_*.rs
  support_pinning_*.rs
  evidence_report_*.rs
  synthetic_boundary_denial_*.rs

crates/worth-spatial/tests/
  query_adoption_*.rs
  support_pinning_*.rs
  evidence_report_*.rs
  synthetic_boundary_denial_*.rs

crates/worth-topo/tests/
  query_adoption_*.rs
  runtime_boundary_*.rs
  support_pinning_*.rs
  evidence_report_*.rs
  synthetic_boundary_denial_*.rs
```

## Phase Plan

### Phase 1: Cross-Crate Reality Inventory

This phase freezes the current adoption truth before any cleanup is allowed to
claim progress. The output is an inventory of every Worth path that still
depends on pre-Query assumptions, synthetic proof, unpinned support, local
evidence construction, or lower-authority identity promotion.

**Relevant subsystems**
- `worth-kernel/src/workload_composition`
- `worth-spatial/src/workload_platform`
- `worth-topo/src/projection/runtime_boundary`
- `worth-topo/src/workload_platform`
- Forge Query consumer-kit support, evidence, and prohibition facilities

**Relevant APIs**
- Query support snapshot and support pinning facade
- Query evidence report declaration and sealed report APIs
- Query hard-prohibition registry and boundary-audit APIs
- Worth workload receipts, workload evidence ledgers, and runtime boundary
  receipts

**Warnings**
- Do not classify a test as real just because it touches production types. A
  test is synthetic when the proof artifact is hand-authored instead of
  produced by the production lane.
- Do not treat broad `rg` matches as sufficient. The inventory must map each
  finding to an owned Worth surface and a required production replacement.

**Test requirements**
- Add an inventory coverage test proving every audited Worth source set is
  classified as production, test support, certification-only, or explicit
  residue.
- Add an adversarial seeded-source test proving synthetic receipts, forged
  evidence rows, direct support-posture assumptions, and lower-authority
  identity reconstruction are detected and localized.

**Engineering decisions**
- The inventory artifact must be typed and machine-checkable. A markdown table
  may summarize it, but cannot be the authority.
- Each finding must name the owning crate, the owning responsibility, the
  forbidden pattern, and the production surface needed to replace it.

**Open questions**
- None.

### Phase 2: Authority Boundary Classification

This phase classifies each affected Worth surface as authoritative, derived,
diagnostic, certification-only, or test-support-only. It closes the ambiguity
that lets topology truth, spatial evidence, kernel orchestration, and test
fixture scaffolding masquerade as the same category of artifact.

**Relevant subsystems**
- `worth-topo` topology truth, topology operators, validation, and runtime
  boundary projection
- `worth-spatial` witness resolution, placement, and workload-platform evidence
- `worth-kernel` construction and workload composition programs
- Forge Query runtime and consumer-kit facades

**Relevant APIs**
- `worth-topo` public facade and runtime boundary receipt APIs
- `worth-spatial` workload receipt and witness-resolution APIs
- `worth-kernel` workload catalog and operator harness APIs
- Query evidence identity and consumer support APIs

**Warnings**
- A surface can be production code and still not be authoritative. The spec
  must distinguish production derivation from production truth.
- Do not collapse topology identity, spatial witness identity, kernel workload
  identity, and evidence identity into string-compatible values.

**Test requirements**
- Add an authority-classification parity test proving each audited public
  surface reports the same authority category through docs, support report, and
  machine-readable inventory.
- Add a compile-fail or denial test proving lower-authority representations
  cannot be promoted into topology truth, spatial witness truth, support pins,
  or evidence reports.

**Engineering decisions**
- `worth-topo` is the only Worth crate allowed to answer topology legality.
- `worth-spatial` may produce spatial evidence, but must not promote spatial
  evidence into topology truth.
- `worth-kernel` may orchestrate construction programs, but must consume
  declared receipts and cannot forge proof artifacts on behalf of lower crates.

**Open questions**
- None.

### Phase 3: Query Consumer-Kit Adoption

This phase makes Worth consume the real Query consumer kit rather than carrying
local imitations of support snapshots, pinning, evidence reports, and hard
prohibition boundaries.

**Relevant subsystems**
- `forge-query/src/consumer_kit`
- `worth-kernel/src/query_adoption`
- `worth-spatial/src/query_adoption`
- `worth-topo/src/query_adoption`

**Relevant APIs**
- Query support snapshot projection and comparison APIs
- Query support pin declaration, document load, evaluation, and report APIs
- Query evidence report declaration and sealed report APIs
- Query hard-prohibition registry and boundary-audit report APIs

**Warnings**
- A local Worth support matrix is allowed only as a Worth-domain projection of
  Query support truth. It must not become a second support authority.
- Pinning must be real pinning: declared requirements evaluated against a live
  or durable Query support snapshot, not a checked string or fixture constant.

**Test requirements**
- Add support pin success tests for each Worth crate proving current required
  Query support rows are evaluated through the real Query pinning API.
- Add support pin drift tests proving a stale, missing, or semantically changed
  support row fails with typed context localized to the affected Worth crate and
  requirement.
- Add evidence-report participation tests proving Worth evidence reports derive
  digest participation through Query evidence report APIs rather than private
  digest helpers.

**Engineering decisions**
- Each Worth crate gets a narrow `query_adoption` module that owns only the
  bridge from Worth-domain requirements into Query consumer-kit APIs.
- The public Worth facade may expose adoption status, but not Query internals.
- Query hard prohibitions must be referenced from Query-owned registry rows,
  not duplicated as Worth-local string lists.

**Open questions**
- None.

### Phase 4: Synthetic Proof Removal

This phase removes or quarantines every path where tests currently manufacture
the artifact that production should create. The goal is not fewer tests; the
goal is tests that are forced to travel through the same proof-bearing lanes as
real consumers.

**Relevant subsystems**
- `worth-kernel` workload catalog and operator harness tests
- `worth-spatial` workload platform and witness-resolution tests
- `worth-topo` runtime boundary, topology workload, and public facade tests
- Query compile-fail suites for forbidden construction

**Relevant APIs**
- Worth workload declaration and receipt APIs
- Query public submission and evidence APIs
- Query hard-prohibition compile-fail manifest
- Worth topology and spatial receipt constructors

**Warnings**
- Do not delete hostile tests because their fixture path is synthetic. Replace
  the fixture path with a production declaration/admission/execution path.
- Private constructors in tests are allowed only for test-support types whose
  responsibility is explicitly test support. They are not allowed for proof
  artifacts that production claims.

**Test requirements**
- Add compile-fail tests proving forged evidence reports, forged support pin
  documents, forged runtime receipts, and direct workspace mutation shortcuts
  are unavailable to downstream consumers.
- Add integration tests proving representative kernel, spatial, and topology
  workload proofs are produced by production declarations and receipts rather
  than hand-built rows.
- Add residue tests proving any still-incomplete path is reported as explicit
  typed residue rather than silently counted as production coverage.

**Engineering decisions**
- Synthetic sources discovered in Phase 1 must be either replaced by production
  rails or moved into named residue with a follow-on owner.
- Any replacement production surface must be narrow and owned by the crate whose
  domain meaning it represents.

**Open questions**
- None.

### Phase 5: Runtime Boundary Hardening

This phase hardens `worth-topo` at the Query runtime boundary. It ensures
topology mutation receipts, query runtime identities, validation rows, and
bridge diagnostics preserve authority instead of reconstructing it from
strings, projections, cached labels, or test fixtures.

**Relevant subsystems**
- `worth-topo/src/projection/runtime_boundary`
- `worth-topo/src/topology_operators`
- `worth-topo/src/validation`
- Forge Query runtime workspace and mutation surfaces

**Relevant APIs**
- Query runtime workspace mutation and submission APIs
- `worth-topo` query runtime bridge verification APIs
- topology validation facade APIs
- topology operator application and local rewrite APIs

**Warnings**
- Identity-like values with different authority must remain distinct types even
  when their representation is identical.
- Rejection must structurally precede construction. Runtime boundary code must
  not build rich topology objects before eligibility and authority are proven.

**Test requirements**
- Add replay-equivalence tests proving the same topology workload produces the
  same runtime receipt and validation conclusion across live and replayed Query
  runtime entry.
- Add denial tests proving malformed, stale, lower-authority, or projection-
  reconstructed topology identities fail before topology object construction.
- Add bridge diagnostic tests proving rejection context names the exact boundary
  and identity authority that failed.

**Engineering decisions**
- Runtime boundary APIs must accept proof-bearing Worth/Query types, not raw
  strings standing in for entity or relation authority.
- Validation facade rows may summarize decisions, but may not become authority
  for topology truth.

**Open questions**
- None.

### Phase 6: Spatial Workload Evidence Honesty

This phase hardens `worth-spatial` so spatial workload evidence is production
vocabulary rather than private fixture truth. Spatial may own spatial meaning,
but it must not bypass Query support, evidence, or receipt lanes when its
artifacts are used as milestone proof.

**Relevant subsystems**
- `worth-spatial/src/workload_platform`
- `worth-spatial/src/witness_resolution`
- `worth-spatial/src/placement`
- `worth-spatial/src/query_adoption`

**Relevant APIs**
- spatial workload receipts and blocker-provenance APIs
- witness resolution and frame admission APIs
- placement admission, constraint, and motion APIs
- Query evidence report and support pinning APIs

**Warnings**
- Retained replay, projection parity, blocker provenance, and response workload
  rows must come from production spatial lanes when they certify spatial
  behavior.
- Spatial witness helpers must not become authority bridges. A helper may
  assemble admitted evidence, but may not forge admission.

**Test requirements**
- Add spatial replay-parity tests proving retained replay and projection parity
  evidence remains stable when loaded through real workload declarations.
- Add spatial denial tests proving forged witness evidence, unsupported frame
  admission, dirty planar posture, and missing blocker provenance fail with
  typed residue.
- Add support pin tests proving spatial workload families declare and evaluate
  their Query support posture through real pins.

**Engineering decisions**
- `worth-spatial/src/query_adoption` owns the translation from spatial support
  needs into Query consumer-kit declarations.
- Existing workload-platform modules keep their domain ownership; Query
  adoption must not turn into a generic cross-crate helper bucket.

**Open questions**
- None.

### Phase 7: Kernel Composition Honesty

This phase hardens `worth-kernel` so construction and workload composition
consume Query-lowered Worth contracts and crate-owned receipts. Kernel is the
program orchestrator, not a lower-authority source of topology, spatial, or
evidence truth.

**Relevant subsystems**
- `worth-kernel/src/workload_composition`
- `worth-kernel/src/construction`
- `worth-kernel/src/query_adoption`
- `worth-topo` topology workload declarations
- `worth-spatial` workload declarations

**Relevant APIs**
- kernel workload catalog and operator harness APIs
- construction result and authoring APIs
- topology workload receipt APIs
- spatial workload receipt APIs
- Query evidence and support APIs

**Warnings**
- Kernel summaries are not closeout proof unless they are derived from real
  lower-crate receipts and Query evidence participation.
- The kernel may compose batches, but it must not scalarize bulk domains into
  external loops that hide cost and fracture amortization.

**Test requirements**
- Add kernel composition parity tests proving representative workloads consume
  topology and spatial receipts without re-extracting or forging proof.
- Add denial tests proving missing topology receipts, missing spatial receipts,
  stale support pins, and forged evidence reports prevent kernel closeout.
- Add workload cardinality tests proving admitted batch semantics are preserved
  through the kernel orchestration surface.

**Engineering decisions**
- Kernel `query_adoption` owns only kernel-level support and evidence
  declarations. It may depend on topology and spatial public facades, not their
  internals.
- Workload composition must retain domain-specific submodules rather than
  centralizing all adoption logic into a broad manager.

**Open questions**
- None.

### Phase 8: Performance And Counter Closure

This phase makes the hardening measurable. Every boundary touched by this gate
must expose counters that explain work performed and must include exact tests
for the claimed cost shape.

**Relevant subsystems**
- Query support pin evaluation and boundary audits as consumed by Worth
- `worth-kernel` workload catalog and operator harness
- `worth-spatial` witness resolution and workload platform
- `worth-topo` runtime boundary and topology operator paths

**Relevant APIs**
- Query support pin evaluation reports
- Query boundary audit reports
- Worth runtime receipts and workload receipts
- Worth support/adoption reports

**Warnings**
- Elapsed time is not proof. Tests must assert structural counters such as
  audited source count, support rows evaluated, receipt count, touched topology
  scope, witness resolution breadth, replay breadth, and denial localization
  breadth.
- Avoid abstraction that hides different cost classes behind one cheap-looking
  surface. Topology validation, spatial witness resolution, and kernel workload
  composition may share a lifecycle without sharing a cost model.

**Test requirements**
- Add exact-counter tests for support pin evaluation breadth, boundary audit
  source count, and synthetic-denial localization.
- Add exact-counter tests for topology runtime touched scope, spatial witness
  resolution breadth, and kernel workload receipt breadth.
- Add regression tests proving diagnostics richness can be policy-controlled
  without changing domain outcomes or forcing rich artifacts onto the hot path.

**Engineering decisions**
- Counter fields must live in the relevant production reports or receipts, not
  in side-channel test logs.
- A performance contract may be marked `Debt` only when the blocker satisfies
  the engineering-mentality debt rules and is tied to a named follow-on
  milestone.

**Open questions**
- None.

### Phase 9: Gate Certification And Closeout

This phase proves the gate is closed and publishes the closeout. It does not
add new functionality; it demonstrates that kernel, spatial, and topology are
ready to enter the next Worth workload and boolean lanes on honest Query-native
rails.

**Relevant subsystems**
- `_docs/worth/query-native-hardening-closeout.md`
- Query consumer-kit certification surfaces
- `worth-kernel`, `worth-spatial`, and `worth-topo` adoption reports
- Worth roadmap and test-requirement bookkeeping

**Relevant APIs**
- crate-level adoption reports
- support pin reports
- evidence report summaries
- boundary audit reports
- runtime and workload receipt reports

**Warnings**
- The gate is not closed because representative tests pass. It closes only when
  every inventoried synthetic or pre-Query path is replaced, denied, or
  published as explicit owned residue.
- Do not permit broad "future hardening" language. Remaining work must name the
  exact blocker, owner, and follow-on milestone.

**Test requirements**
- Add cross-crate certification tests proving `worth-kernel`, `worth-spatial`,
  and `worth-topo` all publish passing Query adoption reports.
- Add adversarial closeout tests proving known synthetic source families cannot
  contribute to support, evidence, runtime, or workload closeout.
- Add documentation agreement tests proving AI-facing docs, roadmap language,
  closeout evidence, and machine-readable adoption reports agree on admitted
  and denied surfaces.

**Engineering decisions**
- The closeout document must name all remaining residue. No unnamed debt may
  pass this gate.
- The gate must update roadmap sequencing so `Milestone 6.5` and `Milestone
  7.x` are understood to depend on Query-native Worth hardening.

**Open questions**
- None.

## Must Ship

- A machine-checkable adoption inventory for `worth-kernel`, `worth-spatial`,
  and `worth-topo`.
- Query consumer-kit adoption modules in the three Worth crates, scoped to each
  crate's authority.
- Real support pin declarations and evaluation tests for the Query support rows
  each Worth crate depends on.
- Real evidence report participation for Worth support, runtime, workload, and
  certification claims.
- Boundary audits and compile-fail tests preventing forged receipts, forged
  support pins, forged evidence, direct workspace mutation shortcuts, and
  lower-authority identity promotion.
- Runtime boundary hardening in `worth-topo` preserving identity authority.
- Spatial evidence hardening in `worth-spatial` preserving production workload
  provenance.
- Kernel composition hardening in `worth-kernel` preserving lower-crate receipt
  authority.
- Exact performance counters and proof tests for all hot or proof-bearing
  boundaries touched by this gate.
- A closeout document that names admitted surfaces, denied surfaces, explicit
  residue, verification commands, and remaining owned debt.

## Must Preserve

- `forge-query` remains the owner of consumer support, support pinning, hard
  prohibitions, evidence reports, and consumer-kit closure.
- `worth-topo` remains topology-truth and topology-legality authority.
- `worth-spatial` remains spatial and topology/geometry interaction authority.
- `worth-kernel` remains construction and workload orchestration authority.
- Runtime and workload receipts remain self-describing boundary artifacts.
- Rich diagnostics must be policy-controlled and must not change domain
  outcomes.
- Public facades remain narrower and more stable than internal topology.
- Tests obey production composition rules and must not hide broad fixture
  authority behind convenience helpers.

## Acceptance Evidence

- `cargo test -p forge-query --lib consumer_kit -- --nocapture`
- `cargo test -p worth-kernel query_adoption -- --nocapture`
- `cargo test -p worth-spatial query_adoption -- --nocapture`
- `cargo test -p worth-topo query_adoption -- --nocapture`
- `cargo test -p worth-topo runtime_boundary -- --nocapture`
- compile-fail suites for forged evidence, forged pins, forged runtime
  receipts, direct workspace mutation shortcuts, and lower-authority identity
  promotion
- exact-counter tests for Query support pin evaluation, boundary audit source
  coverage, topology touched scope, spatial witness breadth, and kernel receipt
  breadth
- documentation agreement tests tying the closeout, AI-facing docs, roadmap
  references, and machine-readable adoption reports together

## Sequencing Notes

This gate belongs after Forge Query 9.8 because the Query consumer kit is now a
real surface Worth can depend on. It belongs before Worth `Milestone 6.5` and
the `Milestone 7.x` boolean band because those milestones explicitly reject
synthetic fixtures, hand-built evidence rows, kernel summaries, and
re-extraction replay as closeout proof.

Completing this gate should make the next Worth implementation plan smaller,
not larger: later phases can assume real support pins, real evidence reports,
authority-preserving runtime identities, and measured workload receipts instead
of re-litigating proof authenticity in every phase.
