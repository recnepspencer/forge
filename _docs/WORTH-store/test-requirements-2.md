# WORTH Store Test Requirements 2: Physical Adversarial Certification Harness

## Purpose

This document extends
[test-requirements.md](/Users/Esther/Documents/Programming/WORTH_workspace/WORTH/_docs/worth-store/test-requirements.md)
for the Roadmap 2 physical database foundation.

The first test-requirements document defines what must be proven. This document
defines how the physical proof system itself must be built so the tests are not
minimum-effective-dose unit checks.

Roadmap 2 cannot be certified by hand-written happy paths, direct private-field
mutation, post-hoc byte pokes, or tests that only exercise the public API under
ordinary process execution. It requires a realistic adversarial harness capable
of simulating hostile media, crashes, I/O behavior, memory pressure, corruption,
maintenance interference, operator repair, security boundaries, and long-running
workload churn in a repeatable and machine-checkable way.

## Core Standard

The physical certification harness must be adversarial infrastructure, not a
collection of clever tests.

The harness must:

- drive the store through normal production-facing storage boundaries
- inject faults at declared physical seams, not by editing private state after
  the fact
- produce reproducible evidence bundles
- compare independent observers, not a run against itself
- enforce resource envelopes during the workload, not only inspect final state
- simulate realistic crash, media, I/O, memory, and operator conditions
- make shortcuts visible and forbidden
- scale to long-running and large-data workloads
- keep semantic truth and physical byte behavior both observable

If the test harness cannot inject a failure without special-casing production
code paths, the architecture is not testable enough.

## Global Harness Adversarial Constraint

The Roadmap 2 physical certification harness must survive this hostile testing
condition:

> A deterministic workload generator drives stores larger than memory through
> writes, reads, checkpoints, compaction, reclaim, scrub, blob streaming,
> replication preparation, backup, restore, repair, and security operations
> while a fault scheduler injects crashes, torn writes, reordered persistence,
> delayed flushes, byte corruption, stale generations, I/O stalls, memory
> pressure, key failures, tenant-boundary attacks, and operator actions at
> declared physical seams. The harness must prove semantic parity,
> physical-bounds compliance, corruption localization, recovery determinism,
> latency isolation, and operator diagnosis from persisted bytes and independent
> evidence, not from live heap state or implementation-local assumptions.

## Harness Non-Negotiables

- Tests must use the same storage boundary shape production uses.
- Fault injection must be implemented as explicit harness layers or backend
  capability shims, not as arbitrary mutation of private store structs.
- Crash injection must terminate or discard process/runtime state and reopen
  from persisted bytes.
- Corruption injection must target declared physical artifacts: pages, frames,
  WAL records, manifests, index pages, chunk trees, blob chunks, key envelopes,
  or audit records.
- Memory tests must run with data larger than the configured memory envelope.
- Blob tests must use data large enough that whole-object materialization would
  violate the declared memory envelope.
- Performance tests must include structural counters and latency envelopes; raw
  elapsed time alone is never proof.
- Every hostile lane must have a control lane and an independent verifier lane
  where the claim can be checked offline.
- Tests must assert forbidden shortcuts: full-store heap load, live-state reuse,
  backend residue guessing, unbounded allocation, unsupported durability claims,
  and logical decode after physical integrity failure.
- Test outputs must be machine-checkable and reproducible from a seed,
  workload profile, backend profile, and harness profile.

## Harness Architecture Requirements

### 1. Workload Generator

The harness must include a deterministic workload generator that can produce
long, mixed, legal store histories.

The generator must be a stateful legal-history engine, not random API fuzzing
and not a bag of calls. It must maintain its own explicit semantic world-state
and use that world-state to decide which operations are legal next.

Required capabilities:

- seed-controlled generation
- explicit model state independent of the runtime under test
- legal-next-operation selection from the model state
- illegal-input generation only in lanes explicitly marked as illegal-input
  lanes
- expected semantic invariants recorded after each step
- operation mixes for commits, branch creation, snapshots, deltas, retention,
  compaction, tiering, subscription-support artifacts, bulk ingest, blob ingest,
  replication preparation, backup, restore, audit, repair, and security actions
