# Physical Reconstruction C.1 Engineering Spec: Direct Test Execution

## Goal

Make Worth Store fast to change and hard to fool by executing the right tests
directly. C.1 owns test discovery, product selection, execution, CI assignment,
and iteration cost. It does not create a second authority system around tests.

## Why This Milestone Exists

The physical foundation cannot be reconstructed responsibly when a leaf edit
rebuilds unrelated certification crates or when a warm command spends more
time proving its bookkeeping than running tests. The correction must preserve
the useful topology already created—owner tests, consolidated scenarios,
cache-sharing UI runners, and real process probes—while removing recursive
test-control machinery.

## Governing Summaries

- `MENTALITY.md` protects the actual adversarial constraint. C.1 must optimize
  trustworthy feedback, not the sophistication of its evidence vocabulary.
- `arch_laws.md` requires one parameterized execution path. Revalidation occurs
  at a real trust boundary, not at every internal step.
- `composition_laws.md` requires each mechanism to die with its responsibility.
  Selection, execution, and reporting remain separate, but no phase exists
  merely to certify another phase.
- `domain_structure_laws.md` keeps authority and derivation asymmetric. Cargo,
  Git, test processes, and CI are authoritative; local reports are disposable
  projections.
- `perf_laws.md` names repeated rediscovery and defensive re-proof as defects.
  Warm work must scale with the selected test delta.
- `dx_laws.md` requires common commands to expose useful truth without forcing
  forensic artifacts or huge success output.
- `physical-foundation-reconstruction-roadmap.md` places C.1 first because all
  later physical work depends on a fast owner loop and reachable hostile tests.

## Adversarial Constraint

> After a one-line leaf edit, an engineer must reach the owning tests without
> compiling the certification courtroom or waiting for repository-wide source
> analysis. CI must still execute every Cargo test target in exactly one
> responsibility-appropriate partition, preserve real UI and process-boundary
> behavior, and fail locally when selection contains a missing or duplicate
> target.

C.1 fails if speed comes from hiding tests. It also fails if correctness is
claimed through seals, fingerprints, ledgers, or reports that only demonstrate
agreement among the test runner's own derived artifacts.

## Product Decision Lock

1. Git owns source identity. C.1 does not hash every test body or recreate
   version control.
2. Cargo metadata and manifests own package and target identity.
3. Test executables own behavioral verdicts. The runner never promotes a green
   exit into a stronger domain claim.
4. GitHub Actions owns matrix completion. No custom certification aggregate is
   required to prove that required jobs succeeded.
5. Boundary check, agent-context check, and line-cap tools retain their own
   authority. The test runner invokes them only in the structural CI product.
6. One target catalog classifies each Cargo test target once for CI. It is
   derived on demand from `cargo metadata` and is not checked in as generated
   JSON.
7. One execution planner lowers owner, smoke, UI, or CI requests into unique
   units. Parallel selection paths that later deduplicate are forbidden.
8. Owner CI is two direct workspace Cargo calls: ordinary test targets and
   doctests. Scenario and UI targets remain independently addressable and may
   run through a fixed, bounded worker pool so one large package cannot
   serialize the whole product.
9. Stable commands are `store-owner`, `store-smoke`, `store-ui`, and
   `store-ci`. Soak, release, and hardware qualification remain direct named
   tests until those products have enough real cases to justify orchestration.
10. Reports are optional disposable output. Successful local commands write
   nothing unless `--report` is requested.
11. C.1 has no closeout capability, readiness witness, preservation authority,
    source-edit manifest, plan seal, run seal, or C.2 token.
12. C.2 consumes the repository, Cargo catalog, stable commands, and current CI
    result directly.
13. No production Store crate or shared Store test-support crate is a dependency
    of the test runner.

## Direct Truth Contract

```text
Git revision ─────────────── source identity
Cargo metadata ───────────── package and target catalog
Test product request ─────── selected unique execution units
Cargo/libtest/UI executable  behavioral result
GitHub matrix ────────────── required CI completion
optional TestRunReport ───── disposable observation
```

The optional report may record revision, product, selected units, durations,
and exit status. It grants no authority, is never an input to another command,
and can be deleted without losing any repository truth.

## DX Target

