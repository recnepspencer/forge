# C.5.1 Closure Ledger

This ledger is the authority for the post-implementation QA of C.5.1 Phases
13-16. It records logical guarantees rather than test cases. A green command or
previously published courtroom report is candidate evidence until its source
binding, world, boundary, oracle, and fault sensitivity have been inspected.

## Audit Source

- branch: `forge-store`
- frozen starting source: `a79fcbe8d073bb5d5fc231350676f9fdaf92cb73`
- C.5.1 checkpoint: `382b540bb8b37bd3259930401c3a2c0eab08b4aa`
- merged master: `ba3cbeafab398ade102fdcab78c342f976d3a58c`
- governing specification:
  `physical-reconstruction-c5-1-physical-store-work-runtime.md`

`OPEN` means evidence has not yet earned closure. `DEFECT` means a concrete
source or evidence flaw invalidates the claim. `PROVED` is reserved for evidence
bound to the final source. `BLOCKED` and `N/A` require an explicit reason.

## Guarantee Ledger

| ID | Closure claim | Required evidence | Current result |
|---|---|---|---|
| C51-13-01 | Locate, external-locator readmission, bounded point read, scan, and scan continuation enter the canonical physical read admission, Signal, scheduler, executor, settlement, and observation topology whenever media-capable work is required. | Source trace for every public read route, causal work records, exact media/scheduler counters, and absence of a direct media fallback. | `PROVED` — final-source trace joins `ServingPhysicalRuntime::records` through `CanonicalRecordReadPort` and `CanonicalFrameReadSource`; serving locate, readmission, point, scan, and continuation tests expose canonical causal records and exact effects, while direct frame sources remain confined to admission/bootstrap construction. |
| C51-13-02 | Hot-resident and cold-file reads preserve identical C.5 bytes and semantic observations while differing only in the declared residency and media effects. | One real Store cold/hot metamorphic journey with independent payload oracle and exact residency, media, work, and Signal cleanup counters. | `PROVED` — `cold_and_hot_reads_share_canonical_work_but_only_cold_work_reads_frame_bytes` passes with equal independent payload and semantic observations, distinct work identities, exact cold/hot media and residency deltas, committed Signal completion, and zero residual locality. |
| C51-13-03 | Read and scan capabilities are independently borrowable from mutation authority, retain generation/health/allocation leases for their operation lifetime, and require no whole-runtime mutable borrow or global lock. | Positive compile proof, concurrent read/scan/append journey, lifecycle counter inspection, and close residue check. | `PROVED` — the authority UI accepts independent scan and mutation capabilities; `disjoint_payload_writes_overlap_while_root_cutover_orders_both_batches` keeps a scan and external readmission live during two paused appends; `record_owner_propagates_through_every_lifecycle_boundary` proves exact read/scan lease release and clean close. |
| C51-13-04 | Wrong Store, stale locator/cursor, damaged manifest or continuation, cancellation, and partial backend reads deny at the exact boundary; physical damage revokes the shared health that fences later read and mutation work. | Hostile locator/cursor cases, injected partial reads, continuation-damage journey, cancellation cleanup, exact health and effect-fate evidence. | `PROVED` — final-source hostile locator/cursor, truncation, partial-read, continuation-damage, cancellation, transient-retry, projection-failure, and capacity-cleanup journeys pass with exact effect identity, bytes, Signal invalidation, health fencing, and zero residual locality (`C51-F016`, `C51-F019`, `C51-F020`, `C51-F023`–`C51-F028`). The complete structured integration suite passes 214/214. |
| C51-13-05 | Only admitted Store-native read facts select Signal aspects and partitions; raw Signal values, untyped/JSON-shaped semantic basis, and caller labels cannot admit or redirect reads. | Exact complete read-binding and causal-partition journey, typed-basis positive compile proof, and independent negative compile boundaries. Phase 15 sealing changes must reopen this row for functional deletion-mutant proof. | `PROVED` — the runtime reports exactly four ReadFault bindings and exact ordinary/scan causal partitions; the clean authority UI independently rejects foundational, raw Signal, and JSON values while accepting supported typed authority. |
| C51-13-06 | Point and scan work remain bounded by declared access breadth rather than Store, manifest, or Signal graph size. | Scale worlds with exact manifest comparisons/bytes, allocations, frame faults, work lookups, and invalidation counters; warm cost measurement. | `PROVED` — 1/9/65-record worlds bracket actual point and scan operations, consume and independently verify point payload bytes, and assert exact media reads/bytes, manifest blocks/bytes, comparisons, work, faults, allocations, invalidations, and cleanup. Final warm execution was 7.43 seconds. |
| C51-14-01 | Independently borrowable physical mutation capabilities can prepare disjoint appends concurrently while only the real root-publication authority serializes publication. | Public-surface inspection, real concurrent publication journey, independent persisted-state oracle, narrow-lock mutant. | `PROVED` — the final-source dual-pause publication courtroom proves overlapping effects, ordered root cutover, clean close, and exact fresh-reopen point/scan state (`C51-F017`). Consequential mutant 34 installs a real global execution lock and is killed at the independent backend-overlap assertion with exact `global-mutation-lock` localization. |
| C51-14-02 | Append, writeback, synchronization, replacement, and publication effects enter one canonical admission, scheduler, executor, settlement, Signal, recovery, and drain topology. | Source trace through every effect family, duplicate-route gates, bypass mutants, exact effect and settlement counters. | `PROVED` — final-source source and test review traces append/publication through `CanonicalRecordMutationPort` and exact C.6 writeback through the same admission, execution, settlement, Signal, recovery, and drain owners. Consequential mutants 15–18 and controls 37–42 are killed at their exact settlement, scheduler-admission, backend-receipt, derived-completion, generic-Signal, scheduler-counter, skipped-effect, raw-dispatch, readiness, and exact-writeback-receipt predicates (`C51-F031`). The final 214/214 integration suite, strict Clippy, formatting, diff, compiler, boundary, and agent-context gates pass. |
| C51-14-03 | Physical outcomes retain exact work identity and recovery disposition without Store owning branch, MVCC, or semantic-writer authority. | Facade/type inspection, hostile compile/source gates, recovery locator journey, branch-label non-authority mutant. | `PROVED` — publication success and settled failure carry immutable identity/effect/fate/recovery facts; denied planning reads retain exact identity and retry posture (`C51-F015`, `C51-F016`). The parser-governed complete physical runtime rejects branch/MVCC/semantic-writer authority, and consequential mutants 34/35 are killed at distinct global-lock and branch-label concurrency assertions (`C51-F018`). |
| C51-15-01 | Every ordinary Store product graph excludes `legacy-s2-models`, certification authority, and legacy background-work substitutes. | Cargo feature trees for Store, blob, maintenance, test-support fixture products, scheduler, residency, and backend; hostile manifest mutants. | `PROVED` — the seven-test sealing gate enumerates and hostile-mutates every ordinary product graph; blob and maintenance no longer activate legacy substrates, and isolated physical-isolation, layout, and certification fixture graphs compile with their declared authority only. |
| C51-15-02 | There is one production physical-work registry and one owner composition; no second pending registry, callback settlement path, timer/retry/policy registry, or special writeback executor survives. | Whole-graph owner/source inspection, sealing gate, functional deletion mutants, canonical writeback trace. | `PROVED` — whole-graph inspection and sealing retain one canonical owner composition; functional mutants 22–27 and 43 introduce real reconciliation, registry, lifecycle, reopen, or C.6-local substitutes and are killed at their intended runtime predicates. Exact writeback still enters the canonical executor. |
| C51-15-03 | Raw backend tickets/sessions, raw Signal aspect construction, and generic completion authority remain private to their admitted owners. | Visibility and dependency gates, positive facade consumers, compile-fail evidence, functional authority mutants. | `PROVED` — raw backend authority, tickets, builders, and direct sessions are private or deleted; compile-fail consumers and facade users pass, while mutants 31, 37, 38, 40–42 prove raw Signal, generic completion, counter settlement, direct dispatch, readiness, and foreign-receipt substitutions open no governed door. |
| C51-15-04 | Ordinary physical runtime graphs contain no internal JSON carrier or Query, Relational, branch-head, MVCC, or semantic writer authority. | Dependency trees, bounded source gates with explicit compatibility allowlists, hostile insertion mutants. | `PROVED` — exact source/dependency gates reject every semantic authority family. The 14-test JSON inventory admits only three exact Courtroom A certification-tool protocol files and rejects a near-miss path; production remains JSON-free. Mutants 29, 32, 33, and 35 are killed at internal-carrier, foundational-substitution, scope-broadening, and branch-label predicates. |
| C51-15-05 | Ordinary test-support and blob fixtures use canonical residency and recovery authority without certification-only or legacy substrate leakage. | Feature-isolated compilation, fixture provenance trace, real consumer tests, absence of legacy features. | `PROVED` — physical-isolation, layout, and certification fixture products compile independently; blob passes 176/176, test-support 21/21 plus its integration target, recovery-physics 37/37 plus five integrations, maintenance 3/3, layout 200/200 plus integrations, and certification 403/403 active tests. Canonical residency/recovery allocation and real publication receipts replace the legacy envelopes and searched authority. |
| C51-15-06 | The final Phase 15 tree has no warning, dead production branch, unused dependency, file-cap violation, facade leak, or miscomposed responsibility. | Strict scoped Clippy, unused/dead checks, line-cap gate, code-quality review of every changed C.5.1 file. | `PROVED` — formatting, all-target/all-feature check, warnings-denied Clippy, and diff checks pass. The complete 311-file dirty Rust inventory has zero non-allowlisted files over 400 lines; 133 advisory functions were inspected with no unresolved structural finding. The five changed manifests contain 96 direct dependencies with zero unused candidates after removing three dead edges. |
| C51-16-01 | `C6PhysicalWorkHandoff` exposes only canonical frame-load, dirty/writeback, candidate-publication, scheduler-demand, settlement, lifecycle, observation, and recovery seams, with no C.6-local runtime or pending registry. | Facade and visibility inspection, positive C.6 consumer, compile-fail sealing, functional no-local-runtime mutant. | `PROVED` — the final facade remains narrow, the positive C.6 consumer joins canonical submission/writeback/lifecycle fencing, and final-source mutant 43 installs a consequential local scheduler that is killed only at `c6-local-scheduler`. |
| C51-16-02 | Courtroom A proves the lifecycle maelstrom against the final source and complete retained-mutant catalog. | Final-source report, real Store/filesystem/process trace, independent serial oracle, exact counters, all retained mutants killed. | `PROVED` — final report `3b42862d…d76a9f5` accepts the real lifecycle maelstrom with an independent oracle, 29/29 killed mutants, 56 causal records, zero overflow, a current sealed source manifest, and a current test executable. |
| C51-16-03 | Courtroom B proves hostile physical truth across all named kill points with fresh offline observation and fresh Store reopen. | Final-source standalone report, five kill families and 25 stages, independent bytes/prefix/residue oracle, complete retained mutants. | `PROVED` — final report `7a868f21…e863ac8` accepts all five kill families, their 25 process stages and five case verifications, exact expected/observed bytes and residue, fresh offline/reopen oracles, 29/29 mutants, current binaries, and 43 unique timing phases. |
| C51-16-04 | Courtroom C proves bounded C.6 inheritance under a Store larger than residency budget, including dirty pressure, eviction/refault, cancellation, and close. | Final-source standalone report, exact residency/effect counters, offline observer and fresh reopener, complete retained mutants. | `PROVED` — final report `29345cce…a7dc91` accepts a 576,000-byte/192-record world under a 65,536-byte residency budget, with bounded peak residency, eviction/refault, effect-free cancellation, one exact dirty writeback, clean close, clean fresh reopen, current binaries, and 29/29 mutants. |
| C51-16-05 | The Phase 16 mutation lane executes and localizes every required production defect through a functionally consequential mutant. | Complete physical-work report for IDs 15-43, source/binary bindings, intended-cause localization, no marker-only mutant. | `PROVED` — final v2 report `af6174a8…52cae2` binds source closure `5a713be6…d8f205`, executes IDs 15–43 exactly, localizes all 29 at their intended predicates, retains 29 distinct current binaries plus an exact owner marker, and leaves no pending artifacts. |
| C51-16-06 | Courtroom and ordinary test costs remain lane-separated, measured, budgeted, and usable as routine engineering feedback. | Cold/warm compile, link, fixture, execution, retained-artifact, stage, and campaign measurements; hostile budget rejection tests. | `PROVED` — ordinary suites remain separate from mutation generation and standalone certification lanes. B completes in 10.859 seconds (10,560 ms runner-controlled) and C in 10.734 seconds (10,409 ms runner-controlled); every recorded stage and aggregate is within budget. Post-build source, final-source, and executable verification are separately timed and independently hostile-tested. |
| C51-X-01 | Boundary, agent-context, feature, and facade machine contracts accept the final C.5.1 source. | Boundary checker, agent-context check, C.5.1 sealing gate, generated-context consistency. | `PROVED` — the final boundary checker, generated agent-context checker, seven-test C.5.1 sealing gate, workspace compiler, and strict Clippy all pass on the source bound into the retained reports. |

