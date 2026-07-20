# C.2: Manual Physical Reality Audit And Claim Quarantine

## Status

Planned. C.1 is the required predecessor. C.2 implements no missing physical
behavior; it establishes an honest, manually resolved starting point for C.3
through C.13.

## Roadmap Position

```text
C.1 direct test execution and iteration cleanup
  -> C.2 manual physical reality audit and claim quarantine
  -> C.3 sealed physical runtime authority and lifecycle
  -> C.4 production media boundary and stable store namespace
  -> C.5-C.13 physical reconstruction and recertification
```

## Goal

Determine what Worth Store S.1 through S.9 code actually does, which ordinary
product paths can reach it, which physical effects are real but disconnected,
and which platform-grade claims are unearned. Preserve that resolution in one
checked-in CSV and make dishonest promotion mechanically unavailable in code.

The CSV is an audit record and planning input. It is not a proof system,
readiness authority, generated source of truth, or substitute for inspecting
code and running disputed behavior.

## Boundary

C.2 may:

- search source, manifests, public exports, tests, and Cargo metadata broadly;
- manually trace callers and callees across crates;
- run narrow production-facing probes to resolve material ambiguity;
- correct or remove public claims, promotion constructors, and closeout paths
  that grant physical credit without physical effects;
- classify existing mechanisms for preservation, refactoring, later
  connection, quarantine, or deletion; and
- assign each unresolved physical obligation to C.3 through C.13.

C.2 may not:

- implement the sealed runtime, media boundary, page path, WAL join, recovery,
  integrity, isolation, layout, blob adoption, or recertification promised by
  later milestones;
- build a semantic call-graph mapper, source-hash authority, evidence receipt
  hierarchy, readiness token, or program that claims to infer repository truth;
- treat string matches, dependency edges, public exports, type names, tests, or
  counters as proof that an ordinary product path executes a physical effect;
- let the audit CSV authorize runtime behavior or platform-grade status; or
- preserve a false claim merely because later work is expected to make it true.

## Governing Summaries

- `MENTALITY.md` protects hard-problem-first foundations. Here the hard problem
  is honest human resolution of a tangled implementation, not construction of
  another layer that certifies its own interpretation.
- `arch_laws.md` protects compiler-visible authority. The CSV observes
  authority topology; production types, constructors, dependencies, and tests
  must prevent an unearned promotion.
- `composition_laws.md` protects named responsibilities. Candidate discovery,
  manual tracing, behavioral probing, and disposition are separate audit acts;
  they must not collapse into a broad truth-engine subsystem.
- `domain_structure_laws.md` protects ownership and truth-source topology.
  Every resolved claim names its claiming surface, actual effect owner,
  terminal truth source, and artifact family.
- `perf_laws.md` protects honest cost boundaries. The audit distinguishes heap
  mutation from media effects, ordinary from reconstructive work, and bounded
  operations from whole-store materialization without inferring runtime cost
  from naming.
- `physical-foundation-reconstruction-roadmap.md` protects sequencing. C.2
  inventories and quarantines; C.3 through C.12 reconstruct; C.13 alone
  recertifies the joined physical platform.

## Global Adversarial Constraint

> Begin with physical-looking names, public claims, real file writers, tests,
> and dependency edges that disagree. Broad searches must expose candidates,
> but a reviewer must trace each claimed facade to its terminal effect and each
> real writer back to its actual caller class. Heap mutation, replay-supplied
> state, certification-only execution, and disconnected file I/O must remain
> visibly distinct. A narrow probe may settle disputed behavior, but no
> generated semantic map or audit artifact may promote its own conclusion.

## Product Decision Lock

1. The primary deliverable is
   `_docs/worth-store/physical-reality-audit.csv`.
2. One CSV row represents one claimed operation or one independently meaningful
   mechanism-to-effect boundary. It must not combine unrelated claims merely
   because they share a crate.
3. Candidate discovery uses ordinary repository tools: `rg`, Cargo metadata,
   manifest inspection, public-export inspection, and test-name searches.
4. Search results are candidates only. A resolved row requires manual source
   tracing to a terminal effect, an explicit stopping boundary, or a recorded
   narrow probe.
5. Tracing runs in both directions: public physical claims downward and actual
   file writers upward. Neither direction substitutes for the other.
6. “Executable” evidence means an exact command another engineer can rerun for
   a materially disputed row. It does not mean a program generated the row or
   inferred its classification.
7. The CSV may be checked with standard CSV tooling for parseability. C.2 does
   not create a bespoke validator, ledger runtime, readiness object, source
   fingerprint, or semantic discovery script.
8. During work, a row may be `pending_manual_resolution`. C.2 cannot close with
   that value, a blank terminal effect, or a blank disposition.
9. Platform-grade status is denied by production code and compiler/test
   boundaries. Editing the CSV cannot grant capability.
