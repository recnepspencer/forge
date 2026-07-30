# C.6 Phase 9 Closure Ledger

## Scope

This ledger audits C.6 Phase 9, **Publish The Current Contract And Audit
Reality**. Phase 9 documents the Phase 8 source state; it does not change
runtime behavior, revive historical S.2 authority, or claim C.7-C.11 policy.

The phase begins from clean commit
`659c5775de9637a2f8a7c1c7d7205d0851ada683` on branch `worth-store`.
The original Phase 9 implementation was audited before Phase 10 work began.
This re-audit judges the documentation against the current mixed worktree
without attributing later Phase 10 production changes to Phase 9. Phase 10
remains closed until every reopened guarantee below is proved.

## Adversarial Constraint

> A developer follows a current-facing document into a deleted S.2 surface,
> direct buffer-pool control, an owning whole-record read, or a future
> successor policy. The examples may still look plausible. The documentation,
> generated API surface, reality audit, links, and compiler-bound examples
> must expose the lie before it becomes new implementation precedent.

## Closure Guarantees

| ID | Exact closure claim | Required evidence | Result |
| --- | --- | --- | --- |
| `C6-P9-L01` | The Phase 9-owned artifact family is explicit, and current evidence does not misattribute overlapping Phase 10 production changes to Phase 9. | Base commit, original 21-path audit history, current Phase 9 path inventory, and exact correction review. | `PROVED` — the original six document/audit/roadmap/ledger paths plus 15 rustdoc-only paths remain historical evidence; the nine-path re-audit correction contains prose, rustdoc, compiler-specimen, and ledger changes only. The 155-entry mixed worktree is accounted as later Phase 10 work, not Phase 9 evidence. |
| `C6-P9-L02` | This ledger covers every Phase 9 deliverable, cleanup obligation, public-documentation boundary, and credible stale-document failure. | Spec trace, risk review, surviving-defect attack. | `PROVED` — seven findings remain in history, including the two guarantees missed by the original ledger; all twelve surviving-defect attacks below now have direct evidence. |
| `C6-P9-P01` | The bounded physical record access guide teaches the stable Store facade, admitted policy, borrowed chunks, bounded copies, pressure handling, observation, successor allocation handoff, and physical-versus-semantic boundary in developer language. | Feature-doc review against public source and strongest tests. | `PROVED` — the guide names durable physical ownership, adapter-scoped allocation, denial handling, and successor non-authority; five compiler-bound examples, 10 chunk journeys, one exact scope journey, and all 243 physical-record journeys passed. |
| `C6-P9-P02` | Every public Rust block in the feature guide is normalized-code compile-bound; configuration blocks execute directly, and behavioral read/view/copy/allocation claims are proved by real Store journeys or adapter UI evidence. | Exact block inventory, `physical_runtime_authority_ui`, `physical_adapter_authority_ui`, and focused journeys. | `PROVED` — exactly five Rust blocks match the checked-in compiler specimen; both UI suites, 10 chunk journeys, and the exact successor-scope journey passed. |
| `C6-P9-P03` | The buffer-pool README states the final lower-owner boundary, admitted dimensions, counter meaning, forbidden authorities, and Store-facing entry point without milestone archaeology or direct-use guidance. | README/source/dependency comparison and stale-claim search. | `PROVED` — owner/counter source review, zero current-doc archaeology hits, and Cargo metadata showing zero forbidden direct dependencies. |
| `C6-P9-P04` | The isolated S.2 specification is visibly historical and superseded, preserves useful context, links to C.6, and does not present deleted authority as the current implementation target. | Status/link inspection and current-facing vocabulary review. | `PROVED` — one prominent Historical status, current guide/spec/roadmap links, and zero absolute-path hits. |
| `C6-P9-P05` | Every C.6 physical-reality audit row names the exact current production source, retained non-production boundary, or deletion evidence and carries a disposition consistent with that truth. | CSV schema/path review, source trace, deletion gates. | `PROVED` — 40 rows/22 columns, three exact C.6 rows, zero blank/schema/path/disposition issues, 33 removal-gate tests, and 241 Store journeys. |
| `C6-P9-P06` | The roadmap records current C.6 evidence and links C.7-C.11 as successors without claiming either their unimplemented responsibilities or final Phase 10 closure. | Roadmap/spec comparison and link resolution. | `PROVED` — the roadmap explicitly reserves hostile courtroom, mutation, and full-ledger verdicts for Phase 10; all five successor anchors resolve and grant no successor policy. |
| `C6-P9-P07` | Generated API documentation explains the public residency policy, read session, chunk view and durable owner, pressure evidence, retry posture, observation, and successor allocation surfaces well enough to use them without reading private modules. | Public rustdoc review and warnings-denied `cargo doc`. | `PROVED` — Store entry, five exact admission methods, runtime-borrowed grants, identity/byte accessors, failure classification/reason/pressure, and the prior read/pressure/observation surfaces are documented; warnings-denied API generation passed. |
| `C6-P9-P08` | Documentation distinguishes stable application behavior, stable successor-adapter allocation, certification-only probes, historical context, and future recovery, integrity, isolation, scheduling, and blob policy. | Bidirectional claim/source trace and successor-boundary review. | `PROVED` — the guide's Stable Entry Points, Successor Physical Allocation, Current Limits, and anti-patterns agree with the public exports, adapter UI cases, S.2 banner, README boundary, and roadmap non-verdict. |
| `C6-P9-P09` | Stale paths, examples, phase-progress claims, and deleted S.2/C6 vocabulary are absent from current-facing documentation; every changed local link resolves. | Targeted stale-content search and local-link check. | `PROVED` — final scan found zero deleted-surface, milestone-archaeology, or absolute-path hits; all 24 local links and five anchors resolve. |
| `C6-P9-P10` | Phase 9-owned changes and corrections change no runtime behavior, authority topology, dependency direction, or public signature. | Exact correction review, Cargo check, and boundary checks; the mixed Phase 10 diff is accounted separately. | `PROVED` — the nine-path correction is prose, rustdoc, compiler specimen, roadmap, and ledger only; warnings-denied Store all-target check, formatting, diff integrity, boundary-check, and agent-context check passed. |
| `C6-P9-P11` | Successor physical adapters can discover and compile the exact Recovery, Scrub, Maintenance, Verification, and Blob allocation handoff, while the docs make clear that its runtime-borrowed grants authorize only temporary bytes. | Guide/compiler specimen, public rustdoc, positive and negative adapter UI cases, and successor-scope runtime journey. | `PROVED` — the guide example compiles; all five positive grant types compile; scope substitution, forgery, lower-grant access, runtime escape, and close-with-live-allocation fail to compile; the runtime journey reaches each ceiling, denies one past it, preserves scope isolation, hits the global envelope, and releases every charge. |