## Finding History

### C51-F001 — Ordinary blob graph activates legacy residency

- status: `CORRECTED` — blob uses canonical residency authority, the legacy
  feature edge is removed, the ordinary graph passes the sealing gate, and the
  final all-feature blob suite passes 176/176.
- severity: P1 architecture
- affected guarantees: `C51-15-01`, `C51-15-05`, `C51-16-04`
- evidence: `worth-store-blob-chunks/Cargo.toml` enables
  `worth-store-buffer-pool/legacy-s2-models` in its ordinary dependency.
- required correction: migrate blob execution and fixtures to canonical
  `PhysicalResidencyPool` allocation authority, then remove the feature edge.
- closing proof: isolated ordinary blob feature tree, real blob journeys, and a
  hostile manifest mutant rejected by the central sealing gate.

### C51-F002 — Physical-isolation recovery fixture graph is internally inconsistent

- status: `CORRECTED` — physical-isolation, layout, and certification fixture
  graphs compile independently and their real downstream consumer suites pass.
- severity: P1 harness architecture
- affected guarantees: `C51-15-05`, `C51-16-04`
- evidence: physical-isolation fixture consumers import `recovery::closeout`,
  while `recovery/mod.rs` exposes it only under `certification-world`.
- required correction: keep ordinary recovery meaning on canonical production
  residency/recovery authority and isolate only genuinely certification-owned
  dirty-publication machinery.
- closing proof: isolated `physical-isolation-fixtures`, `layout-fixtures`, and
  `certification-world` compilation plus real downstream consumers.

### C51-F003 — Phase 16 reports do not bind the live mutant catalog

- status: `CORRECTED` — the final mutation report and all three standalone
  courtroom reports bind IDs 15–43, current source, and current binaries.
- severity: P1 evidence
- affected guarantees: `C51-16-02` through `C51-16-06`
- evidence: current source requires mutant IDs 15-43; retained reports were
  generated before mutant 43, before the consequential rebinding of mutants
  15-18, and before the latest source changes. The Courtroom A expectation for
  mutant 34 also still names the displaced branch-writer predicate rather than
  the live global-mutation-lock predicate.
- required correction: finish production and harness fixes first, regenerate the
  complete mutation report, and then regenerate all three standalone courtrooms
  without further source changes.
- closing proof: report source/binary bindings match the final source and every
  report carries the complete killed-mutant denominator.

### C51-F004 — Ordinary maintenance still carries legacy background envelopes

- status: `CORRECTED` — maintenance envelopes carry canonical operation grants,
  the legacy graph edge is absent, and the final maintenance suite passes 3/3.
- severity: P1 architecture
- affected guarantees: `C51-15-01`, `C51-15-05`
- evidence: `memory_envelopes.rs` wraps `AdmittedBackgroundEnvelope` for
  compaction planning and import/export work, and ordinary layout projection
  consumes those wrappers.
- required correction: preserve the two semantic envelope types while backing
  them with canonical `OperationAllocationGrant` values admitted for the
  maintenance scope.
