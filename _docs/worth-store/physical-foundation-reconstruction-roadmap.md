# Worth Store Physical Foundation Reconstruction Roadmap

## Purpose

This is the mandatory reconstruction program inside Part I of Worth Store.
It reopens the physical-foundation claims currently attributed to `S.1`
through `S.9`, repairs the development proof loop first, then joins the real
physical mechanisms into one sealed production database runtime before `S.10`
operational recovery continues.

This roadmap does not discard useful page, WAL, buffer-pool, integrity,
isolation, scheduling, layout, blob, model, or certification work merely
because the current composition is not a database. It also gives none of that
work closure credit merely because its vocabulary, local tests, or isolated
mechanisms are strong. Existing code is substrate to inspect and either admit,
refactor, or delete. Only execution through the reconstructed production path
earns a physical claim.

## Roadmap Position

```text
Physical Database Roadmap S.9 implementation state reopened
  -> C.1 direct test execution and iteration cleanup
  -> C.2 executable reality ledger and claim quarantine
  -> C.3 sealed physical runtime authority and lifecycle
  -> C.4 production media boundary and stable store namespace
  -> C.5 durable page, segment, extent, and manifest path
  -> C.6 buffer pool and bounded physical access join
  -> C.7 WAL, checkpoint, root publication, and acknowledgment join
  -> C.8 fresh-process recovery and reopen
  -> C.9 physical integrity, corruption localization, and offline truth
  -> C.10 stable reads, scheduled I/O, and maintenance interference
  -> C.11 layout, index, and native blob adoption
  -> C.12 formal protocol rebinding to executable owner transitions
  -> C.13 physical-foundation recertification and S.10 re-entry
  -> Physical Database Roadmap S.10
  -> S.11
  -> S.12
  -> Runtime And Query Integration Roadmap Milestone 1
```

The `C.*` labels are reconstruction sequence numbers used only in planning,
specification, evidence, and closeout. Production modules, types, functions,
tests, scenarios, counters, and errors must use responsibility names rather
than `C.*`, phase, cleanup, or roadmap provenance.

No new `S.10` implementation receives closeout credit while this program is
open. Work already present in `S.10` may be preserved as unadmitted substrate,
but it must later bind to the reconstructed runtime and repeat its production-
path proof.

## Relationship To The Two Store Roadmaps

The [Physical Database Roadmap](physical-database-roadmap.md) owns Part I. It
must produce the platform that makes bytes survive through real pages,
segments, WAL, checkpoints, stable reads, bounded memory, corruption handling,
maintenance, blobs, recovery, and declared media assumptions. This cleanup is
therefore a Part I correction program, not a bridge feature and not a new
product layer.

The [Runtime And Query Integration Roadmap](runtime-integration-roadmap.md)
owns Part II. It starts from an already real physical platform and composes it
with the existing Query, Relational, Signal, and Runtime Bridge authorities.
Part II owns semantic lowering, hydration, durable commit publication,
Store-backed reads, branch concurrency, residency, recovery readmission, and
Query parity. This reconstruction must expose the physical contracts Part II
needs, but it must not import Query, decide MVCC visibility, persist runtime
authority, or create a second semantic Store runtime in anticipation of Part
II.

The dependency is deliberately asymmetric:

```text
existing Query / Relational / Signal / Runtime Bridge runtime
                         |
                         | Part II composition and semantic integration
                         v
sealed physical Store platform produced by Part I
                         |
                         v
files, pages, WAL, checkpoints, manifests, indexes, chunks, and media
```

The physical platform knows nothing about Query. The Part II integration
boundary knows both public contract families and owns only their join.

## Governing Summaries

- `MENTALITY.md` protects hard-problem-first foundations. The strongest
  constraint is that iteration speed and a real write/reopen vertical slice
  must be repaired before more operational surface is added.
- `arch_laws.md` protects proof-carrying authority and explicit lifecycle. The
  strongest constraint is that the production runtime must be sealed,
  non-cloneable as authority, decomposed into independently borrowable
  subsystems, and progressed through real durable effects rather than supplied
  replay representations.
- `composition_laws.md` protects one named responsibility per unit. The
  strongest constraint is that test support, runtime orchestration, media I/O,
  recovery, and certification cannot remain broad bags or duplicated execution
  paths.
- `domain_structure_laws.md` protects ownership and truth-source topology. The
  strongest constraint is that local tests must construct only the owner they
  falsify, while cross-owner proof lives in an explicit courtroom and physical
  boundary crossings are spatially locatable.
- `perf_laws.md` protects visible cost and semantic locality. The strongest
  constraint is that test execution cost, resident memory, I/O breadth,
  recovery work, amplification, and interference all require named boundaries
  and counters rather than elapsed-time folklore.
- `physical-database-roadmap.md` protects byte survival. This program must make
  its S.1 through S.9 mechanisms true on one production path before S.10 can
  safely manipulate damaged or restored stores.
- `runtime-integration-roadmap.md` protects one Store-backed Query runtime.
  This program must finish with stable physical facades and handoffs that Part
  II can consume without duplicating persistence, recovery, access planning,
  or byte authority above Store.
- `test-requirements.md` protects proof of the declared behavior. Each
  reconstruction milestone must retain control, hostile, reopen, semantic-
  parity, and forbidden-shortcut lanes where applicable.
- `test-requirements-2.md` protects the physical proof mechanism. Real process
  death, real persisted bytes, production-boundary fault delivery,
  independent verification, memory envelopes, and mutation sensitivity are
  mandatory; simulation labels and live heap reuse are not evidence.
- `storage-foundation-s10.md` protects operational recovery without trusting
  the live store. Its workflow design remains downstream and unclosed until
  the underlying store can actually write, die, reopen, and be independently
  inspected through the production physical platform.

## Global Adversarial Constraint

The reconstruction must survive this hostile condition:

> An engineer changes one physical owner and receives a trustworthy local
> result quickly; CI then drives the same production runtime through real file
> writes, page and WAL publication, process death, fresh-process reopen,
> corruption, memory pressure, concurrent reads, maintenance, layout access,
> and blob streaming. No lane may obtain persisted truth from a replay object,
> copied heap layout, surviving process state, test-owned oracle, test-only
> authority, mock-only backend, or an isolated mechanism that the production
> runtime does not call. Every admitted claim must name the exact bytes,
> authority transition, cost boundary, and independent observer that proved it.

The program has failed if it merely makes the current tests faster while
preserving fake physical proof, or makes the physical runtime real while
leaving ordinary iteration so expensive that broad verification is avoided.

## Product Decision Lock

1. Test cleanup is the first milestone and blocks all reconstruction work that
   would otherwise inherit the current ten-minute feedback loop.
2. Faster testing means less repeated compilation and correctly separated
   proof lanes. It never means deleting hostile assertions, weakening physical
   scenarios, or calling a smoke lane certification.
