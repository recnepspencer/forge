# Physical Reconstruction C.1 Engineering Spec: Test Execution Architecture And Proof-Lane Cleanup

## Goal

Make Worth Store cheap to change without making it cheap to lie about.

C.1 replaces the current broad, repeatedly compiled test topology with explicit
owner, smoke, UI, certification, soak, release, and hardware proof products.
It preserves every legitimate assertion, hostile scenario, process boundary,
and closeout obligation while making ordinary owner feedback narrow enough for
the physical reconstruction program to iterate quickly.

## Why This Milestone Exists

The physical foundation cannot be reconstructed responsibly when a small edit
causes the courtroom, compile-fail fixtures, broad test support, and unrelated
integration binaries to rebuild together. That cost encourages partial runs,
discourages end-to-end correction, and makes expensive compilation feel like
strong evidence even when the production physical path is still fake.

C.1 is therefore an architecture milestone, not build polish. It establishes
which proof exists, who owns it, when it runs, what it costs, and how later
milestones add proof without recreating the current explosion.

## Governing Summaries

- `MENTALITY.md` protects foundation-first construction. C.1 must repair the
  feedback foundation before any runtime reconstruction inherits its cost.
- `arch_laws.md` protects compiler-visible contracts and measurement
  boundaries. Proof selection, execution, preservation, and cost must be
  explicit artifacts rather than command-line folklore.
- `composition_laws.md` protects one named responsibility per test unit.
  Sharing a binary may amortize linking, but it may not collapse unrelated
  scenario meaning into a test bag.
- `domain_structure_laws.md` protects production ownership through test
  topology. An owner-local test must not construct or depend on unrelated
  physical domains, and shared fixtures must have genuinely shared authority.
- `perf_laws.md` protects bounded execution breadth. Compile, link, execution,
  subprocess, feature, invalidation, and artifact breadth must be observable
  and must scale with the selected proof product.
- `physical-foundation-reconstruction-roadmap.md` makes C.1 the first hard
  gate. Every later cleanup milestone depends on fast owner feedback and on
  expensive proof remaining reachable through honest explicit modes.

## Adversarial Constraint

> A one-line change in a leaf physical owner must receive trustworthy owner
> feedback without compiling or linking the certification courtroom, broad
> platform fixtures, unrelated owners, or cold per-case Cargo projects. The
> same revision must still expose every pre-C.1 proof through exactly one
> declared execution product, and CI must detect any assertion, scenario, UI
> denial, process-boundary proof, controlled defect, or evidence obligation
> that was lost, duplicated, silently weakened, or made unreachable during the
> cleanup.

C.1 fails if local iteration becomes fast by hiding proof, or if all proof is
preserved by retaining the same indiscriminate build graph.

## Product Decision Lock

1. C.1 changes test architecture and execution, not production Store behavior.
2. Every existing test is classified before it is moved, merged, rewritten, or
   deleted.
3. Proof meaning remains owned by owner tests or certification. Test support
   owns mechanics only.
4. Owner tests compile only their owner and the narrow dependencies required by
   the production contract they exercise.
5. Scenario consolidation shares compilation and linking, not semantic
   ownership. One suite binary may contain multiple modules only when the suite
   name and registry preserve each scenario family's responsibility.
6. A separate executable exists only when process identity, address-space
   isolation, environment, binary provenance, or deliberate process death is
   part of the proof.
7. Compile-fail proof uses one standardized cache-sharing UI architecture.
   Hand-written per-case Cargo projects and disposable target directories are
   not an accepted long-term lane.
8. Structural preflight, boundary checks, agent-context checks, source scans,
   and generated topology validation are explicit preflight products. They are
   not nested inside behavioral scenario execution.
9. `store-owner`, `store-smoke`, `store-ci`, `store-ui`, `store-soak`,
   `store-release`, and `store-hardware` are stable developer-facing proof
   products. Their implementation may be Cargo aliases or workspace tooling,
   but their meaning is fixed here.
10. `cargo test --workspace` is not the canonical developer command and must
    not be presented as one. If retained, its cost and semantics must be
    explicit and it may not be the only route to any closeout proof.
11. Timing is diagnostic evidence, never correctness authority. Structural
    breadth counters explain timing changes.
12. `cargo-nextest` or equivalent scheduling may improve execution after the
    target graph is corrected; it cannot substitute for target consolidation,
    dependency narrowing, or compile-fail redesign.
13. No C.1 type, module, target, test, or command uses milestone or phase
    provenance as its production responsibility name. `C.1` remains in specs
    and evidence only.
14. C.1 does not grant S.1-through-S.9 physical closure. It produces a sealed
    handoff that C.2 consumes to audit executable physical reality.

## Authority And Artifact Contract

The test-control progression is:

```text
DiscoveredTestSurface
  -> ClassifiedProofInventory
  -> ValidatedProofInventory
  -> SelectedProofExecutionPlan
  -> ExecutedProofRun
  -> IndependentlyObservedProofRun
  -> PreservationCheckedProofRun
  -> C2TestArchitectureReadiness
```

The names above describe required semantic artifacts. Implementation may use
stronger responsibility-specific names, but it may not collapse discovery,
classification, selection, execution, observation, and preservation into one
mutable runner state.

- The repository and Cargo metadata are authoritative for what targets and
  dependencies exist.
- Owner and certification declarations are authoritative for what a test
  claims to prove.
- The proof selector owns which declared products should execute for an input
  change or requested mode. It owns no test verdict.
- The underlying test executable owns behavioral pass/fail.
- External process and filesystem observation own compile/link/subprocess and
  artifact-cost evidence.
- The preservation checker owns comparison between the pre-cleanup inventory
  and the post-cleanup reachable proof graph. It cannot reclassify a missing
  proof as unnecessary without a reviewed disposition.
- Certification owns closeout verdicts. A runner success code alone is not a
  C.1 verdict.

## DX Target

The intended operator surface is:

```text
cargo store-owner -p worth-store-physical-format
cargo store-smoke
cargo store-ui
cargo store-ci --partition recovery
cargo store-soak --profile checkpoint-heavy --seed 42
cargo store-release --backend windows-file
cargo store-hardware --profile windows-ntfs-nvme
```

Every command must print before execution:

- selected proof product and profile
- packages, targets, suites, and process probes selected
- feature set and build profile
- whether the run is warm/cold evidence or ordinary execution
- expected evidence destination
- excluded proof products and the reason they are excluded

Every command must produce after execution:

- behavioral verdicts by responsibility
- compile/link/test/subprocess counts
- target and feature breadth
- evidence identity and rerun command
- explicit statement that the run does or does not qualify milestone closeout

## Non-Fake Acceptance Setup

The C.1 closeout test uses both a clean target root and a warmed target root on
a declared Windows reference-development profile. It records OS, filesystem,
CPU, storage class, Rust toolchain, source revision, lockfile digest, profile,
features, antivirus/exclusion posture if known, command, and target-root
identity.

The test performs these independent edits and runs:

1. Touch one private leaf-owner implementation without changing public API,
   then run `store-owner` for that package.
2. Touch one shared physical public contract, then run `store-smoke`.
3. Change one UI fixture expectation, then run `store-ui`.
4. Change one certification scenario assertion, then run the owning `store-ci`
   partition.
5. Execute one real fresh-process crash/reopen scenario through its dedicated
   process probe topology.

An external observer, not the runner alone, records compiler and linker
processes, Cargo subprocesses, target directories, produced binaries, PDBs,
incremental directories, file counts, logical bytes, elapsed intervals, and
peak concurrent work. Cargo metadata independently records target and
dependency breadth.

The setup forbids declaring success through a no-change green run, deleting
tests from every product, preserving only test names without assertions,
moving proof into ignored tests with no selected mode, or reporting nextest
parallelism as target-graph repair.

## Phase Plan

### Phase 1: Test Surface Discovery And Baseline Evidence

Freeze the pre-cleanup test universe and its structural cost before changing
target topology. This phase produces the authority against which proof
preservation is judged.

**Relevant subsystems**

- Worth Store workspace manifests and package graph
- unit, integration, doc, UI, scenario, formal, benchmark, process-probe, and
  structural-preflight targets
- CI workflows, Cargo configuration, feature declarations, and build profiles
- target-directory and process observation

**Relevant APIs and artifacts**

- `DiscoveredTestSurface`
- `TestTargetIdentity`
- `TestCaseIdentity`
- `ObservedBuildGraph`
- `ObservedArtifactFootprint`
- `PreCleanupProofInventory`

**Required structure**

- Discovery reads Cargo metadata, manifests, suite registries, libtest listing,
  UI fixture roots, process-probe declarations, workflow commands, and feature
  graphs.
- Every discovered item records owner crate, target, case or scenario identity,
  proof family, current invocation path, features, dependencies, process model,
  external tools, expected evidence, and whether it runs by default.
- Unit tests may be grouped under an owner target for execution planning, but
  their individual test identities remain in the preservation inventory.
- The baseline records cold and warm observations separately. It never
  subtracts startup or cache effects by assumption.
- Observed cost includes compiler, linker, rustdoc, Cargo, child-process,
  produced-executable, PDB, rlib/rmeta, incremental-directory, file-count, and
  logical-byte measurements.
- The inventory records unknown and contradictory classifications as explicit
  blocking rows. Discovery does not guess.

**Warnings**

- Source scans alone cannot discover what Cargo builds or which features unify.
- Cargo metadata alone cannot prove which nested processes a test launches.
- Timing without target, process, and artifact breadth is not actionable.
- The existing target directory is historical residue, not a clean-build
  baseline; it is observed separately and never confused with one run's cost.

**Test requirements**

- **Discovery parity test:** compare Cargo metadata, manifest targets, libtest
  listing, registered scenarios, UI fixtures, and CI commands. Every item must
  appear in the canonical inventory exactly once or carry an explicit alias
  relationship.
- **Hidden-target rejection test:** introduce an unregistered integration test
  target and an ignored UI fixture. Inventory validation must fail with the
  exact unclassified path and may not silently place it in developer smoke.
- **Process-observation honesty test:** add a fixture that launches a nested
  Cargo process. External observation must record the process even if the test
  runner omits it from its own report.
- **Cold/warm separation test:** two identical runs against clean and warmed
  roots must produce distinct cache-posture evidence while preserving the same
  selected test identities.

**Engineering decisions**

- Baseline evidence is generated and machine-checkable; a prose audit is not
  the authority.
- Stable identities derive from package, target, suite/scenario responsibility,
  and case nameâ€”not filesystem order or discovery timestamp.
- Historical target residue is inventoried before any cleanup but is excluded
  from clean-run attribution.
- Phase 1 makes no deletion or consolidation decisions.

**Open questions**

- None.

### Phase 2: Proof Classification And Preservation Ledger

Assign every discovered test to one semantic proof family, one owning
responsibility, and one or more explicit execution products before movement
begins.

**Relevant subsystems**

- owner crate tests
- certification courtroom and physical-certification drivers
- compile-fail/UI proof
- structural and constitutional checks
- formal-model conformance and process/crash proof

**Relevant APIs and artifacts**

- `ProofFamily`
- `ProofOwner`
- `ProofProductSet`
- `ClassifiedProofInventory`
- `ProofDisposition`
- `ProofPreservationLedger`

**Required structure**

- Proof families distinguish at least owner behavior, owner invariant,
  cross-owner integration, compiler boundary, dependency boundary, structural
  topology, deterministic simulation, fresh-process isolation, formal
  conformance, performance envelope, soak, release qualification, and hardware
  qualification.
- A test has one proof owner even when several products execute it. Product
  membership does not duplicate semantic ownership.
- Every pre-cleanup test receives one disposition:
  `PreserveUnchanged`, `PreserveAndMove`, `PreserveAndConsolidate`,
  `ReplaceWithStrongerProof`, `DuplicateProofRemoveAfterParity`, or
  `InvalidClaimQuarantine`.
- `ReplaceWithStrongerProof` requires the old and new assertion surfaces to be
  compared before deletion.
- `InvalidClaimQuarantine` removes closeout authority but preserves the row,
  rationale, and follow-on owner; it may not be used to make C.1 faster by
  hiding legitimate proof.
- The preservation ledger is checked against the discovered inventory and may
  not contain handwritten phantom tests.

**Warnings**

- A test's current directory does not determine its rightful owner.
- Similar mechanics do not imply shared proof meaning.
- Duplicate assertions may be intentional defense at different trust
  boundaries; deduplication requires semantic equivalence.
- â€œCovered elsewhereâ€ is not a disposition without exact replacement identity
  and assertion parity.

**Test requirements**

- **Preservation completeness test:** every discovered identity must map to one
  owner, one proof family, at least one product or an explicit quarantine, and
  exactly one disposition.
- **Replacement parity test:** for a representative consolidation, deliberately
  omit one old rejection assertion from the replacement suite. The ledger must
  deny removal and identify the lost predicate.
- **Ownership leakage test:** classify an owner-local invariant as generic
  certification support. Dependency and ownership validation must reject the
  widened semantic radius.
- **Duplicate identity test:** assign the same scenario identity to two owners.
  Validation must fail rather than silently count two executions as stronger
  coverage.

**Engineering decisions**

- Proof inventory and preservation ledger are separate: the first says what
  exists; the second says what transformation is admitted.
- Test names are evidence locators, not proof authority. Assertions and subject
  paths are compared during replacement.
- Quarantined fake physical claims remain visible for C.2 rather than being
  erased during cleanup.

**Open questions**

- None.

### Phase 3: Stable Proof Products And Selection Contract

Freeze the meaning of every user-facing test command and lower each request
into an inspectable execution plan before any test process starts.

**Relevant subsystems**

- workspace test-control entrypoint
- change-impact and explicit package selection
- proof profiles and evidence destinations
- support for local, CI, release, and hardware environments

