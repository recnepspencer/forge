# C.6 Phase 4 Closure Ledger

## Scope

This ledger audits C.6 Phase 4, **Install Lease-Scoped Chunk Views And Bounded
Copies**, against the governing C.6 specification, the WORTH engineering laws,
the actual public Store contract, adjacent residency and physical-format
owners, and credible defects that could satisfy the wording while defeating
the intent.

Phase 5 and later are out of scope and remain blocked. In particular, this
ledger does not claim ordinary dirty/writeback settlement, speculative work
lowering, final adapter cutover, or Phase-8 S.2 cleanup.

## Current Authority

This is a living audit. A row is `PROVED` only for the source freeze and
evidence named here. Any correction reopens every row whose assumptions or
evidence changed. The final QA pass and final-source freeze at the end of this
document will own the closure result.

## Audit-Start Source Freeze

- base commit: `f617cdd8ee34e3dc5d8ff6ec65bf93aaedb60b73`
- dirty entries excluding this ledger: `311`
- tracked entries: `185`
- untracked entries: `126`
- path/status/blob manifest SHA-256:
  `ef71c91c0b63f6191f9baac7161a323e629f0f2aac54daf782522ea476e41069`
- row shape: `<two-column Git status><TAB><Git blob id or
  DELETED><TAB><porcelain path>`
- source: `git status --porcelain=v1 --untracked-files=all`
- ordering: Git porcelain order
- encoding: UTF-8 without BOM, one LF after every row
- rename hashing: hash the destination path while retaining the complete
  porcelain path in the manifest row
- self-exclusion: only this ledger
- independent reproduction: PowerShell/.NET and Python `hashlib` produced the
  same counts and digest

Unrelated dirty work is preserved. Earlier C.6 Phase 1-3 changes are part of
the current source authority and are not reclassified as Phase 4 work.

## Completeness Re-Audit Start

The holistic re-audit began from a fresh identity rather than trusting this
ledger's earlier closure labels:

- source entries excluding this ledger: `311`
- source manifest SHA-256:
  `3c03c27350a9ee7340d39ef33e7d70486ff3cdd81ac0550f51973bdc1b274d29`
- ledger SHA-256 at re-audit start:
  `d379f9daab861e571e0301125ae2025e735f7b96b30f0431f4c184ff2e4ab36f`
- independent source reproduction: PowerShell/.NET and Python/hashlib agreed

The ledger is excluded only to avoid self-hashing. Its separate start digest
makes the audited ledger revision explicit; the findings history below records
every change made after that identity.

## Closure Guarantees

