# C.6 Phase 3A Fault/Join Closure Ledger

## Scope

This ledger closes the fault, hit, coalescence, failure-terminal, and refault
slice of C.6 Phase 3. It does **not** close Phase 3 as a whole. The
responsibility-named Store-private residency capability and the complete
hostile eviction proof/cleanup remain later Phase 3 work.

Final audited source freeze:

- base commit: `f617cdd8ee34e3dc5d8ff6ec65bf93aaedb60b73`
- tracked binary diff Git SHA-1: `1f13b6bb885422815508c90da3b5f38d84176c41`
- sorted untracked-path manifest Git SHA-1:
  `4d3e570701c1c8c6d0f032c73dbea1861a767c47`
- untracked path count: `66`

This freeze is historical. The complete reproducible final-source manifest and
all holistic reopenings are owned by
`physical-reconstruction-c6-phases-1-3-holistic-qa-ledger.md`; its
status/blob SHA-256 supersedes these partial tracked/untracked digests.

## Guarantee Ledger

| ID | Closure claim | Required evidence | Result |
| --- | --- | --- | --- |
| P3A-G01 | The frame table reserves one pool-incarnation-bound loading identity before exact frame source work can start. | Production trace from `access_frame` through `reserve_loading`; loading identity is opaque and non-constructible publicly. | **PROVED** |
| P3A-G02 | An identical in-flight fault has one move-owned source authority; later callers receive waiter authority with no load method. | `PhysicalFrameAccess::{Fault, Coalesced}` typestate; fault owner is non-`Clone`; compile-fail proofs for waiter load, owner clone, and identity construction. | **PROVED** |
| P3A-G03 | Store decides hit/fault/coalesced before preparing canonical exact or bounded-read work; only `Fault` can reach the canonical source. | Exhaustive `load_exact` and `load_bounded` matches; bounded decision-before-source boundary gate and controlled pre-source mutant; forced Store overlaps. | **PROVED after holistic reopening** |
| P3A-G04 | A resident exact or bounded hit executes no frame source read and carries no frame-source work trace. | Lower exact and bounded hot-access tests; exact C6 hot pin has zero work; the public composite oracle isolates its one inherited segment-completeness validation from zero bounded discovery/positioned work. | **PROVED after holistic reopening** |
| P3A-G05 | Forced identical exact or bounded overlap executes exactly one source load; the waiter creates no second media, physical-work, or Signal authority. | Deterministic lower barriers, public paused-then-failed bounded journey, shared terminal, and exact fault/coalesced/source/work deltas. | **PROVED after holistic reopening** |
| P3A-G06 | Every participant in one failed load observes the exact same typed terminal, retained failure cannot loop as another miss, and terminal publication wakes an already-sleeping participant. | Source-execution and pre-source hostile tests compare owner, waiter, and third-caller terminals before explicit refault; deterministic collision regression observes the waiter inside `Condvar::wait`; missing-notification mutant. | **PROVED after holistic reopening** |
| P3A-G07 | Owner abandonment, waiter drop, source panic, source rejection, failure, and close release exact loading, pin, resident-byte, and frame-entry posture. | Dedicated deterministic lifecycle/failure tests plus allocation-counter reconciliation suite. | **PROVED** |
| P3A-G08 | Refault is possible only after the prior loading identity is reconciled and travels through the canonical C.5.1 path in Store. | Lower failure/refault tests; C6 pressure eviction/refault journey with exact positioned-read delta and physical-work trace. | **PROVED** |
| P3A-G09 | Direct media remains bootstrap-only; serving reads use the canonical physical-work port and existing root/artifact/frame/scan `ReadFault` bases. | Route and source production trace; all-target compile; Store overlap and hot/refault journeys. | **PROVED** |
| P3A-G10 | Terminal-bearing owner failures and coalesced failures remain semantically distinct through Store's public C6 API. | Exhaustive internal/public failure enums; all-target all-feature compile with warnings denied. | **PROVED** |
| P3A-G11 | Documentation and physical topology describe the shipped API honestly and admit later semantic growth without a god file. | README frame-access contract; expected-growth `frame_access/`, `tests/frame_access/`, and `pool/bounded_frame_admission/`; all dirty C6 Rust files at or below 400 lines. | **PROVED after holistic reopening** |

No row is `OPEN`, `DEFECT`, or `BLOCKED` for this slice.

## Findings And Corrections

### Q3A-01 — stale public lower documentation

- Affected guarantees: P3A-G02, P3A-G03, P3A-G11.
- Defect: the buffer-pool README still described fault/eviction cutover as
  future work after the exhaustive access API shipped.
- Root correction: document the exact `Hit`/`Fault`/`Coalesced` contract,
  exclusive authority split, shared terminal, cleanup, and remaining Phase 3
  work.
- Closure evidence: README inspection plus all API/compiler evidence below.
- Status: **CLOSED**.

### Q3A-T01 — missing pre-source and retained-terminal proof

- Affected guarantees: P3A-G06, P3A-G07.
- Defect: tests covered source execution failure but did not prove that
  pre-source rejection records zero source loads or that a third caller sees a
  retained terminal before waiter reconciliation.
- Root correction: add a focused hostile pre-source test and strengthen the
  execution-failure test with a third-caller tombstone probe.
- Closure evidence: four-test `frame_access::failure` family and the complete
  127-test lower suite.
- Status: **CLOSED**.

### Q3A-C01 — frame-admission god file

- Affected guarantee: P3A-G11.
- Defect: dirty `pool/frame_admission.rs` reached 523 lines and mixed
  fault/loading lifecycle, frame-space eviction admission, and clean
  publication.
- Root correction: move to the expected-growth `pool/frame_admission/`
  directory; retain access/loading lifecycle in `mod.rs`; extract
  `frame_space.rs` and `clean_publication.rs`; keep frame-space visibility at
  the exact pool radius.
- Closure evidence: zero dirty Rust files above 400 lines, lower compile, full
  tests, Store journeys, and repository gates.
- Status: **CLOSED**.

## Final Evidence

- `cargo fmt --manifest-path workspaces/worth-store/Cargo.toml --all -- --check`
- `RUSTFLAGS="-D warnings" cargo test --manifest-path
  workspaces/worth-store/Cargo.toml -p worth-store-buffer-pool --all-features`
  - 127 unit tests passed
  - 37 compile-fail doctests passed
- `RUSTFLAGS="-D warnings" cargo test --manifest-path
  workspaces/worth-store/Cargo.toml -p worth-store --test
  physical_record_journeys --all-features
  physical_work::c6_residency_inheritance`
  - 2 integrated journeys passed
- `RUSTFLAGS="-D warnings" cargo check --manifest-path
  workspaces/worth-store/Cargo.toml --workspace --all-targets --all-features`
- `python scripts/quality/scrutinize_rust_functions.py --dirty .`
  - superseded by the holistic scan: 246 files scrutinized, 84 advisories
    inspected, and zero scan errors
- final dirty Rust line-cap inventory
  - zero dirty non-allowlisted files above 400 lines
  - the repository-global gate remains red on 114 unrelated baseline files
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
  - Road 1 Cargo topology valid
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
  - passed

## Phase Handoff

Phase 3A remains implementation-closed after holistic reopening. Phase 3 has
also landed, but the combined Phase 1-3 holistic audit is qualified/open on the
repository-global line-cap baseline. Phase 4 has not begun.