**Relevant APIs and artifacts**

- `StoreProofMode`
- `StoreProofRequest`
- `StoreProofSelection`
- `SelectedProofExecutionPlan`
- `ProofProductUnavailable`
- stable `store-*` commands

**Required structure**

- `store-owner` requires explicit package ownership or mechanically derives one
  from an admitted changed path. Ambiguous ownership denies before build.
- `store-smoke` selects deterministic vertical specimens, a bounded UI smoke
  slice, and structural essentials. It never claims milestone closure.
- `store-ui` selects the complete compiler and dependency-boundary product.
- `store-ci` selects CI-certifiable proof partitions and emits closeout-eligible
  evidence only when every required partition succeeds for the same source and
  profile identity.
- `store-soak`, `store-release`, and `store-hardware` require named profiles and
  cannot fall back to smoke-sized inputs.
- Selection prints and serializes included and excluded packages, targets,
  suites, fixtures, features, profiles, subprocess probes, evidence paths, and
  closeout posture before execution.
- A selection plan is immutable once execution begins. Retry creates a new run
  bound to the original request and records the changed attempt identity.
- Unsupported tools or profiles return typed unavailability before compiling
  unrelated targets.

**Warnings**

- Mode names without mechanical selection are documentation, not architecture.
- Automatic change impact is an optimization; explicit owner selection remains
  available and ambiguity must widen or deny visibly.
- A mode must not change semantic assertions based on time budget. Profiles may
  change scale, seed count, and schedule breadth only where the proof contract
  declares that variation.
- Environment-driven hidden filters are forbidden.

**Test requirements**

- **Selection determinism test:** the same repository identity, request,
  profile, and toolchain must produce the same ordered execution plan and plan
  digest on repeated runs.
- **Under-selection denial test:** remove a required suite from `store-ci` or
  classify a closeout run as successful with one partition absent. Selection or
  aggregation must fail with the missing proof identity.
- **Ambiguous-owner test:** change a shared contract consumed by multiple
  owners. `store-owner` must widen explicitly or deny; it may not select one
  convenient leaf silently.
- **Mode escalation test:** request hardware qualification on an unsupported
  host. The command must return typed unavailability before executing a smoke
  substitute.

**Engineering decisions**

- The product selector plans; Cargo/nextest/test binaries execute. The selector
  does not reinterpret behavioral verdicts.
- Mode membership is versioned repository data validated against the inventory.
- Closeout eligibility is computed from complete product evidence, never from
  command name alone.

**Open questions**

- None.

### Phase 4: Workspace Profiles, Features, And Build-Graph Hygiene

Make the nested Worth Store workspace own honest development/test profiles and
prevent certification authority from inflating ordinary builds through feature
unification.

**Relevant subsystems**

- Worth Store workspace `Cargo.toml`
- `.cargo/config.toml` or equivalent stable command configuration
- crate normal/build/dev dependencies and features
- Windows debug/PDB and incremental behavior

**Relevant APIs and artifacts**

- `StoreBuildProfileIdentity`
- `StoreFeatureLane`
- `ObservedFeatureGraph`
- `BuildGraphPolicyViolation`
- profile-bound execution-plan evidence

**Required structure**

- Worth Store declares its own development and test debug-information policy;
  it does not assume parent-workspace profile inheritance.
- Developer and CI profiles state incremental posture separately. CI artifacts
  must not rely on a local incremental directory for validity.
- Normal production dependencies do not enable certification/test-authority
  features. Test-only feature activation lives at dev/certification boundaries.
- `store-owner` uses the narrowest production-equivalent feature lane capable
  of exercising the owner contract.
- `store-ci --all-features` is not a default reflex. Each partition declares
  the feature set required by the proof it owns, and one explicit feature-
  compatibility product checks admitted combinations.
- Profile and feature identities participate in cache and evidence keys.
- Any optimization or debug setting that changes test semantics or panic,
  overflow, assertion, or instrumentation behavior requires explicit parity
  proof.

**Warnings**

- Reducing debug information is a compile/link optimization, not permission to
  lose usable failure localization.
- Feature cleanup can expose production code that accidentally relied on test
  authority; that is a finding to fix, not a reason to preserve leakage.
- Caching an entire unbounded target directory is not a cache strategy.
- Profile parity must be checked where release-only behavior matters.

**Test requirements**

- **Production-feature isolation test:** generate the normal dependency feature
  graph and assert no certification/test-authority feature reaches a production
  package except through an explicitly admitted certification build edge.
- **Profile parity test:** run a representative deterministic smoke scenario
  under local-test and CI-test profiles; behavioral identity and structural
  counters must match where the profile contract says semantics are equal.
- **Feature-leak mutant:** enable one certification feature on a normal
  dependency. The build-graph gate must identify the introducing manifest edge.
- **Cache-identity drift test:** change toolchain or feature lane without
  changing `Cargo.lock`. Cache identity must change and stale evidence must be
  rejected.

**Engineering decisions**

- Reduced Windows debug information is the preferred ordinary test posture;
  full symbols are reserved for an explicit diagnostic product if needed.
- Feature compatibility is a distinct proof product rather than forcing every
  behavioral suite through every feature union.
- Build profiles belong to the Worth Store workspace because it is the build
  authority for these crates.

**Open questions**

- None.

### Phase 5: Owner-Local Test Topology And Support Ownership

Make the cheapest trustworthy proof align with production ownership by
removing broad fixture dependencies from leaf crates and relocating mechanics
to the narrowest honest scope.

**Relevant subsystems**

- unit and owner integration tests in every Worth Store crate
- `worth-store-test-support`
- physical-certification fixtures and certification scenario support
- dev-dependency and test dependency graph

**Relevant APIs and artifacts**

- `OwnerTestBoundary`
- `OwnerFixtureDependency`
- `TestSupportAuthorityClass`
- `OwnerBuildClosure`
- generated owner-test topology report

**Required structure**

- Owner-local builders, fakes, assertions, and fixtures live inside the owner
  crate's test scope when only that owner uses them.
- Cross-crate support is split only where multiple owners depend on the same
  mechanic for the same semantic reason and share lifecycle/failure behavior.
- Certification-wide world construction remains in certification or physical
  certification; leaf crates do not depend on it for local invariants.
- Test support may construct inputs and drive public contracts. It cannot mint
  production proofs, owner receipts, durability completion, recovery truth, or
  certification verdicts.
- Test dependency cycles and reciprocal normal/dev relationships are removed
  unless a narrow compiler-enforced reason is documented and validated.
- An owner test's build closure is generated from Cargo metadata and compared
  against its admitted owner boundary.
- Shared support names predict their exact mechanic or scenario family; no new
  `common`, `helpers`, `utils`, `world`, or generic `support` bags are admitted.

**Warnings**

- Moving a broad support module into a different crate without narrowing its
  authority is relocation, not cleanup.
- Some duplication is safer than falsely shared fixtures with different
  semantic owners.