- closing proof: ordinary maintenance feature tree contains no legacy feature,
  real queue consumers retain exact pool counters, and certification fixtures
  do not leak into the ordinary graph.

### C51-F005 — Raw backend queue execution authority is publicly exported

- status: `CORRECTED` — raw authority, ticket, builder, and session exports are
  removed; obsolete direct-session code is deleted; all backend public
  compile-fail documentation tests and the final 209/209 integration suite
  pass.
- severity: P1 authority
- affected guarantees: `C51-14-02`, `C51-15-03`, `C51-15-06`
- evidence: the physical-backend queue facade publicly re-exports execution
  authority, ticket, completion builder, Store-owned session, and direct
  execution session types even though no external production crate consumes
  them.
- required correction: keep the authority/ticket/builder private to scheduled
  backend execution and delete the obsolete direct-session surface and tests.
- closing proof: backend public-surface tests and compile-fail consumers prove
  only admitted scheduled operations can reach media.

### C51-F006 — The central sealing gate audits too small a product denominator

- status: `CORRECTED` — the central gate enumerates every ordinary C.5.1 product
  graph and its seven hostile denominator/source tests pass.
- severity: P1 enforcement architecture
- affected guarantees: `C51-15-01`, `C51-15-02`, `C51-15-04`, `C51-X-01`
- evidence: `c5_1_sealing_gate.rs` scans and feature-checks only Store,
  scheduler, buffer pool, and backend, so the live blob and maintenance legacy
  edges fall outside its contract.
- required correction: enumerate every ordinary C.5.1 product graph and its
  relevant production roots, with narrow allowlists for explicit external
  compatibility and test-only JSON.
- closing proof: hostile manifest/source mutants in each product are rejected
  and every clean ordinary product tree passes independently.

### C51-F007 — Required Phase 16 deletion mutants are marker-only

- status: `CORRECTED` — IDs 22-35 and 43 now alter real execution,
  admission, lifecycle, reopen, concurrency, or C.6 progression behavior. Their
  selectors exercise those boundaries instead of killing inserted identifiers.
- severity: P1 evidence
- affected guarantees: `C51-15-02`, `C51-15-04`, `C51-16-01`, `C51-16-05`
- evidence: mutants 22-35 and 43 originally inserted unused aliases or structs, then their
  selected tests kill those identifier strings through source scanning. No
  production execution, authority, state, or public surface changes.
- required correction: replace every marker insertion with a functionally
  consequential mutation of a real production boundary and make the selected
  test fail for the intended runtime, compile, visibility, or dependency cause.
- closing proof: each clean selector passes, each applied mutant fails only its
  named predicate, and the complete campaign retains and hashes all binaries.

### C51-F008 — The Phase 16 specification carries a stale mutant denominator

- status: `CORRECTED` — the specification and catalog-shape validator now
  require 29 executed mutants, IDs 15-43.
- severity: P2 specification
- affected guarantees: `C51-16-02` through `C51-16-05`
- evidence: the live catalog and validators require 29 mutants, IDs 15-43, but
  the Phase 16 engineering decision still states that reports bind 28 binaries.
- required correction: derive or state the live 29-mutant denominator
  consistently and keep the catalog-shape validator authoritative.
- closing proof: specification, reader tests, all reports, and artifact
  inventories agree on 29.

### C51-F009 — Courtroom C does not hostile-test every timing budget

- status: `CORRECTED` — B and C now distinguish pre-report from complete
  timing fixtures, exceed every enforced stage for its exact named cause,
  cover all 25 B child stages and five case verifications, and prove failed
  pre- and post-publication validation leaves no report artifact.
- severity: P2 evidence
- affected guarantees: `C51-16-04`, `C51-16-06`
- evidence: production timing validation enforces mutation-evidence, world, and
  executable-verification limits, but Courtroom C timing tests never exceed
  those limits independently.
- required correction: add hostile cases for every stage class and preserve the
  existing missing/duplicate, report-encoding, aggregate, and publication
  cleanup evidence.
- closing proof: every specified stage and aggregate budget has an independent
  rejection test and failed publication leaves no accepted artifact.

### C51-F010 — Locator readmission duplicates failure classification

- status: `CLOSED`
- severity: P2 architecture
- affected guarantees: `C51-13-04`
- evidence: `readmit_locator_detailed` converts a manifest failure to the
  public read denial, but separately calls shared health with the hard-coded
  `ArtifactUnavailable` denial.
- required correction: classify the manifest failure once and propagate that
  same denial to health and the detailed readmission failure.
- closing proof: a real damaged root-manifest block makes `open_external`
  report `ArtifactDamaged`, makes locator readmission report
  `CurrentRootUnavailable`, and fences subsequent mutation through shared
  health. The exact final-source test passes.

### C51-F011 — Untyped-basis compile proof contains unreachable-code noise

- status: `CLOSED`
- severity: P2 evidence
- affected guarantees: `C51-13-05`
- evidence: `untyped_physical_work_basis_is_rejected.rs` constructs a Signal
  aspect with `todo!()`, so the compiler reports the intended three type
  mismatches but also reports that the latter statements are unreachable.
- required correction: express each rejected source type as a typed function
  parameter so every mismatch is independently compiled without fabricating a
  value or producing an unrelated diagnostic.
- closing proof: the corrected trybuild fixture reports only the three intended
  type mismatches and the complete authority UI harness remains green in
  16.71 seconds.

### C51-F012 — Exact binding oracle erases unexpected installed bindings

- status: `CLOSED`
- severity: P1 evidence
- affected guarantees: `C51-13-05`
- evidence: `read_partitions.rs::record_partitions` filters observations to the
  four expected partition strings before comparing them with the expected set.
  A broadened runtime that installs any additional binding therefore still
  satisfies the assertion claiming exactly four bounded dependencies.
- required correction: compare the complete relevant installed binding set
  against an independently declared expected set without filtering away
  unexpected values; retain causal route assertions for ordinary reads and
  scans.
- closing proof: the ordinary runtime reports exactly the expected installed
  partitions, and adding or broadening any installed read binding makes the
  courtroom fail. The final exact partition journey passes.

### C51-F013 — Scale media counters do not bracket the measured reads

- status: `CLOSED`
- severity: P1 evidence
- affected guarantees: `C51-13-06`
- evidence: `manifest_scale::world::observe_runtime_world` snapshots media
  before and immediately after Store reopening, then passes those two snapshots
  into `observe_live_reads`. `open_reads` and `open_bytes` consequently measure
  admission/reopen work rather than the point read and scan executed afterward.
  The scale observation also omits the point read's `manifest_bytes`.
- required correction: snapshot media immediately before, between, and after
  point and scan operations; carry point/scan media bytes and manifest bytes
  into the observation and assert their breadth-bounded behavior across scale
  worlds.
- closing proof: the 1/9/65-record worlds report independently bracketed point
  and scan media/manifest costs, exact semantic results, and a 7.43-second warm
  cost.

### C51-F014 — Append planning retains a direct-media read pipeline

- status: `CORRECTED` — all ordinary publication, planning, and
  segment-membership reader construction uses the canonical source; the
  complete source gate, reopened append trace, publication-focused journeys,
  and final 209/209 integration suite pass.
- severity: P1 architecture
- affected guarantees: `C51-14-02`
- evidence: `RecordPublicationDirector::prepare_rebasable` and the root-rebase,
  free-space, segment-membership, and reusable-inline planning contexts pass
  `QualifiedFilesystemMedia` plus `FrameLoadPort` into bootstrap readers.
  These ordinary append reads can reach media without canonical read admission,
  Signal readiness, scheduler reservation, executor settlement, recovery, or a
  causal work identity.
- required correction: construct one canonical planning-read source from the
  Store work runtime, attach it to the publication director, and remove raw
  loader authority from every ordinary append-planning reader/context. Raw
  media may remain only for immutable Store identity where no effect occurs.
- closing proof: a reopened Store append that traverses and reuses published
  physical truth attributes every planning media read to causal canonical read
  work, retains exact publication work, and leaves no direct planning-reader
  constructor reachable from the ordinary publication topology.