10. A real mechanism that is unreachable from the ordinary facade is valuable
    substrate, not production behavior.
11. A test or certification path proves only the path it actually invokes. It
    does not make a disconnected mechanism ordinary.
12. C.2 favors deletion or plain quarantine over compatibility wrappers around
    false authority.

## Primary Audit CSV

The CSV uses this fixed column order:

```text
claim_id,roadmap_scope,claiming_surface,entry_source,owner_crate,claimed_effect,actual_terminal_effect,artifact_family,physical_writer,reader,durability_boundary,reopen_source,independent_verifier,classification,path_kind,candidate_search,trace_locations,probe_command,observed_result,disposition,target_milestone,review_notes
```

Column law:

- `claim_id` is a stable, responsibility-shaped identifier, not a row number or
  milestone-phase name.
- `claiming_surface` names the facade, method, type, witness, receipt, test, or
  document-facing capability under review.
- `entry_source` is the exact source location where an ordinary or claimed path
  begins.
- `actual_terminal_effect` states the last effect the traced code truly
  performs. “Persists” and “durable” are invalid without naming the media call
  and artifact.
- artifact columns use `none` when the traced behavior has no physical artifact;
  blanks are not optimistic unknowns.
- `classification` is exactly one final classification from the vocabulary
  below.
- `path_kind` is `ordinary`, `reconstructive`, `certification`, `test-only`, or
  `disconnected`.
- `candidate_search` records the reproducible search or metadata command that
  found the candidate. It is provenance, not reachability proof.
- `trace_locations` records the inspected call chain as repository-relative
  file and symbol locations. It must include both ends of the conclusion.
- `probe_command` is required when source tracing alone leaves a material
  runtime claim disputed; otherwise it is `not_required`.
- `observed_result` reports concrete files, bytes, process behavior, typed
  denial, or absence. “Passed” is insufficient.
- `disposition` and `target_milestone` make the roadmap handoff explicit.
- `review_notes` records assumptions, uncertainty that does not affect the
  final class, and why a tempting neighboring mechanism does not change the
  conclusion.

Final classifications:

- `production_physical_effect`
- `production_in_memory_behavior`
- `isolated_real_mechanism`
- `certification_only_mechanism`
- `test_only_mechanism`
- `vocabulary_without_execution`
- `duplicate_or_conflicting_authority`
- `explicitly_unavailable`
- `quarantined_false_claim`

Final dispositions:

- `preserve`
- `refactor_before_connection`
- `connect_in_target_milestone`
- `quarantine_outside_production`
- `delete`

## Non-Fake Acceptance Setup

- **Production subject:** every public Worth Store physical facade and every
  method, witness, receipt, readiness surface, or certification promotion that
  claims open, append, locate, read, flush, checkpoint, recover, compact,
  verify, reopen, durability, or platform-grade status.
- **Initial world:** use a uniquely identified absent or empty temporary store
  root. Invoke the current ordinary facade exactly as product code can. Do not
  supply pages, manifests, WAL frames, persisted layouts, replay artifacts, or
  writer-owned decoded state.
- **Execution topology:** perform candidate searches, manually trace both
  directions, then attempt an append, its declared durability boundary,
  process termination, and fresh-process reopen using only store root and
  ordinary configuration. The probe is diagnostic and earns no implementation
  credit for a behavior that does not exist.
- **Independent observation:** a fresh observer uses OS filesystem APIs to
  report actual paths, lengths, and bytes. Cargo metadata and callsite searches
  nominate edges for inspection but never decide semantic reachability.
- **Assertions:** every claim resolves to a final classification and terminal
  effect; every actual writer resolves to an ordinary, reconstructive,
  certification, test-only, or disconnected caller class; supplied replay,
  heap-only mutation, and disconnected I/O receive no physical-platform credit.
- **Mutation sensitivity:** expose a heap-only append as platform-grade and
  make a disconnected file writer appear production-reachable through naming.
  Manual tracing must classify both correctly, while code-level promotion
  tests independently deny the heap-only platform claim.
- **Mechanical anti-substitution gates:** ordinary product code cannot obtain
  platform-grade or readiness authority from `PersistedPhysicalLayout`,
  `PlatformPhysicalReplayArtifact`, counters, supplied pages, or certification
  verdicts. The CSV has no production consumer.
- **Evidence and rerun:** each disputed row contains the exact search, source
  chain, command, initial-root posture, and observed artifacts needed for a
  second reviewer to repeat the conclusion.

## Phase Plan

## Phase 1: Freeze Audit Semantics And The CSV Contract

### Objective

Create the empty primary CSV with the fixed header, final vocabularies, and
review rules before inspecting individual claims so classifications cannot be
invented to excuse discoveries later.

### Relevant Subsystems And APIs

