# Milestone 9.16.1 Canonical Graph Progression Closure Ledger

**Owner:** Milestone 9.16.1
**Canonical specification:** `milestone-9.16.1.md`
**Status:** Open — the rejected Phase 1 and 2 implementation was removed and
the 2026-08-03 safe pre-implementation checkpoint restores the inherited Query,
Bank, Runtime Bridge, Relational, and Worth UI feature paths. No 9.16.1 phase
is claimed implemented. Work may continue from the corrected specification's
parity-gated semantic-surface migration model rather than crate-wide monolith
deletion.
**Historical policy:** Milestones 9.9, 9.10, 9.11, and 9.15 and completed
Milestone 9.16 rows retain their recorded status. This ledger adds stronger
guarantees; it does not reopen, revoke, or rewrite historical status or
closure.

A requirement is `PROVED` only when its production owner, public consumer
evidence, adversarial evidence, lifecycle and performance posture, and residue
posture agree. A finding is `CLOSED` only when the root cause and every current
dependent guarantee have been checked through the destination path.

## Requirement Ledger

| ID | Phase | Guarantee | Status | Required closure evidence |
|---|---:|---|---|---|
| C1 | 1 | Every installed query and operation owns one sealed graph-obligation set with exhaustive kind, owner, selection, graph-read requirement, resource, effect, and terminal-evidence meaning. | **DEFECT** | Installation authority tests, exhaustive owner matrix, schema/generation substitution denials, entry/byte budgets, no public constructor, and exact-zero warm digest-text materialization. Current residue and owner-lane evidence are red. |
| C2 | 1 | Relational owns graph truth and exact mechanics; Query owns Query-to-neutral translation, portable comparison, and Query authority continuity; Runtime Bridge owns installed neutral correspondence/crossing/lowering; Signal owns policy evidence; and Query owns legal composition and progression. | **PROVED** | Owner-conformance tests, foreign-evidence hostility, dependency checks including the Runtime-Bridge-to-Query prefix denial, and no recreation of lower decisions outside their owners. |
| C3 | 2 | There is one installed selection path and selection carries no execution authority. | **OPEN** | Focused destination evidence is green, but the hostile owner matrix and integrated predecessor-to-destination cutover must agree before proof is restored. |
| C4 | 2 | Every covered read and mutation uses one requirement, inventory, structural-cost, budget, capacity-reservation, and sealed-plan admission chain without losing predecessor feature behavior. | **OPEN** | Real Bank read/mutation transcripts, monolith and Worth UI parity, non-authority inspection, forged input/review denial, and exact admission counters. |
| C5 | 2 | One managed provider session begins before every decision-bearing graph read and binds runtime, generation, query/operation, principal/context, typed branch, branch-qualified basis, provider, run, and reservation. | **OPEN** | Focused session evidence remains green; proof requires repaired warm identity carriage and integrated monolith, public-journey, UI, and Bank evidence. |
| C6 | 3 | Current, continuation, historical, preview, and live application-query lanes consume one canonical graph-read planning authority. | **OPEN** | Public consumer evidence for every lane, plan-identity equality, direct-executor denial, and zero parallel-plan residue. |
| C7 | 3 | Read-only execution terminates from session-owned read products and cannot construct proposed-state, invariant, or commit authority. | **OPEN** | Compile denial, runtime terminal matrix, cancellation/failure cleanup, and terminal read receipt. |
| C8 | 4 | Capability and ordinary authorization retain exact Relational, Bridge, and Signal evidence inside the same provider session and complete decision read-set. | **OPEN** | Principal, purpose, request, time, exact-grant/path, negative-fact, equivalent-grant, and policy substitution hostility through the public facade. |
| C9 | 4 | Mutation continues from the common session through proposed state, actual installed invariant execution, and provider compare-and-commit. | **OPEN** | Relevant/unrelated drift twins, invariant mutation probes, proposal/session substitution denial, commit conflict, response-loss, and retry evidence. |
| C10 | 2-4 | Success, denial, conflict, cancellation, failure, and indeterminate terminals release reservations, sessions, bases, buffers, proposals, and continuations exactly once. | **OPEN** | Exact baseline and non-wrapping lifecycle counters at every terminal boundary. |
| C11 | 5 | Obligation outcomes, receipts, envelopes, and inspection views are derived only from actual terminal evidence and grant no execution authority. | **OPEN** | Selected-versus-executed mutation probe, public construction denials, causal identity checks, and publication dependency direction. |
| C12 | 5 | Destination packages own each covered authority after parity-gated atomic cutover; no destination package imports the monolith, no retired predecessor remains authoritative, and unrelated monolith behavior remains intact. | **OPEN** | Per-surface migration rows, parity proof, cutover receipts, boundary checks, Cargo dependency inspection, visibility checks, exact residue, and full downstream feature evidence. |
| C13 | 1-5 | Ordinary warm work scales only with selected obligations and admitted semantic delta and performs zero canonicalization or SHA work. | **OPEN** | Phase-separated counters; independent scale twins for unrelated obligations, grants, graph population, result rows, and live consumers; exact-zero warm canonical/SHA evidence. |
| C14 | 5 | Public documentation and examples describe and compile against the single real facade, distinguish one-way integration from competing authority, and name every parity-gated migration. | **OPEN** | AI README, canonical migration table, named feature-doc audit, and executable examples. |
| C15 | 5 | Milestone 9.16 resumes at Runtime Phase 7.3 by consuming the existing Phase 6 feature contracts and the new session authority without a parallel selector, planner, authorization lane, or receipt model. | **OPEN** | Phase 6 and 7.2 consumer evidence through the new path, 7.3 prerequisite audit, predecessor-guarantee reconciliation, and explicit handoff record. |
| C.L | all | The ledger covers every authority, lifecycle, owner, public boundary, semantic-surface migration, exact retirement target, downstream parity, and performance claim without a Cartesian test matrix. | **DEFECT** | The earlier ledger allowed local green evidence to hide broad consumer failures. Closure requires a skeptical audit mapping each proof to a plausible defect and independent oracle. |