3. `PhysicalStoreRuntime` or its replacement is the sole production physical
   composition authority. It is not `Clone`, does not expose constructors that
   mint duplicate authority, and does not reopen from caller-supplied heap
   layouts or replay artifacts.
4. A stable store root plus admitted configuration and platform authority are
   sufficient to open or recover the physical store. Callers do not supply the
   bytes that recovery is supposed to discover.
5. Real file/media operations are reached through one production storage
   boundary used by ordinary execution and wrapped by the adversarial harness.
6. Existing file-writing mechanisms in backend, WAL, isolation, replication,
   or operations crates earn no database claim until the canonical runtime
   invokes them in the required order.
7. Physical format owns byte encoding and decoding; the backend owns media
   effects; the buffer pool owns residency; recovery owns source precedence
   and replay; isolation owns stable physical visibility; no one crate absorbs
   all of these laws.
8. Certification may observe, inject faults, compare, and report. It cannot
   mint production authority, supply expected persisted state, or become the
   only caller of a mechanism claimed as production behavior.
9. Compile-fail proof remains mandatory for authority and dependency
   boundaries, but it runs through consolidated, cache-sharing UI suites rather
   than hundreds of cold nested Cargo projects.
10. S.1 through S.9 closeout is reopened. C.13, not historical green runs,
    decides which claims are restored.
11. S.10, S.11, and S.12 remain in their existing conceptual order after this
    program. Part II still begins only after the complete physical roadmap
    closes.
12. Query, Relational, Signal, and Runtime Bridge are consumers of the final
    platform through Part II. No cleanup milestone imports or partially
    integrates Query.

## Non-Fake Physical Acceptance Test Contract

Every `C.2` through `C.13` engineering spec must contain a
`Non-Fake Acceptance Setup` section. C.1 is the test-execution foundation and
uses its own direct truth contract; making its reports certify themselves would
be recursive. A physical test requirement is incomplete unless it fixes all of
the following before implementation begins:

### Production subject

- Name the exact production facade, executable, and owner methods under test.
- Name every real crate expected on the call path.
- Name the physical artifacts expected to exist after execution.
- Name the exact test-only layers allowed around the production boundary.

### Initial world

- State whether the store root begins absent, empty, or pre-populated by a
  separately identified producer process.
- State the backend profile, format version, page size, memory budget, workload
  seed, and fault profile.
- State which artifacts must not exist before the action.
- Generate semantic expectations independently of the runtime under test.

### Execution topology

- Drive work only through the named public production facade.
- Deliver faults through the production storage boundary or named production
  yieldpoints, never by mutating private runtime state.
- Record process identity, runtime identity, storage-root identity, and every
  authority transition.
- State which process must terminate without normal cleanup and which fresh
  executable performs reopen or verification.

### Independent observation

- Reopen from the store root and admitted configuration only.
- The verification process must not receive `PhysicalStoreRuntime`,
  `PersistedPhysicalLayout`, `PlatformPhysicalReplayArtifact`, cached pages,
  runtime registries, decoded records, or expected state from the writer.
- The independent oracle may share stable format declarations but must not
  share live recovery decisions, caches, normalization, or authority paths.
- Expected outcomes and expected corruption locations must be fixed before the
  verifier runs.

### Assertions

- Assert semantic equality or intentional inequality against an independent
  model.
- Assert exact required artifacts and absence of forbidden residue.
- Assert typed failure localization rather than accepting any error.
- Assert exact or weakest-sufficient structural counters, including counters
  that must remain zero.
- Assert memory, allocation, I/O, amplification, and recovery bounds at their
  named measurement boundaries.

### Mutation sensitivity

- Name at least one controlled defect the milestone must detect.
- Name the exact lane and localization expected to fail for that defect.
- A mutant that merely fails compilation does not prove runtime sensitivity.
- The closeout bundle records mutant id, expected failing predicate, actual
  failing predicate, and localization.

### Mechanical anti-substitution gates

Each spec must add dependency, source, manifest, or compile-time checks that
reject the substitutes relevant to its claim. The common forbidden set is:

- replay artifacts or persisted heap layouts crossing a process boundary
- live runtime reuse after the declared crash
- test-authority constructors on an ordinary production path
- a mock-only or certification-only backend satisfying a physical claim
- same-path producer and verifier comparison
- private-state corruption or post-hoc byte editing used to simulate a write
  seam when the storage interposer can express it
- whole-store or whole-blob materialization hidden behind a helper
- broad test-support dependencies pulled into owner-local tests
- nested Cargo builds with per-case target directories in ordinary test runs
- evidence based only on success, non-empty digests, elapsed time, or logs

### Evidence and rerun

- Emit a machine-checkable bundle containing source identity, binary identity,
  backend and hardware profile, seed, workload trace, fault schedule, process
  identities, artifact manifest, counter snapshot, oracle result, runtime
  result, and every certification predicate.
- The bundle must identify the command and mode required to rerun the lane.
- Stale source, binary, format, profile, or harness identity invalidates the
  bundle rather than silently reusing it.

This contract is a floor. Each milestone below fixes its own concrete setup so
an implementation cannot satisfy the words through a neighboring fake.

## Certification Modes And Time Budgets

The proof model has five distinct execution products:

- **Owner check**: the changed crate's unit and narrow integration tests. It
  constructs no unrelated owner and targets feedback in seconds.
- **Developer smoke**: deterministic production-path vertical specimens,
  consolidated UI smoke, and small hostile schedules. The warm target is under
  one minute on the declared reference development machine.
- **CI certification**: owner tests plus medium physical scenarios, real fresh-
  process reopen, representative mutation lanes, and stores larger than the
  configured memory budget. Jobs are partitioned by proof family.
- **Release certification**: long crash, corruption, maintenance, blob, and
  cross-backend campaigns with independent offline verification.
- **Hardware qualification**: filesystem, flush, rename, direct-I/O, mmap,
  sector, device, and latency claims on a named deployment profile.

Time budgets are measurements and regression gates, not correctness authority.
A slower test does not become invalid merely by exceeding a target; it must be
classified into the proper lane and its structural cost explained. Conversely,
a fast smoke run cannot promote a release claim.

## Milestone Plan

## C.1: Direct Test Execution And Iteration Cleanup

Engineering spec:
[physical-reconstruction-c1-test-execution-architecture.md](physical-reconstruction-c1-test-execution-architecture.md)

### Goal

Restore a fast, structurally honest feedback loop before physical runtime
reconstruction begins.

### Boundary

This milestone changes how tests are selected, compiled, linked, and scheduled.
It does not certify currently fake physical behavior. Cargo, Git, the test
executables, and CI remain the truth sources; C.1 does not construct a parallel
authority hierarchy around them.

### Must Ship

- one on-demand Cargo-derived target catalog that assigns every test target to
  exactly one CI lane without checked-in generated inventories