- `_docs/worth-store/physical-reality-audit.csv`
- the C.2 classification, path-kind, and disposition vocabularies
- S.1 through S.9 claim families named by the physical reconstruction roadmap

### Requirements

- Create exactly one primary CSV, with no parallel Markdown ledger, generated
  inventory, or readiness artifact.
- Define row granularity as one claimed operation or one independently
  meaningful mechanism-to-effect boundary.
- Use `pending_manual_resolution` only as temporary working state and remove it
  before closeout.
- Record repository-relative paths and exact symbols rather than prose-only
  crate summaries.
- Keep multi-value cells ordered and semicolon-delimited; commas belong only to
  CSV field separation or quoted field content.

### Warnings

- A beautifully complete schema can become the same recursive proof machinery
  C.1 deleted. Do not add columns for hashes, seals, receipts, reviewer tokens,
  source authority, or generated confidence scores.
- Do not make absence of a row mean absence of a claim; coverage is established
  by the independent searches in Phase 2.

### Test Requirements

- Add a hostile example row whose method says `persist` but whose terminal
  effect is a `Vec` mutation. A reviewer must be able to classify it without a
  new category or optimistic physical wording.
- Add then remove a hostile example row for a real `File::write` called only by
  certification. The schema must represent real I/O and non-production
  reachability simultaneously.
- Open the CSV with standard CSV parsing and confirm the header and quoted
  source/probe fields parse without creating a repository-specific validator.

### Engineering Decisions

- The CSV is historical audit evidence, so storing it under `_docs/worth-store`
  is intentional; production code must not depend on it.
- Stable claim ids survive row reordering and later disposition updates.

### Open Questions

- None. Any additional column requires a demonstrated unresolved decision that
  cannot be represented in `review_notes`.

## Phase 2: Discover Candidates Without Pretending To Resolve Them

### Objective

Build complete candidate coverage from independent search directions before
manual resolution begins.

### Relevant Subsystems And APIs

- the `worth-store` facade and its public re-exports
- every `worth-store-*` Cargo manifest and workspace dependency edge
- physical-effect calls such as file open/create/write/append/sync/rename,
  directory publication, mmap, and backend port invocation
- claim vocabulary such as persist, durable, physical, flush, checkpoint,
  recover, reopen, compact, verify, readiness, platform-grade, replay, layout,
  and artifact

### Requirements

- Run at least four independent sweeps: public claims, physical-effect calls,
  promotion/readiness surfaces, and Cargo dependency/export topology.
- Add candidates before deciding whether they are real, duplicate, irrelevant,
  or disconnected.
- Search tests and certification separately so their callers cannot be confused
  with ordinary product reachability.
- Record exact discovery commands in `candidate_search`.
- Record apparent duplicates separately until manual tracing establishes they
  share one owner and effect.

### Warnings

- A dependency edge proves only that one crate may reference another.
- A public re-export proves availability, not orchestration.
- A filesystem symbol may be an offline tool, fixture, certification path, or
  dead island rather than the production database.

### Test Requirements

- Seed the review set with a physical-looking type that performs no file I/O
  and a plainly named file writer with no “physical” vocabulary. Independent
  sweeps must find both.
- Search for `PersistedPhysicalLayout` and
  `PlatformPhysicalReplayArtifact` from constructors through promotion callers;
  missing either candidate family blocks the phase.
- Compare the facade-down and writer-up candidate sets. At least one candidate
  unique to each direction must remain visible rather than being deduplicated
  by name.

### Engineering Decisions

- Ordinary `rg` output and Cargo metadata are sufficient discovery inputs.
  Their raw output is disposable; resolved rows are the durable result.
- No call-graph generator or semantic index is introduced.

### Open Questions

- Candidate searches may expand during later tracing. New searches are appended
  to affected rows rather than freezing a generated inventory.

## Phase 3: Trace The Ordinary Facade And Composition Roots Downward

### Objective

Determine what an ordinary product caller can construct and what terminal
effects each claimed facade operation actually reaches.

### Relevant Subsystems And APIs

- `worth-store` public exports and ordinary constructors
- `PhysicalStoreRuntime` or any competing physical composition authority
- open, append, locate, read, flush, checkpoint, recover, compact, verify, and
  reopen surfaces
- subsystem handles exposed by the facade

### Requirements

- Begin at each ordinary public constructor and method, not at a subsystem test
  or convenient internal type.
- Follow every delegated call until reaching OS/media I/O, heap-only state,
  caller-supplied recovered representation, typed unavailable, or a disconnected
  boundary.
- Record branches that change the effect class; do not summarize a mixed path
  as physical because one branch writes a file.
- Identify every candidate for the sole physical composition root and whether
  product code can actually construct it.
- Record dependency seams that later milestones must connect, but do not add
  adapter code in C.2.

### Warnings

