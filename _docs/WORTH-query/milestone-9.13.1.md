# Milestone 9.13.1: Authority-Local Certification And Livable Query Iteration

## Goal

Make `worth-query` development and certification authority-local,
deterministic, and fast enough for ordinary iteration without weakening the
compiler-visible and hostile proof that protects Query's public authority
boundaries. Establish the proof ownership map, execution lanes, and cost
evidence that the later production crate split will consume.

## Why This Milestone Exists

Query has reached a scale where its test topology obscures rather than teaches
its authority topology. The current library harness aggregates 3,098 tests,
including 38 `trybuild::TestCases` constructions across 37 source files and
1,106 Rust UI fixtures. A warm authority-unaware run measured 126.6 seconds for
the non-trybuild portion and 399.2 seconds for trybuild-containing modules;
cold or cache-contaminated all-up runs exceeded ten minutes.

The cost is not explained by proof volume alone. Three compile-test suites
mutate `HOME`, temporary-directory variables, and `CARGO_TARGET_DIR` as
process-global state while the Rust test harness executes suites concurrently.
Unrelated suites have been observed compiling into another suite's isolated
target directory. The fixture tree also contains 370 flat root fixtures and 96
`.stderr` files without corresponding Rust fixtures. These are structural
failures: suite outcome and cache reuse can depend on thread order, and a local
edit has no mechanically reliable way to select the complete proof owned by
the affected authority.

Milestone 9.14 would add another large compiler-visible authority surface. The
production authority split planned after this milestone would also be unsafe
if the existing tests could not first identify which authority they falsify.
This milestone therefore fixes proof execution and ownership before either
change proceeds. It changes no Query product meaning and claims no production
crate decomposition.

## Governing Summaries

- `MENTALITY.md` protects foundation-first work under an explicit hostile
  constraint. This milestone must repair the structural test foundation rather
  than hide latency behind narrower default commands or delete expensive proof.
- `arch_laws.md` protects contractual facades, compiler-enforced authority, and
  named measurement boundaries. Compile-fail proof remains compiler proof, but
  its runner and ownership must become explicit and deterministic.
- `composition_laws.md` protects one predictable semantic responsibility per
  production and test file. Fixture manifests, runners, and suites must be
  named for the invariant family they own rather than becoming new generic
  harness buckets.
- `domain_structure_laws.md` protects physical structure as an encoding of
  authority, lifecycle, truth source, and dependency direction. Test topology
  must falsify the same authority boundaries the production split will later
  enforce, without prematurely inventing those production crates.
- `perf_laws.md` protects honest boundary-local cost and structural measurement.
  This milestone must expose compiler invocations, selected cases, cache roots,
  and fixture breadth beside wall-clock budgets; elapsed time alone is not
  sufficient evidence.
- `WORTH_query_roadmap.md` protects one canonical Query meaning across
  declaration, lowering, execution, live maintenance, history, policy, and
  provider certification. Milestone 9.13 establishes installed-domain and
  Foundational-native authority, while 9.14 consumes it. This milestone belongs
  between them so the existing proof portfolio is trustworthy and selectable
  before the next authority surface is added.

## Adversarial Constraint

For any edit confined to one Query authority, the authority-local developer
lane must select every regular, hostile, compile-pass, and compile-fail proof
owned by that authority and no unrelated certification work. Running those
proofs alone, in any suite order, under any supported test-thread count, and as
part of the all-up certification lane must produce identical selected-case
identity, diagnostics, pass/fail outcomes, and coverage digests.

No test may mutate process-global environment in a concurrent harness, choose
an order-dependent target directory, rely on an unowned fixture, execute the
same proof through overlapping manifests, or disappear from certification
because a filter, glob, cache key, or local fast-path command failed open.
Invalid selection and stale or foreign cache state must deny before a nested
Cargo or compiler process starts. Improving iteration time may not make any
currently enforced public construction, substitution, phase-ordering,
move-only, facade, or authority denial representable.

## Product Decision Lock

- Milestone 9.13.1 reorganizes and executes proof; it does not change Query
  product semantics, public capability meaning, runtime authority, or Store
  handoff contracts.
- The production `worth-query` crate is not split here. This milestone emits
  the authority ownership and dependency evidence consumed by the subsequent
  crate-decomposition milestone.