| ID | Exact closure claim | Required evidence | Current result |
| --- | --- | --- | --- |
| `C6-P4-L01` | The source state supporting closure is complete and independently reproducible, including tracked, deleted, renamed, and untracked files without hashing this ledger into itself. | Two independent final-source manifest computations using the declared schema. | `PROVED` — PowerShell/.NET and Python/hashlib independently produced the same 315-entry manifest, 46,419 manifest bytes, and digest. |
| `C6-P4-L02` | The ledger covers every Phase 4 must-ship, preserve, proof, cleanup, authority, lifecycle, performance, DX, and causally necessary intent guarantee. | Clause-to-row coverage, an exact row-to-evidence index, and a final attack asking what meaningful defect could survive every row. | `PROVED` — `F010` added exact source, test, compiler-specimen, and command ownership for every row; the renewed attack found and closed `F011` through `F015`. |
| `C6-P4-P01` | `PhysicalRecordReader::{open, open_external}` return one public logical read lease named `RecordReadSession`; the session owns the lifecycle permit, operation allocation, cursor, identity, and at most one current frame. | Public API/source trace, session construction trace for inline and extent, external-locator view journey, lifecycle/drop review, alias-absence compile fence. | `PROVED` — both public entrances converge on the same lease and the external entrance retains the readmitted record basis. |
| `C6-P4-P02` | `PhysicalRecordChunkView<'session>` and its returned byte slice cannot outlive, advance, copy from, move, or drop the borrowed session while live. | Compile-pass supported use; compile-fail escape, re-entrance, drop, extracted-byte re-entrance, and extracted-byte escape attacks; diagnostic cause inspection. | `PROVED` — every attack fails at the intended Rust lifetime or borrow boundary. |
| `C6-P4-P03` | A chunk exposes only physical-format-decoded payload bytes for one exact logical range and observational basis: stable Store, Store lifecycle generation, physical record, and durable frame coordinate. | Inline and multi-frame extent journeys with independent payload and durable-coordinate oracles; exact range continuity; basis/source trace; constructor sealing. | `PROVED` — six extent chunks match the independent artifact, offset, full-frame, short-final-frame, logical-range, identity, generation, and payload oracles exactly. |
| `C6-P4-P04` | `next_chunk` and `read_next` advance one monotonic cursor. Interleaving cannot duplicate, omit, rewind, or fork logical bytes. | Partial-copy/view/copy/view/remaining-view journey with independent reconstructed payload and exact range assertions. | `PROVED`. |
| `C6-P4-P05` | Chunk access is zero-copy at the Store and pool accounting boundaries. `read_next` copies only into caller storage and reports exact nonzero operations, bytes, and maximum width. | Resident-frame/public-slice pointer identity, Store read observation, independent residency counter deltas, caller-counted operations/bytes/width, a record larger than resident budget, and repeated eviction. | `PROVED` — every extent view aliases the certified resident frame payload directly; views add no copy deltas, while complete bounded copying reconciles exact caller and Store evidence. |
| `C6-P4-P06` | View iteration remains one resident frame at a time and does not materialize the record or acquire pool authority. | Extent state/source review; peak resident counter bounded to two frames while consuming a larger record; compile-fail lower-lease reachability; public API absence scan. | `PROVED` — the final source retains one current frame, stays within the two-frame adversarial envelope, and exposes no owning or pool-control surface. |
| `C6-P4-P07` | Caller maximum-payload policy, access scratch limits, external-locator readmission, stale-placement classification, pressure evidence, record observation, cancellation, health, and shutdown behavior remain honest after the new access path. | Existing canonical journeys plus targeted regression runs; source trace through shared open/session failure paths; Store-owned pressure type inspection. | `PROVED` — the exact `P07` index names passing caller-limit, scratch, locator, stale-placement, pressure, damage, cancellation, observation, release, and lifecycle journeys for both access methods. |
| `C6-P4-P08` | Chunk views and their basis grant no pin, allocation, eviction, fault, retry, mutation, writeback, Signal, Foundational, `worth-proof`, Query, replay, or semantic-residency authority. No new Signal family or Foundational fact exists for borrowing bytes. | Constructor/privacy and lower-authority compile failures; dependency/source gates; public surface inspection; constitutional boundary checks. | `PROVED` — compile attacks, source/dependency scans, boundary-check, and agent-context all accept only observational access. |
| `C6-P4-P09` | Phase 4 leaves one unreleased-product API, not a compatibility story: `OpenedPhysicalRecord` is absent, `RecordReadSession` is canonical, and no public owning whole-record convenience exists. Phase-8 S.2 cleanup remains untouched. | Alias compile failure; source/API absence searches for aliases and owning conveniences; scoped diff and removal-ledger ownership review. | `PROVED` — the superseded name exists only in its negative compiler specimen; production contains no alias, deprecation, migration, conversion, or removal-inventory addition. |
| `C6-P4-P10` | Public documentation teaches the exact shipped lifetime, basis, cursor, zero-copy, bounded-copy, pressure, and authority model with compilable semantics and no Phase 5+ claim. | Mechanically drift-checked compiler specimens for every Rust block, runtime execution for self-contained examples, plus bidirectional source/doc trace and semantic review. | `PROVED` — all four Rust blocks are token-drift bound and compile warning-free; both self-contained configuration examples execute, and the real-session examples are exercised by Store journeys. |
| `C6-P4-P11` | The tests are honest evidence: real Store admission and persistence, independently generated payloads, real two-frame residency pressure, caller-side copy oracles, intended compiler diagnostics, and proportionate target cost. | `qa-tests` review of setup/action/oracle/teardown, diagnostic snapshots, fixture budgets, mutation-sensitive failure paths, and redundant-proof inventory. | `PROVED` — nine focused journeys, exact compiler causes, resident pointer identity, independent coordinate/copy oracles, and consequential teardown now cover the previously surviving mutants. |
| `C6-P4-P12` | The final Phase 4 source is formatted, warning-clean, line-cap compliant in the dirty scope, structurally coherent, accepted by relevant tests, and green under both mandatory constitutional gates. | Formatting, focused and broad Cargo checks, dirty-file/function scrutiny, scoped line-cap audits, boundary-check, agent-context check, and final-source audit. | `PROVED` — all Phase 4 and dirty-scope gates pass; the separate repository-wide tracked line-cap debt is recorded without being misrepresented as Phase 4 evidence. |