### C51-F015 — Publication results discard settled fate and recovery

- status: `CORRECTED` — canonical settlement is carried into successful and
  settled-failure outcomes; both direct outcome-to-causal-record courtrooms
  and the final 209/209 integration suite pass.
- severity: P1 architecture
- affected guarantees: `C51-13-04`, `C51-14-02`, `C51-14-03`
- evidence: `PreparedCanonicalRecordMutation::execute` consumes
  `SettledPhysicalWork` into evidence, then successful completion retains only
  `PhysicalWorkIdentity`. `RecordPublicationWorkEffect` consequently exposes
  only stage and identity. Settled `NoEffect` failures likewise reconstruct a
  backend failure with no recovery disposition. A future semantic adapter would
  have to query a bounded causal observer or infer physical truth.
- required correction: carry one immutable settlement fact containing identity,
  physical effect identity, effect fate, and recovery disposition from canonical
  settlement into every successful publication work effect and every settled
  failure. Never recreate those fields from stage or counters.
- closing proof: success and denied-before-effect publication courtrooms assert
  the outcome-attached settlement fact directly and join it exactly to the
  causal record without observer lookup being required to discover recovery.
  `successful_publication_exposes_each_causal_work_identity_once` and
  `record_barrier_fault_is_not_laundered_as_recovery_journal_failure` pass
  against the corrected source.

### C51-F016 — Canonical planning-read failures become false layout damage

- status: `CORRECTED` — canonical preparation retains receipt identity; public
  reads, all four read/scan routes, and append planning distinguish transient
  denial from damage and prove same-runtime retry; the final 209/209
  integration suite passes.
- severity: P1 correctness and availability
- affected guarantees: `C51-14-02`, `C51-14-03`
- evidence: canonical reads wrap backend denial as
  `FrameLoadFailureKind::Work(CanonicalRecordReadFailure::Backend(_))`, while
  `inline_plan_failure::layout_failure` recognizes only the old direct
  `FrameLoadFailureKind::Backend` form. The fallback reports
  `PublishedLayoutDamaged`, and range-read admission failures can lose the
  already-created work identity before `PreparedFrameRead` exists. The ordinary
  public-read classifier contains the same root defect: it reports every
  canonical backend failure other than `Absent` as `ArtifactDamaged`, so a
  transient denied-before-effect read revokes healthy Store truth.
- required correction: preserve optional work identity through every canonical
  range-read preparation failure and classify canonical backend, residency,
  lifecycle, scheduler, and damage failures by their actual typed cause. A
  transient denied-before-effect backend read or append-planning read must
  remain retryable and must not revoke healthy Store truth.
- closing proof: fault a real reopened append planning read before effect, assert
  exact causal/work identity and `BackendUnavailable`, then prove an unfaulted
  successor append succeeds without reopening or inspection. The ordinary
  public-read fault family must likewise prove a same-runtime successor read
  and clean close.

### C51-F017 — Concurrent publication evidence stops before durable reopen

- status: `CORRECTED` — the existing overlap courtroom now closes and reopens
  from media, then proves exact seed/left/right point reads and scan payloads;
  the final ordinary integration suite passes. The phase-level retained mutant
  campaign remains a separate open guarantee.
- severity: P2 evidence
- affected guarantees: `C51-14-01`
- evidence: `disjoint_payload_writes_overlap_while_root_cutover_orders_both_batches`
  proves two paused payload effects, ordered generations, and in-process reads,
  but closes without reopening and validating the catalog-selected persisted
  world independently of the director's in-memory root.
- required correction: retain the published record identities, close cleanly,
  reopen from media, and independently read and scan the exact seed and both
  concurrent payloads.
- closing proof: the same concurrency courtroom proves two real overlapping
  payload effects, two ordered cutovers, and an exact three-record fresh-process
  world.

### C51-F018 — Branch-authority exclusion was only partially machine-governed

- status: `CORRECTED` — the complete `physical_runtime` tree is governed by
  exact JSON/projection bans and targeted CamelCase, snake_case, and
  SCREAMING_SNAKE semantic-authority fragments; physical routing-tree branches
  and generic Signal branch epochs remain legal.
- severity: P1 authority evidence
- affected guarantees: `C51-14-03`
- evidence: the machine boundary contract forbids branch/MVCC identifiers under
  `physical_runtime/work` and `physical_runtime/instance` but not
  `physical_runtime/record_serving`, while controlled mutants 34 and 35 insert
  unused marker types that are killed only by exact-string scans.
- required correction: govern the complete physical runtime boundary against
  branch, MVCC, and semantic-writer authority and replace the relevant marker
  mutants with consequential dependency, facade, or concurrency-authority
  mutations.
- closing proof: boundary-check parser tests, the full boundary checker, and
  agent-context check pass against restored final source. Applied mutant 34 is
  rejected by `BC2001` for both `BRANCH_WRITER_AUTHORITY` and
  `_branch_writer_authority`; its runtime campaign is independently killed at
  `global-mutation-lock`. Mutant 35 changes the real concurrency relation and
  is killed at `branch-label-disjointness`. Both observations bind the exact
  executed binary and original source hash; restored sources match those
  hashes. The obsolete exact-marker scan tests were deleted.

### C51-F019 — The partial-read courtroom did not perform a partial read

- status: `CORRECTED` — `AllowPrefix` now performs and accounts for the admitted
  read prefix, and short completed reads enter typed `ReadIncomplete`
  settlement.
- severity: P1 evidence and correctness
- affected guarantees: `C51-13-04`
- evidence: backend exact reads previously converted `AllowPrefix { bytes: 3 }`
  into zero-byte `AccessLimitExceeded`; `matches_read` also rejected every
  short completion as foreign, making the existing `ReadIncomplete` classifier
  unreachable.
- required correction: carry exact completed bytes and backend effect identity
  from the admitted prefix, accept bounded short completions as matching the
  dispatched read, and classify them as terminal `ReadIncomplete`.
- closing proof:
  `partial_backend_read_is_denied_at_the_public_read_boundary_and_revokes_health`
  and `continuation_damage_revokes_shared_health_and_fences_mutation` pass with
  exact three-byte media counters, backend effect identity, `ReadIncomplete`,
  `InspectionRequired`, Signal invalidation, and shared-health fencing.

### C51-F020 — Non-completion Signal release leaked the in-flight request

- status: `CORRECTED` — framework identity release now retires the bound Signal
  request with runtime-policy authority before removing locality.
- severity: P1 lifecycle and bounded-resource safety
- affected guarantees: `C51-13-04`, `C51-14-02`
- evidence: terminal `ReadIncomplete` reconciled physical truth and removed the
  locality entry, but `PhysicalSignalGraph::release_identity` never cancelled
  the underlying in-flight resource request.
- required correction: make framework-owned identity release terminally join
  both Signal in-flight state and locality while leaving committed completion
  release unchanged.
- closing proof: both real partial-read courtrooms reach zero active locality
  and zero active in-flight state before proving their exact damage and health
  outcomes.

### C51-F021 — Timeout-capture evidence could miss its output precondition

- status: `CORRECTED` — the timeout fixture uses the runner's established
  two-second child-process budget rather than a 50-millisecond process-start
  race.
- severity: P2 harness reliability
- affected guarantees: `C51-16-02`, `C51-16-03`, `C51-16-04`, `C51-16-06`
- evidence: the full runner suite passed 112 tests and failed
  `timeout_retains_both_captured_streams` because the nested Rust test process
  could be terminated before emitting flushed `before`/`problem` markers.
  Capture code already killed, reaped, and joined both reader threads.
- required correction: give process creation the same bounded startup posture
  as the other real-child fixtures, while retaining exact stdout/stderr and
  timeout assertions.
- closing proof: the focused timeout test passes five consecutive runs at
  2.01 seconds each; the complete runner suite passes 113 unit and two CLI
  tests with 2.20 seconds of test execution.

### C51-F022 — A dirty read courtroom exceeded the hard file cap

- status: `CORRECTED` — read evidence is split by cold/hot behavior,
  partial-read damage, cancellation, and Signal cleanup responsibility.