- Compile-fail coverage is retained wherever the compiler is the owning
  enforcement mechanism. A slow proof may be moved, sharded, or deduplicated;
  it may not be deleted merely because it is slow.
- Every removed or consolidated probe requires an explicit invariant-family
  mapping and replacement evidence proving equal or stronger detection.
- Trybuild execution is absent from the ordinary library test lane. Compile
  tests run only through explicit authority-addressable certification targets
  or runner-owned child processes.
- Environment and cache configuration is supplied to child processes before
  launch. Test functions may not mutate process-global `HOME`, `USERPROFILE`,
  `TMP`, `TEMP`, `CARGO_HOME`, `RUSTUP_HOME`, or `CARGO_TARGET_DIR`.
- One declared fixture belongs to exactly one owning authority and one proof
  family. Aggregate inventories, coverage reports, and execution plans are
  derived and rebuildable.
- Local fast lanes are not closure evidence. Full certification remains the
  merge and closeout gate, but its outcome and cost must be deterministic.

## Phase Plan

### Phase 1: Proof Inventory And Authority Ownership Map

Freeze one source-backed inventory of what Query currently proves and which
semantic authority each proof falsifies. The inventory is evidence for later
movement; it is not permission to classify tests by milestone provenance or
filesystem convenience.

**Relevant subsystems**

- `crates/worth-query/src/integration_tests.rs` and crate-local unit tests
- `crates/worth-query/tests` and `crates/worth-query/tests/ui`
- facade snapshots, prohibition registries, residue audits, support matrices,
  reference-consumer checks, and source-backed closeout certification
- Cargo test configuration and any nested Cargo/trybuild target directories

**Relevant APIs and artifacts**

- stable `QueryProofCaseId`, `QueryProofFamilyId`, and
  `QueryProofAuthorityId`-equivalent identities owned by certification tooling
- an authoritative proof inventory carrying source path, expected-output path,
  proof kind, authority owner, invariant family, lane posture, feature/target
  inputs, and replacement lineage where applicable
- a derived authority dependency and proof-coverage report suitable as input to
  the later production crate split
- baseline cost evidence for cold, warm, authority-local, and all-up execution

**Warnings**

- Directory names and historical milestone names are discovery inputs, not
  authority truth. Ownership must follow the production capability that mints,
  admits, executes, or exposes the protected artifact.
- A single probe may exercise multiple behaviors but must still have one
  primary invariant owner. Cross-authority prerequisites are dependencies, not
  duplicate ownership.
- Counting files is not coverage. The inventory must name the forbidden or
  required capability each case proves.
- Current failures and orphan artifacts remain evidence to classify; baseline
  capture must not silently bless them as acceptable closure.

**Test requirements**

- Inventory convergence test: independent filesystem discovery, harness-source
  discovery, and authoritative inventory derivation converge on identical case
  identities, proof kinds, source paths, expected-output paths, and exact
  counts.
- Ownership rejection test: duplicate case IDs, duplicate fixture ownership,
  missing authority, missing invariant family, overlapping glob expansion, and
  unclassified test functions deny inventory admission with typed findings.
- Baseline replay test: repeated inventory construction under reversed file
  order and randomized directory enumeration produces a byte-identical
  canonical coverage digest.
- Residue localization test: every orphan `.stderr`, Rust fixture lacking an
  admitted pass/fail posture, and harness reference to a missing source is
  reported against one exact path without starting test execution.

**Engineering decisions**

- Per-authority declarations are authoritative and disjoint. The all-up
  registry, lane plans, coverage summaries, and reports are derived from them.
- Proof identities are semantic and stable across file movement; paths remain
  retained source coordinates but do not define identity alone.
- The inventory distinguishes unit behavior, runtime integration, property or
  convergence proof, source audit, compile-pass transcript, compile-fail
  denial, consumer adoption, and full certification.
- The phase records the measured starting point, including 3,098 library
  tests, 38 trybuild harness constructions, 1,106 Rust UI fixtures, 370 flat
  root fixtures, 96 orphan `.stderr` files, and the observed warm/cold timing
  posture. Counts are remeasured by implementation rather than copied as
  permanent constants.

**Open questions**

- Exact production crate names and final dependency edges remain deliberately
  open until the later authority-decomposition milestone. This phase records
  semantic owners without prematurely freezing package topology.

