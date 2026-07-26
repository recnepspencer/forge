# C.6 Phase 2 Allocation Authority Slice Closure Ledger

## Scope

This ledger closes only the proof-carrying operation-allocation slice of C.6
Phase 2. It does not close Phase 2 as a whole.

The governing authority is
`physical-reconstruction-c6-buffer-pool-runtime-join.md`, especially:

- the authority topology and adversarial constraint;
- the normative configuration and pressure APIs;
- the operation-allocation type progression;
- the scope law and admission-before-allocation law;
- the Phase 2 telos, proof, and cleanup requirements.

The audited WORTH Store source-state fingerprint, excluding this documentation
file, is:

`57c59a740fdbb071b281cf16321a5c85e5d312c4`

This is a composite hash of the binary tracked diff plus the sorted path and
Git blob hash of every untracked file under `workspaces/worth-store`. At this
closure it covers tracked diff
`79fb5c48127eb0fe8d2e13eca75a42bb3d0984f9` and 25 untracked files.

Unrelated dirty workspace changes were preserved. This ledger claims only the
allocation-authority slice described below.

## Risk Map

The causally relevant risks were:

1. a raw scope label or compatibility overload retaining allocation authority;
2. a grant from another pool incarnation opening a same-store admission path;
3. an adapter mutating counters or allocating before it validates grant
   ownership;
4. a read or candidate-publication session escaping the grant that authorizes
   its allocations;
5. grant release failing to reconcile per-scope and aggregate live bytes;
6. close hiding a live grant instead of reporting allocation residue;
7. Store propagation dropping the grant at a manifest, scan, publication, or
   bootstrap boundary;
8. the pool acquiring Signal, scheduler, proof, Foundational, or semantic
   authority while gaining allocation enforcement;
9. test helpers preserving the obsolete raw-scope API or manufacturing
   authority.

Public pressure projection, exact allocation-boundary event reconciliation,
dirty replacement accounting, the remaining hard pressure dimensions, and
complete initialize/open/abort/close Phase 2 propagation remain separate
Phase 2 slices.

## Closure Ledger