### Historical Phase 1 Evidence Snapshot

This section records what the original implementation pass reported. It is not
current closure evidence after the 2026-08-02 audit and does not override the
requirement statuses above.

- `worth-query-installation` owns the sealed installed vocabulary, typed set and
  row identities, fixed kind index, query/operation bindings, exact owner
  progressions, and installation canonical-work evidence. Public consumers can
  inspect but cannot construct or mutate the set.
- Installation evidence covers all five semantic kinds; the distinct
  principal, ability, and capability authorization routes; complete selection,
  resource, effect, and terminal row semantics; read-only absence; unsupported
  invariant ownership; entry and encoded-byte overflow; and exact query and
  operation runtime/generation affinity.
- `cargo test -p worth-query-installation --quiet` passes 155 unit tests and 4
  compile-fail documentation tests. `cargo clippy -p
  worth-query-installation --all-targets --no-deps -- -D warnings` passes.
- `cargo test -p worth-query-execution --lib --quiet -- --test-threads=8`
  passes 404 tests. The governed-live fixture uses its own exactly installed
  operation, while the hostile `TouchAccount` fixture retains no undeclared
  label authority.
- `cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml -p
  bank-domain --test estate_capability_installation --quiet` passes 11 real
  consumer tests without granting execution authority from installation alone.
- The warm-path source guard proves no query/operation obligation
  reconstruction helper and no direct SHA implementation in the installed
  obligation family. Kind lookup reports one index probe and exact-zero
  canonical work.
- `scripts/quality/scrutinize_rust_functions.py --dirty .` scans 86 dirty Rust
  files with zero scan errors; every dirty Rust file is at or below 400 lines.
  Advisory candidates were inspected against semantic responsibility. The
  wide application-query admission boundary remains Phase 2 work rather than a
  Phase 1 helper split.
- `boundary-check --root .` reports valid Road 1 topology, and `agent-context
  check` passes. Query's local warning-denied Clippy gates pass. A broad
  dependency-lint invocation remains blocked by unrelated dirty
  `worth-relational` Clippy findings from the paused convergence surface; this
  is not counted as Phase 1 evidence and is not silently reported green.

### Historical Phase 2 Evidence Snapshot

