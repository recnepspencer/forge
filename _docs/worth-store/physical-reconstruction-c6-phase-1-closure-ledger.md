# C.6 Phase 1 Closure Ledger

This ledger is the closure authority for C.6 Phase 1. A green command is
candidate evidence until its production boundary, oracle, failure cause, and
mutation sensitivity have been inspected.

## Guarantees

| ID | Closure claim | Required evidence | Status |
| --- | --- | --- | --- |
| C6-P1-G01 | Ordinary record reads and publication retain one explicit path through Store residency, canonical work admission, Signal readiness, scheduler admission, executor dispatch, backend effects, and Store settlement. | Source-resolved ordered trace; exact constructor ownership; focused gate; existing canonical read/publication journeys. | `PROVED` — 21 ordered read/publication source anchors resolve to current production files; exact constructor gates pass; the complete 214-test physical-record journey suite passes. |
| C6-P1-G02 | Bootstrap is the only serving-adjacent owner permitted to construct the direct frame source; ordinary serving cannot bypass residency or canonical physical execution. | Complete production constructor inventory; route-arm inspection; serving-bypass and foreign-constructor mutants. | `PROVED` — the durable three-step bootstrap lane resolves; all three direct-source constructions remain in the bootstrap route; serving-arm and foreign-constructor mutants are rejected. |
| C6-P1-G03 | Every temporary C.6 handoff or identifier consumer and every named legacy S.2 authority or feature consumer has one exact path entry, replacement owner, deletion phase, and mechanical absence gate. | Live Rust/manifest discovery compared with the checked-in removal ledger; unclassified-consumer mutant; ledger schema checks. | `PROVED` — live bounded discovery exactly matches 108 removal rows; every row carries Phase 3, 5, 7, or 8 ownership and the source/metadata absence gate; an unclassified consumer is rejected. |
| C6-P1-G04 | Production contains one residency-pool constructor and one inherited physical work, scheduler-admission, and Signal construction authority; residency composition contains no local work runtime. | Complete production source scan; exact constructor-site assertions; local thread, channel, pending-map, timer, retry, callback, and queue gates; controlled mutants. | `PROVED` — exact source inventory finds one pool, work-runtime, scheduler-owner, and Signal-runtime constructor at their declared owners; consequential pool, pending-map, channel, and thread mutants are rejected. |
| C6-P1-G05 | The buffer pool has no direct Signal, proof, Foundational, or aspect-native dependency, import, or API authority, while ordinary products cannot activate a legacy feature edge. | Cargo metadata, manifest-table inspection, complete buffer-pool Rust scan, direct-dependency and legacy-edge mutants. | `PROVED` — current Cargo metadata and every buffer-pool Rust source pass; all four direct-authority dependency mutants and an active legacy-edge mutant are rejected; inherited ordinary feature-tree gates pass. |
| C6-P1-G06 | Phase 1 preserves the C.5.1 ordinary read/write, cancellation, settlement, health, and shutdown guarantees before residency authority moves. | Existing C.5.1 sealing gate, Store owner and smoke products, relevant physical-record journeys, compiler and repository boundary checks. | `PROVED` — Store owner 36/36, smoke 8/8, physical-record journeys 214/214, UI 27/27, and consolidated runner 142/142 pass; boundary-check and agent-context pass. |
| C6-P1-G07 | Phase 1 adds only durable boundary evidence and deletes or migrates no still-live predecessor; no redundant fixture or copied authority is introduced. | Dirty-worktree review, inventory comparison, source topology review, test proof-economy review. | `PROVED` — no production authority or legacy consumer was deleted or copied; the work adds one consolidated runner test family and three durable evidence documents, with no new integration target or fixture world. |
| C6-P1-G08 | The gate family has responsibility-named topology, honest failure localization, bounded compile cost, and no file-cap or function-composition violation. | Complete changed-file structural review, line-cap gate, dirty-function scrutiny, formatting and strict lint. | `PROVED` — warnings-denied all-target/all-feature Clippy, formatting, and diff checks pass; all nine dirty Rust files are at most 235 lines; dirty-function scrutiny found only the existing 65-line runner dispatch orchestrator, which remains a single table-of-contents responsibility. |

## Risk Map

- Authority and dependency direction are primary: a second pool, runtime,
  direct-media constructor, or direct foreign-authority dependency invalidates
  Phase 1.