- real, documented `store-owner`, `store-smoke`, `store-ui`, and `store-ci`
  entrypoints backed by one unique execution planner
- Worth Store-local development and test profiles that avoid full Windows PDB
  generation where it is not required
- consolidation of the scenario explosion into a small number of
  responsibility-named suite binaries, with separate executables retained only
  where process identity is itself part of the proof
- consolidation of compile-fail fixtures behind cache-sharing UI runners with
  stable expected diagnostics and no per-case cold target directory
- removal of nested Cargo invocations from ordinary behavioral tests;
  structural preflight becomes an explicit job with reusable evidence
- owner-local or subsystem-local fixture/support surfaces replacing the broad
  `worth-store-test-support` dependency where local tests do not need the full
  physical platform
- explicit feature hygiene so certification authority is not unified into the
  ordinary production graph by default
- partitioned CI jobs with cache identity bound to OS, toolchain, profile,
  feature lane, and lockfile
- concise elapsed and unit-count observation with optional disposable JSON
  output; successful local commands produce no mandatory evidence files
- deletion of recursive behavior fingerprints, preservation ledgers, plan/run
  seals, source-edit authority, custom CI aggregates, closeout bundles, and C.2
  readiness tokens

### Non-Fake Acceptance Setup

- **Production subject:** the Worth Store workspace manifests, suite entry
  points, UI runners, CI workflow, and test-support dependency graph.
- **Initial world:** start from a clean target directory for cold measurement
  and a completed identical run for warm measurement. Record Rust toolchain,
  OS, CPU, storage, source revision, features, and exact command.
- **Execution:** change one leaf owner source file and run owner check; change
  one shared physical contract and run developer smoke; change one test/UI
  expectation and run its owning product. Run complete UI and CI products from
  the committed revision.
- **Independent observation:** Cargo metadata names targets, Cargo/test process
  results name behavioral outcomes, ordinary target-directory observation
  distinguishes cold from warm work, and GitHub matrix status names CI
  completion. No runner-produced artifact certifies another runner artifact.
- **Assertions:** owner check does not build certification or unrelated owner
  crates; developer smoke runs the declared vertical specimens; UI fixtures
  share compilation roots; no ordinary lane creates a per-case Cargo target;
  the preserved-proof manifest has no unclassified test.
- **Controlled defects:** invert one UI expectation and one ordinary assertion.
  Each direct owning product must fail for the intended reason. Planner tests
  separately reject duplicate units, stale zero-match filters, and ambiguous
  target classification.
- **Forbidden substitutes:** excluding tests from all modes, replacing tests
  with source scans, timing only a no-change green run, or declaring success
  solely because `cargo nextest` is installed cannot close this milestone.

### Closeout Gate

`C.1` closes only when ordinary owner work no longer compiles the courtroom,
developer smoke exercises real declared specimens with a warm reference target
under one minute, UI proof no longer performs per-fixture cold builds, every
Cargo test target has one CI lane, and no recursive test-authority path remains.
Exact timing is observation, but unbounded or unexplained iteration cost blocks
closure.

## C.2: Executable Reality Ledger And Claim Quarantine

### Goal

Replace milestone folklore with an executable map of which physical mechanisms
exist, which production path invokes them, and which claims remain unearned.

### Boundary

This milestone does not implement persistence. It traces S.1 through S.9 code
from public facade to physical effect, classifies disconnected mechanisms, and
mechanically prevents unearned platform-grade promotion while reconstruction
is underway.

### Must Ship

- an owner-by-owner executable topology for physical format, backend, buffer
  pool, WAL, recovery, integrity, isolation, scheduler, layout/indexes, blobs,
  formal models, operations, and certification
- a call-path ledger distinguishing:
  - production-reachable physical effect
  - production-reachable in-memory model
  - certification-only mechanism
  - isolated real mechanism not called by the canonical runtime
  - vocabulary or plan without execution
  - duplicate or conflicting authority
- an artifact-family ledger naming source truth, physical representation,
  writer, reader, durability boundary, reopen source, verifier, rebuild source,
  and current claim tier
- dependency and facade maps showing every existing candidate for the sole
  physical composition root
- quarantine of platform-grade promotion and closeout receipts that can be
  satisfied by `PersistedPhysicalLayout`, `PlatformPhysicalReplayArtifact`, or
  other supplied representations rather than discovered bytes
- a deletion/admission decision for every duplicate runtime, fake backend,
  test-only oracle, and mechanism island
- an ordered blocker graph consumed directly by C.3 through C.13 specs

### Non-Fake Acceptance Setup

- **Production subject:** every public physical Store facade and every method
  that claims open, append, read, flush, checkpoint, recover, compact, verify,
  or reopen.
- **Initial world:** create a uniquely identified empty directory and invoke
  the current ordinary facade exactly as product code would. No test fixture
  may pre-supply pages, manifests, WAL frames, layouts, or replay state.
- **Execution:** attempt one append, declared durability, process exit, and
  fresh-process reopen using only the directory and production configuration.
- **Independent observation:** an external artifact walker records actual files
  and bytes. Cargo dependency metadata and callsite tracing record whether the
  file-writing backend is reachable from the facade.
- **Assertions:** every physical claim resolves to a concrete effect path or a
  typed `Unimplemented/Unavailable/Uncertified` classification. Missing files,
  supplied replay state, heap-only mutation, and certification-only calls are
  recorded as failures, not interpreted optimistically.
- **Controlled defect:** add a promotion row for a heap-only append or make a
  disconnected file writer appear in the capability report. The ledger and
  promotion gate must reject the mismatch.
- **Forbidden substitutes:** markdown inventories without generated source and
  execution evidence, grep-only proof of reachability, and a test that writes
  the expected files itself do not count.

### Closeout Gate

`C.2` closes only when every reopened S.1 through S.9 claim has an executable
production path or an explicit unearned classification, and later specs can
name the exact owner seam they must connect without guessing from vocabulary.

## C.3: Sealed Physical Runtime Authority And Lifecycle

### Goal

Establish one non-forgeable, non-duplicable physical runtime composition root
whose subsystem handles express the authority and borrowing topology needed by
the real database.

### Boundary

This milestone seals construction and lifecycle. It does not yet claim durable
page publication. The runtime may temporarily return typed unavailable for
operations whose real mechanism lands in later milestones, but it may not
simulate completion in heap state.

### Must Ship

- one Store-owned open/admit path consuming a stable store root, nested runtime
  configuration, backend capability admission, and concrete platform authority
- a non-`Clone` composition authority with private construction and explicit
  close/abort/crashed lifecycle
- independently borrowable or cloneable read-only handles only where cloning
  does not duplicate mutation, publication, recovery, or allocation authority
- autonomous subsystem ownership for media, buffer residency, WAL/checkpoint,
  physical visibility, integrity, scheduling, layout, blob, recovery, and
  observation