This section likewise preserves the reported proof snapshot without treating
it as current integrated closure.

- Installation-owned obligation selection progresses linearly through one
  required set, executable review, live resource reservation, sealed admitted
  plan, and managed graph-work session. Selection and review remain
  non-authoritative; public consumers cannot construct a proof-bearing
  transition.
- Query and operation admission open the managed provider session before
  principal, ability, capability, disclosure, or other decision-bearing graph
  observation. Retained decision facts and terminal receipts carry the exact
  session identity and typed branch supplied by the installed runtime basis.
- Every privileged mutation revalidation requires the concrete managed
  mutation session. It derives the current Relational execution basis from
  that session's typed branch, rejects facts or commit bases from another
  session or branch, and releases the temporary basis exactly once. No
  idempotency or commit path opens an implicit/default snapshot.
- Real Bank reads and mutations traverse the public host facade and retain the
  selected obligation, admitted plan, provider session, typed branch, and
  exact release evidence. Unsupported owners and forged construction fail
  before effects, and cancellation/denial paths return reservations and bases
  exactly once.
- `cargo test --manifest-path workspaces/worth-query/Cargo.toml -p
  worth-query-admission -q -- --test-threads=1` passes 101 unit tests and 8
  documentation tests. Strict all-target Clippy passes for the admission
  package.
- `cargo test --manifest-path workspaces/worth-query/Cargo.toml -p
  worth-query-execution -q -- --test-threads=1` passes 423 unit tests and 13
  documentation tests. The five-test commit authorization lifecycle module
  includes ordinary and capability revalidation on non-default branches plus
  a hostile cross-session/cross-branch substitution denial.
- `cargo test --manifest-path workspaces/worth-query-bank-world/Cargo.toml -q
  -- --test-threads=1` passes the full Bank workspace suite with one explicitly
  ignored high-operation probe. Its frozen schema inventory includes the
  intentionally installed, capability-governed `estate_customer_identity`
  query.
- Strict all-target Clippy passes for `worth-query-execution` and the entire
  Bank workspace. Root, Query, and Bank formatting checks pass. `boundary-check
  --root .` and `agent-context check` both pass.
- Residue scans report zero implicit authorization-revalidation snapshots,
  zero obsolete capability-refresh calls, and zero literal `"main"` authority
  defaults in the canonical production roots. All 270 dirty Rust files satisfy
  the 400-line rule after session affinity was rehomed to its own semantic
  module.
- The repository-wide line-cap guard remains independently red on 111 tracked,
  unallowlisted baseline files outside this slice. No exemption was added and
  the failure is not reported as green; Milestone 9.16.1 closure remains open
  while the wider repository debt exists.

### 2026-08-02 Reopen Audit

- The Query and Worth UI workspaces compile, so the package graph and public
  type graph remain connected.
- Runtime Bridge passes 939 tests and its dependency tree contains no
  `worth-query` crate. C2 remains proved.
- `worth-query-admission` passes 101 unit tests and 8 documentation tests;
  `worth-query-execution` passes 423 unit tests and 13 documentation tests.
  These are retained candidate evidence for C3-C5, not integrated closure.
- `worth-query-installation` passes 154 of 155 unit tests. Its warm-path
  residue guard detects `.render_hex()` in graph-work session identity
  construction, reopening C1 and C13.
- The graph-obligation hostile journey passes 14 of 15 tests. The owner matrix
  contains at least one obligation kind with no real supported lane,
  reopening the exhaustive-owner claim.
- `worth-query --lib` passes 2,450 tests and fails 262. The installed operating
  world passes 167 and fails 153; public declarative journeys pass 25 and fail
  12. Most failures panic where legacy string identities are assumed to be
  canonical SHA-256 rather than being migrated at their owning cold seam.
- Worth UI Query binding passes 38 tests and fails 40 at the same Query identity
  boundary. This is downstream parity failure, not forty independent UI
  migration rows.
- The Bank workspace is functionally green except for one canonical-work scale
  twin whose encoded and SHA input byte counts differ by one. The performance
  proof remains open.