## Requirement Coverage

| Governing Phase 4 obligation | Ledger rows |
| --- | --- |
| `RecordReadSession` as the lease-bearing public object | `P01`, `P07` |
| Exact `PhysicalRecordChunkView` and `PhysicalRecordChunkBasis` API | `P02`, `P03`, `P08` |
| Basis, Store/durable generation, logical range, borrowed bytes | `P02`, `P03` |
| Zero-copy extent iteration one frame at a time | `P05`, `P06` |
| Retained bounded `read_next` and exact copy accounting | `P04`, `P05` |
| Pressure evidence without pool internals | `P07`, `P08` |
| Sealed constructors and lifetime constraints | `P02`, `P08` |
| Preserve limits, scratch, streaming, readmission, stale placement, and observation | `P07` |
| Compile-pass and adversarial compile-fail proof | `P02`, `P08`, `P09`, `P11` |
| Runtime proof on records much larger than resident budget | `P03`, `P04`, `P05`, `P06`, `P11` |
| Cleanup without owning compatibility conversion | `P09` |
| Documentation and semantic public DX | `P10` |

## Risk Map

- **Lifetime and authority:** primary. A view or extracted slice that permits
  session progression can expose unpinned or replaced bytes.
- **Cursor semantics:** primary. Two access methods can accidentally maintain
  independent offsets or double-count delivery.
- **Payload truth:** primary. Header, neighboring slot, or unvalidated bytes
  must never cross the public API.
- **Performance/resource honesty:** primary. Calling a copied allocation a view,
  undercounting explicit copies, or retaining multiple extent frames defeats
  the phase.
- **Failure and lifecycle:** high. Health loss, stale placement, readmission,
  pressure, cancellation, and drop must release inherited authority exactly
  once.
- **Public authority and dependency direction:** high. Observation values must
  not become pool, Signal, proof, semantic, or writeback authority.
- **Cleanup and DX:** high. An alias or owning convenience recreates competing
  public models in an unreleased product.
- **Test honesty:** high. Cooperative small records or implementation-derived
  counters alone cannot prove larger-than-memory zero-copy behavior.
- **Phase 5+ behavior:** boundary check only. Dirty/writeback and speculative
  progression must neither be implemented nor claimed here.

## Exact Evidence Index

Paths below use these roots:

- `P`: `workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving`
- `J`: `workspaces/worth-store/crates/worth-store/tests/physical_record_journeys`
- `U`: `workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority`

`L01`

- source: the manifest schema and exclusions in `Audit-Start Source Freeze`
- executable evidence: the final PowerShell/.NET and Python/hashlib
  implementations and their independently equal entry lists, blob identities,
  counts, manifest bytes, and digest recorded in `Final QA And Source Freeze`

`L02`

- authority: C.6 specification `Phase 4`, `Cross-Phase Public API Contract`,
  `Public API Availability Matrix`, `Repository And Module Destination Plan`,
  `Documentation Deliverables`, and `Cleanup Ledger`; roadmap C.6 Phase 4;
  all seven engineering-law documents
- coverage: `Requirement Coverage`, `Risk Map`, this exact evidence index, the
  complete findings/reopening history, and the final surviving-defect attack

`P01`

- production: `P/access/locate.rs`, `P/access/locate/inline.rs`,
  `P/access/locate/extent.rs`, `P/lifecycle/record_lifecycle.rs`
- runtime: `J/record_chunk_views/borrowed_access.rs` functions
  `external_locator_view_retains_the_readmitted_record_basis_without_copying`
  and
  `dropping_a_partially_consumed_extent_releases_its_session_frame_and_allocation`
- compiler: `U/record_chunk_views_supported.rs` and
  `U/opened_physical_record_alias_is_absent.rs`

`P02`

- production: `P/access/record_chunk_view.rs` and
  `P/access/locate/session.rs`