- severity: P2 composition
- affected guarantees: `C51-13-02`, `C51-13-04`, `C51-15-06`
- evidence: `record_read_path.rs` reached 402 lines and was not allowlisted.
- required correction: establish scenario-family topology rather than remove
  two incidental lines.
- closing proof: every dirty Rust file is at or below 400 lines, and the four
  affected read scenarios pass in 0.51 seconds.

### C51-F023 — Real truncation can escape exact Signal invalidation

- status: `CORRECTED` — successful metadata and range reads carry a one-shot
  Signal-issued projection-failure capability through every later structural
  validator; consuming it applies the exact admitted delta once and revokes
  shared health.
- severity: P1 read-fault authority and health coherence
- affected guarantees: `C51-13-04`, `C51-14-02`
- evidence: truncating an initialized segment file to zero produces the honest
  public `ArtifactDamaged` denial and shared-health fence with zero positioned
  reads, but the exact Signal aspect invalidation count remains zero.
- required correction: retain a one-shot, Signal-issued projection-failure
  capability across successful metadata settlement and consume it at every
  later structural length rejection. Diagnostic observations must not become
  authority, and one rejection must produce exactly one invalidation.
- closing proof: the real truncated-file courtroom reports exactly two
  prerequisite locator reads, rejects the segment at metadata without a range
  effect, applies one exact Signal invalidation, fences later work, and leaves
  zero locality. The C.6 projection-failure and successful-range semantic
  rejection courtrooms prove the same authority survives later decoding.

### C51-F024 — Canonical post-dispatch read failure discarded causal identity

- status: `CORRECTED` — every post-dispatch backend and settlement failure
  retains the prepared physical-work identity without duplicating observation.
- severity: P2 causal evidence honesty
- affected guarantees: `C51-13-02`, `C51-13-04`
- evidence: `PreparedCanonicalRecordRead::execute` previously converted
  backend and settlement failures without attaching its known identity, so
  later failure composition could report no causal work for a dispatched read.
- required correction: bind every post-dispatch failure and settlement mismatch
  to the prepared identity without duplicating earlier work observations.
- closing proof: exact partial-read, continuation-damage, all-route transient
  retry, locator, and C.6 projection-failure journeys pass, followed by the
  complete 209/209 structured integration suite.

### C51-F025 — The truncation courtroom counted prerequisite reads as forbidden work

- status: `CORRECTED` — the oracle distinguishes the two prerequisite locator
  reads from the damaged segment operation.
- severity: P2 evidence causality
- affected guarantees: `C51-13-04`
- evidence: the real damaged-record request performed two required positioned
  reads while locating segment membership before its metadata observation
  proved the segment was empty; a global zero-read assertion falsely treated
  both prerequisites as forbidden segment dispatch.
- required correction: require exactly the two locator reads and prove the last
  rejected physical work is `ArtifactMetadataRead`, so any later segment range
  dispatch still kills the courtroom.
- closing proof:
  `truncated_segment_is_structural_damage_before_range_dispatch_and_revokes_health`
  passes with exactly two prerequisite positioned reads and an
  `ArtifactMetadataRead` terminal operation for the rejected segment.

### C51-F026 — The scan-damage courtroom called a short read complete

- status: `CORRECTED` — the media oracle distinguishes a retained partial
  effect from a completed exact-read operation.
- severity: P2 evidence truth
- affected guarantees: `C51-13-04`
- evidence: a faulted continuation read attempted one positioned operation and
  retained three completed bytes, while the backend correctly reported zero
  completed exact-read operations; the test still expected one.
- required correction: assert one attempt, zero completed operations, three
  completed bytes, terminal `ReadIncomplete`, exact scan-partition
  invalidation, and shared-health fencing in the same trace.
- closing proof:
  `continuation_damage_revokes_shared_health_and_fences_mutation` passes with
  one attempt, zero completed exact-read operations, three completed bytes,
  terminal `ReadIncomplete`, exact invalidation, and shared-health fencing.

### C51-F027 — Successful canonical reads lost later projection-failure authority

- status: `CORRECTED` — frame loads retain a one-shot capability derived from
  the admitted semantic basis and installed Signal binding.
- severity: P1 authority and health coherence
- affected guarantees: `C51-13-04`, `C51-14-02`
- evidence: a canonical range read could settle successfully, then later
  manifest, free-space, segment, extent, inline, or tail decoding could reject
  those bytes as structural damage while retaining only diagnostic work
  identity. Signal therefore had no authority-bearing path to invalidate the
  admitted projection.
- required correction: carry an unforgeable projection-failure capability from
  canonical admission through the loaded frame and consume it at every
  structural rejection site, exactly once.
- closing proof: exact locator-damage and C.6 projection-failure tests join a
  completed real range effect to one Signal invalidation and shared-health
  revocation. Manifest, free-space, segment, extent, inline, and tail consumers
  all use the same capability path; the final structured suite passes 209/209.

### C51-F028 — Signal-joined abandonment was retained as unresolved evidence

- status: `CORRECTED` — successful framework Signal abandonment now releases
  capacity as an already joined terminal outcome, while failed abandonment
  retains its consumer for later reconciliation.
- severity: P1 bounded-resource correctness
- affected guarantees: `C51-13-04`, `C51-14-02`, `C51-16-02`
- evidence: `PhysicalWorkAbandonment::complete` cancelled and released Signal
  state, then recorded `ReleasedBeforeDispatch` with a retained consumer. Safe
  repeated abandonment could overflow bounded terminal evidence and falsely
  force Store inspection, breaking scheduler-capacity recovery and the
  lifecycle maelstrom.
- required correction: make the release API distinguish a successfully joined
  Signal terminal from an abandonment failure; only the latter may retain the
  consumer obligation.
- closing proof: the seven capacity tests, exact live scheduler-exhaustion
  courtroom, Phase 16 maelstrom, and final 209/209 suite all pass with exact
  capacity recovery and zero residual work.

### C51-F029 — Courtroom A's close gate could be stolen by append rebase

- status: `CORRECTED` — the backend exposes a certification-only one-shot fault
  activation that is disarmed during append work and atomically consumed by
  exactly one later identified operation.
- severity: P1 evidence and test-time boundedness
- affected guarantees: `C51-16-02`, `C51-16-06`
- evidence: the fixed global positioned-read ordinal intended for the
  dispatched-close read matched an append root-rebase read instead. That read
  held publication state while paused, the second append waited for the same
  state, and the test exceeded both 120-second and 600-second budgets.
- required correction: arm the close fault only after append completion, make
  selection one-shot under concurrency, and compare the paused backend
  operation with the settled close-read effect identity.
- closing proof: sequential and concurrent selector tests prove exactly-once
  consumption; ambiguous activated schedules are rejected; the complete
  backend passes 184 unit and 28 compile-fail tests; Courtroom A passes its
  seven-test family in 0.85 seconds; the final integration suite passes 209/209
  in 32.02 seconds.

### C51-F030 — Final-source quality gates found warning and composition regressions

- status: `CORRECTED` — the redundant read guard was expressed structurally,
  fault-schedule validation moved into its own test module, and proof-carrying
  enum size exceptions are narrow and performance-justified.
- severity: P2 code quality
- affected guarantees: `C51-15-06`
- evidence: strict Clippy rejected a redundant zero-byte guard and two large
  proof-carrying enum variants; adding activated-schedule coverage also pushed
  `fault_interposition.rs` to 416 lines.
- required correction: fix the guard without suppression, split schedule-shape
  tests by responsibility, and keep move-owned read and post-effect settlement
  proofs inline with explicit reasons rather than adding allocator traffic.
- closing proof: strict all-features Clippy passes with warnings denied; all 101
  dirty Rust files are at or below 400 lines; formatting, focused damage tests,
  backend fault tests, and the final 209/209 integration suite pass.

### C51-F031 — Phase 14 progression mutants attacked diagnostic shadows

- status: `CORRECTED` — mutants 15-18 now change consequential production
  progression rather than report or diagnostic projections.