- `boundary-check` and `agent-context check` pass. Mechanical topology cannot
  substitute for the red functional and performance evidence above.

### 2026-08-03 Safe Pre-Implementation Checkpoint

- The rejected Query and Bank graph-obligation/session implementation from
  `f7536ab14` is reversed. Its valid Relational branch-qualified snapshot and
  execution-basis work is retained. The corrected roadmap, Milestones 9.13.2,
  9.16, 9.16.1, and 9.17, and both Query closure ledgers are retained. This
  checkpoint contains no 9.16.1 graph-obligation authority migration and makes
  no Phase 1 or 2 closure claim.
- Relational now keeps branch identity on published snapshot handles and
  execution-basis admission without turning publication into physical
  retention authority. Published handles establish branch-qualified
  eligibility and the logical history reconstruction fence; cache protection,
  retained-state admission, and record retention remain separate authorities.
  Internal historical reads reconstruct transiently, while a physically
  pruned publication cannot mint an execution-basis lease. The full Relational
  library suite is green with 989 passed and 25 intentionally ignored tests.
- Runtime Bridge remains Query-independent. Its dependency tree contains no
  `worth-query` crate, its suite is green, and the lower-runtime inventory
  explicitly classifies the Query-side async live declaration as a one-way
  boundary adapter rather than Bridge-owned Query behavior.
- The disconnected Phase 6 graph-read handoff now carries typed canonical
  digest identities from cold construction through access requirements,
  operation resolution, selectivity, and plan review. Warm parsing and
  per-row dotted-path reconstruction were removed from the restored path.
- The graph-read budget distinguishes the ordinary intermediate-set ceiling
  from the broad-traversal semantic threshold. Mutation authority-ceiling and
  decision-fact tests now exercise the intentionally widened Phase 7
  `TouchAccountOperation` contract without weakening the compile-wide versus
  install-narrow hostility proof.
- `cargo test --workspace` in `workspaces/worth-query` is green, including
  2,712 monolith tests, 404 execution tests, public journeys, compile-fail
  certification, and documentation tests. The full Bank workspace and the
  Worth UI Query-binding package are green; one Bank high-operation probe
  remains explicitly ignored by its existing contract.
- `boundary-check --root .`, `agent-context check`, formatting, and dirty-file
  composition scrutiny are green. The repository-wide line-cap guard remains
  red on 112 unrelated tracked baseline files, with zero failures among dirty
  files. The touched monolith `runtime.rs` remains covered by its pre-existing
  explicit allowlist entry and changes only by one extracted-module
  declaration; no exemption was added. This is not counted as milestone
  closure evidence.
- The broader Worth UI certification workspace still has 24 topology-inventory
  failures inherited from current `master`. The real Query-binding consumer
  lane is green, so this is recorded as a separate master-integration barrier,
  not hidden as Query parity success or broadened into 9.16.1 work.

## Finding Ledger