- compiler pass: `U/record_chunk_views_supported.rs`
- compiler failures:
  `U/record_chunk_view_cannot_escape_session.rs`,
  `U/record_chunk_view_blocks_session_progress.rs`,
  `U/record_chunk_view_blocks_session_drop.rs`,
  `U/record_chunk_bytes_retain_session_borrow.rs`, and
  `U/record_chunk_view_construction_is_sealed.rs`, with their checked-in
  `.stderr` diagnostics

`P03`

- production: `P/access/record_chunk_view.rs`,
  `P/access/locate/inline.rs`, `P/access/extent_read_session.rs`, and
  `P/residency/frame_loading/loaded_frame.rs`
- runtime: `J/record_chunk_views/borrowed_access.rs` functions
  `inline_view_exposes_only_the_record_payload_and_observational_basis` and
  `extent_views_stream_one_resident_frame_at_a_time_without_pool_copies`;
  the latter independently requires exact `Extent { extent: 1, generation: 1
  }` coordinates, full and short-final physical lengths, and logical payload
  continuity
- compiler: `U/record_chunk_view_construction_is_sealed.rs`

`P04`

- production: `P/access/locate/session.rs` and
  `P/access/extent_read_session.rs`
- runtime: `J/record_chunk_views/bounded_copy.rs` function
  `bounded_copies_and_views_share_one_cursor_with_exact_copy_evidence`

`P05`

- production: `P/access/locate/session.rs`,
  `P/access/read_observation.rs`,
  `P/access/extent_read_session.rs`, and
  `P/residency/frame_loading/loaded_frame.rs`
- runtime: `J/record_chunk_views/borrowed_access.rs` function
  `extent_views_stream_one_resident_frame_at_a_time_without_pool_copies`
  requires public-slice/resident-frame pointer identity and zero Store copy
  deltas; `J/record_chunk_views/bounded_copy.rs` function
  `bounded_copy_streams_the_complete_larger_than_memory_record_with_exact_evidence`
  reconciles caller and Store operation, byte, and maximum-width oracles

`P06`

- production: `P/access/extent_read_session.rs`,
  `P/access/locate.rs`, and `P/residency/frame_loading/loaded_frame.rs`
- runtime: `J/record_chunk_views/fixture.rs` fixes a two-frame resident
  envelope; `extent_views_stream_one_resident_frame_at_a_time_without_pool_copies`
  consumes six physical frames, proves exact resident pointer identity, stays
  within the envelope, and forces eviction
- compiler: `U/record_chunk_view_exposes_no_pool_authority.rs`

`P07`

- production: `P/access/locate.rs`, `P/access/readmission.rs`,
  `P/access/locate/session.rs`, `P/access/locate/cancellation.rs`,
  `P/lifecycle/serving_health.rs`, and `P/access/read_observation.rs`
- Phase 4 runtime:
  `J/record_chunk_views/policy_boundaries.rs` function
  `caller_maximum_payload_denies_before_session_delivery_and_releases_allocation`;
  `J/record_chunk_views/borrowed_access.rs` function
  `external_locator_view_retains_the_readmitted_record_basis_without_copying`;
  both functions in `J/record_chunk_views/failure_lifecycle.rs`
- inherited runtime:
  `J/extent_streaming/roundtrip.rs::mixed_batch_streams_extent_and_a_fresh_process_reads_with_seventeen_widths`,
  `J/segment_truth.rs::segment_filename_and_header_disagreement_is_denied_before_record_decode`,
  `J/physical_work/residency_pressure_projection.rs::public_read_and_append_pressure_retains_exact_pre_effect_basis`,
  `J/physical_work/record_read_cancellation.rs::cancelling_a_read_session_reports_unread_delivery_and_releases_its_leases`,
  and
  `J/serving_lifecycle.rs::record_owner_propagates_through_every_lifecycle_boundary`

`P08`

- production/public boundary: `P/access/record_chunk_view.rs`,
  `P/access/mod.rs`, `P/mod.rs`, and
  `workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs`
- compiler: `U/record_chunk_view_construction_is_sealed.rs` and
  `U/record_chunk_view_exposes_no_pool_authority.rs`
- structural/constitutional: the explicit Phase 4 authority/import scans plus
  `boundary-check` and `agent-context` commands in the command catalog

`P09`

- production/public boundary: `P/access/locate.rs`, `P/mod.rs`, and
  `workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs`
