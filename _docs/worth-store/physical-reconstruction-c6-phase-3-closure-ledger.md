# C.6 Phase 3 Closure Ledger

## Scope And Source Freeze

This ledger closes only C.6 Phase 3: fault, hit, coalescence, eviction, the
Store-private serving-residency capability, and the Phase 3 handoff-read
cleanup. Phase 4 and later remain blocked.

- baseline commit: `f617cdd8ee34e3dc5d8ff6ec65bf93aaedb60b73`
- audit-start dirty workspace entries: `207`
- audit-start tracked files in the Store/docs scope: `135`
- audit-start untracked files in the Store/docs scope: `64`
- final source freeze: `CLOSED`
- final dirty workspace entries: `243`
- final tracked files in the Store/docs scope: `164`
- final untracked files in the Store/docs scope: `77`
- Phase 3 closeout code/test manifest: `13` files
- Phase 3 closeout code/test digest:
  `66a2d3fb74c6b20f2b078b8607b48dec3a15ded1387649e3c79540726ff5bf42`

The workspace already contained the earlier C.6 Phase 1, Phase 2, and Phase 3A
changes. This audit preserves all unrelated dirty work. The counts and
13-file digest above are historical, not a reproducible final-workspace
fingerprint; the holistic Phase 1-3 ledger's complete status/blob manifest
supersedes them.

## Guarantee Ledger

| ID | Closure claim | Required evidence | Status |
| --- | --- | --- | --- |
| P3-01 | One pool-incarnation loading identity owns each cold coordinate; overlapping callers can only become typed waiters. | Move-owned owner/waiter API, compile-fail proofs, forced overlap counters. | PROVED by the Phase 3A ledger, final source review, and the 127-test lower suite. |
| P3-02 | Exact and bounded hits and coalesced waiters create no frame source load, media command, physical-work identity, or Signal authority. | Exact and bounded decision-before-source traces, controlled bounded pre-source mutant, lower hot/overlap tests, and composite public deltas that separately identify inherited segment validation. | PROVED after holistic reopening by Phase 3A journeys, the bounded fault/alias tests, and the corrected causal cold/hot oracle. |
| P3-03 | A real miss alone uses the canonical C.5.1 `ReadFault` path with the exact Store-native projection basis; Signal, Foundational, aspect-native, and `worth-proof` do not directly enter the pool. | Direct dependency/source inspection, truthful transitive graph inventory, boundary graph, miss/hit/refault journeys. | PROVED by source/dependency inspection, focused journeys, and the final boundary gate. |
| P3-04 | Ordinary serving reads carry one private `ServingFrameResidency`; raw pool/source recombination and the transitional C6 read API are absent from the ordinary feature graph. | Private capability topology, default/all-feature compile graphs, public API inspection. | PROVED by capability topology, warning-clean compile graphs, and deletion of the handoff read capability. |
| P3-05 | Eviction execution cannot accept a raw coordinate; only checked deterministic selection can mint a move-owned legal-victim token. | `LegalEvictionVictim` constructor privacy, token-consuming executor signature, source inspection, identity-sensitive oldest-victim test. | PROVED by selector-local token construction, the consuming executor, and the oldest/newer hit-refault oracle. |
| P3-06 | Pinned, independently dirty, loading, candidate-reserved, and writeback-claimed frames are all illegal victims; an all-ineligible set denies before a new fault or source load. | Hostile simultaneous-state siege with typed handles, exact state preconditions, and pre/post counters. | PROVED by `every_nominal_victim_ineligible_denies_before_fault_or_source_load`; qa-tests review CLOSED. |
| P3-07 | With exactly one legal victim, eviction releases frame/resident accounting exactly once and refault re-enters sole fault ownership and source execution. | One-legal-victim siege, exact eviction/inspection/fault/source deltas, byte oracle. | PROVED by `sole_legal_victim_releases_exactly_and_refaults_through_fault_ownership`; qa-tests review CLOSED. |
| P3-08 | Source preparation, source execution, abandonment, projection rejection, and later refault cannot loop as repeated cache misses or leave stale loading authority. | Typed shared terminals, lifecycle tests, projection-failure Store journey. | PROVED by Phase 3A evidence, recursive typed fault lowering, transient retry journeys, and lifecycle evidence. |
| P3-09 | Phase-relevant cleanup and documentation tell the current API and authority truth. | No handoff frame-read symbols or read-capable field, updated feature guide/README, removal-ledger update. | PROVED by final source, topology, and documentation review. |
| P3-10 | The Phase 3 change set satisfies warning, line-cap, function-composition, workspace, and constitutional gates. | `-D warnings`, dirty-scope line inventory, scrutiny script, boundary-check, agent-context. | PROVED for the Phase 3 scope: zero dirty line-cap violations and every other gate passes. The holistic repository-global claim remains open on 114 unrelated baseline violations. |