- Production code must not be widened merely to make tests convenient.
- A leaf's public-contract integration test may legitimately compile a direct
  dependency; it may not silently compile unrelated sibling owners.

**Test requirements**

- **Owner closure test:** for each leaf owner, generate the packages and
  features compiled by `store-owner`; assert certification crates and unrelated
  physical owners are absent unless an explicit public-contract edge requires
  them.
- **Fixture authority rejection test:** attempt to construct a sealed owner
  proof or certification verdict from shared support. Compilation must fail at
  the constructor boundary.
- **Support-radius mutant:** route one leaf fixture through a broad platform
  support crate. Topology validation must identify the added dependency edge
  and owning test.
- **Behavior preservation test:** before and after moving a representative
  fixture owner-local, execute the same inputs and compare behavioral verdicts,
  assertion identities, and structural counters.

**Engineering decisions**

- `worth-store-test-support` is not automatically deleted, but every retained
  responsibility must earn shared placement. Its current name grants no
  authority.
- Owner-local tests are the default; certification support is an explicit
  higher-radius exception.
- Build closure, not directory appearance, proves locality.

**Open questions**

- None.

### Phase 6: Responsibility-Named Scenario Suite Consolidation

Collapse repeated integration-test executable and linker cost into a small
number of coherent suite binaries while preserving scenario identity,
filterability, failure localization, and certification ownership.

**Relevant subsystems**

- `worth-store-certification` integration targets
- certification courtroom scenario registry
- physical-certification drivers and oracles
- scenario modules and repeated `#[path]` support inclusion

**Relevant APIs and artifacts**

- `CertificationSuiteDeclaration`
- `CertificationScenarioDeclaration`
- `ScenarioIdentity`
- `ScenarioProofContract`
- `ConsolidatedSuiteInventory`
- suite execution and evidence registry

**Required structure**

- The strongly preferred initial suite binaries are responsibility-shaped:
  physical format/integrity, buffer/memory, durability/recovery,
  isolation/scheduling, layout/blob, operations/security, and formal
  conformance. The implementation may adjust a cut when measured dependency or
  failure topology proves a different boundary more honest.
- Each scenario remains a named module with its own production subject, setup,
  oracle, assertions, evidence requirements, modes, and controlled defects.
- Suite entrypoints aggregate declarations and dispatch. They contain no
  scenario business logic, expected-result computation, or owner mutation.
- Repeated `#[path]` inclusion of the same support source is replaced by a
  compiled library boundary or owner-local module where shared lifecycle is
  real.
- Scenarios can be selected by stable scenario identity without creating one
  binary per identity.
- Suite boundaries stop at distinct cost, failure, process, or correctness
  topologies. A giant universal certification executable is forbidden.
- The number of suite binaries is a measured output, not a vanity target; every
  separate executable records the boundary that justifies its link cost.

**Warnings**

- Fewer binaries can reduce parallel execution if suites are internally
  serialized; the runner must preserve test-level concurrency where safe.
- Combining scenarios with conflicting global state, allocator control, panic
  strategy, environment, or process requirements may be dishonest.
- `mod.rs` or a suite entrypoint must not become a scenario implementation bag.
- Consolidation must not change certification oracle ownership.

**Test requirements**

- **Scenario preservation parity test:** execute every migrated scenario before
  and after consolidation against the same seed/profile and compare scenario
  identities, assertion predicates, verdicts, counters, and evidence fields.
- **Failure localization test:** invert one scenario assertion inside a
  consolidated suite. The run must name the exact scenario and predicate, not
  merely the containing suite binary.
- **Shared-source codegen test:** external artifact observation must prove that
  a representative shared support module is compiled once per admitted suite
  boundary rather than textually included across scenario binaries.
- **False-cohesion rejection test:** attempt to merge a process-isolated or
  allocator-global scenario into an incompatible suite. Topology validation
  must require a justified separate executable.

**Engineering decisions**

- Scenario identity is independent of executable identity.
- Suite declarations are data consumed by the runner; suite files aggregate
  and do not implement.
- Consolidation is accepted only after preservation parity, not merely because
  binary count falls.

**Open questions**

- None.

### Phase 7: Standardized Compiler-Boundary And Dependency UI Proof

Replace fragmented dynamic Cargo-project runners with one cache-sharing UI
architecture that preserves exact denial meaning and separates API compile
proof from dependency-topology proof.

**Relevant subsystems**

- compile-fail fixtures across certification and owner crates
- dynamically generated Cargo manifests and per-case target roots
- compiler diagnostic normalization
- boundary/dependency checks that do not require compiling a fixture

**Relevant APIs and artifacts**

- `UiProofSuiteDeclaration`
- `UiFixtureIdentity`
- `ExpectedCompilerDenial`
- `UiProofRunEvidence`
- `DependencyBoundaryPredicate`
- standardized checked diagnostic artifacts

**Required structure**

- One standardized UI harness is selected using representative authority,
  visibility, lifecycle, feature, and dependency fixtures. The accepted choice
  must share dependency compilation, normalize unstable diagnostic fields, and
  preserve expected stderr or equivalent structured denial evidence.
- API misuse, private constructor, typestate skip, and authority-forging cases
  run as compiler UI fixtures.
- Manifest dependency direction, forbidden feature edges, and source-boundary
  rules run through purpose-built metadata/boundary checks when compilation is
  not the strongest proof.
- Fixtures are grouped by responsibility and dependency/feature environment.
  Cases with identical environments share one compilation root.
- Dynamic manifests are generated once per distinct admitted environment, not
  once per fixture, and their canonical identity is recorded.
- Per-case deletion of target directories is forbidden. Cache invalidation is
  bound to toolchain, dependency, feature, profile, and fixture-source identity.
- UI proof is a separate product and a bounded smoke subset may run in
  `store-smoke`; the complete suite runs in `store-ui` and required CI
  partitions.

**Warnings**

- Snapshotting entire unstable compiler messages makes routine toolchain
  upgrades painful; normalize only non-semantic fields.
- Over-normalization can erase the exact denial being proved.
- A source scan is weaker than compile failure where the compiler can enforce
  the boundary.
- A compile-fail case that fails for the wrong reason is not proof.

**Test requirements**

- **Expected-denial parity test:** migrate representative fixtures from every
  existing environment and prove each fails for its declared semantic denial,
  not a missing dependency or unrelated syntax error.
- **Cache-sharing test:** external process and target observation must show one
  shared dependency build for fixtures with the same environment and no unique
  target directory per case.
- **Wrong-reason rejection test:** break an import so a fixture fails before
  reaching the intended private constructor. The harness must reject the
  diagnostic mismatch.
- **Boundary-strength test:** move a manifest-only dependency violation into a
  compile fixture and prove the selected metadata check localizes the actual
  edge more directly; the spec forbids retaining the weaker duplicate as the
  sole authority.

**Engineering decisions**

- The selection between a maintained UI library and a small workspace-owned
  wrapper is made by the representative compatibility specimen, then frozen;
  parallel homegrown runners are deleted after parity.