### Phase 2: Deterministic Compile-Test Process Isolation

Move compiler-boundary execution out of the concurrent ordinary library
harness and give one runner explicit ownership of process environment, cache
identity, ordering, and child-process lifecycle.

**Relevant subsystems**

- Cargo test targets for `worth-query`
- trybuild harness construction and nested Cargo execution
- environment, temporary directory, compiler target, feature, and cache-key
  selection
- local developer runner and CI shard invocation

**Relevant APIs and artifacts**

- an explicit compile-certification invocation accepting one admitted
  authority or the all-authority aggregate
- immutable `CompileTestExecutionPlan`-equivalent artifact derived from the
  Phase 1 inventory
- typed environment and cache identity containing toolchain, target triple,
  profile, feature set, dependency-lock identity, and compiler-relevant flags
- execution receipt with selected cases, Cargo invocations, compiler probes,
  cache roots, cache hits/misses, outcomes, and elapsed measurements

**Warnings**

- Serializing the entire existing library harness with `--test-threads=1`
  would hide the environment race while making iteration worse. Isolation must
  occur at the child-process boundary.
- A unique throwaway target directory per suite is deterministic but defeats
  compilation reuse. Cache isolation and cache reuse must be keyed by semantic
  compiler inputs, not suite names or whichever test ran first.
- Concurrent CI shards may use separate physical cache roots, but equivalent
  cache identities must still produce equivalent execution plans and outcomes.
- trybuild's expected diagnostic text remains presentation evidence. Case
  identity and authority ownership must not depend on unstable compiler output
  formatting.

**Test requirements**

- Order convergence test: execute representative compile-pass and compile-fail
  authorities in forward, reverse, and randomized order and assert identical
  case selection, outcomes, diagnostics normalization, coverage digests, and
  structural counters.
- Environment sabotage test: attempt process-global mutation from a suite,
  inject a foreign `CARGO_TARGET_DIR`, and race two authority plans; the runner
  rejects the invalid configuration before nested Cargo starts and records
  exact-zero compiler probes.
- Cache identity test: equivalent toolchain/target/profile/feature inputs reuse
  one admitted cache identity, while one-field drift creates a distinct cache
  identity and cannot consume the foreign artifacts.
- Thread-count parity test: ordinary library tests at supported thread counts
  and compile certification in its child-process topology produce identical
  semantic results without environment leakage between them.

**Engineering decisions**

- Ordinary `cargo test -p worth-query --lib` contains no trybuild execution.
- The compile-test runner owns environment before child creation and never
  changes the parent process environment after concurrent test execution has
  begun.
- Authority plans may run independently. All-up execution is the canonical
  ordered or safely sharded composition of the same plans, not a separate
  umbrella glob with different semantics.
- Structural execution counters are authoritative cost evidence; wall time is
  retained as workstation/CI experience evidence and never used to infer which
  proof ran.

**Open questions**

- None.

### Phase 3: Authority-Owned Fixture Manifests And Residue Closure

Replace broad and overlapping fixture discovery with disjoint authority-owned
manifests, migrate the flat UI root into responsibility-predicting topology,
and close every orphan, missing, and multiply owned artifact.

**Relevant subsystems**

- compile-pass and compile-fail fixture trees
- expected compiler-output artifacts
- authority manifests, aggregate registry derivation, and fixture-path
  migration support
- prohibition and facade-boundary certification families

**Relevant APIs and artifacts**

- per-authority fixture manifests with stable case identity and explicit pass
  or fail posture
- derived aggregate compile-certification registry
- path-migration ledger from legacy flat fixtures to semantic owners, deleted
  after closeout once no compatibility lookup remains
- typed fixture-admission findings for missing source, unexpected output,
  orphan output, duplicate case, duplicate invariant, and foreign authority

**Warnings**

- A central `all_tests`, `common`, or `misc` manifest would recreate the
  monolith in data form. Aggregate views must be generated from narrow owners.
- Moving a file does not prove ownership. Each migrated fixture needs an
  invariant-family row and a source capability owner.
- Pass fixtures and fail fixtures have different expected-output contracts;
  absence of `.stderr` is valid only for an explicitly admitted pass case.