- Evidence honesty is primary: source anchors without exact ownership and
  hostile mutants would be documentary theatre.
- Cleanup and lifecycle are boundary risks: every temporary or legacy consumer
  must remain visibly open until its assigned deletion phase rather than
  surviving as an unclassified compatibility path.
- Runtime behavior is inherited rather than changed in Phase 1, so C.5.1
  preservation evidence is required but no new performance claim is made.
- Public DX is unchanged. The developer-facing surface is the focused,
  responsibility-named gate family inside the existing consolidated runner
  target.

## Finding History

### C6-P1-F001 — Initial inventory discovery traversed build artifacts

- status: `CORRECTED`
- affected guarantees: C6-P1-G03, C6-P1-G08
- evidence: unrestricted recursive discovery timed out in `target/`.
- correction: repository inventory discovery now prunes `target/` and `.git`
  and admits only Rust sources and Cargo manifests.
- closing proof: the live source inventory reconciles exactly to 108 ledger
  rows and the independent dirty Rust inventory passes the 400-line cap.

### C6-P1-F002 — First focused run exposed three gate defects

- status: `CORRECTED`
- affected guarantees: C6-P1-G03, C6-P1-G04, C6-P1-G08
- evidence: the pending-map mutant localized as generic `OnceLock`; the trace
  parser accepted an arbitrary first line as its header; one preparatory-shell
  row did not match the case-sensitive Rust authority definition.
- correction: identity-keyed pending maps receive their own predicate before
  generic global ownership; the trace requires its exact schema header; the
  Rust discovery gate alone decides ledger membership.
- closing proof: all 17 focused gates pass after correction.

### C6-P1-F003 — The Store smoke product had a stale exact selector

- status: `CORRECTED`
- affected guarantees: C6-P1-G06, C6-P1-G08
- evidence: the runner required three exact Store smoke tests but resolved only
  two; the bootstrap test exists as
  `baseline_admission::empty_bootstrap_create_and_reopen_converge`.
- correction: the smoke registration now uses the exact current module-qualified
  identity rather than the stale flat name.
- closing proof: catalog and plan tests pass; the smoke product executes exactly
  three Store and five certification tests successfully.

### C6-P1-F004 — Direct-media ownership was absent from the durable trace

- status: `CORRECTED`
- affected guarantees: C6-P1-G01, C6-P1-G02
- evidence: source gates enforced the bootstrap-only constructor but the
  checked-in boundary inventory contained only ordinary read and publication
  lanes.
- correction: add the ordered bootstrap route, direct-source construction, and
  backend-read lane to the same source-resolved inventory.
- closing proof: all 24 trace rows resolve and the complete 17-test focused
  boundary family passes.

### C6-P1-F005 — Strict lint rejected manual iterator repetition

- status: `CORRECTED`
- affected guarantee: C6-P1-G08
- evidence: warnings-denied Clippy rejected `repeat().take()` in the direct
  media constructor inventory.
- correction: use the toolchain's `repeat_n` construction.
- closing proof: warnings-denied all-target/all-feature runner Clippy passes.

### C6-P1-F006 — The first UI run exceeded its shell window

- status: `RESOLVED`
- affected guarantee: C6-P1-G06
- evidence: the first 120-second shell window ended while the exact runner,
  nextest, and trybuild process tree was still compiling.
- resolution: the process tree was inspected until completion; no duplicate
  compiler was started while it remained active; one warm replacement run
  obtained the terminal verdict.
- closing proof: the UI product passes all 27 compile-time authority tests in
  131.31 seconds.

### C6-P1-F007 — The trace validator name omitted its bootstrap responsibility

- status: `CORRECTED`
- affected guarantee: C6-P1-G08
- evidence: the validator covered all three trace lanes after F004, but its
  test name still named only read and publication.
- correction: name read, publication, and bootstrap explicitly.
- closing proof: the final 17-test focused gate passes with the corrected name;
  formatting and diff checks remain clean.

## Environment Note

The Bash line-cap wrapper is unavailable in this Windows environment. The
equivalent hard gate was executed against every dirty Rust path using the
repository allowlist and an independent line count; all nine paths are below
400 lines. The runner's structural listing confirms that the unavailable Bash
wrapper owns only that same line-cap command. Boundary-check and agent-context
were executed directly and passed.