## Risk Map

- **Authority and architecture:** high relevance. The capability boundary,
  feature graph, loading ownership, and eviction token must be compiler-visible.
- **Concurrency and lifecycle:** high relevance. Coalesced faults, dropped
  owners/waiters, claimed frames, and close residue can invalidate residency
  truth.
- **Resource and performance honesty:** high relevance. Hit/media deltas,
  victim scans, exact release, and refault work must reconcile.
- **Test honesty:** high relevance. The hostile set must isolate each exclusion
  reason and the refault oracle must not copy the implementation.
- **Public DX and documentation:** medium relevance in Phase 3. Ordinary callers
  must see records and observation, not pool or transitional frame authority.
- **Persistence/recovery:** boundary check only. Phase 3 does not claim WAL or
  reconstruction authority.

## Findings And Reopening History

### Q3B-01 - ordinary feature quarantine leaves certification-only dead surfaces

- severity: moderate
- affected guarantees: P3-04, P3-10
- evidence: default `cargo check -p worth-store` reports certification-only
  re-exports and helper methods as unused after `c6_handoff` became feature
  gated
- invariant: an ordinary graph should compile only the authority it can use,
  and final warning-clean verification must not depend on all features hiding
  stale seams
- correction: gate certification-only re-exports/helpers at their definition
  boundary without gating ordinary execution internals they share
- closure proof: default and all-feature `-D warnings` checks
- correction applied: certification-only Store imports, inspection methods,
  dirty-frame APIs, backend fault activation constructors, denial types, and
  schedule validators are gated at their definition boundaries
- closure proof: ordinary and all-feature
  `RUSTFLAGS="-D warnings" cargo check -p worth-store` pass
- status: CLOSED

### Q3B-02 - temporary handoff still owns the Phase 3 frame-read surface

- severity: high
- affected guarantees: P3-04, P3-09
- evidence: `c6_handoff/residency/frame_load.rs` still exposed raw pin methods
  and the handoff retained a `ServingFrameResidency` containing the canonical
  source after the private serving capability landed
- invariant: Phase 3 cleanup deletes temporary handoff frame-read types and
  makes the responsibility-named serving capability the only ordinary read
  composition
- correction: delete the handoff frame-load module and all old frame-read
  types; move certification-only fault-driving access into
  `residency/certification/`; narrow `C6PhysicalResidencyWork` to writeback
  frame ports so it cannot construct or execute reads
- closure proof: exact source absence scan; seven focused C6 journeys; nine
  source-bound Phase 16 lifecycle tests; ordinary/all-feature compile graphs
- status: CLOSED

### Q3B-03 - deterministic victim ordering lacks an identity-sensitive oracle

- severity: moderate
- affected guarantees: P3-05, P3-06, P3-07
- evidence: the no-legal and sole-legal sieges could not distinguish oldest
  selection from reversed or arbitrary selection when multiple legal frames
  existed; their counters proved release, not victim identity
- owner: test evidence
- correction: strengthen the existing clean-frame pressure test with two legal
  identities, prove the newer identity remains a hit after pressure, and prove
  the oldest identity refaults through a new source load; assert the hostile
  world's exact simultaneous counter posture alongside its typed state handles
- closure proof: focused ordering and siege tests plus the full 127-unit and
  37-doctest lower suite pass under `-D warnings`
- status: CLOSED

### Q3B-04 - eviction proof minting and certification behavior are too broadly placed

- severity: high
- affected guarantees: P3-05, P3-09, P3-10
- evidence: `LegalEvictionVictim::selected` was visible to every eviction
  sibling, while `residency/certification/mod.rs` implemented behavior instead
  of remaining a declaration-only facade
- invariant: only the checked selector may mint eviction authority, and
  directory/facade topology must preserve named semantic responsibilities
- correction: colocate deterministic checked selection and the sole token
  struct literal in `eviction/legal_victim.rs`; give ordering and release their
  own named modules; remove every constructor; split certification probing and
  resident-frame behavior into responsibility-named files
- closure proof: the construction-site scan finds one struct literal inside the
  checked selector and no constructor; the executor only consumes the token;
  certification `mod.rs` contains declarations/re-exports only; warning-clean
  workspace compilation and the complete composition review pass
- status: CLOSED

### Q3B-05 - fault terminal causes are laundered and the hot-hit oracle requires forbidden work

