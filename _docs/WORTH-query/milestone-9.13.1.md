# Milestone 9.13.1: Query Iteration Rescue

## Goal

Cut the obvious structural waste from `worth-query` test execution before
designing authority packages: ordinary library iteration must not execute
compiler fixtures or inspect Worth UI, and compiler certification must retain
only the counterexamples that uniquely protect a compiler-enforced product
invariant.

## Why This Milestone Exists

Query currently constructs 38 trybuild harnesses over roughly 1,100 UI
fixtures. The cost is dominated by repeated Cargo and trybuild setup, not by a
missing proof-inventory platform. Some Query certification also scans Worth UI
source, reversing ownership and making Query tests depend on a consumer.

The first move is therefore deletion and one negative compiler batch. Positive
public journeys belong in ordinary integration tests or doctests, not in
trybuild executables. Authority manifests, selectable lanes, custom caches,
receipts, shards, and performance frameworks are explicitly outside this
milestone. The following Milestone 9.13.2 splits Query by authority, after
iteration is fast enough to support that work.

## Governing Summaries

- `MENTALITY.md`: solve the actual adversarial constraint first and use AI to
  accelerate judgment rather than manufacture code around an untested premise.
- `arch_laws.md`: match mechanical boundary cardinality to the work; do not
  reconstruct authority or proof inside a trusted test boundary.
- `composition_laws.md`: compiler registration, compiler execution, and
  consumer adoption are separate responsibilities and may not collapse into a
  generic proof runner.
- `domain_structure_laws.md`: tests falsify production behavior; they do not
  recursively certify the topology of other tests.
- `perf_laws.md`: remove repeated structural work before tuning constants,
  caches, process counts, or parallelism.
- `WORTH_query_roadmap.md`: this milestone exists only to restore livable
  iteration before the authority split and Milestone 9.14.

## Adversarial Constraint

One Query compiler-certification invocation must compile the shared dependency
graph once and retain every distinct compiler-enforced product denial that
cannot be proven by an ordinary integration test. The ordinary Query library
lane must perform neither trybuild work nor Worth UI source inspection. No test
may exist merely to preserve a removed API, prove ordinary Rust privacy, or
certify that another test or harness exists.

## Product Decision Lock

- Retain a compile-fail fixture only when it uniquely protects authority
  minting, type or family substitution, phase ordering, move-only ownership, or
  a load-bearing facade denial. Its first compiler error must exercise the
  named invariant rather than an incidental missing import or stale API.
- Delete historical API tombstones, generic report/counter/manifest privacy
  probes, certification-topology fixtures, redundant per-type privacy probes,
  orphan baselines, and any fixture that fails before reaching its claimed
  boundary.
- Use Cargo and trybuild directly. Add no custom runner crate, inventory,
  manifest language, cache manager, compiler wrapper, receipt schema, shard
  scheduler, timing sampler, or migration framework.
- The direct trybuild target is compile-fail only. Positive facade journeys use
  ordinary integration tests or doctests so Cargo can compile them as normal
  test targets without trybuild's per-program rebuild loop.
- Production Query code may not own fixture paths, fixture counts, golden
  transcript registries, or compiler-test manifests. Test registration is test
  code and carries no product authority.
- Query owns Query behavior. Worth UI owns proof that Worth UI adopts Query.
- Verification exercises product tests and the real compiler portfolio. It
  does not add tests that inspect, count, replay, or certify other tests.

## Phase Plan

### Phase 1: Select The Load-Bearing Compiler Denials

Move retained trybuild registration out of the ordinary library harness and
feed it into one direct compile-fail integration-test target. Rationalize the
portfolio before registering it:

- keep phase-order, move-only, authority-forgery, cross-kind substitution, and
  load-bearing facade counterexamples
- keep only representative opaque-construction probes where private
  construction is itself the authority boundary
- move unique positive public journeys to ordinary integration tests or
  doctests, and delete duplicate golden transcripts
- delete all other fixtures and their expected diagnostics

The compiler target may use small responsibility-named registration modules to
stay under the workspace line cap. Those modules contain fixture declarations
only; they do not discover paths from production registries or mint proof
identities, ownership records, plans, or receipts.

**Relevant subsystems**

- the 38 inherited trybuild harness files
- `crates/worth-query/tests/ui`
- `crates/worth-query/src/integration_tests.rs`
- the direct Cargo test target for compiler certification

**Warnings**

- A strong filename does not make a strong fixture. If the first error is an
  unresolved import but the fixture claims to prove wrong-phase substitution,
  the fixture is false confidence and must be deleted or rewritten against the
  real public seam.
- Privacy is not independently valuable for every report, row, counter, and
  receipt. Retain privacy proof only where public construction would mint or
  counterfeit authority.
- One counterexample may cover a family of equivalent implementation-private
  shapes. Do not preserve one probe per field or historical type.

**Verification**

- Run the ordinary library lane once after the cutover and confirm its existing
  product tests pass without invoking trybuild.