- severity: P1 evidence architecture
- affected guarantees: `C51-14-02`, `C51-16-02`, `C51-16-05`
- evidence: the former mutations could be killed without proving that scheduler
  admission, exact backend receipt binding, physical settlement classification,
  or the real Signal request join governed the media effect. Green mutant
  results therefore did not close the Phase 14 bypass requirement.
- required correction: mutate the actual scheduler durability admission,
  settlement classifier, backend-receipt matcher, and Signal lifecycle join;
  give each mutation one intended-cause predicate and consequential state
  oracle.
- closing proof: mutants 15-18 are killed respectively by exact settlement,
  scheduler-admission, backend-receipt, and derived-completion predicates;
  controls 37-42 independently reject generic Signal settlement,
  scheduler-counter settlement, skipped backend effects, raw backend dispatch,
  Signal-readiness bypass, and dirty cleanup without an exact writeback
  receipt. The ten mutant runs restore all nine distinct source files
  byte-for-byte. The retained Phase 16 reports remain reopened
  until regenerated from final source.

### C51-F032 — Read and publication planning still collapse semantic steps

- status: `CORRECTED` — admission, discovery, structural validation, projection,
  cursor positioning, successor projection, and publication assembly now have
  domain-named owners; final composition scrutiny finds no unresolved collapse.
- severity: P2 composition architecture
- affected guarantees: `C51-15-06`
- evidence: `access/locate/inline.rs::open_inline` owns runtime and health
  admission, segment-membership discovery, artifact-length validation, page
  loading, projection decoding, generation and format classification, and
  session construction in one function. `access/locate/extent.rs::open_extent`
  combines the analogous extent concerns, `access/scan.rs::scan` combines
  allocation admission, cursor readmission, manifest positioning, health
  revocation, and session construction, and
  `planning/rebased_root.rs::rebase` combines three independent manifest
  projections with publication-artifact assembly. These are named semantic
  responsibilities rather than incidental control-flow length.
- required correction: preserve the public typestate and authority boundaries
  while extracting narrow, domain-named steps for admission, discovery,
  structural validation, projection decoding, cursor positioning, successor
  manifest projection, and publication assembly. Do not introduce catch-all
  helper modules or duplicate truth.
- closing proof: reread every changed Phase 15 file against the composition and
  domain-structure laws, rerun the dirty-function scrutinizer and line-cap
  inventory, and retain the existing point-read, scan, publication,
  concurrency, and recovery courtrooms without changing their semantic or
  effect evidence.

### C51-F033 — The isolated layout fixture graph depends on certification authority

- status: `CORRECTED` — `layout-fixtures` compiles without certification
  authority, real layout consumers pass 200/200 plus integrations, and exact
  allocation/capability release remains visible.
- severity: P1 fixture architecture
- affected guarantees: `C51-15-01`, `C51-15-05`, `C51-15-06`
- evidence: isolated
  `worth-store-test-support --no-default-features --features layout-fixtures`
  compilation reports 22 errors. The LSM/layout fixture family imports
  certification-only security-scope issuers and backend-capability admission
  constructors, calls private replay classification, and constructs compaction
  recovery visibility through certification-only source-precedence fixtures.
- required correction: issue valid layout fixture authority through ordinary
  production admission and persisted/reopened recovery paths, or remove
  fixture responsibilities that have no ordinary causal world. Do not make
  `layout-fixtures` enable certification authority and do not expose private
  constructors merely to satisfy the harness.
- closing proof: isolated `layout-fixtures` compilation, real downstream
  layout consumers, exact allocation/capability release, and a feature tree
  showing no certification or legacy authority in the ordinary graph.

### C51-F034 — Recovery allocation migration is incomplete across certification

- status: `CORRECTED` — the legacy recovery envelope is gone, canonical
  move-owned allocation reaches all certification consumers, and recovery plus
  certification owner suites pass.
- severity: P1 fixture and evidence architecture
- affected guarantees: `C51-15-01`, `C51-15-05`, `C51-15-06`, `C51-X-01`
- evidence: the integrated certification owner build still imports the deleted
  `RecoveryMemoryEnvelope` and `RecoveryMemoryEnvelopeDenial` across courtroom,
  scheduling-evidence, recovery integration-fixture, compile-contract, and
  canonical-basis registry consumers. The scheduling normalizer also requires
  a nonzero resident-frame count even though canonical
  `RecoveryMemoryAllocation` owns operation bytes and correctly reports zero
  frame leases. The owner-coverage ledger calls the layout cutover observer
  through a nonexistent root re-export instead of its public
  `compaction_projection` facade.
- required correction: migrate the complete semantic family to move-owned
  `RecoveryMemoryAllocation` values issued by real `PhysicalResidencyPool`
  admission, preserve wrong-scope and over-budget denial evidence, validate
  recovery counters according to recovery allocation semantics, update compile
  contracts and source registries, and call the layout observer through its
  actual facade. Do not restore a compatibility alias or invent resident-frame
  evidence.
- closing proof: no legacy recovery-envelope symbol remains, wrong-scope and
  over-budget hostile tests reach real residency admission, recovery-physics
  owner tests pass, integrated certification owner coverage passes, and the
  affected ordinary/certification feature graphs remain isolated.

### C51-F035 — Default recovery tests activate certification-only proof families

- status: `CORRECTED` — ordinary recovery tests remain on the canonical graph;
  certification-only durability/layout evidence is feature-gated and the final
  all-feature recovery suite passes 37/37 plus five integrations.
- severity: P1 feature-graph and evidence architecture
- affected guarantees: `C51-15-01`, `C51-15-05`, `C51-15-06`
- evidence: default `worth-store-recovery-physics --tests --no-run` compilation
  reaches page-LSN tests built on `legacy-s2-models`, WAL durability tests that
  issue backend capability through certification-only constructors, and a
  durable-publication support module whose owner tests are already explicitly
  certification-gated. The default graph fails with 21 missing legacy or
  certification-authority symbols.
- required correction: keep ordinary recovery production and ordinary tests on
  the canonical graph; compile legacy page-LSN evidence only with its explicit
  legacy certification feature, and compile backend fault/durability fixture
  authority only with `certification-test-authority`. Do not enable either
  feature in the default graph.
- closing proof: default recovery tests compile without legacy or certification
  authority, the certification feature graph compiles and executes its owned
  durability/layout evidence, and Cargo feature trees show the separation.

### C51-F036 — Ordinary blob tests import a certification-world compaction fixture

- status: `CORRECTED` — compaction evidence is owned by the explicit
  certification feature while ordinary blob tests remain canonical; the final
  all-feature blob suite passes 176/176.
- severity: P1 fixture topology
- affected guarantees: `C51-15-01`, `C51-15-05`, `C51-15-06`
- evidence: the default blob library test graph imports
  `test-support::physical_isolation::compaction`, whose real world joins
  recovery completion, integrity clearance, stable-read execution, temporary
  Store publication, and recovery-visible cutover and is therefore correctly
  owned by `certification-world`. The default graph cannot resolve that module.
  It also compiles unconsumed harness, closeout, and heavy-fixture exports whose
  only consumer is the certification facade.
- required correction: classify the compaction family as integrated
  certification evidence, gate it and its harness-only support on blob
  `certification-test-authority`, and leave ordinary blob tests on canonical
  production/test-local authority without widening test-support features.
- closing proof: default blob tests pass without certification or legacy
  features; explicit certification blob tests execute the compaction family;
  both graphs compile warning-free and their feature trees retain the intended
  separation.

### C51-F037 — Recovery integrity fixtures collapse physical form and checksum extent

- status: `CORRECTED` — page and frame fixtures materialize distinct protected
  forms and bind checksums to the exact admitted extent; affected consumer
  suites and hostile damage cases pass.
- severity: P1 fixture honesty and integrity
- affected guarantees: `C51-15-05`, `C51-15-06`, `C51-16-03`
- evidence: the shared recovery integrity fixture always protects bytes encoded
  with a record-frame header, including for page admission, while page and
  frame requests declare `crc32c(payload)` rather than the checksum of the
  protected header-plus-payload extent. Full-extent production admission
  correctly rejects the fixture, causing eight blob publication/recovery tests
  to fail at setup with `ChecksumMismatch`.