- Incremented flush or rename counters do not prove the corresponding effect.
- A method named `reopen` that consumes caller-supplied layout or replay state
  is reconstruction from supplied heap truth, not physical reopen.
- A facade depending on many crates does not prove it composes them.

### Test Requirements

- Trace an append followed by its strongest declared publication or durability
  call and identify the exact final mutation or media operation. A counter-only
  or collection-only terminus must classify as in-memory.
- Trace reopen from the ordinary facade and prove whether a fresh caller needs
  only root plus configuration or must supply recovered state. The latter must
  receive no physical-reopen credit.
- Attempt to locate an ordinary path from the facade into one known real writer.
  A missing path must remain visible even if certification calls that writer.

### Engineering Decisions

- Source tracing is primary. A narrow runtime probe is required only where
  dynamic dispatch, feature selection, or platform branching leaves the
  terminal effect materially ambiguous.
- The audit records actual composition, not the architecture intended for C.3.

### Open Questions

- Competing public composition roots remain separate rows until Phase 11 makes
  a preservation or deletion decision.

## Phase 4: Reverse-Trace Real Writers To Their Actual Callers

### Objective

Find every mechanism that really touches media and determine whether it is
ordinary product behavior, reconstructive tooling, certification, test-only,
or an isolated island.

### Relevant Subsystems And APIs

- filesystem and backend open/create/write/append/sync/rename/delete operations
- WAL artifact stores and durability executors
- operational control stores, import/export paths, and offline tools
- test filesystem helpers and certification storage adapters

### Requirements

- Start at concrete media effects and trace callers upward until reaching a
  public product facade, executable boundary, certification/test entry, or an
  explicit dead/disconnected end.
- Distinguish a production implementation of a port from actual construction
  and invocation of that implementation by the ordinary composition root.
- Record which artifacts the writer creates, who reads them, and whether any
  runtime reopen path discovers them.
- Separate ordinary and reconstructive writers even when they use the same
  format or backend primitive.
- Record feature gates and platform branches that can remove a writer from the
  ordinary build.

### Warnings

- “Real file write” and “real database path” are different claims.
- Reverse tracing may stop at a generic trait. Inspect the concrete constructor
  and caller before assigning reachability.
- Tests that directly instantiate a writer do not prove the facade owns it.

### Test Requirements

- Select one concrete real writer and trace it upward. If its first non-owner
  caller is certification or a test, classify it accordingly even when its
  bytes are genuine.
- Select one backend trait implementation and verify whether the ordinary
  facade constructs that concrete implementation under its default feature
  lane. Trait conformance alone must not pass.
- Search for direct `std::fs` or equivalent calls in tests and support crates;
  expected-artifact writes by the test must not be attributed to production.

### Engineering Decisions

- The two-direction audit is intentionally redundant: disagreement between
  facade-down and writer-up traces is a finding, not duplicated paperwork.
- A writer can be marked `preserve` while its capability remains unearned.

### Open Questions

- None. Dynamic plugin-style construction, if present, requires a narrow probe
  and explicit feature/configuration identity.

## Phase 5: Resolve Physical Format And Media Ownership

### Objective

Classify the current page, segment, extent, manifest, reference, allocation,
and backend mechanisms without confusing byte grammar with media effects.

### Relevant Subsystems And APIs

- `worth-store-physical-format`
- `PhysicalStoreRuntime`
- `PersistedPhysicalLayout`
- `PlatformPhysicalReplayArtifact`
- page, segment, extent, manifest, allocation, and physical-reference types
- concrete backend and filesystem implementations

### Requirements

- Trace construction, append, locate, scan, publication, snapshot/layout, and
  reopen behavior in the physical-format surface.
- Identify precisely which state lives in vectors, maps, counters, supplied
  representations, and real files.
- Record whether format encoders/decoders are called on the ordinary path or
  only by tests, certification, or offline support.
- Record every backend capable of physical effects and whether format/runtime
  code reaches it.
- Assign page, segment, extent, and manifest artifact rows even when the current
  physical representation is `none`.

### Warnings

- Correct framing and checksum code can coexist with a heap-only database.
- A persisted-layout value is a representation of state, not proof that media
  persisted or rediscovered it.
- Publication counters named after flush or rename are observations of intent
  unless the traced path invokes those operations.

### Test Requirements

- Run the current strongest ordinary append/publication specimen against an
  empty unique root and inspect the filesystem independently. Any absent page,
  manifest, segment, or extent artifacts must be recorded exactly.
- Start a genuinely fresh process with root plus ordinary configuration only.
  If reopen requires `PersistedPhysicalLayout` or replay data, classify the
  current reopen claim as supplied-state reconstruction.
- Compare a format encoder path with the runtime append path. If the encoder is
  never invoked by ordinary append, keep “format exists” and “format used” as
  separate rows.

### Engineering Decisions