- compiler: `U/opened_physical_record_alias_is_absent.rs`
- cleanup: exact production/API scans for `OpenedPhysicalRecord`,
  deprecation/compatibility/migration machinery, owning whole-record
  conveniences, and scoped inspection proving no Phase-8 S.2 removal-inventory
  change; the existing later cleanup ownership is detected by
  `workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/removal_inventory.rs`

`P10`

- documentation: `_docs/worth-store/bounded-physical-record-access.md` and the
  `RecordReadSession::cancel` contract in
  `P/access/locate/cancellation.rs`
- mechanical proof:
  `workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs`
  inventories exactly four Rust blocks and drift-checks their executable tokens
  against `U/bounded_physical_record_access_examples.rs`; that specimen and
  `U/record_chunk_views_supported.rs` compile in the same trybuild session,
  while the two self-contained configuration examples also execute in the UI
  target

`P11`

- world and budgets: `J/record_chunk_views/fixture.rs`
- runtime setup/action/oracles/teardown: every named function in
  `J/record_chunk_views/{borrowed_access,bounded_copy,failure_lifecycle,policy_boundaries}.rs`
- compiler setup/action/oracles: every Phase 4 specimen named under `P02`,
  `P08`, `P09`, and `P10`, its intended diagnostic cause, and checked-in
  `.stderr`
- breadth/cost: focused Phase 4 journeys, inherited preservation journeys,
  one reused trybuild session, full `physical_record_journeys`, and library
  tests in the command catalog

`P12`

- formatting and composition: scoped `rustfmt --check`, dirty-scope
  `scrutinize_rust_functions.py`, and Rust line-cap commands
- compilation/tests: focused Phase 4, inherited preservation, full journey,
  library, UI, and `cargo check` commands
- constitution/freeze: `boundary-check`, `agent-context`, explicit cleanup and
  phase-boundary scans, and both independent source-manifest computations

## Evidence Command Catalog

All Cargo commands run from `workspaces/worth-store`; repository tools run from
the repository root.

- `E01`: `cargo test -p worth-store --features
  certification-test-authority --test physical_record_journeys
  record_chunk_views -- --nocapture`
- `E02`: `cargo test -p worth-store --features
  certification-test-authority --test physical_runtime_authority_ui --
  --nocapture`
- `E03`: targeted inherited tests named under `P07`, selected by their complete
  module-qualified test names
- `E04`: `cargo test -p worth-store --features
  certification-test-authority --test physical_record_journeys`
- `E05`: `cargo test -p worth-store --features
  certification-test-authority --lib`
- `E06`: `cargo check -p worth-store --lib`
- `E07`: `python scripts/quality/scrutinize_rust_functions.py --dirty .`
- `E08`: because this Windows host has no `bash`, a PowerShell reproduction of
  `scripts/ci/check_workspace_rust_line_caps.sh` uses the same Git pathspecs,
  LF-count semantics, 400-line cap, and exact allowlist; the separate
  dirty/untracked and exact Phase 4 scopes use the same LF-count rule