- Path aliases and fallback globbing must not survive closeout. They would make
  one fixture reachable through multiple truths.

**Test requirements**

- Manifest/aggregate parity test: the union of admitted authority manifests is
  disjoint and byte-identical to the derived all-up registry regardless of
  manifest declaration order.
- Fixture residue test: seed an orphan `.stderr`, missing `.rs`, duplicate case
  ID, duplicate source path, overlapping owner, and unlisted flat-root fixture;
  each denies before compiler work with one exact typed finding.
- Migration parity test: every moved legacy fixture produces the same admitted
  proof kind and expected compiler outcome before and after movement; the final
  run resolves only the new semantic path.
- Deletion test: removing an authority directory deletes only its owned proof
  rows and causes the aggregate coverage requirement for that authority to fail
  rather than silently shrinking.

**Engineering decisions**

- No Rust UI fixture remains directly under `tests/ui` at closeout; the first
  structural level identifies its semantic authority.
- Expected-output artifacts live beside their compile-fail source and are
  admitted through the same manifest row.
- Manifest admission forbids wildcard ownership. Implementation may expand
  authoring conveniences into exact rows, but the admitted execution plan
  carries exact case identities and paths.
- The 96 observed orphan `.stderr` files are classified as restored proof,
  intentional deletion, or historical residue removal. None is retained
  merely to minimize the diff.

**Open questions**

- None.

### Phase 4: Proof Portfolio Rationalization Without Coverage Loss

Reduce redundant compiler and repository-wide work only after the inventory can
prove which invariant each case protects. Preserve compiler enforcement for
authority-bearing negative space and move broad observational checks to the
narrowest proof mechanism that can honestly own them.

**Relevant subsystems**

- constructor privacy, type substitution, move-only progression, facade
  reachability, trait implementation, and phase-ordering compile denials
- public API snapshots and exported-symbol inventories
- prohibition, source-residue, documentation, consumer-adoption, and
  historical closeout checks
- exhaustive convergence/property matrices and representative fast proofs

**Relevant APIs and artifacts**

- invariant-family coverage matrix mapping each product prohibition or
  guarantee to one or more proof cases
- typed replacement record for consolidated or removed probes, naming old
  cases, new cases, proof strength, and detection parity evidence
- seeded mutation/sabotage corpus for validating that the reduced portfolio
  still catches forbidden capability resurrection
- explicit `fast`, `authority`, and `certification` posture per retained proof

**Warnings**

- Many private constructors are not automatically one invariant. Consolidation
  is valid only when one stronger compiler probe or public-surface proof
  mechanically detects every protected resurrection class.
- Source grep is weaker than compiler or public API evidence and must not
  replace it merely because it is faster.
- Combining unrelated expected compiler errors into one fixture can make one
  early error mask later violations. Consolidated probes must demonstrate
  independent sensitivity to every claimed mutation.
- Historical closeout evidence may be cold-path certification, but it cannot
  remain in the ordinary lane simply because it already exists there.

**Test requirements**

- Mutation-detection parity test: seed representative public constructor,
  field exposure, marker substitution, phase skip, reuse-after-move, deep
  import, and facade bypass mutations; the post-rationalization portfolio
  rejects every mutation caught by the baseline portfolio and localizes it to
  the same invariant family.
- Replacement rejection test: deleting or consolidating a case without an
  admitted replacement record and equal-or-stronger detection evidence fails
  coverage admission before test execution.
- Masking test: sabotage each claimed multi-invariant fixture one invariant at
  a time and prove no earlier compiler error prevents detection of the remaining
  invariant. Split the fixture when independent sensitivity cannot be proven.
- Proof-kind honesty test: attempts to replace compile-time unreachability with
  runtime assertion, documentation, or text-only source scanning are rejected
  for compiler-owned invariant families.

**Engineering decisions**

- Proof count is not a success metric. Invariant detection, authority coverage,
  determinism, and local execution cost are the metrics.
- Public API inventory may cover broad absence from exported surfaces;
  trybuild remains required for type relationships that an export list cannot
  prove, including substitution, phase ordering, ownership, trait bounds, and
  move semantics.
- Whole-repository scans and documentation/consumer closeout checks become
  named certification lanes with explicit external workspace prerequisites.