- configurable store size, branch depth, blob size, artifact count, tenant
  count, history depth, checkpoint cadence, compaction pressure, and I/O pressure
- workload profiles for:
  - small deterministic smoke
  - medium CI certification
  - large local soak
  - long-running stress
  - domain-shaped workloads for geometry, web/data, AI, and chip/simulation
- generated-operation trace persisted as part of the evidence bundle
- canonical trace artifact that can be replayed exactly
- trace shrink/minimization for failing generated histories
- semantic expectation digest derived from the generator model, not from the
  runtime under test

This is not random API fuzzing. This is legal hostile history generation with
known semantic expectations and physical pressure knobs.

Weak implementations that do not count:

- unordered random API calls
- fuzzing without semantic expectation
- traces that cannot be replayed exactly
- generators that rely on the runtime under test to tell them what is legal
- workloads that vary only call count or byte volume, not history shape

Certification assertions:

- same seed, profile, and harness version produce the same operation history
- replaying the same trace produces the same semantic result
- generated histories target named profiles instead of ad hoc constants
- generated histories include structural pressure such as branch shape,
  checkpoint cadence, compaction debt, restore points, blob reachability,
  audit/security actions, and repair decisions

Required evidence:

- seed
- generated trace
- profile id
- semantic expectation digest
- shrink/minimization result for failing traces

### 2. Fault Scheduler

The harness must include a fault scheduler that can inject failures at declared
operation phases.

Required capabilities:

- deterministic schedule by seed
- phase-aware injection before, during, and after:
  - WAL append
  - frame checksum write
  - data-page write
  - page flush
  - manifest write
  - root/manifest cutover
  - checkpoint publication
  - compaction publication
  - blob chunk write
  - chunk-tree root update
  - dedupe index update
  - reclaim decision
  - backup snapshot
  - restore apply
  - audit append
  - key rotation
  - tenant boundary check
- repeatable crash points
- probabilistic campaigns with minimized failing seeds
- fault-budget controls so tests can run one fault, multiple faults, or sustained
  hostile campaigns

This is not sleeping a thread and hoping the crash lands in an interesting
place. This is phase-addressed failure delivery.

### 2A. Storage-Boundary Fault Interposer

Fault injection must be delivered through a storage-boundary interposer.

Mature implementation shape:

- production store talks to a storage interface
- the test harness swaps in an adversarial wrapper around that interface
- the wrapper forwards real operations to a real or simulated backend
- the wrapper can distort read, write, append, flush, sync, rename, list,
  delete, mmap, direct-I/O, and metadata operations at declared seams
- every distortion is chosen by the fault scheduler and recorded as evidence

The storage interface must expose operations such as:

- `read_at`
- `write_at`
- `append`
- `flush`
- `fsync`
- `directory_sync`
- `rename`
- `open`
- `list`
- `delete`
- `truncate`
- `punch_hole`

For example, when the store appends a WAL record, the wrapper receives the
append request with operation context. The scheduler then decides whether that
seam is allowed, short-written, torn, delayed, reordered, corrupted, failed,
reported successful without durability, or followed by a crash. The wrapper
applies that behavior at the byte boundary and logs requested behavior versus
physical outcome.

This is not magical test mutation. The hostile behavior must pass through the
same I/O boundary production depends on.

### 3. Adversarial Storage Backend

The harness must include an adversarial storage backend or backend wrapper that
implements the production storage traits while exposing controllable hostile
behavior.

Required capabilities:

- short writes
- torn writes
- reordered persistence
- delayed flush
- lost flush
- directory sync failure
- durable rename failure
- stale read
- read-after-write delay
- sector/page corruption
- bit flips
- block disappearance
- duplicate old block return
- ENOSPC simulation
- permission failure
- I/O latency spikes
- bandwidth throttling
- queue-depth saturation
- trim/punch-hole failure

This is not test-only file editing after the store closes. The backend must
deliver hostile behavior through the same I/O boundary the store depends on.

### 4. Crash Harness