- typestate or equivalent sealed transitions preventing open, recover,
  current-serving, maintenance, and closed states from being confused
- exhaustive construction and lifecycle propagation so adding a subsystem
  breaks every incomplete open, recover, fork, close, and inspection site
- removal or quarantine of `PhysicalStoreRuntime` cloning, public replay-based
  reopen authority, and caller-provided persisted-layout construction
- explicit phase-scoped observation handles that cannot mutate or publish
- a narrow production facade that reveals expensive I/O, recovery, and
  maintenance boundaries in its API shape

### Non-Fake Acceptance Setup

- **Production subject:** the sole physical runtime facade and its construction,
  handle, lifecycle, and unavailable-operation contracts.
- **Initial world:** one empty store root and one existing-but-unopened store
  root; no runtime, replay artifact, or persisted layout is preconstructed.
- **Execution:** open the first root, obtain two observation handles and one
  mutation handle, attempt invalid lifecycle transitions, close it, then open
  the second root through the same sealed constructor.
- **Independent observation:** compile-fail fixtures live outside the owner
  crate and attempt to clone runtime authority, construct internal state,
  mutate through observation, reopen from supplied layout, operate after close,
  and construct with a missing subsystem.
- **Assertions:** invalid authority and lifecycle states are uncallable; runtime
  identity is unique; observation handles expose no mutation or proof-minting
  surface; unavailable physical operations fail before heap mutation.
- **Controlled defect:** reintroduce `Clone` on the composition authority or a
  public constructor for recovered state. The UI suite must fail at the exact
  forbidden capability and the topology gate must identify the widened facade.
- **Forbidden substitutes:** a private field with a public cloning wrapper,
  runtime assertions after illegal calls, generic marker authority, or
  certification-only construction standing in for ordinary open cannot close
  the milestone.

### Closeout Gate

`C.3` closes only when the compiler enforces one physical authority lifecycle,
callers cannot supply recovered truth, and subsequent milestones have one
unambiguous runtime into which real mechanisms must bind.

## C.4: Production Media Boundary And Stable Store Namespace

### Goal

Create the single production I/O boundary through which the physical runtime
owns files, directories, offsets, synchronization, publication, and backend
capability truth.

### Boundary

This milestone establishes media mechanics and namespace law. It does not yet
assign page, WAL, checkpoint, index, or blob semantics to every file. Those
artifact owners consume this boundary in later milestones.

### Must Ship

- a sealed production media port covering open/create, positioned read/write,
  append, truncate, allocate, flush, file sync, directory sync, atomic rename,
  list, metadata, delete, and declared optional mmap/direct-I/O operations
- one real filesystem implementation with explicit Windows and supported POSIX
  semantics, error topology, and capability admission
- a stable store-root namespace with versioned identity, lock/ownership file,
  directory roles, temporary/staged/publication names, and no ambiguous residue
  discovery
- process-level exclusive mutation ownership or an explicitly stronger
  admitted multi-process design; read-only and offline access posture is
  separate
- storage-boundary operation context sufficient for fault scheduling and exact
  counters without giving the harness mutation authority above I/O
- durable create, replace, rename, and directory-publication protocols tied to
  backend capability evidence
- typed short-write, partial-read, ENOSPC, permission, stale-handle,
  unsupported-sync, and indeterminate-publication outcomes
- migration of existing file-writing primitives onto this boundary or an
  explicit deletion decision where they duplicate it

### Non-Fake Acceptance Setup

- **Production subject:** the sealed runtime opening the real filesystem media
  implementation; the harness may wrap only the media port.
- **Initial world:** an absent store root on a real temporary filesystem. A
  second process is prepared to contend for mutation ownership. Backend and OS
  profiles are recorded.
- **Execution:** open/create the namespace, append and replace known framed
  bytes, force file and directory durability, attempt concurrent ownership,
  terminate the writer, and inspect from a fresh process.
- **Independent observation:** the fresh process uses OS file APIs and stable
  format declarations, not runtime caches or writer-returned byte buffers.
- **Assertions:** exact path set, file lengths, offsets, bytes, sync sequence,
  ownership denial, cleanup posture, and zero writes outside the admitted root
  are checked. Short write, sync failure, and rename interruption localize to
  typed media outcomes.
- **Controlled defect:** report sync completion without invoking the backend
  barrier, or allow path escape beyond the root. The durability-sequence or
  namespace-confinement predicate must fail.
- **Forbidden substitutes:** `std::fs::write` in the test, a memory backend,
  inspecting a buffer returned by the writer, or declaring rename durable
  without namespace synchronization cannot close this milestone.

### Closeout Gate

`C.4` closes only when the canonical runtime owns a real store namespace, all
ordinary physical effects cross one fault-interposable production boundary,
and backend capability claims are tied to observed media behavior.

## C.5: Durable Page, Segment, Extent, And Manifest Path

### Goal

Replace the heap-shaped physical-format runtime with ordinary record append,
locate, scan, and reopen over real page, segment, extent, and manifest files.

### Boundary

Physical format continues to own byte grammar and verification. Media owns
effects. The runtime composes them. `PersistedPhysicalLayout` may remain a
bounded offline/test representation where honestly named, but it is not the
production store, reopen input, or whole-store transport.

### Must Ship

- file-backed page, segment, extent, root-manifest, segment-manifest,
  extent-manifest, and free-space structures
- bounded bootstrap catalog sufficient to find current roots without loading
  every page or extent
- stable physical references, generation reuse detection, framed records, slot
  directories, allocation classes, and append/locate/scan operations over the
  real media port
- copy-on-write or append-and-publish manifest updates with exact publication
  ordering
- bounded readers and writers that operate on ranges or frames rather than
  copying the complete store into `Vec` collections
- versioned format compatibility and typed unsupported-version outcomes
- explicit removal of production `persisted_layout()` round trips and replay-
  artifact reopen
- exact page, byte, allocation, manifest, reference, copy, and scan counters
- a typed handoff to C.6 naming the frame-loading and dirty-publication seams

### Non-Fake Acceptance Setup

- **Production subject:** canonical runtime append, locate, scan, close, and
  reopen operations backed by the C.4 filesystem media port.
- **Initial world:** absent root, small page size forcing multiple pages, at
  least two segments, one extent-backed record, and a memory budget too small
  to hold all persisted page bytes at once.
- **Execution:** append deterministic records through the facade, publish
  manifests, close, discard the process, and reopen in a fresh executable using
  only root plus configuration. Locate records in non-write order and perform a
  bounded scan.
- **Independent observation:** an offline format walker parses the directory
  and emits page/segment/extent/reference topology without constructing the
  runtime.
- **Assertions:** record parity, exact physical references, stable generations,
  exact artifact topology, bounded bytes read, no full layout construction,
  and zero backend-residue guessing. Reopening with one stale manifest and one
  unsupported version fails at the declared boundary.