- Exhaustive combinatorial convergence remains certification proof. Each
  authority also retains a bounded representative smoke proof suitable for
  local iteration, and the smoke proof is not presented as exhaustive closure.

**Open questions**

- None.

### Phase 5: Selectable Developer And Certification Lanes

Expose one supported command surface that selects proof by semantic authority
and purpose. Local selection and all-up certification derive from the same
inventory so the fast path cannot become a weaker parallel truth.

**Relevant subsystems**

- authority-local unit, integration, property, compile-pass, and compile-fail
  execution
- ordinary library tests, source-backed audits, reference-consumer checks, and
  full certification
- changed-path impact selection and explicit authority selection
- developer documentation and CI job topology

**Relevant APIs and artifacts**

- a single Query test-runner facade with commands equivalent to:
  - `fast --authority <authority>`
  - `authority --authority <authority>`
  - `compile --authority <authority>`
  - `certify --authority <authority>`
  - `certify --all`
- immutable selection receipt carrying requested lane, resolved authorities,
  dependency expansion, exact case IDs, excluded case IDs with reasons, and
  coverage digest
- changed-path impact report derived from production/test ownership and
  authority dependencies
- human-readable command summary derived from the same execution plan

**Warnings**

- Changed-path selection is advisory convenience unless the ownership map can
  prove completeness. Unknown or cross-cutting paths must expand to a broader
  lane or fail closed, never guess narrowly.
- A fast lane is allowed to omit exhaustive and cross-workspace certification
  only because its selection receipt says so explicitly. It cannot emit a
  milestone-closure or merge-ready proof.
- Separate command spellings must not call separate harness logic. Every lane
  resolves through one selection and execution-plan authority.
- External workspace prerequisites must be declared and checked before source-
  backed consumer certification starts; missing workspaces are typed
  unavailable posture, not mid-run file panics.

**Test requirements**

- Lane-union convergence test: the canonical union of every authority's
  certification lane equals `certify --all` in exact case IDs, proof families,
  outcomes, and coverage digest.
- Impact-selection test: representative edits to declaration, installation,
  runtime execution, projection consumption, workflow/effect, live/subscription,
  facade, and certification surfaces select their owner plus declared
  dependents; unrelated authorities remain absent with exact reasons.
- Fail-closed selection test: unknown path, ambiguous ownership, missing
  dependency edge, unsupported feature set, and unavailable external workspace
  cannot yield a misleading green receipt.
- Fast/full parity test: every case shared by fast and full lanes produces
  identical semantic outcome and structural counters; only admitted exhaustive
  breadth differs.

**Engineering decisions**

- Authority selection is the primary interface. Raw test-name filters remain a
  debugging mechanism and cannot mint coverage or closure receipts.
- The ordinary library lane excludes compile tests and broad historical or
  cross-workspace certification. It remains possible to run all ordinary tests
  explicitly.
- Dependencies expand monotonically: requesting a higher authority includes
  the lower proofs it contractually consumes when the inventory says those
  proofs can be invalidated by the edit.
- Runner output leads with resolved scope, proof posture, and cost counters so
  developers know whether they ran smoke, authority closure, or full closure.

**Open questions**

- None.

### Phase 6: Structural Cost Accounting And Time Budgets

Make iteration cost a named, reviewable contract. Enforce structural work
budgets on every run and reference-machine wall-clock budgets on supported
developer lanes, while retaining separate cold and warm evidence.

**Relevant subsystems**

- runner planning and child-process execution
- Cargo/rustc/trybuild invocation breadth and cache behavior
- ordinary test counts, exhaustive matrix breadth, repository scans, and
  external workspace checks
- local reference workstation and CI timing evidence

**Relevant APIs and artifacts**

- `QueryTestCostContract`-equivalent declarations per lane
- structural counters for selected authorities, selected cases, duplicate case
  executions, Cargo child processes, rustc probes, cache roots, cache
  hits/misses, repository files scanned, external workspaces opened, and
  exhaustive matrix cases
- warm and clean timing receipts labeled by machine profile, toolchain, target,
  feature set, cache identity, and concurrency posture
- checked-in performance baseline and regression report derived from receipts

**Warnings**

- Wall time varies with hardware and load. Structural counters are the
  architecture contract; time budgets are developer-experience gates tied to a
  named reference profile.