The harness must simulate process death honestly.

Required capabilities:

- crash by terminating the store instance without running ordinary cleanup
- reopen from persisted bytes with a fresh process or equivalent isolated runtime
- discard all live heap state, handles, caches, buffer-pool contents, and
  runtime references
- run repeated crash-restart loops
- run crash campaigns across a generated workload
- persist crash point, seed, backend profile, and pre-crash operation trace
- persisted artifact manifest captured before restart
- fresh-runtime identity captured after restart
- proof that live heap, handles, caches, buffer pools, in-memory indexes, mmap
  views, singletons, arenas, and registries from the crashed instance did not
  survive into recovery

This is not calling a recovery function after keeping the same in-memory
objects alive.

Weak implementations that do not count:

- calling recovery on the same process objects
- reconstructing state from in-memory references
- simulating crash by throwing an exception while keeping runtime state alive
- allowing cached derived state to survive restart
- reusing an in-process singleton, arena, mmap view, buffer pool, or global
  registry as part of "restart"

Certification assertions:

- post-crash truth comes only from persisted authority
- repeated crash at the same point gives the same recovery classification
- recovery is deterministic from bytes, format version, and profile
- derived state may be rebuilt, but is never treated as authority

Required evidence:

- crash point id
- pre-crash trace prefix
- persisted artifact manifest
- fresh-runtime identity
- recovery classification
- recovered semantic digest
- offline verifier comparison

### 5. Corruption Injector

The harness must corrupt physical artifacts with structural awareness.

Required capabilities:

- locate pages, frames, manifests, WAL records, indexes, blob chunks, chunk-tree
  roots, audit records, key envelopes, and tenant metadata
- corrupt header fields, payload bytes, checksums, generation counters, lengths,
  versions, root pointers, and digest links
- remove or duplicate physical records
- corrupt one artifact at a time and correlated artifact sets
- emit expected localization target before recovery runs
- verify logical decode is skipped for physically invalid bytes

This is not arbitrary byte scribbling with vague "should fail" expectations.
The injector must know what boundary it is attacking and what diagnosis should
result.

The corruption injector must enumerate artifact classes and target concrete
fields within persisted structures.

Artifact classes include:

- pages
- frames
- WAL records
- segment manifests
- root manifests
- index pages
- chunk trees
- blob chunks
- audit chains
- key envelopes
- tenant metadata

Target field kinds include:

- checksum fields
- length fields
- generation numbers
- page, segment, extent, and chunk pointers
- digest links
- version fields
- header tags
- payload slices
- root pointers
- key ids
- tenant ids

Weak implementations that do not count:

- arbitrary byte scribbling with vague expected failure
- corruption after logical decode
- mutating private structs instead of persisted artifacts
- accepting any thrown error as success
- failing to specify the expected localization target before recovery runs

Certification assertions:

- physically invalid bytes are not logically decoded
- corruption localizes to the attacked boundary or declared blast radius
- offline verifier and live recovery classify damage independently
- authority corruption and derived-only corruption produce distinct expected
  outcomes

Required evidence:

- artifact kind
- field kind
- corruption operator
- target offset, page id, frame id, record id, or chunk id
- expected localization target
- actual localization result
- decode refusal or quarantine result

### 6. Memory And Allocation Pressure Harness

The harness must enforce memory and allocation envelopes during execution.

Required capabilities:

- configurable resident-memory budget
- configurable per-operation allocation budget
- allocation counters by operation class
- page-pin and dirty-page limit enforcement
- synthetic memory pressure while reads, writes, recovery, compaction, and blob
  streaming run
- failure if forbidden whole-store or whole-blob materialization occurs
- distinction between admitted allocation scopes and accidental allocator use
- custom allocator, allocator instrumentation, or equivalent operation-class
  allocation accounting
- admitted memory scopes for read, write, recovery, compaction, scrub, blob,
  verifier, and repair work

This is not measuring RSS after a test finishes. The harness must catch budget
violations at the boundary where they occur.

Weak implementations that do not count:

- only measuring RSS after completion
- using stores that fit comfortably in memory
- not distinguishing heap allocation from mmap or file-cache behavior
- asserting a counter was nonzero instead of verifying exact bounds
- allowing whole-blob or whole-store materialization because the test still
  finishes quickly

Certification assertions:

- store size exceeds configured memory budget
- forbidden materialization causes immediate failure
- working-set ceilings hold during read, recovery, compaction, scrub, verifier,
  repair, and blob operations
- allocation totals are attributable to operation classes

Required evidence:

- configured memory budget
- maximum observed admitted memory
- allocation histogram by operation class
- peak page pins
- peak dirty pages
- over-budget trip report if failure occurs

### 7. Latency And I/O Pressure Harness

The harness must test foreground isolation under realistic background pressure.

Required capabilities:

- foreground read/write streams
- background compaction, checkpoint, scrub, blob ingest, blob migration, backup,
  replication preparation, and repair streams
- backend-level latency injection
- queue-depth pressure
- bandwidth throttling
- fsync/fdatasync delay simulation
- p50/p95/p99/p999 or declared equivalent latency distributions
- structural interference counters explaining why latency changed
- lane-level foreground SLO declarations
- causal attribution for queueing, backpressure, flush delay, reclaim debt,
  compaction debt, blob contention, scrub pressure, backup pressure, and repair
  pressure

This is not a throughput benchmark. This proves foreground behavior under
hostile physical work.

Weak implementations that do not count:

- single-stream benchmarks
- aggregate throughput only
- no attribution counters
- no distinction between foreground and maintenance work
- no p95/p99 lane-level visibility

Certification assertions:

- foreground SLOs are measured under declared background pressure
- latency deviations correlate to causal counters
- queueing, backpressure, flush delay, reclaim debt, compaction debt, scrub
  debt, repair debt, or blob contention are visible
- pathological stalls are bounded or explicitly classified

Required evidence:

- latency distributions by lane
- interference counters
- maintenance backlog counters
- queue-depth statistics
- causal explanation table

### 8. Blob-Scale Harness

The harness must include large-object workloads that would fail if blobs were a
sidecar or whole-object heap path.

Required capabilities:

- multi-GB logical blob profiles, with smaller deterministic local profiles that
  still exceed configured memory
- streaming ingest/read/verify/export/import
- interrupted upload and resume
- chunk dedupe
- chunk corruption
- chunk migration
- partial replication
- retention and orphan reclaim
- memory and I/O counters tied to chunk operations

This is not writing a small string to a blob API and asserting it comes back.

Weak implementations that do not count:

- tiny blobs
- tests that pass through a convenience in-memory path
- single-shot read/write APIs that hide chunk streaming
- no chunk-level counters
- no interrupted/resume lanes

Certification assertions:

- blob workloads exceed the configured memory envelope
- chunk path, not whole-object path, is exercised
- dedupe and corruption behavior is visible at chunk granularity
- retention and reclaim leave no silent orphan leaks

Required evidence:

- blob sizes versus memory budget
- chunk counts
- streaming counters
- resume checkpoints
- reclaim report
- chunk corruption localization

### 9. Offline Verifier

The harness must include an independent offline verifier path.

Required capabilities:

- inspect store files without constructing the live store runtime
- parse physical pages, frames, WAL records, manifests, indexes, chunks, audit
  chains, key envelopes, and tenant metadata
- validate checksums, generation counters, digest links, pageLSNs, checkpoint
  manifests, and chunk trees
- produce trusted/degraded/quarantined/unrecoverable reports
- compare against live recovery output without sharing implementation state

This is not the same code path returning the same answer twice.

The verifier must be structurally independent read-only analysis. It may share
stable format definitions, but it must not share the live recovery authority
path, runtime decode path, runtime caches, or normalization path being verified.

Weak implementations that do not count:

- calling runtime decode in a different binary
- sharing recovery code, parser core, or normalization path with the live store
  in a way that makes identical mistakes invisible
- using runtime-only metadata caches
- reading data through public runtime APIs
- being "independent" in name only

Certification assertions:

- verifier can disagree with runtime recovery
- disagreement is surfaced as evidence, not hidden
- verifier emits its own classification output
- verifier does not depend on live runtime construction

Required evidence:

- verifier binary or provenance id
- artifact parse report
- classification report
- runtime-versus-verifier comparison table
- disagreement report if any

### 10. Evidence Bundle System

The harness must emit structured evidence bundles for every certification run.

Required bundle fields:

- run id
- seed
- workload profile
- fault profile
- backend profile
- hardware assumption profile
- store format version
- source revision
- binary/provenance identity where available
- operation trace digest
- physical layout report
- resource envelope report
- latency envelope report
- corruption localization matrix
- recovery report
- semantic parity report
- offline verifier report
- failure digest
- counter snapshot
- hazard-analysis links where relevant
- harness version
- verifier version
- fault-delivery log
- crash isolation identity
- coverage matrix summary
- certification predicate results per lane

Evidence bundles must be diffable, machine-checkable, and sufficient for
offline pass/fail evaluation.

Weak implementations that do not count:

- loose logs
- human-only inspection
- missing seeds
- missing binary identity
- missing trace digests
- evidence that cannot prove what actually executed

Certification assertions:

- pass/fail can be decided from bundle contents
- rerun can be reconstructed from bundle fields
- bundle is diffable across revisions
- bundle captures both semantic and physical observations

### 11. Security And Tenant-Boundary Harness

The harness must prove security and tenant boundaries under mixed-tenant
physical pressure, not only at API admission.

Required capabilities:

- mixed-tenant histories where artifacts from multiple tenants coexist
  physically
- scoped credentials and capability profiles for repair, backup, restore, blob,
  audit, verifier, and replication operations
- wrong-tenant access attempts
- stale capability attempts
- rotated-key attempts
- old-snapshot access attempts
- tenant metadata corruption
- shared physical chunk and dedupe boundary pressure

Weak implementations that do not count:

- testing authorization only at the public API layer
- not exercising shared physical artifact paths
- assuming tenant separation because ids differ
- only testing success cases

Certification assertions:

- no cross-tenant decode, restore, repair, backup, verifier, replication, or
  blob access leaks
- stale metadata cannot reauthorize access
- key rotation and backup interaction preserve isolation
- tenant metadata corruption is detected and classified, not ignored

Required evidence:

- tenant topology
- credential or scope profile
- denied-access matrix
- attempted cross-tenant artifact report
- isolation verdict

### 12. Operator Repair And Maintenance Workflow Harness

The harness must certify operator workflows, not only repair APIs.

Required capabilities:

- inspect a degraded store
- run offline verifier
- present admissible repair, quarantine, rebuild, restore, and deny actions
- execute a chosen repair action
- execute wrong-action negative lanes
- rerun verification
- reopen runtime after repair
- compare semantic and physical posture before and after repair

Weak implementations that do not count:

- magical one-call `repair`
- repair with hidden runtime assumptions
- no predeclared expected posture
- no evidence of what was rebuilt versus quarantined
- no negative lane where an operator chooses an inadmissible action

Certification assertions:

- diagnosis is actionable from evidence
- repairs are deterministic and bounded
- repair cannot silently bless corrupted authority
- post-repair classification is explicit
- wrong operator actions fail typed and leave audit evidence

Required evidence:

- pre-repair classification
- chosen repair action
- modified artifact set
- post-repair classification
- semantic delta report
- wrong-action rejection report where applicable

### 13. Recovery Determinism Harness

The harness must prove that identical persisted bytes produce identical
recovery outcomes.

Required capabilities:

- capture persisted artifact digest after crash or fault injection
- recover the same bytes under the same format version and profile multiple
  times
- compare recovery classification, semantic digest, quarantine decisions,
  degraded-derived decisions, and verifier conclusions
- list explicitly allowed nondeterministic metadata

Weak implementations that do not count:

- comparing only broad success or failure
- allowing wall-clock, environment, map iteration order, or thread timing to
  alter recovery result
- nondeterministic replay hidden behind helper APIs

Certification assertions:

- same bytes, format version, runtime version, and profile produce the same
  recovery classification