- **Controlled defect:** make reopen accept a stale generation or load all
  pages into a vector before locate. Generation or materialization predicates
  must fail independently.
- **Forbidden substitutes:** writer-supplied `PersistedPhysicalLayout`, replay
  artifacts, a test-generated manifest, or proof against a memory-backed
  `PhysicalStoreRuntimeStorage` cannot close this milestone.

### Closeout Gate

`C.5` closes only when physical records survive a real process boundary and
fresh reopen discovers them from bounded on-disk roots without complete heap
materialization or caller-supplied persisted state.

## C.6: Buffer Pool And Bounded Physical Access Join

### Goal

Make the C.5 file-backed store operate through the bounded buffer pool for
ordinary reads and writes, with leases, dirty state, eviction, and memory
admission governing real frames rather than parallel test models.

### Boundary

The buffer pool owns residency and frame lifecycle. Physical format owns
decoding. The media port owns reads and writes. Stable semantic MVCC and Query
residency remain outside Part I.

### Must Ship

- runtime-owned buffer-pool construction with hard resident, pinned, dirty,
  prefetch, writeback, and operation-allocation budgets
- page fault, pin, lease, decode, dirty transition, writeback, eviction, and
  stale-generation flows against C.5 files
- zero-copy or explicitly bounded-copy record views with lifetime tied to a
  frame lease
- rejection or backpressure before a request can exceed memory, pin, or dirty
  limits
- separation of foreground, recovery, scrub, maintenance, verifier, and blob
  allocation scopes
- prefetch/read-ahead and write-behind admission through the scheduler seam,
  without claiming C.10 QoS closure early
- exact resident-byte, allocation, pin, dirty, fault, hit, eviction, copy,
  writeback, and denial counters
- compile-time prevention of record views outliving their frame authority

### Non-Fake Acceptance Setup

- **Production subject:** the C.5 runtime read/write facade with its real
  buffer-pool and filesystem path.
- **Initial world:** fresh-process reopen of a store at least eight times larger
  than the configured resident-byte budget; deterministic hot, cold, sequential,
  and pinned access sets.
- **Execution:** interleave reads, appends, forced evictions, dirty-page
  pressure, prefetch, and a denied over-pin request while continuously sampling
  admitted memory at the allocation boundary.
- **Independent observation:** process-level allocation instrumentation and
  media-port counters are compared to buffer-pool receipts. Final data is
  checked after a second fresh-process reopen.
- **Assertions:** peak admitted bytes never exceed the hard budget; exact pins
  and dirty pages respect limits; cold reads cause real file I/O; hot reads do
  not; evicted records re-fault correctly; forbidden whole-store allocation and
  stale record views fail.
- **Controlled defect:** hide one complete-store copy behind a fixture helper or
  permit eviction of a pinned frame. Allocation or lease predicates must fail
  at the causal boundary.
- **Forbidden substitutes:** a synthetic frame table not used by the C.5
  runtime, final RSS alone, data smaller than the memory budget, or nonzero-only
  counters cannot close this milestone.

### Closeout Gate

`C.6` closes only when all ordinary file-backed record access is mediated by
bounded residency, stores materially larger than memory remain operational,
and frame-lifetime and budget claims are mechanically falsifiable.

## C.7: WAL, Checkpoint, Root Publication, And Acknowledgment Join

### Goal

Join the existing WAL, durability-ordering, checkpoint, pageLSN, manifest, and
root-publication mechanisms into one canonical durable write progression.

### Boundary

This milestone owns physical transaction ordering, not semantic MVCC or Query
acknowledgment. A physical write acknowledgment means exactly the declared
physical durability boundary was reached under the admitted backend profile.

### Must Ship

- one sealed progression from admitted physical batch through WAL append,
  required WAL barrier, page/extent mutation, pageLSN assignment, checkpoint or
  root publication, namespace durability, and physical acknowledgment
- canonical WAL frame and transaction identities bound to exact physical
  effects and idempotency keys
- WAL-before-data enforcement and typed barriers for backend capability tiers
- group-commit-compatible batching without changing individual mutation
  identity or allowing premature acknowledgment
- fuzzy or non-blocking checkpoint capture with a bounded WAL tail and exact
  source range
- atomic current-root publication and old-root retention sufficient for C.8
  recovery
- explicit indeterminate physical outcome when failure occurs after possible
  durability but before the caller can observe completion
- exact frames, bytes, fsyncs, directory syncs, page writes, pageLSNs,
  checkpoints, root swaps, grouped mutations, and acknowledgment counters
- elimination of isolated WAL/file durability demonstrations that do not feed
  the canonical runtime progression

### Non-Fake Acceptance Setup

- **Production subject:** canonical runtime batch append and physical
  acknowledgment facade using real C.4 media, C.5 artifacts, and C.6 residency.
- **Initial world:** one real store with a published root, an admitted durable
  backend profile, three mutations sharing a group-commit opportunity, and
  named yieldpoints before and after every persistence boundary.
- **Execution:** run the control batch, then one fresh store per crash seam:
  before WAL append, during WAL append, after WAL write/before sync, after WAL
  sync/before data, during data write, after data/before root publication,
  after root publication/before directory sync, and after durability/before
  observed acknowledgment.
- **Independent observation:** capture files after abrupt process termination;
  a raw WAL/page inspector records durable prefixes, pageLSNs, roots, and sync
  evidence. C.8 will later decide full recovery, but C.7 must already prove
  impossible acknowledgment states.
- **Assertions:** no acknowledged batch lacks its required barriers; no data
  page outruns its WAL basis; torn tails are distinguishable; group commit
  preserves three identities; exact barrier counters match the injected seam.
- **Controlled defect:** acknowledge before WAL sync and separately write a
  pageLSN ahead of durable WAL. The acknowledgment and ordering predicates must
  fail at distinct locations.
- **Forbidden substitutes:** an in-memory WAL, simulated durability receipt,
  calling the file durability executor without the canonical runtime, or a
  crash represented by `Err` while the process survives cannot close this
  milestone.

### Closeout Gate

`C.7` closes only when one real production write progression owns every
physical durability edge and no acknowledgment can be manufactured without
the exact file, directory, ordering, and root-publication effects required by
its backend profile.

## C.8: Fresh-Process Recovery And Reopen

### Goal

Recover the canonical physical runtime deterministically from checkpoint,
WAL tail, pages, and manifests after real process death.

### Boundary

Recovery decides physical source precedence and reconstructs physical current
state. It does not readmit Query, Relational, Signal, or Bridge authority. It
may produce typed physical truth and handoff evidence for later Part II
readmission.

### Must Ship

- bounded bootstrap from current checkpoint plus WAL tail, never history from
  genesis or a supplied heap layout
- deterministic recovery-source precedence across current and previous roots,
  checkpoints, WAL segments, partial pages, compaction products, and residue