- Exact semantic denial is part of fixture identity.
- Compiler and metadata proof are distinct products even when `store-ui`
  invokes both.

**Open questions**

- None.

### Phase 8: Honest Fresh-Process And External-Tool Probes

Preserve genuine process, crash, formal-tool, and external-observer boundaries
without paying one integration-test binary per scenario.

**Relevant subsystems**

- process/crash scenario drivers
- child executables used by certification
- formal-model tool invocation
- offline-verifier and independent-observer probes
- environment- and allocator-isolated tests

**Relevant APIs and artifacts**

- `ProcessProbeDeclaration`
- `ProcessRole`
- `ProcessIsolationRequirement`
- `ProcessProbeExecution`
- `ProcessIdentityEvidence`
- `ExternalToolIdentity`

**Required structure**

- Separate binaries are retained for named roles such as writer, crash target,
  recovered runtime, offline verifier, formal checker adapter, or allocator-
  isolated probeâ€”not for each scenario.
- A process probe is parameterized by a sealed scenario/profile input and emits
  structured evidence. The parent runner cannot inject expected runtime truth
  through the input.
- Process roles, executable digest, PID, parent relationship, environment,
  working directory, input artifact identity, output artifact identity, and
  exit/termination mode are recorded.
- Crash proof distinguishes graceful exit, panic unwind, abort, parent kill,
  and OS termination. Only the mode required by the scenario satisfies it.
- External tools are invoked by explicit preflight or certification products
  with version/provenance evidence and timeout/resource posture.
- Shared child binaries are built once per profile/feature identity and reused
  across scenario invocations without sharing runtime heap state.

**Warnings**

- Reusing an executable is safe; reusing its process, singleton, target store,
  or live state across crash scenarios may not be.
- An `Err` or panic inside the parent process is not process death.
- Process probes must not become a second orchestration framework with scenario
  meaning hidden in string arguments.
- External tool absence must be typed unavailability, not a silently skipped
  passing test.

**Test requirements**

- **Fresh-process identity test:** run a representative crash/reopen probe and
  prove writer, recovered runtime, and verifier have distinct process and
  runtime identities while agreeing on the allowed persisted evidence.
- **Live-state leakage test:** attempt to pass decoded expected state or a live
  runtime handle through the probe protocol. Type/serialization boundaries and
  evidence validation must reject it.
- **Termination-mode test:** replace the required parent kill with graceful
  shutdown. The scenario must fail the crash-isolation predicate even if the
  recovered result matches.
- **Probe amortization test:** multiple scenario invocations using one role
  binary must produce one linked child executable identity while retaining
  separate process/evidence identities.

**Engineering decisions**

- Process roles are stable responsibility names; scenario ids remain inputs.
- Structured probe protocols contain intent, fault schedule, and artifact
  pathsâ€”not expected semantic results or authority objects.
- Process proof remains outside owner-local runs unless that owner specifically
  owns the process boundary.

**Open questions**

- None.

### Phase 9: Structural Preflight Extraction And Generated Enforcement

Move repository, boundary, context, source-residue, and topology validation out
of behavioral scenarios into explicit reusable preflight products whose
evidence can be consumed without rerunning nested Cargo work.

**Relevant subsystems**

- boundary checker and agent-context checker
- structural source and manifest scans
- generated context and topology validation
- certification structural preflight
- proof inventory and preservation validation

**Relevant APIs and artifacts**

- `StructuralPreflightRequest`
- `StructuralPreflightPlan`
- `StructuralPreflightEvidence`
- `PreflightEvidenceIdentity`
- `PreflightEvidenceFreshness`
- `StructuralPredicateFailure`

**Required structure**

- Boundary, agent-context, inventory, preservation, feature, dependency,
  line-cap, naming, and admitted residue checks are explicit named predicates.
- One preflight execution produces a machine-checkable bundle keyed by source,
  tool binaries/versions, configuration, manifests, and predicate set.
- Behavioral suites may require a fresh compatible preflight bundle, but they
  do not invoke Cargo to rebuild or rerun the tools internally.
- Freshness is checked before behavioral execution and stale evidence returns a
  typed denial naming the changed identity.
- Source hashing is scoped to declared inputs per predicate rather than
  recursively hashing unrelated repositories on every scenario.
- Preflight has a developer-smoke subset and a complete CI product. No
  predicate required for closeout exists only inside an ignored unit test.
- Generated reports are projections. They cannot authorize production Store
  behavior or replace behavioral proof.

**Warnings**

- Extracting preflight must not make it optional for closeout products.
- Reusing stale evidence is worse than rerunning because it creates false
  structural confidence.
- One giant â€œpreflight passedâ€ boolean destroys predicate localization.
- Hashing every file for every predicate hides the actual invalidation basis.

**Test requirements**

- **Fresh-evidence reuse test:** run complete preflight once, then execute two
  compatible behavioral partitions. Both must consume the same bundle identity
  without launching nested boundary or context Cargo processes.
- **Stale-evidence rejection test:** change one manifest or boundary config
  after preflight. The consuming product must reject the bundle and name the
  invalidated predicate inputs.
- **Predicate localization test:** introduce one forbidden dependency and one
  stale generated context independently. The bundle must preserve separate
  failures rather than one generic preflight error.
- **Behavioral-substitution rejection test:** provide a green preflight bundle
  while a behavioral assertion is inverted. Closeout aggregation must still
  fail the behavioral product.

**Engineering decisions**

- Preflight is a first-class proof product with reusable evidence, not a
  library helper called by scenarios.
- Each predicate declares its source/config/tool invalidation basis.
- Constitutional tools remain their own authorities; C.1 orchestrates and
  records them without reimplementing their law.

**Open questions**

- None.

### Phase 10: Execution Runner, Concurrency, And Evidence Accounting

Execute an immutable proof plan with bounded concurrency, correct isolation,
fail-fast policy that preserves evidence, and structural cost accounting.

**Relevant subsystems**

- Cargo and optional nextest execution
- suite and process-probe scheduling
- evidence collection and run aggregation
- console progress and machine-readable output

**Relevant APIs and artifacts**

- `ProofExecutionUnit`
- `ProofExecutionIsolation`
- `ProofExecutionSchedule`
- `ExecutedProofRun`
- `ObservedProofRunCost`
- `ProofRunAttempt`

**Required structure**

- The selected plan lowers into execution units with declared packages,
  targets, cases, environment, resources, isolation, dependencies, timeout,
  retry posture, and evidence outputs.
- Units run concurrently only when their target roots, process-global state,
  environment, allocator control, store roots, ports, and external tools are
  structurally disjoint.
- The runner does not discover semantic test dependencies during execution.
  Preflight and prerequisite edges are planned first.
- Failure stops dependent work but preserves completed and failed evidence.
  Independent units may continue according to the product's declared policy.
- Retry is explicit and never overwrites the first failure. Flake posture is a
  structured outcome, not â€œpassed on retry.â€