- Physical format owns byte meaning; media effects remain a separate owner
  even if current code collapses them conceptually.
- C.5 owns the eventual durable record path. C.2 records the exact seams and
  false assumptions C.5 must replace.

### Open Questions

- Any currently public whole-layout representation requires an explicit
  offline/test use decision in Phase 11; public existence alone does not force
  preservation.

## Phase 6: Resolve Buffer, WAL, Recovery, And Integrity Mechanisms

### Objective

Determine whether residency, logging, checkpoint, recovery, and verification
mechanisms operate on the same ordinary physical state or on parallel models.

### Relevant Subsystems And APIs

- buffer-pool construction, pin/lease, dirty, eviction, and writeback surfaces
- `StoreDurabilityRuntime`, WAL artifact storage, barriers, checkpoints, and
  root publication
- recovery source selection, redo, replay, and reopen paths
- checksums, framing, quarantine, scrub, and offline verification

### Requirements

- Trace the object identities and artifact references crossing each subsystem
  boundary; matching vocabulary without a shared owner is not integration.
- Record whether dirty pages write through the actual backend or only change
  model state and counters.
- Record the WAL writer, durability boundary, consumer, recovery source, and
  whether ordinary append participates in its ordering.
- Distinguish physical recovery from replay of supplied logical or layout
  representations.
- Determine whether integrity checks guard ordinary decode/recovery or run only
  in certification and offline paths.

### Warnings

- A correct WAL subsystem does not establish WAL-before-data if ordinary data
  writes bypass it or do not exist.
- Buffer-pool tests over synthetic pages do not prove the product runtime is
  memory-bounded.
- Offline verification can be real and still disconnected from ordinary
  recovery and claim promotion.

### Test Requirements

- Trace one ordinary mutation identity across buffer dirtying, WAL append,
  barrier, page publication, and acknowledgment. Every missing transition must
  be a separate recorded gap rather than inferred from neighboring tests.
- Kill the writer process at the strongest available durability seam and launch
  a fresh recovery process with root plus configuration only. Supplied replay
  or surviving heap state invalidates physical-recovery credit.
- Corrupt a predeclared physical field only if the ordinary path actually
  produced its artifact; otherwise record that the integrity scenario proves
  only its isolated fixture path.

### Engineering Decisions

- C.6, C.7, C.8, and C.9 receive separate blocker assignments because
  residency, durability, recovery, and integrity have different truth owners.
- Existing strong local mechanisms may be preserved without granting joined
  production status.

### Open Questions

- If a mechanism spans several later milestones, select the first milestone
  that must change it and name later dependencies in `review_notes`.

## Phase 7: Resolve Isolation, Scheduling, And Maintenance Mechanisms

### Objective

Classify physical stable-read, concurrency, scheduling, compaction, rewrite,
reclaim, scrub, and maintenance behavior against actual persisted artifacts.

### Relevant Subsystems And APIs

- physical read plans, leases, generations, epochs, latches, and reclaim guards
- foreground/background I/O scheduling, admission, pacing, and cancellation
- checkpoint, scrub, compaction, rewrite, cleanup, and reclaim operations
- maintenance receipts, counters, and certification schedules

### Requirements

- Trace what concrete bytes or heap objects a read plan protects and who can
  publish or reclaim them.
- Determine whether scheduler work corresponds to backend operations or only
  simulated tasks/counters.
- Record whether maintenance uses the ordinary page/root path or independent
  fixtures and model stores.
- Distinguish physical visibility from semantic MVCC, branches, and Query
  visibility; C.2 must not pull Part II authority downward.
- Record every cleanup or reclaim policy that can delete physical artifacts,
  including whether the current artifacts are real.

### Warnings

- Branch labels or version numbers do not prove stable physical bytes.
- Deterministic simulated schedules can prove algorithms without proving the
  production I/O seam.
- A reclaim receipt without an actual artifact deletion is not physical
  reclaim.

### Test Requirements

- Hold the strongest available read lease while invoking the corresponding
  rewrite/reclaim path. Trace whether both operate on the same root and artifact
  identity; parallel model identities must be classified as disconnected.
- Inject observable backend latency through the actual production boundary, if
  one exists, and verify whether foreground/background scheduler counters align
  with real operations. If no such seam exists, record the claim as unearned.
- Trace a maintenance call exposed by the facade to its terminal effect and
  separately reverse-trace one maintenance writer/deleter to its actual caller.

### Engineering Decisions

- C.10 owns the eventual join of physical isolation and scheduled I/O.
- Existing algorithmic models may remain valuable certification substrate but
  cannot receive physical-interference credit in C.2.

### Open Questions

- None. Semantic branch isolation belongs to the later runtime-integration and
  merging roadmaps, not this audit.

## Phase 8: Resolve Layout, Index, Blob, Formal, Operations, And Certification

### Objective