| ID | Impact | Finding | Status | Root correction |
|---|---|---|---|---|
| CQ1 | Critical | The monolith selection-backed executor can report blocking invariants or validators as executed without invoking their semantic owner. | **OPEN** | Route every selected executable obligation to its exact owner and mint completion only from owner terminal evidence; delete the fake executor. |
| CQ2 | Critical | Public execution-input, dispatch, planning-input, requirement-evidence, inventory, and review constructors allow callers to assemble authority-like products. | **OPEN** | Make proof transitions private and sealed; expose declarations and non-authoritative inspection only. |
| CQ3 | Critical | Manual invariant-pack entry points and a no-op composition path permit invariant execution outside installed provider progression. | **OPEN** | Delete callbacks/defaults and require installed invariant authority through the session. |
| CQ4 | Critical | Actual graph-obligation authority remains in the monolith rather than destination packages. | **OPEN** | Migrate each exact authority row with real-consumer parity, cut over atomically, and retire the covered predecessor; retire the obligation tree only after every contained authority row closes. Do not delete unrelated monolith behavior. |
| CQ5 | Critical | The graph-read access facade publicly exports construction and review surfaces that should belong to installation/admission transitions. | **OPEN** | Narrow construction and review to crate-private transitions; publish inspection views only. |
| CQ6 | Critical | The monolith admitted graph-read plan/executor and application-query plan/direct Relational executor form parallel execution products. | **OPEN** | Replace both authority roles with one destination-package plan and session-owned graph port. |
| CQ7 | Critical | Application-query admission does not consume one canonical installed obligation selection before graph-read planning. | **OPEN** | Focused code now consumes the Phase 1/2 spine, but monolith, public-journey, UI, and per-lane parity must prove that every real consumer reaches it before the predecessor authority retires. |
| CQ8 | Critical | Application-query and authorization observations can occur before the Milestone 9.15 provider session that later consumes their evidence. | **OPEN** | Focused session evidence is present, but integrated consumer and terminal-lifecycle evidence must prove the session precedes every real decision-bearing observation. |
| CQ9 | High | Application operations can synthesize graph-read, touch, or invariant scope beside the installed obligation grammar. | **OPEN** | Lower typed declaration meaning once at installation and retain typed bindings thereafter. |
| CQ10 | High | Earlier convergence documentation attempted to reopen closed milestones and ledger rows. | **CLOSED** | The uncommitted reopen/revoke edits were removed. Milestone 9.16.1 records the stronger guarantee append-only while historical statuses remain unchanged. |
| CQ11 | High | Historical consumer-kit/reference-consumer proof can demonstrate local authority construction rather than real public-facade adoption. | **OPEN** | Classify each consumer-kit surface: integrate non-authoritative inspection through the public facade, or migrate authority-capable adoption with parity and retire only that exact predecessor. |
| CQ12 | High | Existing Phase 7.2 lower-owner evidence is correct locally but Query composes it outside the new canonical obligation/session progression. | **OPEN** | Preserve the lower evidence and bind its observation, decision facts, and revalidation to the Phase 2/4 session path. |
| CQ13 | High | A broad green suite could conceal two paths, synthetic execution, or warm hashing if residue and structural counters are not independently enforced. | **OPEN** | Require path-count residue scans, mutation-sensitive owner execution, and phase-separated scale evidence. |
| CQ14 | Critical | The convergence path can infer `"main"` or bind only runtime/snapshot/version, allowing equal-version evidence from another future branch to satisfy session affinity and forcing global commit coordination into the authority contract. | **OPEN** | Phase 2 plan, session, decision facts, read terminals, mutation revalidation, commit authorization, and receipts are typed-branch bound with zero literal-default residue. Phase 4/5 must finish proposal, invariant, retry, and publication closure without making global serialization authoritative. |
| CQ15 | Critical | The Phase 6-to-admission bridge accepts legacy graph identities as strings and then panics while parsing them as canonical SHA-256, disconnecting monolith, public, and Worth UI feature paths. | **CLOSED** | Typed canonical digest identity is established at cold construction and carried through requirements, operation resolution, selectivity, and review without warm parsing. Query, Bank, and Worth UI Query-binding suites are restored before any 9.16.1 cutover. |
| CQ16 | Critical | Milestone 9.13.2's whole-monolith deletion requirement leaked into the earlier 9.16.1 specification and conflated removal of competing authority with deletion of unrelated feature surfaces. | **CLOSED** | Milestones 9.13.2, 9.16, and 9.16.1, the roadmap, and this ledger now define parity-gated semantic-surface migration, atomic authority cutover, exact predecessor retirement, and explicit scope amendment for broader discoveries. |
| CQ17 | High | Worth UI compiled while half of its Query-binding tests failed, allowing package-local proof to conceal downstream feature loss. | **CLOSED** | The real Worth UI Query-binding package is green at the safe checkpoint and remains a required parity gate for every affected read, continuation, live, projection, and receipt migration row. |
| CQ18 | High | Graph-work session identity construction materializes canonical digest text on the warm path. | **OPEN** | The rejected graph-work-session implementation carrying this defect was removed. The corrected 9.16.1 implementation must carry a typed fixed-width session identity through execution and format only at an explicit reporting boundary with separate cost evidence. |

## Semantic-Surface Migration And Retirement Ledger

One row is one authority surface. Before its cutover, the predecessor is the
sole executable authority. After cutover, the destination is the sole
authority and only the exact predecessor capability becomes residue.