- `E09`: `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
- `E10`: `cargo run --manifest-path tools/agent-context/Cargo.toml -- check`
- `E11`: scoped `rustfmt --edition 2021 --check` over every Phase 4 Rust file
  named in this ledger
- `E12`: exact production/API/authority/dependency/cleanup/Phase-5 absence
  searches recorded with their results in the final evidence
- `E13`: independent PowerShell/.NET and Python/hashlib final-source manifest
  computations using the schema under `Audit-Start Source Freeze`

## Findings And Reopening History

### `C6-P4-F001` — Runtime fixture could not reach the read contract

- affected rows: `P05`, `P06`, `P11`
- defect: the first fixture admitted only 1 MiB of foreground-write operation
  memory while real publication required about 3.26 MiB, so all tests stopped
  at typed pre-effect pressure before chunk access
- correction: reuse the canonical 16 MiB operation envelope while retaining
  the adversarial two-frame resident envelope
- closure evidence: all four chunk-view journeys now reach the real Store and
  pass; the tested logical records remain larger than resident capacity
- status: `CLOSED`

### `C6-P4-F002` — Constructor-sealing specimen stopped at malformed syntax

- affected rows: `P02`, `P08`, `P11`
- defect: an empty basis literal failed before the compiler evaluated the view
  constructor attack
- correction: call both real constructors with caller-supplied valid argument
  types
- closure evidence: both calls fail specifically with `E0624`, private
  associated function
- status: `CLOSED`

### `C6-P4-F003` — Initial proof did not cover extracted slices or full-copy streaming

- affected rows: `L02`, `P02`, `P05`, `P07`, `P11`
- defect: the first suite proved only wrapper lifetime and two interleaved copy
  calls; a slice-lifetime defect or later-frame bounded-copy defect could
  survive
- correction: add extracted-byte escape/re-entrance attacks and copy an entire
  eight-chunk record through 997-byte caller buffers under two-frame residency
- closure evidence: compiler diagnostics fail for the intended borrow reasons;
  caller-counted operations, bytes, and maximum width reconcile exactly with
  session and residency observations across repeated eviction
- status: `CLOSED`

### `C6-P4-F004` — Documentation ignored the final short copy width

- affected rows: `P10`
- defect: the first bounded-copy example consumed the complete target slice
  instead of `target[..count]`
- correction: branch on the returned count and expose only initialized bytes
- closure evidence: documentation now mirrors the production and tested
  `read_next` contract
- status: `CLOSED`

### `C6-P4-F005` — Session basis identity remained three independent fields

- affected rows: `P01`, `P03`, `P08`
- defect: Store identity, lifecycle generation, and record identity were
  populated correctly but remained independently assignable fields on
  `RecordReadSession`
- correction: introduce one private `RecordReadIdentity` relationship owner;
  both placement constructors consume that value and only it derives a
  per-frame public basis
- closure evidence: scalar session fields are absent; library, runtime, and
  compiler proof families remain green
- status: `CLOSED`

### `C6-P4-F006` — New journeys ignored their terminal close outcomes

- affected rows: `P01`, `P07`, `P11`
- defect: dropping local sessions did not consequentially prove that the read
  lifecycle, frame pin, and operation allocation were released
- correction: add a partially consumed extent-drop journey with live-before
  and zero-after session/pin/allocation observations; require every Phase 4
  journey to end with released record ownership and no inspection-required
  residency
- closure evidence: five Phase 4 journeys pass; partial drop observes one live
  session, pin lease, pinned frame, and operation allocation before drop and
  zero after drop
- status: `CLOSED`

### `C6-P4-F007` — Phase 4 composition mixed semantic levels

- affected rows: `P03`, `P05`, `P11`, `P12`
- defect: extent loading combined planning, canonical work, decode admission,
  and cursor installation in one 95-line body; copy evidence used three
  parallel mutable counters; trybuild registration remained a flat 61-line
  catalog
- correction: decompose extent progression into named plan/load/admit/install
  steps, group caller copy evidence, and isolate Phase 4 compiler cases behind
  a responsibility-named harness function
- closure evidence: library and five runtime journeys pass; dirty scrutiny
  reports 275 Rust files, 86 advisory candidates, zero scan errors, and only
  one inspected coherent Phase 4 advisory
- status: `CLOSED`

### `C6-P4-F008` — Basis derivation was visible beyond its semantic owner

- affected rows: `P02`, `P03`, `P08`, `P11`
- defect: the public constructors were sealed, but `RecordReadIdentity` and its
  basis derivation were visible across the wider record-serving module instead
  of only to the access boundary that owns read-session progression
- correction: narrow the grouped identity to the access module, make both
  public-value constructors module-private, and route basis plus view minting
  through one access-scoped `chunk_view` operation
- closure evidence: the library and five runtime journeys pass; the external
  constructor attack reaches both real constructors and receives `E0624`
  specifically because each associated function is private
- status: `CLOSED`

### `C6-P4-F009` — Borrowed access was not joined to the external-locator entrance

- affected rows: `L02`, `P01`, `P03`, `P07`, `P11`
- defect: external locator readmission was covered by inherited bounded-copy
  journeys, but no runtime evidence drove `next_chunk` through
  `PhysicalRecordReader::open_external`; a future record or generation
  misbinding at that public entrance could survive the Phase 4 suite
- correction: add a real-Store external-locator journey that opens the
  canonical session and verifies payload, logical range, stable Store,
  lifecycle generation, record identity, durable frame coordinate, zero copy,
  and clean teardown
- closure evidence: all six focused Phase 4 journeys pass; the external view
  retains the exact readmitted basis without any pool-copy observation
- status: `CLOSED`

### `C6-P4-F010` — Ledger rows were not independently reproducible

- affected rows: `L02`
- defect: the ledger named broad evidence families and final pass counts, but
  did not map each guarantee to exact production owners, test functions,
  compiler specimens, and commands; a later auditor could not reproduce a row
  without rediscovering its proof
- correction: add an exact row-to-evidence index, then attack every row
  against that index instead of treating a green aggregate target as proof
- closure evidence: `Exact Evidence Index` names the precise production
  owners, runtime functions, compiler specimens, diagnostic snapshots, and
  command families for `L01` through `P12`; the renewed attack produced
  `F011` through `F015`
- status: `CLOSED`

### `C6-P4-F011` — Public examples were reviewed but not compiler-bound

- affected rows: `P10`, `P11`, `P12`
- defect: no CI evidence compiled the Rust blocks in
  `bounded-physical-record-access.md`, and the bounded-copy helper accepted an
  empty target whose immediate zero result was indistinguishable from EOF
- correction: make the non-empty caller-buffer precondition explicit,
  compile every Rust block as one existing trybuild pass family, execute the
  two self-contained configuration examples, and add a mechanical drift check
  between the document and compiler specimen
- closure evidence: the warning-free UI target inventories four blocks,
  rejects token drift, compiles both documentation specimens, and executes both
  self-contained examples; nine real-Store journeys exercise the session APIs
- status: `CLOSED`

### `C6-P4-F012` — Borrowed iteration had no direct failure-path proof

- affected rows: `P07`, `P11`, `P12`
- defect: inherited damage and cancellation journeys exercised `read_next`;
  `next_chunk` has its own health-observation branch and frame-bearing
  cancellation/release behavior, so a defect isolated to borrowed iteration
  could survive; the public cancellation method documentation also omitted
  work started by `next_chunk`
- correction: add real-Store later-extent damage and post-view
  cancellation journeys with independent completed-range, health, pin,
  allocation, and terminal-lifecycle assertions
- closure evidence:
  `later_extent_damage_through_a_view_revokes_health_and_releases_read_authority`
  and
  `cancelling_after_a_view_reports_unread_bytes_and_releases_the_held_frame`
  both pass, and the public cancellation contract now names `next_chunk`
- status: `CLOSED`

### `C6-P4-F013` — Durable frame coordinates had only shape evidence

- affected rows: `P03`, `P11`, `P12`
- defect: the extent journey required six increasing frame offsets and the
  inline journeys required the admitted frame length, but neither compared a
  public basis to an independently expected durable artifact and byte range
- correction: derive the fresh Store's expected extent artifact and all
  six physical frame ranges independently, then require exact coordinate
  equality for every returned chunk
- closure evidence: the six-frame extent journey requires exact artifact
  generation, `16_384`-byte offsets, five full frame ranges, the exact
  `141`-byte short-final range, and continuous logical payload
- status: `CLOSED`

### `C6-P4-F014` — Zero-copy evidence trusted the implementation's counters

- affected rows: `P05`, `P06`, `P11`, `P12`
- defect: zero copy-counter deltas would not detect a dishonest unaccounted
  copy into session-owned storage
- correction: under certification authority, pin the exact resident frame
  named by each public basis and require public-slice pointer identity with its
  decoded payload range while the chunk borrow is live
- closure evidence: every one of six returned public slices has the exact
  pointer of the certified resident frame's decoded payload while copy counters
  remain unchanged under repeated eviction
- status: `CLOSED`

### `C6-P4-F015` — The claimed caller-limit regression journey did not exist

- affected rows: `P07`, `P11`, `P12`
- defect: source enforced `RecordReadLimits::maximum_payload`, but the ledger's
  claimed targeted regression had no test that asserted
  `RecordReadDenial::CallerLimitExceeded`
- correction: deny a real extent read one byte below its payload size,
  require exact requested/completed observations and zero surviving
  session/allocation authority, then admit the same canonical session at the
  exact limit
- closure evidence:
  `caller_maximum_payload_denies_before_session_delivery_and_releases_allocation`
  passes with exact denial and observation, zero leaked authority, exact-limit
  admission, borrowed delivery, and cancellation accounting
- status: `CLOSED`

## Evidence Recorded Before The Completeness Re-Audit

The earlier ledger revision recorded green compilation, compiler-boundary,
runtime, structural, and constitutional evidence. Those results were inputs,
not authority for this pass. In particular, the earlier description of a
targeted caller-limit journey was false; `F015` records and corrects that
specific ledger defect. The final evidence below supersedes all earlier counts.

## Cleanup Decision

This is an unreleased product. Phase 4 deletes `OpenedPhysicalRecord` outright.
It does not add deprecation, compatibility, migration, or “legacy alias”
machinery. The negative compiler specimen is direct public-boundary proof that
the competing name is absent.

The existing S.2 `ZeroCopyRecordView`, `BoundedCopyRecordView`, and
materialization graph remain assigned to Phase 8 by the existing removal
inventory. Phase 4 does not pull that later cleanup forward merely because the
new Store API now exists.

## Final QA And Source Freeze

Phase 4 is closed. Phase 5 and later remain blocked.

Final source authority, excluding this ledger:

- base commit: `f617cdd8ee34e3dc5d8ff6ec65bf93aaedb60b73`
- dirty entries: `315`
- tracked entries: `186`
- untracked entries: `129`
- manifest bytes: `46,419`
- path/status/blob manifest SHA-256:
  `588ead85412d21c512ab2bb1cfe5ad1d42cadf64e987c22497ec8b92bcb4a43d`
- independent reproduction: PowerShell/.NET using `git hash-object
  --no-filters` and Python/hashlib using raw Git blob framing converged on the
  same counts, per-entry blob identities, manifest bytes, and digest

Final evidence:

- `cargo check -p worth-store --lib` — passed
- `cargo test -p worth-store --features certification-test-authority --lib` —
  68 passed
- focused `record_chunk_views` journeys — 9 passed
- the five exact inherited preservation journeys under `P07` — each passed
- full `physical_record_journeys` target — 226 passed
- `physical_runtime_authority_ui` — passed; every Phase 4 compile failure was
  inspected for its intended privacy, lifetime, borrow, reachability, or
  absence cause; all four documentation blocks are drift-bound and compile;
  the two self-contained examples also execute; the target is warning-free
- explicit production scans — no superseded API, compatibility machinery,
  whole-record owning convenience, semantic/proof authority import, or Phase-5
  addition in the causally scoped Phase 4 owners
- `python scripts/quality/scrutinize_rust_functions.py --dirty .` — 279 Rust
  files, 90 advisory candidates, zero scan errors; the five Phase 4 advisories
  are reviewed causal runtime courtrooms or the exact public documentation
  configuration specimen, not mixed-responsibility production functions
- dirty-scope line-cap audit — 281 Rust files checked, zero files above 400
  lines
- exact Phase 4 line-cap audit — 25 files checked, maximum 270 lines, zero
  files above 400 lines
- explicit Phase 4 `rustfmt --check` manifest — 25 Rust files passed
- `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .` —
  passed
- `cargo run --manifest-path tools/agent-context/Cargo.toml -- check` — passed

The repository-wide tracked line-cap reproduction is red on preexisting files
outside this Phase 4 and outside the dirty scope. This Windows host has no
`bash`; the PowerShell reproduction used the shell script's exact Git
pathspecs, LF-count rule, 400-line cap, and allowlist. The red result is not
laundered into an allowlist, attributed to Phase 4, or reported as a passing
gate. It does not invalidate `P12`, whose claim is explicitly Phase 4 and
dirty-scope compliance.

The final intent attack asked whether a dishonest implementation could still
forge basis values, retain bytes past their lease, advance under a live borrow,
misreport a durable short-final coordinate, materialize a whole record, hide an
uncounted copy, keep multiple extent frames, split the cursor, bypass external
readmission or caller limits, skip health revocation on borrowed iteration,
leak pool authority, introduce semantic facts, leave lifecycle capacity live,
drift documentation away from compilable code, preserve a competing public
name, or smuggle in Phase-5 behavior. `F010` through `F015` were the surviving
ledger and evidence defects. Each is now blocked by a named compiler boundary,
mutation-sensitive runtime oracle, source/dependency gate, lifecycle
assertion, executable documentation check, or explicit API-absence proof.