- semantic result is deterministic
- verifier conclusion is deterministic
- any nondeterministic fields are explicitly listed and excluded from semantic
  comparison

Required evidence:

- persisted artifact digest
- runtime version
- format version
- recovery digest
- verifier digest
- determinism comparison report

### 14. Cross-Backend Qualification Harness

The harness must prove backend and deployment capability claims through a
profile matrix.

Required capabilities:

- backend profile declarations for filesystem, OS, hardware, sector/page
  alignment, flush semantics, rename semantics, direct I/O behavior, mmap
  behavior, and latency assumptions
- qualification lanes for each declared capability
- distinction between emulated and real capabilities
- automatic downgrade or typed rejection for unsupported capabilities

Weak implementations that do not count:

- assuming POSIX means durable
- using one backend to qualify all backends
- not distinguishing simulated capabilities from real deployment capabilities
- making production claims without profile-specific evidence

Certification assertions:

- each durability claim is tied to a qualified deployment profile
- unsupported backend semantics downgrade claims automatically
- release claims are profile-scoped, not universal

Required evidence:

- backend profile id
- filesystem, OS, and hardware assumption profile
- qualification result
- unsupported-capability report

### 15. Mutation-Style Harness Validation

The harness must prove it catches known storage defects.

Required capabilities:

- seeded mutant backends and controlled mutant runtime variants
- expected-failure lanes for each mutant
- evidence that the harness fails the mutant in the intended place

Required mutant classes:

- checksum ignored
- generation check skipped
- stale manifest accepted
- pageLSN ignored
- WAL frame accepted after checksum failure
- compaction publication without durability fence
- reclaim ignores active read lease
- whole-store materialization hidden behind helper
- whole-blob materialization hidden behind helper
- live-state reuse after crash
- verifier shares runtime parser/recovery path
- cross-tenant boundary bypass
- audit-chain tamper accepted
- wrong key accepted
- unsupported backend capability reported as supported

Weak implementations that do not count:

- mutants that do not compile but do not exercise the harness
- mutants with no expected failing lane
- harness validation that only proves one generic failure
- deleting assertions to make mutant tests pass

Certification assertions:

- every required mutant fails in the expected suite lane
- failure localizes to the intended missing check or forbidden shortcut
- mutation validation runs in CI Certification mode for sequences it protects

Required evidence:

- mutant id
- expected failing lane
- actual failing lane
- failure localization
- mutation coverage summary

### 16. Harness Maturity Ladder

Each harness subsystem and Roadmap 2 sequence must declare its maturity level.

Maturity levels:

- `Exists`: the harness component or suite can run a smoke lane
- `SmokeWorks`: deterministic developer smoke lanes pass and emit evidence
- `CiCertifiable`: CI Certification mode covers the sequence's required
  hostile lanes and forbidden-shortcut lanes
- `ReleaseCertifiable`: release-scale, cross-backend, long-running, large-data,
  offline-verifier, and hazard-analysis lanes are available where required

Rules:

- `Exists` is not closeout.
- `SmokeWorks` is not closeout.
- Roadmap 2 sequence closeout requires `CiCertifiable` for that sequence.
- Beta, financial-platform, or aerospace-grade claims require
  `ReleaseCertifiable` for the relevant deployment profile.
- Maturity must be recorded per subsystem, not only for the harness as a whole.

## Certification Modes

The harness must support multiple modes so physical tests can run at different
cost levels without weakening the proof model.

### Mode 1: Developer Smoke

- short deterministic seeds
- small stores
- one or two fault classes
- fast enough for local iteration
- proves harness plumbing, not platform readiness

### Mode 2: CI Certification

- medium generated histories
- store size exceeds configured memory budget
- representative crash, corruption, and I/O pressure lanes
- must run on ordinary CI without special hardware assumptions
- blocks closeout of individual Roadmap 2 sequences

### Mode 3: Local Soak

- longer runs
- larger stores and blobs
- many interleavings
- repeated crash-restart loops
- sustained background work
- used before closeout and after substantial storage changes

### Mode 4: Release Certification