Audit the remaining S.1 through S.9 surface, especially mechanisms whose strong
local vocabulary can hide disconnection from the physical runtime.

### Relevant Subsystems And APIs

- B-tree, LSM, index, scan, amplification, and rebuild surfaces
- blob/chunk ingest, stream, dedupe, reachability, and reclaim surfaces
- formal models, conformance mappings, and counterexample lowering
- operations, import/export, backup, control stores, readiness, claim promotion,
  and certification
- `PlatformGradeClaimWitness` and Worth Store readiness surfaces

### Requirements

- Record the source authority and rebuild basis for every index and derived
  artifact family.
- Determine whether blob operations stream through the ordinary backend or use
  in-memory/sidecar representations.
- Trace formal model actions to actual owner transitions without treating model
  verdicts as runtime authority.
- Reverse-trace operations/control file writers and determine whether they are
  part of the database, offline tooling, or disconnected operational state.
- Trace every platform-grade, readiness, closeout, and certification promotion
  to the concrete facts it demands.

### Warnings

- Real operational files are not automatically database pages, WAL, manifests,
  or runtime composition.
- An index tested against generated fixture pages is not an adopted physical
  access path.
- A witness that checks counters and references can still promote a completely
  heap-only execution.

### Test Requirements

- Trace one index point/range operation and one blob ingest/read operation to
  actual page/chunk I/O. If the artifact is supplied or fixture-owned, record
  the precise non-production path.
- Construct the strongest currently public platform/readiness claim using only
  heap layout, replay state, or counters. If it succeeds, Phase 10 must remove
  or quarantine that promotion surface.
- Compare at least one formal-model transition with the production owner method
  it claims to represent; an absent owner call or model-only identity must be a
  visible C.12 blocker.

### Engineering Decisions

- C.11 owns layout/index/blob adoption; C.12 owns formal rebinding; C.13 owns
  final readiness. C.2 grants none of those closeouts.
- Operations mechanisms are classified by their actual responsibility, not
  forced into the database artifact family because they write files.

### Open Questions

- Any readiness API needed by S.10 is redesigned only in C.13 after the joined
  system can support it honestly.

## Phase 9: Run The Empty-Root Physical Reality Probe

### Objective

Settle the milestone's central behavioral question with one small, reproducible
production-facing experiment: what survives when the ordinary runtime is given
an empty root, asked to write durably, terminated, and reopened fresh?

### Relevant Subsystems And APIs

- the ordinary public Store facade and its default production feature lane
- the strongest currently claimed append/publication/durability operation
- process launch and termination boundaries
- OS filesystem inspection and the current fresh-process reopen surface

### Requirements

- Use a unique absent or empty root and record its absolute probe path outside
  the checked-in repository.
- The writer receives only admitted production configuration and input records.
- The writer process terminates before observation/reopen; no runtime, page,
  decoded value, registry, layout, or replay object crosses the boundary.
- Record exact files, directories, lengths, and relevant byte prefixes through
  ordinary OS APIs before launching reopen.
- The reopen process receives only root and ordinary configuration.
- Record the exact command, feature lane, initial state, process topology, and
  observed result in each disputed claim row affected by the probe.

### Warnings

- This is an audit probe, not a C.5 or C.8 implementation. An honest failure is
  the expected evidence when the physical path does not exist.
- Graceful in-process reconstruction or a copied persisted layout invalidates
  the fresh-process claim.
- The test harness must not create expected page, WAL, manifest, or checkpoint
  files on the runtime's behalf.

### Test Requirements

- Run the control probe without supplied state and assert the exact artifact set
  rather than checking only success. If the set is empty or operational-only,
  record that outcome without reinterpretation.
- Run a hostile variant that attempts reopen with root plus configuration after
  the writer is dead. A requirement for layout/replay input must be reported as
  typed unavailability or failed physical reopen, never silently supplied.
- Run a residue check proving no test/support layer wrote inside the root except
  through the named production subject.

### Engineering Decisions

- One minimal vertical probe is sufficient for C.2 because later milestones own
  comprehensive crash and durability certification.
- Exact artifact observation is more valuable here than long stress duration.

### Open Questions

- If no ordinary facade can accept a root, record that construction failure as
  the first terminal boundary and do not invent a temporary facade.

## Phase 10: Remove Unearned Claim Promotion

### Objective

Make the code as honest as the audit by denying physical-platform, readiness,
or closeout promotion that can be satisfied by heap state, supplied replay,
counters, references, model verdicts, or certification-owned facts.

### Relevant Subsystems And APIs

- `PlatformGradeClaimWitness`
- `worth-store-readiness`
- claim-promotion and closeout receipts
- constructors accepting `PersistedPhysicalLayout`,
  `PlatformPhysicalReplayArtifact`, supplied pages, or equivalent representations
- facade re-exports that expose false production authority

### Requirements