| Surface | Inherited contract and parity gate | Cutover and retirement | Status |
|---|---|---|---|
| `worth-query/src/runtime/mutation/graph_composition/obligation/` | Preserve every covered 9.9 obligation kind, selector, owner route, denial, effect, receipt, preview/branch posture, and cost guarantee through real mutation consumers. | Cut over row by row to installation/admission/execution owners; retire the tree only after every contained authority row closes. | **OPEN** |
| Selection-backed obligation executors | Every executable kind reaches its real semantic owner and mutation probes fail when owner execution is skipped. | Atomically replace each covered executor with owner completion; retire synthetic completion for that kind. | **OPEN** |
| Manual invariant-pack callbacks and no-op default composition | Every supported invariant family produces the same lawful success/violation and consequential state through installed provider-session execution. | Switch the family and remove its callback/default authority in the same slice. | **OPEN** |
| Public proof-like graph-read planning constructors and executable review | Preserve 9.10 requirement, inventory, cost, budget, denial, and inspection meaning without caller-mintable authority. | Seal proof construction; retain read-only inspection if it has a real audience. | **OPEN** |
| Monolith admitted graph-read plan and executor | Current, continuation, historical, preview, and live lanes retain exact result, lifecycle, receipt, and warm-cost behavior. | Cut over one lane family at a time to the admitted plan and session-owned port; retire only that lane's predecessor authority. | **OPEN** |
| Application-query graph-read plan authority and raw Relational execution | Preserve Phase 6 identity, parameter, basis, ordering, projection, continuation, history, preview, live, and result contracts. | Keep result shaping in application query; move graph authority to the session and remove raw execution only after each lane proves parity. | **OPEN** |
| Pre-session authorization observation | Preserve Phase 7.1-7.2 principal, purpose, trusted-time, exact-grant/path, negative-fact, policy, and revalidation outcomes. | Bind each observation family to the managed session and remove only its pre-session entry after the Bank and hostile twins pass. | **OPEN** |
| Consumer Kit graph-obligation surfaces | Preserve 9.9/9.11 public adoption and inspection value while forbidding local authority construction. | Integrate lawful inspection through the public facade; migrate and retire only authority-capable registration or execution seams. | **OPEN** |
| Facade adapters and aliases | Preserve the ordinary caller workflow and source behavior; an adapter must lower one way into the destination and mint no authority. | Retain lawful one-way adapters; retire only aliases or wrappers that reopen a predecessor authority. | **OPEN** |
| Unrelated `worth-query` monolith responsibilities | Full Query, public declarative, Worth UI, and Bank suites prove no unsupported feature deletion or semantic drift. | No cutover or retirement is authorized by 9.16.1; broader findings require an explicit phase amendment or successor milestone. | **N/A** — preserve |

## Evidence Selection Policy

Evidence is selected from the causal ledger rather than multiplied across every
category and input combination:

1. one positive public transcript for each materially distinct terminal;
2. one one-axis negative for each independent authority or affinity dimension;
3. one interaction case for each composition rule that combines dimensions;
4. one mutation probe for each mechanism whose bypass could create false
   success;
5. one independent scale twin for each input axis that could widen warm work;
6. one lifecycle matrix shared across terminal outcomes where the ownership
   contract is identical; and
7. one migration inventory that names every predecessor, destination, covered
   consumer, parity proof, cutover, and exact authority residue.

Security, architecture, correctness, privacy, lifecycle, performance, DX, and
other risk categories are considered when they can invalidate a ledger row.
They do not create a fixed Cartesian test matrix.

## Closure Rule

Milestone 9.16.1 closes only when C1-C15 and C.L are `PROVED`, every high or
critical finding is `CLOSED`, every covered migration row has parity and an
atomic cutover or a justified integration result, every retired predecessor is
absent, and unrelated Query, public, Worth UI, and Bank behavior is green. The
hostile external consumer must demonstrate exactly one obligation and
graph-read planning authority path, while the public docs and permanent
enforcement agree with production topology. Closure adds a handoff to
Milestone 9.16 Runtime Phase 7.3; it changes no historical milestone status.