- long-running hostile campaigns
- large stores
- blob-scale workloads
- cross-backend capability matrix
- offline verifier required
- formal model evidence required where applicable
- required for beta, financial-platform, or aerospace-grade claims

### Mode 5: Hardware Qualification

- backend and hardware specific
- validates declared flush, rename, direct I/O, mmap, filesystem, sector, and
  latency assumptions
- required before making claims tied to a physical deployment profile

## Roadmap 2 Harness Mapping

Every Roadmap 2 sequence must identify which harness subsystems it depends on.

| Sequence | Required Harness Subsystems |
| --- | --- |
| S.0 | workload generator, evidence bundle system, maturity ladder |
| S.1 | adversarial storage backend, storage-boundary interposer, offline verifier, evidence bundle system, coverage matrix |
| S.2 | memory/allocation pressure harness, workload generator, mutation-style harness validation |
| S.3 | corruption injector, adversarial storage backend, offline verifier, coverage matrix, mutation-style harness validation |
| S.4 | fault scheduler, storage-boundary interposer, crash harness, adversarial storage backend, offline verifier, recovery determinism harness, mutation-style harness validation |
| S.4.5 | golden-path scenario authoring API, aspect-native scenario definitions, deterministic scheduler, named production-boundary yieldpoints, production-facing driver contracts, actor model, fault/corruption/crash event vocabulary, observer registry, reusable certification-owned oracle families, counter-strength contracts, production-backed fixtures, replayable transcripts, evidence bundle system, generated coverage matrix, maturity ladder, S.4 recovery public-authoring slice, shortcut-rejection public-authoring slice, S.5 readiness shape-probe with non-claim evidence, extension slots, mutation-style harness validation |
| S.5 | fault scheduler, latency/I/O pressure harness, crash harness, recovery determinism harness, coverage matrix |
| S.6 | adversarial storage backend, storage-boundary interposer, latency/I/O pressure harness, cross-backend qualification harness |
| S.7 | blob-scale harness, corruption injector, memory/allocation pressure harness, storage-boundary interposer, coverage matrix |
| S.8 | workload generator, corruption injector, offline verifier, coverage matrix, mutation-style harness validation |
| S.9 | formal model runner or checker integration, evidence bundle system, coverage matrix, mutation-style harness validation |
| S.10 | offline verifier, crash harness, corruption injector, operator repair workflow harness, recovery determinism harness, evidence bundle system |
| S.11 | adversarial storage backend, offline verifier, security and tenant-boundary harness, cross-backend qualification harness, evidence bundle system |
| S.12 | all harness subsystems |

No Roadmap 2 sequence may close before the harness subsystems required for that
sequence exist at least in CI Certification mode and the sequence's coverage
matrix rows are satisfied.

## Required Simulation Profiles

The harness must define named profiles instead of ad hoc test constants.

At minimum:

- `single_page_authority`
- `multi_segment_authority`
- `store_larger_than_memory`
- `deep_branch_history`
- `checkpoint_heavy`
- `compaction_heavy`
- `blob_larger_than_memory`
- `multi_blob_dedup`
- `corrupt_derived_only`
- `corrupt_authority`
- `partial_restore`
- `foreground_under_background_io`
- `tenant_boundary_pressure`
- `key_rotation_under_backup`
- `operator_repair_after_damage`
- `release_soak_mixed`

Each profile must declare:

- data shape
- operation mix
- minimum size relative to memory budget
- enabled fault classes
- required evidence outputs
- expected pass/fail posture

## Minimum Adversarial Coverage Matrix

Roadmap 2 closeout requires minimum coverage across artifact classes,
publication seams, fault phases, resource envelopes, background subsystems,
security boundaries, and repair workflows.

This matrix is a floor, not a target. A sequence may require more.

### Artifact Class Coverage

Each admitted physical artifact class must have corruption, reopen, offline
verification, and recovery classification lanes.

Required artifact classes include:

- authority pages
- derived pages
- WAL records
- checkpoint manifests
- root manifests
- segment manifests
- free-space maps
- index pages
- blob chunks
- chunk-tree roots
- dedupe index entries
- audit-chain records
- key envelopes
- tenant metadata