- Trace and list every public and crate-visible route that constructs or
  promotes physical-platform/readiness authority.
- Remove false constructors when safe; otherwise quarantine them behind an
  explicitly non-production certification/test boundary with honest names.
- Where later code must compile before reconstruction, return a typed
  unavailable/unearned result rather than minting a weaker witness.
- Ensure certification can observe failure but cannot grant the production
  authority it is meant to judge.
- Add the smallest compile-time or behavioral denial at the actual promotion
  boundary. Do not create an audit framework to test the audit.
- Update CSV rows with the resulting `quarantined_false_claim`,
  `explicitly_unavailable`, or narrower honest classification.

### Warnings

- Renaming a false witness without changing what constructs it is cosmetic.
- A private constructor reached through a public wrapper remains public
  authority in practice.
- Do not manufacture a C.2 readiness token. C.3 starts from typed unavailable
  physical operations and C.13 later owns genuine readiness.

### Test Requirements

- Attempt the previous strongest platform/readiness promotion using a heap-only
  runtime, nonzero counters, and supplied replay/layout state. It must fail at
  the production promotion boundary for the specific missing physical basis.
- Attempt the same promotion through certification or a public re-export. No
  alternate constructor or wrapper may reopen the authority path.
- Confirm that an ordinary caller receives typed unavailable/unearned behavior
  where a later milestone has not implemented the physical effect, and that no
  heap mutation occurs after denial.

### Engineering Decisions

- Code is the enforcement source; the CSV merely records what was changed and
  why.
- Honest temporary unavailability is preferable to a compatibility layer that
  preserves a false platform contract.

### Open Questions

- If removing a false type would cause a large unrelated rewrite, quarantine it
  narrowly and assign deletion to the first milestone that replaces its caller.
  The audit must state that residual debt explicitly.

## Phase 11: Resolve Artifact Families And Reconstruction Dispositions

### Objective

Turn traced facts into one explicit decision for every duplicate runtime,
mechanism island, fake backend, test oracle, artifact family, and physical
obligation.

### Relevant Subsystems And APIs

- pages, segments, extents, manifests, free space, WAL, checkpoints, roots,
  indexes, chunks/blobs, control files, imports/exports, and verifier artifacts
- current runtime, backend, fixture, model, readiness, and certification owners
- C.3 through C.13 roadmap boundaries

### Requirements

- For every artifact family, name source truth, physical representation, writer,
  reader, durability boundary, reopen source, independent verifier, and rebuild
  basis, using `none` where current implementation lacks one.
- Give every audited mechanism exactly one final disposition: preserve,
  refactor before connection, connect in target milestone, quarantine outside
  production, or delete.
- Give every unearned obligation one first blocking target milestone from C.3
  through C.13.
- Preserve algorithmic value separately from production status; a strong local
  subsystem may still require structural refactoring before connection.
- Identify duplicate/conflicting authority and state which owner survives.
- Do not create a separate blocker graph; `target_milestone` and `review_notes`
  in the CSV are the ordered handoff.

### Warnings

- “Preserve” is not “already production-ready.”
- “Connect later” without a first responsible milestone is deferred ambiguity.
- Do not force operational control files or certification evidence into the
  page/WAL truth hierarchy when they serve a different responsibility.

### Test Requirements

- Select one strong isolated mechanism and demonstrate that its disposition can
  preserve code while its classification remains non-production and its target
  milestone owns connection.
- Select one duplicate authority and verify that exactly one owner is chosen;
  wrappers around both implementations do not count as resolution.
- Sort/filter the CSV by `target_milestone` and confirm C.3 through C.13 specs can
  identify their inputs without consulting a second ledger or generated map.

### Engineering Decisions

- The first milestone that must change a seam owns the blocker; downstream
  consequences remain notes, not duplicate blocker rows.
- Deletion decisions are explicit even if physical deletion is scheduled in the
  milestone that replaces the path.

### Open Questions

- None may remain at closeout. Material uncertainty becomes an additional trace
  or Phase 9-style narrow probe, not an “unknown” final classification.

## Phase 12: Hostile Reconciliation And Closeout

### Objective

Challenge the completed audit from independent directions, correct omissions
and optimistic classifications, and hand C.3 through C.13 a trustworthy manual
map without pretending it is permanent runtime truth.

### Relevant Subsystems And APIs

- the complete physical-reality audit CSV
- the `worth-store` facade and all `worth-store-*` manifests
- all discovered physical-effect sites and promotion surfaces
- code-level denial tests changed in Phase 10
- C.3 through C.13 target milestone assignments

### Requirements

- Re-run the public-claim, writer, promotion, dependency, test, and certification
  searches from Phase 2 after code quarantine changes.
- Have the hostile pass begin from raw search results, not from the completed
  CSV ordering, and reconcile every candidate to an existing or new row.