## Requirement Coverage

| Phase 9 obligation | Ledger rows |
| --- | --- |
| Bounded physical record access guide with compiled examples | `P01`, `P02` |
| Buffer-pool owner README | `P03` |
| S.2 supersession notice | `P04`, `P09` |
| Exact physical-reality audit dispositions | `P05` |
| Roadmap closeout and successor links | `P06`, `P08` |
| Generated public API documentation | `P07` |
| Successor physical allocation handoff | `P01`, `P02`, `P07`, `P08`, `P11` |
| Executed versus future responsibility honesty | `P01`, `P03`, `P04`, `P06`, `P08`, `P11` |
| Stale documentation cleanup | `P04`, `P09` |
| Ledger completeness and source truth | `L01`, `L02`, `P10` |

## Finding History

| ID | Severity and guarantees | Concrete defect | Required correction and closure proof | Status |
| --- | --- | --- | --- | --- |
| `C6-P9-F01` | Medium — `P05` | `physical-budget-enforcement` cited nonexistent `physical_residency/allocation`; the pool-residency row pointed at composition rather than the sole residency owner as its entry source. | Exact owner/accounting paths installed; CSV schema/path/disposition validation, removal gate, and Store journeys passed. | `CLOSED` |
| `C6-P9-F02` | Low — `P03` | The buffer-pool README said a basis “belongs to C.6 Phase 5” and did not state current/peak, attempted/executed, and non-authoritative counter semantics directly. | Milestone sequencing removed; observation contract added and verified against source, stale scan, and metadata. | `CLOSED` |
| `C6-P9-F03` | Medium — `L02`, `P02` | The original ledger claimed every guide block executes, but the compile specimen executes only configuration examples; borrowed-view and bounded-copy snippets are compile-only. | Proof split corrected; UI suite and nine real Store record-chunk journeys passed. | `CLOSED` |
| `C6-P9-F04` | Medium — `P07` | Rustdoc described pressure/observation values but omitted the public error and runtime methods callers use to obtain them. | Store-facing accessors documented; warnings-denied API generation passed. | `CLOSED` |
| `C6-P9-F05` | Low — `P01` | The guide named `ServingPhysicalRuntime` operations without stating the public open handoff that produces that runtime. | `MediaOwnedPhysicalRuntime::open_record_store` now names the handoff and the broader admission sequence remains linked rather than duplicated. | `CLOSED` |
| `C6-P9-F06` | Medium — `P01`, `P02`, `P07`, `P08`, `P11` | The guide called exact successor-scope controls certification-only even though `ServingPhysicalRuntime::physical_allocations()` is an intentional public adapter handoff used by current successor crates. It also omitted the chunk's durable `physical_owner()`, while the scoped-allocation facade, grants, failure type, and failure accessors lacked public rustdoc. | Document the adapter-only authority and denial topology precisely, compile-bind one realistic call, document runtime borrowing and release, and rerun positive/negative adapter UI plus successor-scope runtime evidence. | `CLOSED` — five guide blocks, both UI suites, exact runtime scope evidence, and warnings-denied rustdoc/check passed. |
| `C6-P9-F07` | Medium — `P06`, `P08` | The roadmap heading and prose presented Phase 9 publication as C.6 closeout even though Phase 10 hostile courtroom, mutation closure, and full-ledger proof remain mandatory. | State the current contract and predecessor evidence without a final verdict; rerun successor-anchor resolution and compare the wording to the phase plan. | `CLOSED` — the roadmap now withholds final verdict from Phase 9 and all five successor anchors resolve. |