```text
cargo store-owner -p worth-store-physical-format
cargo store-smoke
cargo store-ui
cargo store-ci --partition scenario --shard-index 0 --shard-count 2
```

`--list` prints the exact units without executing them. `--target-root` selects
an explicit Cargo target directory for clean/warm measurement. `--report`
writes one JSON report when a human or CI job asks for it.

Successful runner output is limited to product, unit count, elapsed time, and
optional report location; Cargo and the test executable retain ownership of
failure diagnostics.

## Phase Plan

### Phase 1: Remove Recursive Test Authority

Delete every C.1 mechanism whose only consumer is another C.1 mechanism.

**Relevant subsystems**

- existing Store test-control tool and checked-in `test-control` data
- closeout, preservation, mutation orchestration, and source-edit evidence
- CI partition aggregation and generated evidence upload

**Relevant APIs**

- removed `ProofBehaviorAuthority`, `ProofPreservationLedger`, plan/run seals,
  `TestArchitectureCloseoutBundle`, and `C2TestArchitectureReadiness`
- removed seal, baseline, artifact, and closeout commands

**Warnings**

- Do not delete consolidated suites, UI runners, process probes, or real tests
  merely because the old control plane described them badly.
- Do not retain a compatibility wrapper around deleted authority; that would
  leave the competing path alive.

**Test requirements**

- **No recursive consumer test:** repository search finds no generated test
  authority consumed by another test-control stage.
- **No production dependency test:** the replacement runner depends only on
  generic tooling libraries and cannot import any Store crate.

**Engineering decisions**

- Historical generated inventories are deleted. Git history is their archive.
- Missing pre-C.1 measurements remain unknowable and cease to be a gate.

**Open questions**

- None.

### Phase 2: Cargo-Derived Test Catalog

Derive one small catalog from Cargo metadata and classify every executable test
target into its CI responsibility.

**Relevant subsystems**

- nested Worth Store workspace metadata
- library, binary, doctest, integration, UI, scenario, and formal targets
- consolidated certification suite targets

**Relevant APIs**

- `TestCatalog`
- `TestTarget`
- `CiTestLane::{OwnerUnit, Scenario, Ui, Formal}`

**Warnings**

- Source-level assertion parsing is not target discovery.
- Filename/path classification must deny ambiguity instead of guessing.

**Test requirements**

- **Current-workspace classification test:** every Cargo test target belongs to
  exactly one CI lane and every smoke target resolves to a real target.
- **Ambiguous-target denial test:** a synthetic target matching conflicting
  lane rules is rejected with its package, target, and source path.

**Engineering decisions**

- UI targets are identified by explicit compile-fail/UI paths and names.
- Formal-model integration targets remain in the formal lane; other integration
  targets are scenarios. Library, binary, and doctest work is owner-unit work.

**Open questions**

- None.

### Phase 3: One Unique Execution Plan

Lower every product through the same planner and reject duplicate execution
unit identities before starting Cargo.

**Relevant subsystems**

- product request parsing
- smoke case registration
- CI sharding
- Cargo command construction

**Relevant APIs**

- `TestProduct`
- `TestExecutionUnit`
- `TestPlan`
- `TestPlanError`

**Warnings**

- Sorting and calling `dedup` after combining plans hides a planner defect.
- A filter that matches zero tests is a successful Cargo process but a failed
  test product.

**Test requirements**

- **Duplicate-unit denial test:** two routes producing the same unit identity
  fail plan construction and name both routes.
- **Zero-filter denial test:** every filtered smoke unit lists at least one
  libtest case before execution; a stale filter fails before reporting green.
- **Shard convergence test:** all shards are disjoint and their union equals
  the unsharded ordered plan.

**Engineering decisions**

- Product selection creates units once; there is no UI post-pass, doctest
  append pass, or closeout augmentation pass.
- Sharding operates on stable ordered target identities, not timing folklore.
- Bounded execution concurrency is a property of an already-complete plan. It
  never discovers, adds, removes, or deduplicates units.

**Open questions**

- None.

### Phase 4: Fast Owner Execution

Make owner feedback a thin pass-through to the selected Cargo package with no
repository-wide test listing, source hashing, or certification dependency.