- Re-trace a representative row from each final classification and every row
  that claims `production_physical_effect`.
- Re-run every material `probe_command` whose code path changed during C.2.
- Remove all `pending_manual_resolution` values, blank terminal effects, blank
  dispositions, and unassigned blockers.
- Verify that the CSV is not read by production crates, build scripts, boundary
  tools, or claim-promotion code.
- Record later code changes normally in their owning milestone; the C.2 CSV is
  a reconstruction baseline, not a perpetually self-updating truth service.

### Warnings

- Agreement between two rows derived from the same mistaken trace is not
  independent review.
- A green denial test does not prove the candidate inventory is complete.
- Do not postpone an obvious false public claim merely to keep C.2 documentation
  only; C.2 explicitly owns claim quarantine.

### Test Requirements

- Use the existing heap-backed `PhysicalStoreRuntime` and the real writers in
  `worth-store-physical-certification` as controlled review seeds. Starting
  from their physical-looking names alone, the hostile process must classify
  the former as in-memory and the latter as certification-only by tracing in
  opposite directions.
- Re-run the empty-root probe after claim quarantine and confirm no remaining
  platform/readiness path reports success for the observed heap-only or absent
  artifact result.
- Search the repository for the CSV filename and for its claim ids. Production
  dependency or code consumption fails closeout.
- Run the relevant owner tests for every code-level promotion change plus the
  constitutional boundary checks required by the workspace.

### Engineering Decisions

- Closeout is a human-reviewed resolution with reproducible commands, not a
  generated completeness certificate.
- Later discoveries may correct the CSV through ordinary reviewed edits; they
  do not retroactively turn C.2 into a runtime authority.

### Open Questions

- None. Unresolved material behavior blocks C.2 rather than being translated
  into a confidence score.

## Logical Implementation Slices

The phases remain linear, but work may be assigned in these coherent batches:

1. **Audit contract:** Phases 1-2.
2. **Core physical truth:** Phases 3-6.
3. **Extended subsystem truth:** Phases 7-8.
4. **Behavioral settlement and honesty correction:** Phases 9-10.
5. **Roadmap handoff and hostile closeout:** Phases 11-12.

Do not begin Phase 10 from a partial trace set. Claim quarantine must be based
on the joined facade-down, writer-up, subsystem, and behavioral evidence.

## Strongly Opinionated Directory Target

```text
_docs/worth-store/
  physical-foundation-reconstruction-roadmap.md
  physical-reconstruction-c2-manual-reality-audit.md
  physical-reality-audit.csv

workspaces/worth-store/crates/
  <existing production owner>/
    src/<responsibility-named claim or facade files>
    tests/<narrow authority-denial tests where externally observable>
  <existing certification owner>/
    tests/<certification-only denial or observation tests>
```

C.2 creates no `audit`, `ledger`, `evidence`, `proof`, `topology`, or
`readiness` crate; no generated-source directory; no checked-in search dumps;
and no generic helper bag. Code changes live in the owner of the dishonest
surface they correct.

## DX Target

The critical reviewer workflow must remain boring and inspectable:

```text
1. Search broadly with rg and Cargo metadata.
2. Add or locate the candidate row.
3. Open the exact claiming source and trace to the terminal effect.
4. Reverse-trace the corresponding writer when one exists.
5. Run one copied probe command only when source leaves material ambiguity.
6. Resolve classification, disposition, and target milestone in the CSV.
7. If the public claim is false, correct its owning code and run its narrow tests.
```

An engineer must not need to regenerate an inventory, refresh hashes, mint a
receipt, understand an audit DSL, or trust a tool's semantic inference before
editing or reviewing one row.

## Closeout Gate

C.2 closes only when:

- every reopened S.1 through S.9 physical claim and promotion surface has a
  manually resolved CSV row;
- every discovered real writer has been reverse-traced to its actual caller
  class;
- every `production_physical_effect` row names the exact media operation and
  artifact and survives hostile retracing;
- every heap-only, supplied-state, certification-only, test-only, vocabulary-
  only, duplicate, or disconnected path is classified without platform credit;
- every false platform/readiness promotion is deleted, honestly unavailable,
  or mechanically quarantined outside production;
- every mechanism has one disposition and every unearned obligation has one
  first target milestone from C.3 through C.13;
- the empty-root/fresh-process probe is reproducible and its actual outcome is
  recorded without test-created artifacts or live-state transfer;
- no unresolved rows, blank terminal effects, confidence scores, generated
  semantic maps, audit authorities, or CSV consumers remain; and
- the relevant owner tests, boundary check, and agent-context check are green
  for code-level quarantine changes.

The closeout result is an honest reconstruction baseline. It grants no restored
S.1 through S.9 capability. Only later milestones can implement those effects,
and only C.13 can recertify the joined physical platform for S.10 re-entry.