- Faster execution obtained by silently selecting fewer proofs is a coverage
  regression, not a performance improvement.
- Compiler cache hits cannot excuse duplicate case execution or overlapping
  manifests. Structural duplication must remain exact zero.
- Warm-only evidence hides the first-run experience; clean-only evidence hides
  the ordinary edit loop. Both are required.

**Test requirements**

- Cost convergence test: equivalent authority plans under different manifest
  order and supported scheduling produce identical structural counters and
  case coverage; warm versus clean runs differ only in admitted cache and time
  fields.
- Budget rejection test: seed a duplicate compiler invocation, second cache
  root, unexpected repository scan, unrelated authority case, and hidden
  exhaustive matrix expansion; each violates the exact responsible counter.
- Cache sabotage test: corrupt, stale, foreign-toolchain, foreign-feature, and
  partially populated caches are rejected or rebuilt without changing semantic
  outcomes or coverage identity.
- Timing-regression test: repeated reference-profile samples report median and
  slowest admitted posture; a lane exceeding its locked budget fails with the
  slow authority and structural breadth identified.

**Engineering decisions**

- On the recorded local reference profile, a warm authority-local fast lane is
  capped at 30 seconds, an authority-local compile lane at 60 seconds, and a
  combined authority lane at 90 seconds.
- The warm all-authority non-trybuild lane is capped at 150 seconds, the warm
  all-authority compile lane at 300 seconds, and the warm full certification
  lane at 360 seconds.
- A clean full certification run is capped at 600 seconds on the reference
  profile. CI may define a separately calibrated profile, but cannot weaken
  structural counter or coverage requirements.
- Budgets include runner planning and prerequisite checks. Commands may not
  stop the clock while doing hidden setup that developers must still wait for.
- Any intentional budget change requires a spec or reviewed cost-contract
  amendment with before/after structural evidence; updating a golden duration
  alone is forbidden.

**Open questions**

- The implementation must record the exact reference-machine profile before
  locking closeout measurements. The numerical budgets above remain fixed;
  the profile identifies where they are enforced.

### Phase 7: Workflow Cutover And Hostile Closure

Cut local development, milestone QA, and CI onto the authority-local runner;
delete the monolithic compile-test path and compatibility residue; then certify
that the new topology preserves every protected Query boundary under hostile
ordering, cache, selection, and mutation pressure.

**Relevant subsystems**

- developer and agent verification commands
- implementation-batch, focused QA, full QA, and CI invocation surfaces
- old library-embedded trybuild modules, broad globs, environment-mutating
  harnesses, path aliases, and compatibility runners
- authority split input report and milestone closeout evidence

**Relevant APIs and artifacts**

- stable documented local, authority, compile, and full-certification commands
- CI jobs derived from authority plans with one aggregate closure receipt
- permanent prohibitions against library-embedded trybuild, process-global test
  environment mutation, root UI fixtures, wildcard ownership, orphan expected
  outputs, and unreceipted closure claims
- authority ownership/dependency/proof bundle consumed by the next production
  crate-decomposition milestone

**Warnings**

- The old command path cannot remain as a second supported lane. If retained
  temporarily during migration, it has no closure authority and is deleted
  before this phase closes.
- CI sharding must compose the same admitted authority plans. Hand-maintained
  shard lists would become a new parallel inventory.
- Green authority shards do not imply green aggregate certification until the
  aggregate receipt proves exact union, prerequisite availability, and no
  duplicates or omissions.
- This closeout does not certify the future production crate graph. It certifies
  that the proof portfolio is ready to guide that split.

**Test requirements**

- Hostile schedule convergence test: run all authorities repeatedly under
  reversed and randomized scheduling, clean and warm caches, and supported
  thread counts; case identity, outcomes, diagnostics normalization, coverage
  digests, and structural counters converge.
- Boundary mutation matrix: seed representative public-construction,
  substitution, phase-order, move-only, facade-bypass, raw-authority,
  cross-runtime, and stale-generation violations across the recorded authority
  map; the correct authority lane and full certification both reject them.
- Workflow parity test: developer command, milestone QA command, CI authority
  shard, and all-up closeout resolve equivalent requests into identical
  execution plans and receipts.