- External observation records compile/link/test/subprocess/artifact breadth
  and binds it to execution units and attempts.
- Console output is derived from structured run state. Logs are not the
  authoritative evidence format.

**Warnings**

- Parallelism without isolation can create nondeterministic false failures or
  hidden shared-state success.
- Unlimited jobs can make wall time better while destroying machine stability
  and iteration predictability.
- Fail-fast must not discard the exact first failure needed for diagnosis.
- Retrying all failures hides deterministic defects and inflates cost.

**Test requirements**

- **Schedule determinism test:** identical plans on the same resource profile
  must produce the same dependency ordering and isolation assignments, while
  allowing nondeterministic completion timestamps to remain diagnostic only.
- **Isolation denial test:** declare two units that share a store root or
  allocator-global control as parallel. Planning must deny or serialize them
  before execution.
- **Failure evidence test:** force one unit to fail while an independent unit
  succeeds. The run must retain both verdicts, skip only dependents, and expose
  no generic â€œcommand failedâ€ collapse.
- **Flake honesty test:** make a unit fail once and pass on one admitted retry.
  The final posture must remain flaky/indeterminate according to policy rather
  than green.

**Engineering decisions**

- Nextest is preferred for compatible Rust test-unit scheduling after suite
  consolidation; the workspace runner remains the authority for multi-product
  plans and external process probes.
- Resource concurrency is configured by declared machine profile and execution
  unit requirements, not a universal thread count.
- Evidence is append-only per attempt.

**Open questions**

- None.

### Phase 11: CI Partitioning, Sharding, And Cache Identity

Turn CI into explicit proof-family jobs with narrow invalidation and aggregation
instead of compiling all targets/all features twice on every platform.

**Relevant subsystems**

- root CI workflow
- Worth Store proof-product entrypoints
- Rust/compiler and artifact caches
- Linux and Windows job matrix
- closeout evidence aggregation

**Relevant APIs and artifacts**

- `CiProofPartition`
- `CiShardPlan`
- `CiCacheIdentity`
- `CiPartitionEvidence`
- `CiCertificationAggregate`
- `MissingCiProofPartition`

**Required structure**

- CI partitions at least owner/unit, scenario certification, UI/dependency,
  fresh-process crash/recovery, structural preflight, and formal/external-tool
  proof.
- Clippy and formatting operate on responsibility-appropriate targets; they do
  not blindly rebuild every scenario binary after shared suite/support source
  has already been linted.
- OS matrices are claim-driven. Cross-platform semantic and filesystem claims
  run where required; OS-independent pure proof need not duplicate by habit.
- Scenario partitions may shard by stable suite/scenario weights after
  consolidation. Sharding does not split one scenario's evidence across
  incompatible profiles.
- Cache keys include OS, architecture, Rust toolchain, profile, feature lane,
  lockfile, relevant configuration, and proof partition.
- CI does not preserve an ever-growing entire target directory as one immutable
  cache object. Compiler-object caching may be introduced when provenance and
  invalidation are explicit.
- Final CI certification requires all mandated partition evidence bound to one
  source identity; a rerun of one partition retains attempt history.

**Warnings**

- Fewer jobs are not automatically cheaper if each job rebuilds the world.
- More shards can multiply compile/link work unless compiled artifacts are
  shared or suite boundaries are chosen honestly.
- Caches are accelerators, not proof artifacts.
- Skipping Windows because it is slower would erase relevant physical and PDB
  behavior rather than fix it.

**Test requirements**

- **Partition coverage test:** every CI-certifiable proof identity maps to at
  least one required partition, and aggregation denies closure when any
  partition is absent or stale.
- **Cross-partition duplication test:** external observation identifies targets
  compiled in multiple partitions. Every duplicate requires an explicit
  profile/feature/OS reason or fails the cost contract.
- **Cache-poisoning test:** reuse a cache after changing toolchain, feature
  lane, or profile with unchanged lockfile. Cache identity must differ and
  evidence provenance must prevent stale promotion.
- **OS-claim test:** remove the Windows lane for a filesystem/profile claim.
  Coverage validation must fail even if Linux remains green.

**Engineering decisions**

- CI organization follows proof and platform claims, not crate count.
- Compilation reuse is optimized only after feature/profile equivalence is
  explicit.
- Aggregate closeout evidence is derived from partition bundles, not workflow
  job names.

**Open questions**

- None.

### Phase 12: Artifact Lifecycle, Cleanup Policy, And Disk-Bloat Prevention

Make build artifacts bounded and explainable after the target graph is fixed,
without destructive surprise or cache folklore.

**Relevant subsystems**

- local Worth Store target roots
- incremental, PDB, executable, rlib/rmeta, UI, and process-probe artifacts
- CI cache retention
- developer cleanup and diagnostic preservation

**Relevant APIs and artifacts**

- `BuildArtifactClass`
- `BuildArtifactInventory`
- `BuildArtifactRetentionPolicy`
- `BuildArtifactCleanupPlan`
- `BuildArtifactCleanupReceipt`
- `ProtectedDiagnosticArtifact`

**Required structure**

- Artifact inventory distinguishes current reusable objects, stale hashed
  variants, incremental state, symbols, evidence bundles, UI expectations,
  process outputs, and diagnostic captures.
- Cleanup is planned against an explicit target root and prints resolved
  absolute paths, expected classes, counts, and logical bytes before mutation.
- Evidence bundles and checked UI expectations are not build-cache residue and
  follow their own retention law.
- Local cleanup has inspect, plan, execute, and receipt stages. No automatic
  recursive deletion occurs outside an admitted Worth Store target/evidence
  root.
- CI cache retention is bounded by key family and age/usage policy where the
  platform supports it.
- Ordinary builds prevent uncontrolled symbol/incremental growth through the
  Phase 4 profile decisions and stable target/profile topology.
- One-time historical cleanup occurs only after user approval and after the
  new graph is validated; C.1 implementation must not silently delete existing
  artifacts.

**Warnings**

- Disk cleanup before structural repair only creates temporary relief.
- Logical size and allocated size are distinct observations.
- Deleting the target directory is not a recurring lifecycle policy.
- Cleanup must preserve failure artifacts needed to diagnose the current run.

**Test requirements**

- **Dry-run parity test:** cleanup planning and execution against a disposable
  synthetic target root must report the same selected paths/classes; execution
  may not discover additional deletion targets.
- **Root-confinement test:** introduce a symlink/junction or path traversal
  toward a location outside the admitted root. Planning must deny before any
  deletion.
- **Evidence-preservation test:** place build residue beside protected evidence
  in a disposable hierarchy. Cleanup removes only the declared build classes
  and emits exact retained evidence identities.
- **Bloat-regression test:** repeat a fixed warm smoke cycle across several
  source edits. Hashed executable/PDB/incremental growth must remain within the
  declared profile contract or identify the invalidation source.

**Engineering decisions**