- Run the direct compiler-certification target once and require every retained
  compile-fail fixture to fail for its intended reason.
- Run migrated positive journeys through their ordinary integration or doctest
  owner; do not recreate a positive-fixture inventory.
- Do not add a test that scans source files, counts harness constructors,
  compares inventories, or otherwise proves the test topology.

**Engineering decisions**

- Cargo owns target reuse, locking, invalidation, and job count.
- trybuild owns negative-fixture compilation and diagnostic comparison.
- A direct integration-test target is sufficient process isolation. No child
  process protocol or custom executable is required.
- The compile portfolio is deliberately not sharded. Revisit parallelism only
  after the authority crate split changes the natural Cargo package graph.

**Open questions**

- None.

### Phase 2: Cut Consumer Dependencies And Adopt The Direct Workflow

Delete Query-side source audits and certification rows that read Worth UI.
Consumer adoption belongs to the consumer workspace and may use Query only
through its public facade. Then make the smallest honest commands the supported
workflow: ordinary Query behavior during development and explicit compiler
certification when public type boundaries change.

**Relevant subsystems**

- Query reference-consumer and product-boundary certification
- installed-domain and native-value certification manifests
- Worth UI's existing Query binding tests
- local and CI verification commands

**Warnings**

- Moving a Query-owned semantic test into Worth UI would invert authority in
  the other direction. Only consumer-adoption evidence moves or disappears;
  Query behavior and public-boundary fixtures remain in Query.
- Do not retain a compatibility registry of former Worth UI paths. Deleting the
  dependency means Query no longer knows those paths exist.
- Do not add authority-local selection, impact analysis, proof manifests, or
  cost receipts here. Package boundaries in Milestone 9.13.2 will provide the
  natural selection boundary.

**Verification**

- Run the affected Query behavior tests after deleting consumer-path scans.
- Run Worth UI's existing Query binding tests from the Worth UI workspace; no
  Query command may be required to certify Worth UI adoption.
- Record one before/after wall-clock observation for the real developer command
  and one for compiler certification. These are observations, not golden tests
  or a repeated benchmark suite.

**Engineering decisions**

- `cargo test -p worth-query --lib` is the ordinary Query iteration command.
- `cargo test -p worth-query --test compile_certification` is the compiler
  certification command.
- Full workspace CI may run both ordinary and compiler-boundary tests, but it
  may not reconstruct or shard the compiler fixture list through production
  code.
- Timing budgets are decision inputs. They do not justify a performance-testing
  subsystem inside Query.

**Open questions**

- None.

## Must Ship

- ordinary Query library tests with no trybuild execution
- one direct compile-fail certification target over the selected load-bearing
  denial portfolio
- no positive trybuild execution; unique public journeys live in ordinary
  integration tests or doctests
- no historical tombstones, generic certification-artifact privacy probes,
  orphan diagnostics, or production-owned fixture registries
- no Query test or certification source that reads Worth UI
- short documented commands for ordinary and compiler-boundary iteration

## Must Preserve

- every distinct compiler-enforced authority-minting, substitution,
  phase-ordering, ownership, move-only, and facade invariant
- every unique positive ordinary facade journey, through ordinary test
  ownership rather than a trybuild transcript
- Query product semantics, public API, runtime authority, and Store handoffs
- Worth UI's ownership of its own consumer behavior and adoption tests

## Acceptance Evidence

- one successful ordinary library run with no trybuild output
- one successful direct compiler-certification run covering the retained
  denial set, with each fixture failing at its named boundary
- one successful ordinary run covering migrated positive journeys
- one successful Worth UI Query-binding run owned by the Worth UI workspace
- before/after elapsed observations showing whether the obvious batching cut
  made iteration livable
- repository review showing historical compiler tombstones, orphan baselines,
  production-owned fixture registries, custom proof runners, receipts, shards,
  cache systems, compiler wrappers, meta-tests, and compatibility paths are
  absent

## Transition Line-Cap Exception

This milestone removes dead source-audit rows and the test-suite certification
gate from the following inherited Milestone 9.6 source-topology implementation
without preserving deleted targets:

- `crates/worth-query/src/application/support/closure.rs`
- `crates/worth-query/src/application/support/identity_boundary_inventory.rs`
- `crates/worth-query/src/application/support/identity_boundary_inventory_sources.rs`

Those files remain temporarily exempt from the 400-line cap only for this
deletion change. They are inherited oversized code and are not a pattern to
copy. Milestone 9.13.2 Phase 1 deletes the source-audit authority and its tests
rather than splitting the implementation into smaller proof-of-code catalogs.
No new row, scan, or consumer may be added while the exception exists.

## Sequencing Notes

Milestone 9.13.1 follows 9.13 because it changes no Query meaning and repairs
the development loop around the authority surface 9.13 established. Milestone
9.13.2 follows immediately and performs the production crate split by
authority. Milestone 9.14 begins only after that split, so its installed
operation semantics are born into the intended package graph rather than added
to the current monolith.