- Prohibition/residue test: exact-zero library-embedded trybuild invocations,
  process-global environment writes, flat UI fixtures, wildcard owners, orphan
  outputs, legacy runner entrypoints, duplicate executions, and unowned cases.
- Split-readiness test: destroying and rebuilding the derived ownership,
  dependency, lane, and coverage reports from authoritative manifests yields
  identical artifacts suitable for production crate-boundary design.

**Engineering decisions**

- One aggregate certification receipt is the sole milestone closure artifact.
  Human summaries, CI job status, timing dashboards, and per-authority receipts
  derive from or are referenced by it.
- CI may parallelize disjoint authority plans, but each plan remains runnable
  locally and retains its own exact scope and cost receipt.
- The supported verification documentation teaches the smallest honest lane
  first, then authority closure, then full closure. It never describes a smoke
  result as merge-ready.
- The next milestone consumes the authority dependency/proof bundle to design
  production package boundaries. It does not rediscover ownership from the
  current directory tree.

**Open questions**

- None.

## Must Ship

- one authoritative, disjoint, authority-owned Query proof inventory
- deterministic compile-test child-process isolation with stable semantic cache
  identity and no process-global environment mutation
- semantic fixture topology, exact per-authority manifests, and zero orphan or
  multiply owned compile artifacts
- invariant-family coverage and replacement records proving any portfolio
  consolidation preserves or strengthens detection
- one authority-addressable test-runner facade with fast, authority, compile,
  certification, and all-up lanes derived from the same inventory
- structural cost contracts, exact execution counters, reference-profile warm
  and clean budgets, and regression enforcement
- developer/QA/CI cutover, permanent prohibitions, aggregate hostile closure,
  and a derived authority dependency/proof bundle for production splitting

## Must Preserve

- every compiler-visible Query authority, construction, substitution,
  phase-ordering, ownership, move-only, facade, and denial invariant currently
  enforced
- canonical Query meaning, public API behavior, support posture, runtime
  authority, lower-runtime ownership, and Store handoffs
- full certification as the merge and milestone-close authority
- exact distinction between smoke evidence, authority closure, and full closure
- tests as production-quality semantic responsibilities rather than generic
  fixtures or runner glue
- one authoritative proof inventory with every report, lane, shard, and cost
  summary derived and rebuildable

## Acceptance Evidence

Milestone 9.13.1 is complete only when Query can prove all of the following:

- ordinary library tests execute no trybuild cases and perform no concurrent
  process-global environment mutation
- every regular and compiler fixture has one stable case identity, authority
  owner, invariant family, proof kind, and lane posture
- the authority-manifest union and all-up certification registry are identical,
  with exact-zero duplicate, missing, orphan, wildcard-owned, or unclassified
  cases
- the post-rationalization mutation corpus detects every protected violation
  caught by the baseline portfolio and localizes it to the correct authority
- authority-local, developer, QA, CI, and all-up entry paths resolve through one
  selection and execution-plan authority
- forward, reverse, randomized, clean-cache, warm-cache, and supported-thread
  schedules produce identical semantic outcomes, coverage digests, and
  structural work counters
- warm and clean lanes satisfy the locked reference-profile budgets without
  reducing selected proof or hiding setup cost
- missing external workspaces, unknown paths, unsupported feature sets, stale
  caches, and ambiguous ownership fail closed before misleading closure
- the old monolithic compile-test path, broad fixture globs, path aliases, and
  environment-mutating harnesses are deleted and mechanically prohibited
- the derived authority dependency/proof bundle rebuilds identically and gives
  the next milestone sufficient evidence to design production crate boundaries

## Sequencing Notes

This milestone follows 9.13 because the installed-domain, declarative facade,
and Foundational-native authority surfaces must exist before their proof
ownership can be frozen honestly. It interrupts the previous direct
`9.13 -> 9.14` sequence because current iteration and compile-certification
topology cannot safely absorb another large authority surface.

It precedes 9.14 because installed operation semantics must be developed and
certified through authority-local deterministic lanes from their first commit.
It also precedes the production authority-decomposition milestone: test
ownership is evidence for crate ownership, not something to retrofit after
packages move.

The milestone is not blocked on `worth-store`. It changes no provider,
durability, replay, or Store-facing product contract. Store and Milestone 13
continue to consume the same Query semantic oracles after their execution
topology becomes deterministic.