| ID | Closure claim | Required evidence | Final result |
| --- | --- | --- | --- |
| `C6-P2-AA-01` | A raw `PhysicalOperationAllocationScope` cannot authorize load, dirty admission, candidate reservation, per-frame candidate admission, or speculative admission. | Governed API signatures require `&OperationAllocationGrant`; absence search finds no raw-scope call or compatibility overload; compile-fail proofs reject raw scope for load and candidate reservation. | `PROVED` |
| `C6-P2-AA-02` | A grant is sealed, non-clone allocation authority and must still be live at every consuming boundary. | `OperationAllocationGrant` has no public constructor or `Clone`; compile-fail proof rejects use after move/drop; every governed lower API borrows the grant. | `PROVED` |
| `C6-P2-AA-03` | Pool ownership is exact to the pool incarnation, not merely the stable store identity or scope value. | `scope_for` uses `Arc::ptr_eq`; same-store/different-incarnation adversarial tests attack load, dirty, candidate batch, per-frame candidate, and speculative admissions and receive `AllocationGrantMismatch`. | `PROVED` |
| `C6-P2-AA-04` | Foreign-incarnation denial occurs before source work, Store candidate counters, key-vector allocation, request-detail inspection with pool bookkeeping, or residency posture changes. | Lower adversarial tests prove the source closure is not invoked and the complete governed counter snapshot remains unchanged even when dirty bytes and candidate sequence are malformed; Store adversarial test proves zero candidate submissions/declarations/retentions and unchanged residency posture; `validate_operation_allocation` is the first bounded-publisher operation. | `PROVED` |
| `C6-P2-AA-05` | One ordinary read allocation is owned by the read or scan session and reaches every real manifest, routing, inline, extent, and frame-load boundary used by that session. | Source trace from `begin_read_allocation` and scan admission through readers/cursors to `PhysicalResidencyPool::load`; `RecordReadSession` and scan admission own `OperationAllocationGrant`; no obsolete raw-scope load calls remain; Store all-target check and library tests pass. | `PROVED` |
| `C6-P2-AA-06` | One append allocation remains live across planning, payload publication, root publication, and every candidate-frame reservation. | Director creates one named grant before preparation; contexts borrow it; both `StoreCandidateFramePublicationSession` values borrow it; the bounded session stores that borrow and passes it to every `reserve_next`; all downstream session signatures retain the lifetime parameter. | `PROVED` |
| `C6-P2-AA-07` | Grant release reconciles exact per-scope and aggregate operation bytes, and close exposes rather than erases live allocation residue. | `Drop` calls `release_operation`; exact operation-allocation test observes 40 live bytes then zero after drop; shutdown snapshots include active operation bytes and `requires_inspection`; Store shutdown carries `PhysicalResidencyShutdown` into terminal observation. | `PROVED` |
| `C6-P2-AA-08` | Bootstrap, verification, temporary handoff, certification, and recovery test-support consumers cannot bypass the new grant contract. | Workspace consumer sweep shows grants acquired and passed at every direct load/admit call; `worth-store-test-support` all-target check and 21 library tests pass. | `PROVED` |
| `C6-P2-AA-09` | Allocation enforcement does not give the pool Signal, scheduler, `worth-proof`, Foundational, aspect-native, Query, or semantic-residency authority. | Manifest/source absence probe finds no forbidden dependency or import; pool APIs accept only physical allocation, frame, and speculative vocabulary; lower all-feature check passes. | `PROVED` |
| `C6-P2-AA-10` | Negative authority evidence is mechanically sensitive rather than prose-only. | Thirty lower compile-fail doctests pass, including the new raw-scope and consumed-grant cases; the complete Store runtime authority UI suite passes. | `PROVED` |
| `C6-P2-AA-11` | Cleanup leaves no temporary raw-scope adapter, overload, or over-cap test root in this slice. | Repository searches find no raw-scope governed call or `with_scope`/legacy/unchecked compatibility name; semantic test modules hold allocation evidence; every touched Rust file is at or below 400 lines. | `PROVED` |

## Audit History

### `C6-P2-AA-F01`: bounded publisher recorded activity before grant ownership

- Severity: high for `C6-P2-AA-03`, `C6-P2-AA-04`, and `C6-P2-AA-06`.
- Defect: `BoundedCandidateFramePublisher::begin` incremented candidate counters
  and allocated the key vector before the pool rejected a grant from another
  incarnation.
- Violated invariant: admission and authority validation must precede
  allocation and consequential state; a forged or foreign authority opens no
  doors.
- Root correction: add the pool-owned
  `validate_operation_allocation` no-effect boundary and call it before any
  adapter counter or allocation. Actual pool admissions continue to validate
  the grant again.
- Reopened rows: `C6-P2-AA-03`, `C6-P2-AA-04`, `C6-P2-AA-06`,
  `C6-P2-AA-10`.
- Closure evidence: the Store same-store/different-incarnation adversarial
  test, the lower adversarial test, all candidate-residency tests, the full
  Store library suite, and the runtime authority UI suite all pass against the
  corrected source.
- Final status: `PROVED`.

### `C6-P2-AA-F02`: malformed candidate inputs preceded exact-pool authority

- Severity: high for `C6-P2-AA-03`, `C6-P2-AA-04`, and `C6-P2-AA-10`.
- Defect: `PhysicalResidencyPool::admit_dirty` validated the frame key and byte
  length before grant ownership, while
  `PhysicalCandidateBatchReservation::reserve_next` validated candidate
  sequence before grant ownership.
- Violated invariant: exact-pool authority must be the first observable
  decision at a governed admission boundary. A foreign grant cannot cause
  request-detail denial bookkeeping or learn which malformed condition would
  otherwise win.
- Root correction: authenticate the grant before key, length, or sequence
  inspection in both paths. The lower admission still validates again at the
  eventual candidate reservation boundary.