## Evidence Index

- `cargo test -q -p worth-store --test physical_runtime_authority_ui
  --all-features external_consumers_cannot_forge_or_duplicate_runtime_authority`
  — one UI family passed, including five guide blocks and lifetime/authority
  negative cases.
- `cargo test -q -p worth-store --test physical_adapter_authority_ui
  --all-features` — one UI family passed, including five exact successor
  allocation types and five causal negative boundaries.
- `cargo test -q -p worth-store --test physical_record_journeys
  --all-features record_chunk_views` — 10 focused journeys passed.
- `cargo test -q -p worth-store --test physical_record_journeys
  --all-features successor_scopes_are_exact_isolated_global_and_released` —
  one exact successor-allocation journey passed.
- `cargo test -q -p worth-store --test physical_record_journeys
  --all-features` — 243 journeys passed.
- `cargo test -p store-test-runner
  physical_residency_boundary_gate::removal_inventory --all-features` — 33
  removal-inventory tests passed.
- `RUSTDOCFLAGS="-D warnings" cargo doc -q -p worth-store --all-features
  --no-deps` and `RUSTFLAGS="-D warnings" cargo check -q -p worth-store
  --all-targets --all-features` — passed.
- `cargo fmt --all --check`, `git diff --check`,
  `cargo run --manifest-path tools/boundary-check/Cargo.toml -- --root .`, and
  `cargo run --manifest-path tools/agent-context/Cargo.toml -- check` — passed.
- `python scripts/quality/scrutinize_rust_functions.py --dirty .` — 150 dirty
  Rust files inspected, 51 Phase 10-era advisory candidates, and zero scan
  errors. The Phase 9 compiler specimen's 69-line realistic example remains
  one cohesive mirrored guide block rather than production orchestration.
  The exact PowerShell equivalent of the unavailable Bash line-cap guard found
  zero non-allowlisted violations across 18,959 tracked Rust files and all 150
  dirty Rust files. Phase 10 still owns review of its 51 advisories.
- Final one-shot validators: 40 CSV rows, 22 columns, three C.6 rows, zero CSV
  issues; 24 local links and five anchors, zero broken; zero current-facing
  deleted-surface, milestone-archaeology, or absolute-path hits; zero forbidden
  pool dependencies and zero legacy feature declarations.

## Surviving-Defect Attack

| Attack | Closure evidence |
| --- | --- |
| A guide teaches private or certification-only entry points as ordinary. | Stable entries are traced to public exports; ordinary consumers remain on Store, while the successor allocation example is explicitly adapter-only and runtime-borrowed. |
| A public successor allocation is hidden as certification-only or mistaken for successor policy. | `physical_allocations()` and its five exact grants are documented as stable adapter surfaces that authorize temporary bytes only; positive and negative adapter UI cases must prove the lifetime boundary. |
| A block matches merely similar specimen code. | The harness compares each of exactly five normalized code bodies with the compiled specimen; feature-doc review separately requires all five semantic examples. |
| The README teaches direct pool use or moves Signal/proof/Foundational authority below Store. | The README directs applications to Store, metadata shows no forbidden direct dependency, and the removal gate rejects unauthorized pool consumers. |
| S.2 remains planned/current or retains a stale absolute link. | Its prominent status is Historical and superseded; absolute-path scan is empty. |
| An implemented audit row cites a deleted path. | Every current C.6 source/trace path exists; the sole deleted row is typed `deleted_false_authority/deleted/deleted` and points to deletion evidence. |
| The roadmap claims successor policy. | The handoff names only current C.6 evidence and links future policy to the C.7-C.11 owner sections. |
| The roadmap claims C.6 closed before the hostile courtroom and mutations pass. | The roadmap now labels this section as the current contract and explicitly reserves final verdict for Phase 10. |
| Generated docs list names without lifecycle, authority, failure, or retry meaning. | Rustdoc covers the session lifetime, sealed policy, Store error accessors, descriptive pressure/retry posture, read-only observation boundary, successor allocation lifetime and authority, and allocation-failure classification/reason/pressure. |
| Current prose promotes physical residency into semantic residency, durability, integrity, or completeness. | The guide defines the physical/semantic distinction in its opening and repeats successor non-guarantees at the use boundary. |
| All rows pass with a broken changed-document link. | The final resolver checks every changed local link and anchor; all 24 links and five anchors resolve. |
| Documentation cleanup changes production behavior or a public signature. | The exact nine-path Phase 9 correction changes docs, rustdoc, and the compiler-only guide specimen/inventory; no production executable line or public signature changes. The separate Phase 10 production diff is not used as Phase 9 evidence, and compilation plus architecture gates pass. |

No credible in-scope defect survives these rows. Any later source change to a
documented public surface reopens the affected guarantee.