- Artifact lifecycle is observable policy, not a hidden post-test hook.
- Destructive cleanup always remains explicit and root-confined.
- C.1 may provide the plan and command but does not execute historical cleanup
  without the user's authorization.

**Open questions**

- None.

### Phase 13: Preservation, Mutation Sensitivity, DX Closeout, And C.2 Handoff

Prove that the reconstructed test architecture is faster for the right reason,
still catches meaningful defects, and gives C.2 an authoritative map of every
remaining physical claim.

**Relevant subsystems**

- all proof products and execution plans
- pre/post preservation ledger
- controlled defect program
- cost evidence and developer commands
- C.2 executable-reality audit handoff

**Relevant APIs and artifacts**

- `PreservationCheckedProofRun`
- `ProofMutationSensitivityReport`
- `DeveloperIterationEnvelope`
- `TestArchitectureCloseoutBundle`
- sealed `C2TestArchitectureReadiness`

**Required structure**

- Every pre-C.1 test identity and assertion predicate is preserved, replaced by
  stronger named proof, or quarantined with explicit non-closure posture.
- Closeout runs the top-level Non-Fake Acceptance Setup against clean and warm
  roots and records structural cost before/after by product.
- The warm developer-smoke target is under one minute on the declared reference
  profile. Owner checks target seconds and must exclude unrelated owners by
  dependency observation.
- Time targets do not excuse missing proof; any product exceeding its target
  remains valid evidence but blocks C.1 until the structural cause is fixed or
  the roadmap contract is explicitly amended.
- Controlled defects cover at least lost UI denial, inverted scenario
  assertion, broad support dependency, hidden nested Cargo, omitted CI
  partition, same-process crash substitute, stale preflight evidence, and
  feature leakage.
- The closeout bundle contains plan digests, product verdicts, preservation
  ledger, mutation report, cold/warm cost evidence, dependency closures,
  artifact footprint, commands, profiles, and residual quarantines.
- Closeout assembly re-derives the current proof inventory, owner closures,
  suite topology, preservation verdict, and mutation matrix from their
  production validators. Its manifest cannot substitute arbitrary files or a
  previously serialized verdict for those authorities.
- CI closeout is certified from raw source-bound partition bundles. A prebuilt
  aggregate is an output for inspection, never an assembly input.
- Each developer-iteration specimen is one tracked source edit against `HEAD`.
  The edited file must be the only dirty Store source, its original bytes must
  match the committed worktree representation, and the cold/warm plan and run
  content seals must validate before cost evidence is admitted.
- Assembly runs only from a clean Store source tree and requires the current
  revision, lockfile, toolchain, OS, and architecture to agree with the five
  iteration specimens. CI partitions must agree with the current revision,
  clean source-tree digest, and lockfile through one source identity.
- `C2TestArchitectureReadiness` exposes the validated proof inventory,
  quarantined-claim inventory, production-subject map, and stable command
  contracts. It grants no physical runtime authority.

**Warnings**

- A single impressive timing number cannot close C.1.
- A controlled defect that fails an unrelated build step does not prove the
  intended lane is sensitive.
- Quarantined fake claims must reach C.2; cleanup may not erase the evidence of
  why they were quarantined.
- The C.2 handoff is test-architecture readiness, not S.1 physical readiness.

**Test requirements**

- **Full preservation test:** compare pre/post inventories and assertion
  predicates; every difference must have an admitted disposition and reachable
  replacement or explicit quarantine.
- **Mutation localization matrix:** inject every required defect independently;
  each must fail its named proof product and predicate while unrelated products
  remain interpretable.
- **Iteration-envelope test:** perform the five top-level edit/run cases and
  assert target, dependency, compiler/linker, subprocess, artifact, and warm
  elapsed envelopes together.
- **Fake-speed rejection test:** remove an expensive scenario from every mode.
  Warm timing may improve, but preservation and product-coverage predicates
  must block closeout.
- **C.2 handoff forgery test:** attempt to construct readiness from timing or a
  green smoke run without preservation and mutation evidence. Construction
  must be unavailable.

**Engineering decisions**

- C.1 closeout is a conjunction of preservation, sensitivity, topology, DX,
  and structural costâ€”not a benchmark threshold.
- Reference-profile timing is recorded alongside portable structural bounds so
  different machines can interpret the result honestly.
- Residual fake physical claims are handed to C.2 as quarantined audit inputs.

**Closeout DX contract**

The assembly manifest names observations that cannot be re-derived locally; it
does not name files that pretend to be current repository verdicts:

```json
{
  "schema_version": 2,
  "developer_iteration_manifest": ".store-proof/evidence/closeout/inputs/iteration.json",
  "ci_evidence_root": "C:/evidence/worth-store-ci-partitions",
  "artifact_inventory": ".store-proof/evidence/artifacts/inventories/<identity>.json",
  "artifact_cleanup_plan": ".store-proof/evidence/artifacts/plans/<identity>.json",
  "stable_commands": [
    {
      "product": "store-smoke",
      "command": ["cargo", "store-smoke"],
      "selection_contract": "bounded plumbing plus named behavioral specimens"
    }
  ]
}
```

`cargo store-closeout assemble --manifest <path>` re-runs preservation and the
controlled-defect matrix, derives canonical topology references, certifies the
raw CI lanes, reconstructs the five iteration cases from sealed plans/runs,
and only then issues the closeout bundle and C.2 readiness handoff.

**Retrospective historical-evidence decision**

- This reconstruction did not capture an exact pre-C.1 libtest/rustdoc listing,
  pre-C.1 cold/warm process observations, or pre-consolidation same-seed
  scenario transcripts before the relevant source topology changed. Git
  history also contains S.10 hardening and C.1 consolidation in one commit, so
  no historical revision represents the missing intermediate state.
- C.1 must never claim exact pre-C.1 proof cardinality, a measured pre/post
  speedup, or same-seed pre-consolidation behavioral parity. Those claims are
  permanently unavailable rather than reconstructed from current runs.
- The one admitted retrospective policy is
  `test-control/c1-historical-evidence-policy.json`. It quarantines the unknown
  historical executable universe and behavioral-parity claim for C.2, then
  requires exact current executable reverse parity, current full-body behavior
  seals, reachability for every identity and predicate in the frozen known
  baseline, the complete controlled-defect matrix, and the clean/warm five-case
  iteration envelope.
- The full-preservation predicate therefore means every row in the frozen
  known baseline is preserved, replaced, or explicitly quarantined. It does
  not convert the known-incomplete source-derived baseline into an assertion
  about tests that were never captured.
- This exception is specific to the already-lost C.1 history. Future milestone
  cleanup may not cite it to skip a contemporaneous baseline.

**Open questions**

- None.

## Opinionated Directory Target

The implementation should converge toward this responsibility shape. Exact
cuts may change when Phase 1 or Phase 5 evidence proves a different ownership
boundary, but generic replacement bags are not allowed.