**Relevant subsystems**

- `store-owner`
- package validation from `TestCatalog`
- Cargo target reuse and optional target-root selection

**Relevant APIs**

- `TestProduct::Owner`
- `TestPlan::for_owner`
- `execute_test_plan`

**Warnings**

- The runner cannot make an owner build narrower than the package's real Cargo
  dependencies; manifest leakage is a production topology finding.
- A timing target never authorizes skipped tests.

**Test requirements**

- **Leaf closure test:** a private physical-format edit followed by
  `store-owner -p worth-store-physical-format` does not compile
  `worth-store-certification` or unrelated owner crates.
- **Unknown owner denial test:** a misspelled or non-workspace package fails
  before Cargo test execution and lists the requested package.
- **Warm overhead test:** after the runner itself is built, pre-Cargo planning
  remains under one second on the reference machine.

**Engineering decisions**

- Owner execution uses Cargo's package contract directly instead of maintaining
  a second dependency closure ledger.
- Owner success writes no report unless requested.

**Open questions**

- None.

### Phase 5: Direct Smoke And UI Products

Preserve a small deterministic smoke set and execute every UI target through
one catalog path over a shared Cargo target root.

**Relevant subsystems**

- responsibility-named certification suites
- standardized compile-fail/UI runners
- stable smoke case registration

**Relevant APIs**

- `TestProduct::Smoke`
- `TestProduct::Ui`
- `SmokeCase`
- filtered and whole-target `TestExecutionUnit`

**Warnings**

- Smoke is fast plumbing confidence, not database certification.
- UI success means the runner's expected diagnostic checks passed; generic
  compilation failure remains insufficient inside each UI harness.

**Test requirements**

- **Smoke resolution test:** every registered smoke package, target, and filter
  resolves and lists at least one test.
- **Complete UI target test:** every Cargo target under an admitted
  compile-fail/UI location appears once in the UI plan.
- **Wrong-reason UI test:** breaking a fixture before its declared denial must
  fail the existing standardized UI harness rather than pass generically.

**Engineering decisions**

- The smoke set is deliberately small and reviewed in code.
- UI runners keep their existing cache-sharing fixture mechanics. C.1 does not
  wrap each fixture in another target or evidence object.
- The UI harness returns checked diagnostics in memory. It does not publish
  per-fixture evidence JSON, bind controller execution identities, hash its own
  result object, or revalidate that object after the compiler verdict.
- Whole UI targets run as independent units through at most four workers. This
  preserves target-level failure localization and useful parallelism without a
  custom scheduler subsystem.

**Open questions**

- None.

### Phase 6: Direct CI Partitions

Let GitHub Actions execute claim-driven partitions and use matrix status as the
aggregate result.

**Relevant subsystems**

- `.github/workflows/ci.yml`
- owner-unit, scenario, UI, formal, and structural jobs
- Linux/Windows claim coverage

**Relevant APIs**

- `TestProduct::Ci`
- `CiPartition`
- `TestPlan::for_ci`

**Warnings**

- A custom aggregate that consumes runner-produced claims merely restates
  GitHub's job status.
- OS duplication remains only where Store behavior or compiler diagnostics can
  differ materially.

**Test requirements**

- **Partition completeness test:** library/binary/doctest work and every
  integration target map to one CI partition.
- **Cross-partition uniqueness test:** no target is emitted by two CI
  partitions; smoke overlap is irrelevant because smoke is not a CI partition.
- **Host-loss test:** removing Windows from scenario or UI changes the workflow
  review surface visibly; it is not hidden behind an aggregate artifact.

**Engineering decisions**

- Fresh-process cases remain tests inside scenario suites unless a future cost
  or host boundary earns a distinct target and partition.
- Structural CI invokes boundary, agent-context, and line-cap tools directly.
- CI uploads ordinary logs or an explicitly requested report only for
  diagnosis; successful completion needs no custom evidence bundle.

**Open questions**

- None.

### Phase 7: Practical Closeout

Close C.1 with direct execution, timing, and hostile sensitivity—not a new
capability type.

**Relevant subsystems**

- reference-machine owner and smoke loops
- complete UI and CI runs
- existing domain mutation and hostile scenario tests
- ordinary Cargo target lifecycle