- torn-tail rejection, idempotent redo, pageLSN comparison, incomplete
  publication handling, and closed-work quiescence
- exact resolution of acknowledged, unacknowledged-not-durable, durable-but-
  unacknowledged, and indeterminate physical operations
- new runtime identity and fresh handles after every recovery
- recovery time/work bounded by checkpoint interval, tail, and damaged scope,
  not total store size
- post-recovery checkpoint/root cleanup policy that preserves evidence until
  current truth is safely published
- exact bootstrap bytes, manifests, WAL frames, page redo, skipped redo,
  rejected tails, residue denials, and recovery allocation counters

### Non-Fake Acceptance Setup

- **Production subject:** a dedicated writer executable and a distinct recovery
  executable both using the canonical runtime facade.
- **Initial world:** deterministic operations with periodic checkpoints over a
  store larger than the memory budget. The harness knows expected semantic
  record values from its independent history model.
- **Execution:** the parent harness starts the writer, waits for a named
  production yieldpoint, terminates it without cleanup, records writer process
  identity, then launches the recovery executable with only root,
  configuration, backend profile, and output-evidence path.
- **Independent observation:** a third offline process parses persisted
  artifacts without constructing the runtime. Neither recovery nor verifier
  receives writer heap objects, replay artifacts, decoded values, or expected
  truth.
- **Assertions:** recovered record/model parity, new process and runtime
  identities, deterministic classification for identical bytes, exact
  acknowledged/indeterminate outcomes, bounded tail work, and disagreement
  visibility between runtime and verifier.
- **Controlled defect:** preserve one writer registry across restart or ignore
  pageLSN during redo. Crash-isolation or redo predicates must fail and
  localize separately.
- **Forbidden substitutes:** same-process recovery, graceful close, replay-
  artifact reconstruction, copied `PersistedPhysicalLayout`, global singleton
  state, or calling runtime decode from the offline verifier cannot close this
  milestone.

### Closeout Gate

`C.8` closes only when a killed writer can be replaced by a genuinely fresh
process that reconstructs one deterministic physical truth from persisted
authority alone inside declared memory and recovery-work bounds.

## C.9: Physical Integrity, Corruption Localization, And Offline Truth

### Goal

Bind checksums, framing, generation validation, quarantine, scrub, and offline
verification to the real C.5 through C.8 artifacts before damaged bytes can
enter logical or recovery interpretation.

### Boundary

Integrity owns physical validity and localization. Recovery owns source
precedence. Artifact owners decide whether damaged derived material can be
rebuilt. The offline verifier is read-only observation and cannot mint current
runtime or repair authority.

### Must Ship

- checksum and structural validation on every ordinary page, extent, WAL,
  checkpoint, root/segment/extent manifest, free-space, index, and blob path
  introduced so far
- decode refusal before logical or owner-specific interpretation after physical
  failure
- structurally aware corruption targets and typed localization by artifact,
  field, identity, range, and expected blast radius
- online scrub and independent offline walk over the same stored format through
  distinct authority paths
- quarantine observations separated from reachability mutation and repair
  authorization
- classification of intact authority, damaged authority, rebuildable derived
  state, quarantined region, unsupported version, unknown, and indeterminate
- verifier/runtime disagreement as explicit evidence rather than hidden
  reconciliation
- exact checked, failed, skipped-decode, quarantined, rebuildable, unknown,
  and bytes-read counters

### Non-Fake Acceptance Setup

- **Production subject:** real files produced by C.7 and recovered by C.8;
  corruption is delivered through the C.4 storage interposer at declared write
  seams or by an offline artifact editor only after the process is dead and the
  target field is predeclared.
- **Initial world:** independently recorded clean artifact manifest containing
  at least authority pages, derived pages, WAL, checkpoint, root manifest, and
  free-space metadata.
- **Execution:** apply checksum, length, generation, pointer, payload, removal,
  duplication, and stale-version operators to isolated copies; run runtime
  reopen and offline verification independently.
- **Independent observation:** expected target and blast radius are fixed before
  either observer runs. The verifier shares stable format declarations only,
  not runtime recovery, cache, normalization, or decision code.
- **Assertions:** exact localization, decode refusal, distinct authority versus
  derived outcomes, deterministic repeated classification, and explicit
  disagreement evidence.
- **Controlled defect:** ignore one checksum and separately make the verifier
  call runtime recovery parsing. Mutation and structural-independence predicates
  must fail.
- **Forbidden substitutes:** arbitrary byte scribbling with `is_err()`, private
  struct mutation, corruption after decode, or the same parser/decision path
  returning the same answer twice cannot close this milestone.

### Closeout Gate

`C.9` closes only when every current physical authority family is rejected or
localized before semantic use when damaged, and an independent read-only
process can disagree with live recovery without sharing its authority path.

## C.10: Stable Reads, Scheduled I/O, And Maintenance Interference

### Goal

Join physical leases, epochs, latches, copy-on-write publication, reclaim
barriers, I/O scheduling, and foreground/background admission around the real
runtime so concurrent maintenance cannot destabilize bytes or hide unbounded
interference.

### Boundary

This is physical isolation and I/O coordination, not semantic MVCC. A physical
read plan proves that referenced bytes remain stable; it does not decide which
semantic version a user may observe.

### Must Ship

- stable read plans bound to exact roots, generations, physical references,
  security scope, and lease lifetime
- real latch/epoch/hazard or equivalent protection integrated with C.5 pages,
  C.6 frames, C.7 root publication, and C.8 recovery
- copy-on-write maintenance publication with protected-old-generation
  retention and safe reclaim
- scheduler-owned foreground and background queues, reservations, queue-depth
  admission, pacing, cancellation, and backpressure over the C.4 media port
- checkpoint, scrub, compaction, reclaim, backup-read, verifier-read, and blob
  work classes with explicit cost and interference posture
- typed stale-plan retry/rejection, deadlock prevention or detection,
  starvation bounds, and unsupported-QoS outcomes
- exact latch, lease, retry, blocked-reclaim, queue, yield, sync-delay,
  foreground-wait, and maintenance-debt counters

### Non-Fake Acceptance Setup

- **Production subject:** canonical runtime read and write facades plus real
  checkpoint, scrub, rewrite, reclaim, and scheduler operations.
- **Initial world:** store larger than memory, one pinned old-root reader, one
  current reader, foreground writes, and bounded background queues under a
  deterministic interleaving schedule.
- **Execution:** force root rewrite, checkpoint, scrub, and attempted reclaim
  while readers pause at named production yieldpoints; inject I/O latency and
  crash during publication; reopen fresh afterward.
- **Independent observation:** a serial physical reference model predicts
  allowed roots and generations; media counters explain actual I/O ordering;
  post-crash verifier checks reachable generations.
- **Assertions:** no half-published root, protected bytes remain readable,
  stale plans retry or deny typed, reclaim waits exactly where required,
  foreground reservations remain visible, and scheduler work corresponds to
  actual backend operations.