- required correction: materialize distinct page and frame protected forms,
  derive each decode witness from those exact bytes, and declare the checksum
  over the same protected extent. Keep before-WAL replay bytes explicitly
  frame-backed.
- closing proof: focused page, WAL, recovery-entry, and blob publication tests
  pass; hostile checksum damage still rejects for the intended reason; and the
  ordinary/certification consumers share no payload-only checksum shortcut.

### C51-F038 — Blob allocation provenance oracle confuses capacity with grant bytes

- status: `CORRECTED` — the oracle distinguishes exact admitted bytes from pool
  capacity and independently proves release; the complete blob suite passes.
- severity: P2 evidence
- affected guarantees: `C51-15-05`, `C51-15-06`
- evidence:
  `verified_read_retains_canonical_allocation_provenance_and_releases_authority`
  requests a four-byte blob operation from a pool whose operation capacity is
  eight bytes, then expects the retained operation observation to report eight.
  The canonical grant reports the exact four admitted bytes.
- required correction: verify the production counter contract from the grant
  and pool owners, then assert exact requested/admitted bytes and independent
  post-drop pool release. Do not change production accounting merely to retain
  the stale capacity oracle.
- closing proof: the focused provenance test distinguishes admitted bytes from
  pool capacity, fails under either counter substitution, and the complete blob
  suite passes.

### C51-F039 — Logical decode receives the checksum extent instead of the payload

- status: `CORRECTED` — integrity-checked page/frame owners retain the protected
  extent but pass only the witness-derived payload slice into logical decode.
- severity: P1 correctness and phase progression
- affected guarantees: `C51-15-05`, `C51-16-03`
- evidence: after the fixture supplies honest header-plus-payload bytes,
  pre-decode integrity succeeds but page container inspection reports
  `BodyLengthMismatch`: `IntegrityCheckedPage::logical_decode_gate` and the
  equivalent frame method pass the entire protected checksum extent even though
  the admitted witness declares a payload offset and payload length. WAL
  inspection already had a local payload-slice correction, leaving the shared
  logical-decode transition inconsistent.
- required correction: retain full protected bytes and checksum identity as
  pre-decode evidence, but construct every logical decode gate from the exact
  witness-derived payload slice. Keep the transition owned by the checked
  physical-form type so consumers cannot choose the wrong extent.
- closing proof: page/container, WAL, authenticity-gated decode, and blob
  recovery/publication paths accept intact full forms and continue rejecting
  length, checksum, and authenticity damage at their named boundaries.

### C51-F040 — Partial-publication replay parses residency as integrity authority

- status: `CORRECTED` — replay readmission requires `IntegrityCheckedFrame` and
  consumes its exact payload slice; raw protected views cannot enter the path.
- severity: P1 authority and recovery correctness
- affected guarantees: `C51-15-03`, `C51-15-05`, `C51-16-03`
- evidence:
  `PartialPublicationBeforeWalReplayRead::from_protected_physical_bytes`
  accepts only `ProtectedPhysicalByteView`, which proves bounded physical
  residency but carries no admitted header witness or checksum result, then
  parses the full protected extent as if it began with the logical replay
  record. A real framed fixture therefore reports no crash-edge digest.
- required correction: require and consume `IntegrityCheckedFrame`, derive the
  exact payload slice from its admitted witness, and classify only those bytes.
  Update the compile contract so a raw or merely protected byte view cannot
  enter replay readmission.
- closing proof: intact before-WAL records readmit only after full-frame
  integrity; raw protected bytes remain a compile-time mismatch; wrong crash
  edges and checksum-damaged frames deny before replay authority.

### C51-F041 — Publication fixtures search recovery digests for successor authority

- status: `CORRECTED` — deterministic successor ownership and real publication
  receipts replace every digest-search loop; final fixture and owner suites pass.
- severity: P1 fixture authority, architecture, and test cost
- affected guarantees: `C51-15-05`, `C51-15-06`, `C51-16-06`
- evidence: publication, compaction-mutation, compaction-observation, and
  physical-isolation fixtures repeatedly executed recovery with varying text
  digests until hash-derived root and manifest epochs happened to compare as a
  successor. Search bounds reached 4,096, 512, and 256 attempts. A later root
  therefore came from unrelated recovery meaning rather than publication
  succession, and two warm layout owner tests took 17.4 seconds.
- required correction: give production publication planning a narrow
  deterministic successor owner, derive current read-stability authority only
  from a real publication receipt, and make hostile multi-generation fixtures
  execute the intervening publication. Remove every digest-search loop rather
  than lowering its bound or caching fabricated authority.
- closing proof: `PublicationRootSuccessorOwner` advances root and manifest
  epochs exactly once while retaining Store identity and binding the requested
  physical generation; post-publication authority consumes
  `PhysicalPublicationReceipt`; stale-source, rollback, crash-recovery, and
  compaction fixtures use real predecessor receipts; no successor-search loop
  remains under `workspaces/worth-store/crates`; physical isolation passes
  24/24, test-support 21/21, layout-indexes 200/200, operations 164/164,
  certification library 402/402 active tests, and physical-isolation
  integration 113/113. The warm owner pair falls to 9.8 seconds.

### C51-F042 — Reopened checkpoint tests assert a pre-checkpoint WAL range

- status: `CORRECTED` — reopen assertions use the exact persisted `10..20`
  checkpoint range and preserve the contiguous WAL-tail join.
- severity: P2 evidence oracle
- affected guarantees: `C51-15-05`, `C51-15-06`
- evidence: the persisted recovery materialization declares checkpoint
  coverage `10..20`, reopen reconstructs that exact range, but two test-support
  assertions expected `1..20`. The focused suite failed even though production
  preserved the durable basis.
- required correction: keep production reconstruction bound to the decoded
  checkpoint manifest and correct the tests to assert its exact half-open
  range. Do not widen the recovered range to satisfy a stale fixture oracle.
- closing proof: both reopened-artifact and bounded-closeout assertions require
  `10..20`; the checkpoint-plus-WAL-tail join remains contiguous at LSN 20;
  test-support passes 21/21.

### C51-F043 — Dirty composition exceeds the non-exempt Rust file cap

- status: `CORRECTED` — the mixed files were split by responsibility; the final
  311-file dirty Rust inventory has zero non-allowlisted over-cap files.
- severity: P1 composition gate
- affected guarantees: `C51-15-06`
- evidence: whole-dirty-tree scrutiny found
  `evolution/migration/test_support.rs` at 419 lines and
  `store_json_residue_inventory.rs` at 410 lines, neither allowlisted. The
  first mixed physical publication fixture ownership with layout evolution
  setup; the second combined inventory ownership with the complete
  classification decision table.
- required correction: move migration publication construction into the
  existing `test_support/publication` responsibility and move residue
  classification into an inventory-owned child module. Do not add line-cap
  exemptions for decomposable code.
- closing proof: the four resulting files are 376, 47, 48, and 370 lines;
  affected all-target/all-feature crates compile; the complete 287-file dirty
  Rust inventory reports zero files over 400 lines.

### C51-F044 — Timing mutants were rejected by fixture shape before their named budget

- status: `CORRECTED`
- severity: P1 evidence integrity
- affected guarantees: `C51-16-03`, `C51-16-04`, `C51-16-06`
- evidence: Courtroom B and C stage-budget tests constructed complete timing
  fixtures containing `report-encoding`, then called the pre-report runtime
  validator. Every hostile case therefore failed first because the timing
  schema contained an unexpected phase, regardless of the oversized stage.
  Courtroom B also recorded five case-verification phases without applying the
  specified per-stage bound.
- required correction: construct separate exact pre-report and complete timing
  fixtures, prove both clean fixtures validate, assert each hostile delta is
  rejected for its exact phase label, apply the five-second bound to every
  B child stage and case verification, and keep only the 25 child stages in
  the separate 15-second aggregate.
- closing proof: all enforced B/C campaign, binding, child, verification,
  encoding, and aggregate budgets reject independently for their named cause;
  each pre-publication rejection invalidates stale success and removes pending
  state, each post-publication total rejection removes the unaccepted report,
  the 51 focused courtroom-runner tests pass, and strict runner Clippy passes
  with warnings denied.