**Relevant APIs**

- optional `TestRunReport`
- stable rerun commands
- `cargo clean --target-dir <explicit-root>` for user-directed cleanup

**Warnings**

- A report is observation, not authority and never becomes an input.
- C.1 does not build a general mutation framework. Direct controlled edits and
  existing domain mutants are stronger than mutations of the runner's own
  bookkeeping.

**Test requirements**

- **Three-radius iteration test:** measure one private leaf edit, one shared
  public-contract edit, and one test/UI edit using direct commands. Record cold
  and warm elapsed time without sealing the source edit.
- **Behavior sensitivity test:** deliberately invert one ordinary assertion and
  one UI expectation during QA; each owning product must fail for the intended
  reason, after which the source is restored exactly.
- **Artifact-breadth test:** repeated warm runs reuse one target root and do not
  create per-case Cargo targets.
- **No-meta-artifact test:** a complete C.1 run succeeds with `test-control/`,
  `.store-proof/`, and every historical closeout bundle absent.

**Engineering decisions**

- Reference targets are warm owner feedback under ten seconds and warm smoke
  under one minute. The report includes actual machine context when requested.
- C.1 completion is a reviewed repository state plus green direct commands and
  CI. There is no `store-closeout` command.

**Open questions**

- None.

## Opinionated Directory Target

```text
workspaces/worth-store/
  .cargo/config.toml                  stable store-* aliases
  tools/store-test-runner/
    Cargo.toml                        generic tooling dependencies only
    src/
      arguments.rs                    command-line contract
      catalog.rs                      Cargo-derived target catalog
      classification.rs               one CI lane per target
      product.rs                      owner/smoke/UI/CI selection
      plan.rs                         unique execution units and sharding
      execution.rs                    Cargo process execution
      report.rs                       optional disposable report
      lib.rs                          runner facade
      main.rs                         exit-code adapter
```

There is no checked-in `test-control` directory and no default evidence tree.
The runner is repository infrastructure and cannot be a Store production or
test-support dependency.

## Must Ship

- a Cargo-derived catalog with total, unique CI classification
- one planner for owner, smoke, UI, and CI products
- fast owner execution with no broad Store dependency in the runner
- deterministic smoke cases and complete standardized UI target execution
- direct CI partitions and claim-driven OS coverage
- concise local DX and one optional disposable report
- deletion of recursive authority, preservation, sealing, closeout, and
  generated-inventory machinery

## Must Preserve

- real owner assertions and invariants
- consolidated certification scenario suites
- exact standardized UI denial checks
- real process death and fresh-process behavior inside the owning scenarios
- formal and structural checks as direct CI products
- Store-local build profiles and cache-sharing target topology

## Acceptance Evidence

- `cargo test -p store-test-runner`
- `cargo store-owner -p worth-store-physical-format`
- `cargo store-smoke`
- `cargo store-ui`
- every required `cargo store-ci --partition <name>` job on Linux/Windows as
  declared by the workflow
- reference-machine cold/warm timings reported in the C.1 completion change
- boundary-check, agent-context, formatting, lint, and line-cap gates

## False-Completion Gates

C.1 is not complete if:

- any old test authority, fingerprint, seal, ledger, closeout, or readiness
  object remains on the ordinary path
- a generated report is required to run another product
- one planner combines independent selection paths and deduplicates afterward
- a filtered unit can match zero tests and still pass
- an integration target is absent from every CI lane or present in two
- owner execution scans or lists the entire repository test surface
- the runner depends on a Store crate
- UI fixtures regain per-case target roots
- warm smoke exceeds one minute for unexplained structural reasons

## Sequencing Notes

- Rewrite the requirement before deleting its implementation.
- Remove competing authority before building the replacement runner.
- Establish catalog classification before product planning.
- Establish unique planning before execution and CI wiring.
- Run direct hostile QA only after the old path is gone.

## C.2 Handoff

C.2 receives no token. It begins from:

- the current Git revision
- the Cargo-derived test catalog
- stable owner, smoke, UI, and CI commands
- the current CI result
- direct access to the production and certification source it must trace

That is sufficient to audit executable physical reality without teaching a
test report to authorize the next milestone.