- Reopened rows: `C6-P2-AA-03`, `C6-P2-AA-04`, and `C6-P2-AA-10`.
- Closure evidence: the new same-store/different-incarnation test combines a
  foreign grant with short dirty bytes and an out-of-sequence candidate key,
  requires `AllocationGrantMismatch` for both, and compares the entire governed
  counter snapshot before and after. The complete lower library suite,
  compile-fail doctests, default-feature Store attack, and all-feature Store
  candidate-residency suite pass.
- Final status: `PROVED`.

### `C6-P2-AA-F03`: closure fingerprint omitted untracked source

- Severity: high for audit reproducibility; no production behavior was
  affected.
- Defect: the prior fingerprint hashed `git diff` alone. Git excludes
  untracked files from that stream, including semantic test modules and
  pressure/admission source created during C.6.
- Violated invariant: a closure fingerprint must identify the complete audited
  source state, not only tracked modifications.
- Root correction: hash the binary tracked diff, then combine it with every
  sorted untracked WORTH Store path and its Git blob hash.
- Reopened rows: all rows as evidence-integrity claims.
- Closure evidence: the composite covers the tracked diff and 25 untracked
  files, and recomputes to
  `57c59a740fdbb071b281cf16321a5c85e5d312c4`.
- Final status: `PROVED`.

## Executed Evidence

- `cargo check --manifest-path workspaces/worth-store/Cargo.toml -p worth-store-buffer-pool --all-targets --all-features`
- `cargo test --manifest-path workspaces/worth-store/Cargo.toml -p worth-store-buffer-pool --all-features --lib`
  - 102 passed.
- `cargo test --manifest-path workspaces/worth-store/Cargo.toml -p worth-store-buffer-pool --all-features --doc`
  - 30 compile-fail doctests passed.
- `cargo check --manifest-path workspaces/worth-store/Cargo.toml -p worth-store --all-targets --all-features`
- `cargo test --manifest-path workspaces/worth-store/Cargo.toml -p worth-store --all-features candidate_frame_residency`
  - 10 focused tests passed.
- `cargo test --manifest-path workspaces/worth-store/Cargo.toml -p worth-store --lib foreign_incarnation_grant_is_denied_before_candidate_publication_activity`
  - the default-feature foreign-incarnation attack passed.
- `cargo test --manifest-path workspaces/worth-store/Cargo.toml -p worth-store --all-features --lib`
  - 62 passed.
- `cargo test --manifest-path workspaces/worth-store/Cargo.toml -p worth-store --all-features --test physical_runtime_authority_ui`
  - the authority UI harness and all declared pass/fail cases passed.
- `cargo check --manifest-path workspaces/worth-store/Cargo.toml -p worth-store-test-support --all-targets --all-features`
- `cargo test --manifest-path workspaces/worth-store/Cargo.toml -p worth-store-test-support --all-features --lib`
  - 21 passed.
- normalized absence probes:
  - no governed raw-scope admission calls;
  - no compatibility overloads;
  - no Signal, `worth-proof`, Foundational, or aspect-native dependency in the
    pool.
- all touched WORTH Store Rust files are at or below 400 lines.
- complete dirty-source inventory and structural review:
  - 138 modified or untracked WORTH Store Rust files were enumerated;
  - zero dirty files exceed 400 lines; the maximum is 381 lines;
  - the structural scrutinizer scanned the same 138 files, reported zero scan
    errors, and introduced no new advisory after the authority-order repair.
- repository-wide line-cap baseline:
  - the Bash guard cannot execute in this Windows environment;
  - an exact native reproduction over its tracked workspace scope reports 114
    non-allowlisted over-cap files outside this C.6 dirty set;
  - that unrelated baseline debt was preserved and is not represented as a
    passing whole-repository gate.
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
  - Road 1 Cargo topology is valid.
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
  - generated context is current.

## Slice Result

All allocation-authority slice rows are `PROVED`. No `OPEN`, `DEFECT`, or
`BLOCKED` row remains in this ledger.

Phase 2 remains open for its other named slices. This ledger must not be used
as evidence that Phase 2, Phase 3, or C.6 is complete.