```text
workspaces/worth-store/
  .cargo/
    config.toml                         stable store-* command entrypoints
  Cargo.toml                           Store-owned profiles and workspace law

  tools/store-proof-control/
    src/
      discovery/                       repository/Cargo/test surface discovery
      classification/                  proof owner, family, product, disposition
      selection/                       immutable proof execution planning
      execution/                       unit scheduling and attempt lifecycle
      observation/                     process/build/artifact cost observation
      preservation/                    pre/post assertion and reachability proof
      evidence/                        machine-checkable run bundles
      cli/                             store-* command presentation only

  crates/worth-store-certification/
    src/
      courtroom/                       verdict and proof meaning
      scenario_registry/               declarations, not scenario mechanics
      evidence/                        certification evidence composition
    tests/
      suites/
        physical_format.rs
        physical_integrity.rs
        buffer_residency.rs
        durability_recovery.rs
        physical_isolation.rs
        io_scheduling.rs
        layout_access.rs
        blob_chunks.rs
        operational_recovery.rs
        formal_conformance.rs
      ui/
        authority/
        lifecycle/
        visibility/
        dependency/
        feature_boundary/
        recovery_boundary/

  crates/worth-store-physical-certification/
    src/
      drivers/                          production-boundary mechanics
      process_probe/
        writer/                         writer process role
        recovery/                       fresh runtime process role
        offline_verifier/               independent read-only role
        crash_target/                   deliberately terminated role
      fixtures/                         only genuinely cross-owner physical setup

  <owner crate>/
    src/...                             production owner
    tests/...                           owner public-contract tests
    src/<responsibility>/tests/...      owner-private invariant tests
```

Rules for this target:

- `store-proof-control` is repository/test infrastructure and may not become a
  Store production dependency.
- Final package naming must pass the workspace naming and boundary enforcement
  already governing tools before creation.
- `cli` parses and renders; it does not discover, classify, select, execute, or
  decide verdicts.
- Suite entry files aggregate scenario declarations only.
- Scenario implementation remains under responsibility-shaped courtroom or
  driver modules, not beside generic `support.rs` or `helpers.rs` files.
- Process-probe directories are named by role because their executable
  identity and lifecycle are the boundary they own.
- UI fixture grouping follows the compiler-enforced distinction under test,
  not the milestone that introduced it.

## Must Ship

- complete discovered and classified proof inventory
- reviewed preservation ledger for every pre-C.1 test and assertion predicate
- stable `store-owner`, `store-smoke`, `store-ui`, `store-ci`, `store-soak`,
  `store-release`, and `store-hardware` products
- Worth Store-owned build profiles and explicit feature lanes
- owner-local test support and mechanically checked build closures
- responsibility-named consolidated scenario suites
- one cache-sharing UI proof architecture
- parameterized, role-named process probes for genuine isolation boundaries
- reusable structural preflight evidence with freshness enforcement
- immutable execution planning, bounded scheduling, and append-only attempts
- proof-family CI partitions and honest cache identity
- inspected, root-confined artifact lifecycle policy
- preservation, mutation-sensitivity, cold/warm cost, and DX closeout evidence
- sealed C.2 test-architecture readiness handoff

## Must Preserve

- every legitimate behavioral assertion and rejection predicate
- every compiler-enforced authority, lifecycle, visibility, and dependency law
- genuine process death, fresh-runtime, and independent-verifier boundaries
- certification ownership of closeout verdicts
- owner ownership of local truth and invariants
- formal-tool and controlled-defect evidence where currently legitimate
- deterministic seeds, profiles, transcripts, counters, and evidence identity
- physical proof requirements even where C.2 will later determine that current
  implementation does not satisfy them
- the distinction between smoke plumbing and certification readiness

## Acceptance Evidence

C.1 closeout requires one `TestArchitectureCloseoutBundle` containing:

- repository, source, toolchain, lockfile, profile, feature, OS, filesystem,
  CPU, storage, and target-root identities
- complete pre/post target and test inventory
- proof ownership, family, product, and disposition rows
- assertion-predicate preservation report
- owner build-closure and shared-support topology report
- scenario suite and executable-boundary justification report
- UI fixture/environment and expected-denial report
- process-probe identity and termination-mode report
- structural-preflight predicates and freshness basis
- CI partition, shard, OS-claim, and cache-identity matrix
- cold and warm compile/link/test/subprocess/artifact observations
- artifact lifecycle inventory and dry-run cleanup proof
- controlled-defect sensitivity and localization matrix
- exact rerun commands for every product
- explicit quarantined physical claims passed to C.2
- closeout predicate results and sealed handoff identity

## False-Completion Gates

C.1 is not complete if any of the following is true:

- warm timing improved because tests became unreachable, ignored, or
  unselected
- test files were merged but shared support is still textually recompiled or
  every scenario remains a separate target
- a universal certification binary replaced the previous target explosion
- compile-fail runners still create and delete per-case target directories
- behavioral tests still invoke nested Cargo preflight work
- owner-local tests still pull the certification courtroom or unrelated
  physical owners through broad support
- normal production dependencies still enable certification authority
- `--workspace --all-targets --all-features` remains the only meaningful CI
  proof shape
- cache keys omit toolchain, profile, feature lane, or OS identity
- process death is represented by error return or same-process recovery
- exact compiler denial meaning was weakened to â€œcompilation failedâ€
- a timing threshold is asserted without structural breadth evidence
- artifact cleanup can resolve outside its admitted root or runs implicitly
- preflight or test evidence can be reused after its inputs change
- the preservation ledger contains an unclassified or unreachable test
- mutation tests fail for unrelated reasons or are reported green after retry
- C.2 readiness can be constructed from smoke success or timing alone

## Sequencing Notes

- Phases are implemented in order.
- Phases 1 and 2 freeze discovery, classification, and preservation before any
  destructive movement.
- Phase 3 freezes proof-product meaning before profiles, suites, UI, or CI are
  rearranged.
- Phase 4 removes feature/profile inflation before measuring final target
  topology.
- Phase 5 narrows support ownership before Phase 6 shares suite compilation.
- Phase 6 establishes the scenario module/binary boundary consumed by Phases
  8, 10, and 11.
- Phase 7 closes compiler-boundary proof independently of behavioral suites.
- Phase 8 preserves process boundaries before preflight and execution
  orchestration are generalized.
- Phase 9 extracts structural work before Phase 10 finalizes execution.
- Phase 10 is the local execution authority consumed by CI in Phase 11.
- Phase 12 plans cleanup only after the new graph is stable.
- Phase 13 is the only closeout and C.2 handoff authority.

## C.2 Handoff

The handoff to C.2 contains:

```text
C2TestArchitectureReadiness
  proof_inventory
  preserved_assertion_inventory
  quarantined_physical_claims
  production_subject_map
  owner_build_closures
  stable_proof_products
  process_probe_roles
  preflight_predicates
  evidence_and_cost_contracts
```

The handoff proves only that the test architecture is classified, preserved,
fast enough to iterate, sensitive to named defects, and capable of expressing
honest future proof. C.2 must still trace whether the physical production code
actually performs the effects its tests claim.