### C51-F045 — Courtroom A projected a stale, partial mutant denominator

- status: `CORRECTED`
- severity: P1 evidence integrity
- affected guarantees: `C51-16-02`, `C51-16-05`
- evidence: Courtroom A expected the displaced mutant-34 branch-writer
  predicate, selected mutant 43 through a deleted unit-test path, and validated
  all 29 observations while publishing only IDs 15–41.
- required correction: bind the live predicates/selectors, derive the
  denominator from IDs 15–43, and project every validated localization.
- closing proof: focused shape tests reject omission or substitution and final
  Courtroom A carries exactly 29 killed mutants, IDs 15–43.

### C51-F046 — Mutation evidence did not bind the complete source closure

- status: `CORRECTED`
- severity: P1 evidence provenance
- affected guarantees: `C51-16-02` through `C51-16-05`
- evidence: the v1 report bound each mutation target and retained binary but
  remained machine-acceptable after unrelated compiled test, tool, fixture, or
  build-configuration source changed.
- required correction: bind the complete local package/test/tool/configuration
  closure before execution and immediately before publication, and make every
  courtroom recompute current source before consuming the report.
- closing proof: v2 source binding includes the complete local workspace and
  build inputs; same-length integration/configuration mutations change the
  digest; final report source closure is `5a713be6…d8f205`.

### C51-F047 — Legacy mutation schema failed for the wrong decoding cause

- status: `CORRECTED`
- severity: P2 evidence classification
- affected guarantees: `C51-16-02`, `C51-16-05`
- evidence: a v1 report entered v2 body decoding and failed for a missing
  `source` field before the verifier classified its unsupported schema.
- required correction: decode only the schema header first, reject unsupported
  versions, and decode the v2 body only after version admission.
- closing proof: runner and independent Courtroom A tests classify v1 as
  unsupported before body decoding; the retained v1 adversary published no
  Courtroom A success artifact.

### C51-F048 — Courtroom timing phases conflated source and executable work

- status: `CORRECTED`
- severity: P1 evidence and performance attribution
- affected guarantees: `C51-16-03`, `C51-16-04`, `C51-16-06`
- evidence: Courtroom C rejected `executable-verification` at 1,016 ms because
  that stage silently re-hashed the complete source closure. Post-build binary
  binding also included a whole-source rebind.
- required correction: record post-build source binding, final source binding,
  and executable re-verification as distinct obligations with 2,000 ms,
  2,000 ms, and 1,000 ms budgets, all retained inside the 30-second aggregate.
- closing proof: B reports 43 unique phases and C reports 16; final post-build
  source/final-source/executable timings are respectively 229/216/200 ms for B
  and 229/227/203 ms for C, with independent hostile rejection tests.

### C51-F049 — Courtroom A child evidence protocols lacked JSON classification

- status: `CORRECTED`
- severity: P1 boundary evidence
- affected guarantees: `C51-15-04`, `C51-16-02`, `C51-X-01`
- evidence: complete certification failed six tests because the new independent
  Cargo-source verifier imported `Deserialize` outside the exact terminal/tool
  protocol classification.
- required correction: classify the parent report decoder and its two child
  modules as three exact certification-tool protocol homes, leave only report
  output as terminal projection, and reject nearby unlisted paths.
- closing proof: all 14 residue-inventory tests pass, the hostile near-miss is
  denied, and complete certification passes 403/403 active tests.

### C51-F050 — Phase 15 manifests retained three unused dependency edges

- status: `CORRECTED`
- severity: P2 composition and graph hygiene
- affected guarantees: `C51-15-01`, `C51-15-05`, `C51-15-06`
- evidence: a complete 41-package/447-edge audit found unused maintenance edges
  to scheduler and retention plus an unused optional test-support readiness
  edge that widened `physical-isolation-fixtures`.
- required correction: remove those three edges and recompile every affected
  ordinary and isolated feature product.
- closing proof: maintenance plus physical-isolation/layout/certification
  fixture products compile; the five changed manifests contain 96 direct
  dependencies and zero remaining text-absent candidates.

## Discarded Evidence

- A combined four-file Phase 16 source read exceeded the tool output budget and
  was discarded in full; every file was reread independently before inference.
- The first Phase 16 Cargo probe omitted
  `certification-test-authority` and executed no test; it was discarded.
- One backend exact filter matched zero tests; the fully qualified test was
  discovered with `--list` and then executed once successfully.
- One structured full-suite attempt omitted `RUSTC_BOOTSTRAP=1`, produced an
  empty event stream, and was discarded. Both valid full-suite runs contain
  exactly 209 terminal records and a terminal suite `ok`.
- Shell timeouts left only their verified Phase 16 Cargo/test process trees
  running. Those exact PIDs were inspected by command line and terminated
  before subsequent timing or build evidence was collected.
- The first mutant-16 run failed through missing secure-I/O preservation after
  the durability guard was removed. It was a wrong-reason kill and was
  discarded; the fixture was repaired so durability mismatch is the sole
  hostile delta before rerunning the mutant.
- The first final-source 212-test integration run rejected the stale Courtroom A
  source manifest after `mutant_report.rs` changed. It was discarded as closure
  evidence; the final file was hashed independently, the exact manifest row was
  refreshed, and the focused courtroom plus complete suite were rerun.
- One oversized closure-ledger read exceeded the tool output budget and was
  discarded in full. Every inference used narrow UTF-8 reads anchored by stable
  ledger identifiers.
- One combined remaining-candidate code-quality read exceeded the tool output
  budget and was discarded in full. Every production and test candidate used
  for the final composition decision was then reread independently.
- One combined final-check call returned zero exit codes but exceeded the
  output budget with known checkout EOL warnings. It was discarded rather than
  treated as closure evidence; formatting, whitespace, and ledger verification
  were rerun independently with bounded output.
- The first complete mutation console transcript and one full Courtroom B
  timing-object dump exceeded output limits. Both were discarded in full;
  retained reports, logs, source hashes, binary hashes, and bounded timing
  projections were validated independently.
- Every mutation and courtroom report generated before the final timing,
  residue-classification, and manifest corrections was invalidated as stale.
  The complete mutation campaign and A/B/C chain were regenerated after source
  freeze.
- One combined runner/physical-suite command exceeded the tool timeout without
  terminal summaries. No child process survived; runner 123/123 and physical
  journeys 214/214 were rerun separately to terminal completion.
- The first unused-dependency audit produced an invalid zero result because
  Windows PowerShell rejected case-distinct external Cargo feature keys. A
  `--no-deps` workspace/direct-edge audit replaced it.
- The first PowerShell line-cap emulation failed to load the allowlist and
  produced truncated diagnostics. It was discarded entirely; the corrected
  canonical emulation reports 114 unrelated global violations and zero across
  all 311 dirty Rust files.

## Scoped Repository Caveats

- `scripts/ci/check_workspace_rust_line_caps.sh` is red on 114 pre-existing
  unallowlisted files across the root, Signal, Query, and legacy Store
  workspaces. All 311 dirty Rust files are independently verified with zero
  non-allowlisted files over 400 lines.
- This Windows checkout has `core.autocrlf=true` without explicit Rust-file EOL
  attributes, so Git reports that Rustfmt's LF output will become CRLF when Git
  next rewrites the files. `git diff --check` passes and the final semantic
  diff contains no line-ending churn.

## Risk Map

- authority and security: deep review for raw backend, Signal, recovery, and
  branch-writer authority leakage
- architecture and lifecycle: deep review for duplicate registries, effect
  routes, shutdown ownership, and C.6-local runtime state
- failure and recovery: deep review for partial effects, crash/reopen, retained
  obligations, and stale generation handling
- test and fixture honesty: deep review for ordinary/certification graph
  separation, causal fixture authority, independent oracles, and mutant
  sensitivity
- performance and resource behavior: deep review for bounded residency,
  command/locality capacity, compiler-session topology, and courtroom timing
- composition and DX: targeted review of changed C.5.1 files, facade exports,
  responsibility placement, and final handoff usability