At least three corruption operators must be applied across each artifact class
where structurally meaningful:

- checksum corruption
- length corruption
- generation corruption
- pointer corruption
- payload corruption
- digest-link corruption
- removal
- duplication
- stale old-version substitution

### Publication Seam Coverage

Each publication seam must be exercised in pre, during, and post failure phases.

Required publication seams include:

- WAL append
- WAL flush
- page write
- page flush
- checkpoint manifest write
- checkpoint cutover
- root manifest cutover
- compaction product publication
- reclaim publication
- blob chunk write
- chunk-tree root update
- dedupe index update
- backup snapshot
- PITR restore apply
- audit append
- key rotation
- tenant metadata update

### Authority Coverage

Each authoritative artifact family must have:

- crash lane
- corruption lane
- offline verifier lane
- recovery determinism lane
- semantic parity lane
- derived-rebuild comparison lane where applicable

### Memory And Blob Coverage

Every Roadmap 2 certification mode except Developer Smoke must include at least
one lane where:

- store size exceeds configured memory budget
- blob size exceeds configured memory budget
- forbidden whole-store materialization would fail
- forbidden whole-blob materialization would fail

### Background Interference Coverage

Each background subsystem must appear in at least one foreground-interference
lane:

- checkpoint
- compaction
- reclaim
- scrub
- blob ingest
- blob migration
- backup
- replication preparation
- offline verification
- repair

Each lane must include causal counters for queueing, yielding, backpressure,
flush delay, debt, or contention.

### Security And Tenant Coverage

Each tenant/security feature must have both success and denial lanes:

- tenant-scoped read
- tenant-scoped backup
- tenant-scoped restore
- tenant-scoped repair
- tenant-scoped verifier access
- tenant-scoped blob access
- key rotation
- stale-key rejection
- wrong-key rejection
- audit-chain tamper rejection

### Repair Workflow Coverage

Each repair workflow class must include:

- pre-repair classification
- admissible action lane
- wrong-action negative lane
- post-repair verifier lane
- runtime reopen lane
- audit evidence lane

Required repair workflow classes:

- rebuild derived artifact
- quarantine damaged derived artifact
- quarantine damaged authority
- PITR restore
- tenant-scoped repair
- key/audit-related repair or rejection

### Formal Model Coverage

Every state machine named in Roadmap 2 `S.9` must have:

- checked model artifact
- implementation state mapping
- legal transition lane
- illegal transition lane
- mutant or controlled-defect lane proving the harness catches a weakened
  transition rule

## Forbidden Test Patterns

The following patterns are explicitly disallowed for Roadmap 2 closeout:

- mutating private structs directly to simulate disk failure
- corrupting bytes without recording the expected physical boundary
- calling recovery while retaining live heap state from the crashed store
- using tiny blobs to certify blob streaming
- using stores smaller than memory to certify bounded memory
- using elapsed time without structural counters to certify performance
- accepting any error as proof of corruption handling
- using one implementation path as both producer and verifier with no
  independent observation
- skipping reopen-from-bytes lanes
- hiding broad scans behind helper APIs
- asserting counters are nonzero instead of asserting exact expected values
- manually inspecting logs to decide pass/fail
- disabling production durability or integrity checks to make tests easier
- special-casing production code solely for the test harness

## Harness Closeout Standard

The Roadmap 2 harness is sufficient only when:

- fault injection occurs at declared production-like seams
- crash tests discard all live state
- corruption tests localize the attacked physical boundary
- memory tests fail on forbidden materialization
- latency tests explain interference with counters
- blob tests exceed the configured memory envelope
- offline verifier evidence can disagree with live recovery and be reported
  independently
- evidence bundles are reproducible by seed and profile
- every Roadmap 2 sequence has CI Certification mode coverage
- release certification can run longer, larger, and more hostile campaigns
  without changing the meaning of the tests

If building this harness takes more code than the first physical backend, that
is acceptable. For Roadmap 2, the harness is part of the product architecture.