- **Controlled defect:** reclaim despite a live lease and separately route
  background sync outside the scheduler. Lease-safety and scheduling-bypass
  predicates must fail independently.
- **Forbidden substitutes:** simulated schedules over fake pages, sleeps used
  as correctness, branch labels standing in for physical disjointness, or
  scheduler receipts without backend I/O cannot close this milestone.

### Closeout Gate

`C.10` closes only when readers and real maintenance interleave over persisted
bytes without unstable visibility, and foreground/background cost is governed
and explained at the actual I/O boundary.

## C.11: Layout, Index, And Native Blob Adoption

### Goal

Move B-tree/LSM access paths, rebuildable indexes, chunk trees, blob streaming,
dedupe, and reclaim onto the canonical runtime rather than parallel fixture or
sidecar paths.

### Boundary

Layout and indexes are derived physical access structures unless explicitly
classified otherwise. Blobs may be authoritative artifacts, but chunk
placement and dedupe indexes do not redefine blob identity. Query pushdown and
semantic graph traversal remain Part II.

### Must Ship

- artifact-family registry binding each admitted family to physical layout,
  source authority, access operations, rebuild basis, format version,
  integrity class, retention, and recovery participation
- B-tree and LSM point/range/prefix/scan paths over C.5 pages and C.6 residency
  with C.10 stable plans and scheduling
- deterministic index rebuild and corruption fallback whose cost and support
  posture are explicit
- native content-addressed chunk-tree storage through the same media, WAL/root,
  integrity, lease, scheduling, and recovery boundaries
- constant-memory streaming ingest/read/verify/export/import and interrupted
  ingest recovery
- dedupe, collision, reachability, orphan, tier, tenant/key scope, and reclaim
  policies with exact physical effects
- broad-scan denial where an admitted indexed path is required
- exact page touches, probes, ranges, amplification, chunk bytes, resident
  bytes, copies, dedupe, reachability, and reclaim counters

### Non-Fake Acceptance Setup

- **Production subject:** canonical runtime family registration, B-tree/LSM
  access, and blob streaming facades over the reconstructed physical platform.
- **Initial world:** indexed data and a blob each materially larger than the
  memory budget, repeated content for dedupe, at least two segments, and one
  active reader lease during rewrite/reclaim.
- **Execution:** ingest, point/range query, stream ranges, interrupt and resume
  blob ingest, corrupt derived index and one chunk, rebuild, rewrite layout,
  attempt reclaim, crash, and fresh-process reopen.
- **Independent observation:** canonical key/value and blob-digest models are
  built from the input generator; offline traversal measures pages, chunks,
  roots, reachability, and orphan sets without using runtime access APIs.
- **Assertions:** semantic byte parity, constant-memory slope, exact access and
  amplification counters, rebuild parity, corruption localization, safe lease
  preservation, dedupe scope honesty, and no sidecar or whole-object path.
- **Controlled defect:** hide a full blob materialization and separately accept
  a corrupted derived index as authority. Allocation and rebuild-basis
  predicates must fail.
- **Forbidden substitutes:** tiny blobs, in-memory indexes, test fixture files
  not produced through the runtime, or access receipts unsupported by observed
  page/chunk I/O cannot close this milestone.

### Closeout Gate

`C.11` closes only when every retained S.8/S.7 access and blob mechanism runs
through the one reconstructed platform with bounded memory, recoverable
publication, and destroy/rebuild honesty for derived structures.

## C.12: Formal Protocol Rebinding To Executable Owner Transitions

### Goal

Rebind S.9 models and certification mappings to the reconstructed runtime's
actual durability, recovery, visibility, quarantine, admission, and
publication transitions.

### Boundary

Formal models define finite checked law; production owners execute and decide;
certification observes correspondence. Model actions, copied ids, and checked
verdicts do not become runtime authority.

### Must Ship

- complete state/action mapping from C.7 through C.11 owner transitions to the
  required WAL/checkpoint, recovery-source, stable-read/reclaim,
  compaction/publication, quarantine/readmission, import, and replication
  models
- removal or rejection of modeled states with no production owner outcome and
  production outcomes with no modeled case where the roadmap requires coverage
- explicit backend, atomicity, durability, clock, scheduling, and bounded-state
  assumptions
- counterexample lowering into C.1 certification scenarios through production
  yieldpoints and the C.4 media interposer
- executed-trace lifting back into model actions without allowing trace
  observations to authorize production
- controlled weakened-transition variants for each model family
- exact mapping, action, invariant, deadlock, bound, counterexample, and
  localization evidence

### Non-Fake Acceptance Setup

- **Production subject:** named reconstructed owner transitions, not model-only
  catalog rows or certification-generated receipts.
- **Initial world:** a checked model configuration with finite bounds and a
  generated production scenario covering every mapped action at least once.
- **Execution:** check the model; execute the production scenario through real
  runtime/media boundaries; translate an injected model counterexample into a
  runnable production schedule; lift the resulting owner trace for comparison.
- **Independent observation:** mapping completeness is generated from owner
  declarations and model metadata. Certification may compare identities and
  transitions but cannot supply owner outcomes.
- **Assertions:** exact bidirectional mapping, explicit bound-exhaustion
  posture, invariant parity, deterministic counterexample identity, and no
  model verdict accepted by a production API.
- **Controlled defect:** weaken one owner transition without changing the model
  and weaken one model invariant without changing the owner. Conformance and
  mutation predicates must fail in opposite directions.
- **Forbidden substitutes:** fictional owner cases, checked-model success with
  unreachable actions, bound exhaustion reported as proof, or model verdicts
  used as production witnesses cannot close this milestone.

### Closeout Gate

`C.12` closes only when S.9 checked law corresponds exactly to the real
reconstructed transitions and controlled divergence is detected and localized
from either side.

## C.13: Physical-Foundation Recertification And S.10 Re-entry

### Goal

Re-evaluate every S.1 through S.9 claim over the reconstructed production
runtime, retire obsolete paths, and produce the only handoff allowed to resume
S.10.

### Boundary

This milestone adds no new physical feature. It is the correction courtroom
and source cutover. S.10, S.11, S.12, and Part II retain their own future
closeout obligations.

### Must Ship

- generated S.1 through S.9 capability-to-owner-to-production-path-to-proof
  matrix
- hard deletion or non-production quarantine of heap runtimes, replay-based
  reopen, duplicate backends, fake physical fixtures, obsolete certification
  paths, and shadow authority discovered by the program
- dependency checks proving ordinary product paths reach only the canonical
  runtime facade and physical owners depend in the admitted direction
- cross-milestone hostile execution combining real writes, stores larger than
  memory, checkpoint/WAL recovery, corruption, stable readers, scheduled
  maintenance, index rebuild, and blob streaming