- severity: high
- affected guarantees: P3-02, P3-08, P3-09, P3-10
- evidence: the first full Store test run passed 212 physical-record journeys
  and then failed four; isolated reruns reproduced typed denial as
  `PublishedLayoutDamaged`, and the cold/hot comparison required hot
  `ArtifactRangeRead` work even though Phase 3 forbids source/Signal work on a
  hit
- invariant: fault ownership may share a terminal without erasing its typed
  backend, work, or residency cause; a hot hit must create exactly zero range
  read work
- correction: recursively lower `FrameLoadFaultCause` in access and planning
  classification; preserve typed manifest lookup failures during published
  segment reuse; remove physical-work counts from semantic equality and assert
  cold work is greater than hot work with exactly zero hot range-read terminals
- closure proof: the three transient denial/retry journeys, the cold/hot
  observation journey, all 217 physical-record journeys, and the complete Store
  suite pass under `-D warnings`
- status: CLOSED

## Holistic Reopening Supplement

The later Phase 1-3 audit reopened Phase 3 guarantees rather than treating this
ledger as unquestionable prior evidence:

- HQ-F007 moved bounded length discovery behind pool classification and added
  bounded fault/owner/waiter typestate.
- HQ-F008 added typed fragment-versus-complete candidate coverage so published
  manifest, root, and catalog candidates share one truthful whole-artifact
  alias with their exact frame.
- HQ-F009 replaced a per-traversed-frame work oracle with causal fault/work and
  resident-reuse bounds.
- HQ-F010 centralized bounded completion rejection and proved an
  already-sleeping waiter wakes; a controlled missing-notification mutant is
  rejected.

All four implementation reopenings are closed. HQ-F011 remains external
repository baseline debt: 114 tracked, non-allowlisted Rust files outside the
dirty C6 scope exceed 400 lines.

## Final Verification Evidence

- `cargo test -p worth-store-buffer-pool --all-features`
  - 127 unit tests passed
  - 37 doctests/compile-fail proofs passed
- `cargo test -p worth-store --test physical_record_journeys c6_ --all-features`
  - 7 focused Store journeys passed
- `cargo test -p worth-store --test physical_runtime_authority_ui --all-features`
  - 22 supported/compile-fail cases passed
- `cargo test -p worth-store --test physical_record_journeys phase_16_lifecycle_maelstrom --all-features`
  - 9 source-bound lifecycle tests passed after updating the two intentionally
    changed module hashes
- `RUSTFLAGS="-D warnings" cargo check -p worth-store`
  - ordinary feature graph passed without certification-only dead surfaces
- `RUSTFLAGS="-D warnings" cargo check -p worth-store --all-features`
  - all-feature graph passed
- `RUSTFLAGS="-D warnings" cargo check --workspace --all-targets --all-features`
  - the complete Store workspace passed
- `RUSTFLAGS="-D warnings" cargo test -p worth-store --all-features`
  - 62 library tests passed
  - 3 physical-media UI harness tests passed
  - 17 physical-media journeys passed
  - 217 physical-record journeys passed
  - 22 physical-runtime authority supported/compile-fail cases passed
  - downstream facade, pressure, lifecycle, and doctest targets passed
- source/dependency audit
  - no temporary handoff frame-read symbol or read-capable field remains
  - `DirectFrameReadSource::new` occurs only in the bootstrap arms of
    `RecordFrameReader`
  - `worth-store-buffer-pool` has no direct dependency or source import for
    Signal, `worth-proof`, Foundational, or aspect-native; the Cargo graph
    truthfully retains governed transitive dependencies through lower physical
    owner crates
- qa-tests review
  - exact hostile preconditions prove the pinned, unpinned dirty, loading,
    reserved-candidate, and writeback-claimed world actually exists
  - an identity-sensitive oldest/newer oracle proves deterministic selection
  - the full lower suite is 127 unit tests plus 37 compile-fail doctests
- code-quality review
  - all dirty C6 Rust files satisfy the 400-line limit
  - the holistic scrutiny scan covers 246 files, reports 84 candidates and zero
    scan errors, and every exact candidate span was inspected
  - certification facades contain no behavior, and eviction authority has one
    selector-local construction site
- final hygiene and constitutional gates
  - `cargo fmt --all -- --check` passed
  - `git diff --check` passed
  - `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`
    reports valid Road 1 Cargo topology
  - `cargo run --manifest-path tools/agent-context/Cargo.toml -- check` passed

## Closeout

`CLOSED WITH HOLISTIC QUALIFICATION`: C.6 Phase 3 satisfies its changed-scope
guarantee ledger after all holistic implementation reopenings. The combined
repository-wide audit remains open only on HQ-F011's unrelated global line-cap
baseline. Phase 4 has not started.