- mutation-sensitivity aggregate covering acknowledgment inversion, live-state
  reuse, checksum bypass, generation bypass, reclaim-with-live-lease,
  scheduler bypass, broad scan, full materialization, derived-authority
  promotion, and model/owner drift
- reproducible source, binary, format, backend, harness, and profile-bound
  evidence bundle
- typed `S10PhysicalPlatformReadiness` or equivalently responsibility-named
  handoff whose private construction requires every restored claim and whose
  payload exposes the exact S.10 owner ports
- explicit list of remaining non-platform-grade or unsupported capability
  profiles; no unnamed debt

### Non-Fake Acceptance Setup

- **Production subject:** one release-built canonical runtime and the distinct
  offline verifier executable. No milestone-local runtime or fixture backend
  may satisfy an aggregate predicate.
- **Initial world:** real store at least eight times the memory budget with
  pages, extents, checkpoints, WAL tail, B-tree/LSM indexes, multi-segment
  blobs, derived and authoritative artifacts, and declared backend assumptions.
- **Execution:** foreground reads/writes continue during checkpoint, scrub,
  rewrite, reclaim, index rebuild, and blob streaming; inject crash and
  corruption at declared seams; start a fresh recovery process; run offline
  verification; repeat controlled mutants.
- **Independent observation:** an external history model supplies semantic
  expectations; the offline verifier supplies physical classification; OS/media
  observation supplies actual artifact and barrier evidence. None receives
  runtime heap state.
- **Assertions:** all restored S.1 through S.9 predicates, exact resource and
  interference counters, deterministic recovery, independent classification,
  zero forbidden paths, full mutation localization, and evidence freshness.
- **Controlled defects:** all named aggregate mutants must fail the specific
  predicate and no unrelated green lane may mask that failure.
- **Forbidden substitutes:** combining milestone receipts without rerunning the
  joined system, using a memory backend, reusing a live process, omitting large-
  than-memory pressure, or granting readiness through a public constructor
  cannot close this milestone.

### Closeout Gate

`C.13` closes only when S.1 through S.9 are true of one real physical database
runtime, every obsolete substitute is unreachable from production, the joined
hostile program detects its controlled defects, and the sealed S.10 readiness
handoff is issued from fresh evidence.

## Required Engineering Specs

Each reconstruction milestone receives a separate engineering spec before
implementation:

- `physical-reconstruction-c1-test-execution-architecture.md`
- `physical-reconstruction-c2-executable-reality-ledger.md`
- `physical-reconstruction-c3-sealed-runtime-lifecycle.md`
- `physical-reconstruction-c4-production-media-boundary.md`
- `physical-reconstruction-c5-durable-physical-record-path.md`
- `physical-reconstruction-c6-buffer-pool-runtime-join.md`
- `physical-reconstruction-c7-durable-publication-join.md`
- `physical-reconstruction-c8-fresh-process-recovery.md`
- `physical-reconstruction-c9-integrity-and-offline-truth.md`
- `physical-reconstruction-c10-isolation-and-io-coordination.md`
- `physical-reconstruction-c11-layout-index-blob-adoption.md`
- `physical-reconstruction-c12-formal-owner-rebinding.md`
- `physical-reconstruction-c13-recertification-and-s10-readiness.md`

C.2 through C.13 specs inherit the Non-Fake Physical Acceptance Test Contract,
then make their setup more concrete. Repeating only “control, hostile, and
reopen lane” is not sufficient; the spec must name executables, initial files,
process deaths, forbidden inputs, independent observers, exact counters, and
controlled defects. C.1 instead closes through direct Cargo/test execution and
ordinary CI status as defined by its engineering spec.

## Must Preserve

- Physical Store owns byte survival and physical access.
- Physical format remains meaning about bytes, not the owner of media effects.
- Relational remains semantic MVCC, visibility, branch, and transaction
  authority.
- Query remains the future ordinary domain-facing language through Part II.
- Signal and Runtime Bridge retain derived and causal authority.
- Existing strong mechanisms and tests are preserved when they can bind to the
  production path without weakening ownership.
- Certification remains a courtroom, never a production authority source.
- Store larger than memory, crash, corruption, stable reads, maintenance,
  blobs, and independent verification remain non-negotiable physical claims.

## Acceptance Evidence

The reconstruction program emits:

- Cargo-derived test catalog and direct execution products
- executable reality and artifact-family ledgers
- runtime authority and lifecycle compile-time proof
- media capability qualification evidence
- real physical artifact manifests
- bounded-memory and allocation evidence
- durability and crash-boundary evidence
- fresh-process recovery and determinism evidence
- corruption localization and offline-verifier comparison
- isolation and interference evidence
- layout, amplification, rebuild, and blob-streaming evidence
- formal owner/model conformance and mutation evidence
- joined S.1 through S.9 recertification bundle
- sealed S.10 physical-platform readiness handoff

## Sequencing Rules

- C.1 closes before any later milestone begins implementation.
- C.2 closes before a spec may preserve or delete an existing mechanism based
  on assumed production reachability.
- C.3 and C.4 establish the only runtime and media seams later milestones may
  consume.
- C.5 precedes buffer, WAL, recovery, integrity, isolation, layout, and blob
  claims because those claims require real physical artifacts.
- C.6 precedes greater-than-memory recovery and access claims.
- C.7 precedes C.8 because recovery must consume one real durability law.
- C.8 precedes C.9 operational classification because corruption evidence must
  be tested against an actual recovery path.
- C.9 precedes C.10 and C.11 closeout so maintenance and artifact families
  cannot publish unchecked bytes.
- C.10 precedes C.11 closeout so index and blob rewrite/reclaim use real stable
  reads and scheduled I/O.
- C.12 follows the executable transitions it models.
- C.13 is last and is the only route back into S.10.

## Completion Standard

This roadmap is complete only when Worth Store can honestly say:

- changing one owner has a fast, narrow, trustworthy feedback loop
- all proof lanes are explicit and expensive proof is expensive for a named
  reason rather than repeated compilation
- the physical runtime is sealed and cannot be cloned or reopened from supplied
  heap truth
- ordinary writes create real files through one production media boundary
- records live in real pages, segments, extents, and manifests
- ordinary access is buffer-pool bounded for stores larger than memory
- WAL, page, checkpoint, root publication, and acknowledgment form one durable
  progression
- killed processes recover from persisted bytes in fresh processes
- physical damage is rejected before semantic decode and independently
  classifiable offline
- stable readers survive real maintenance and scheduled I/O interference
- layouts, indexes, and blobs use the same canonical physical platform
- formal laws map to executable owner transitions
- S.1 through S.9 have been recertified over the joined production runtime
- S.10 receives a sealed readiness handoff rather than another vocabulary claim

Only then may operational recovery resume. The later runtime-integration
roadmap can subsequently build the existing Worth runtime on top of this
physical platform without inheriting a fake database boundary.
