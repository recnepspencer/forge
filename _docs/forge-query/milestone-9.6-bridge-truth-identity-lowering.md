# Milestone 9.6 Bridge Truth Identity Lowering

> **Status:** Draft
>
> **Roadmap parent:** [forge_query_roadmap.md](./forge_query_roadmap.md)
>
> **Hard gate for:** [milestone-9.7.md](./milestone-9.7.md) Phases 14–16
>
> **Purpose:** truth-routing identity (commit, snapshot, patch, entity, branch,
> signal routing) is structurally typed from relational export through bridge to
> Query. No display-string collapse at the adapter seam.

## Goal

Relational owns `CommitId`, `EntityId`, `SnapshotId`, `VersionId`. Bridge carries
sealed nominal artifacts. Query adapters and mutation receipts hold those
artifacts — not `String`, not `format!("commit-{}", …)`, not `.as_str().to_string()`.

## Why

`9.6` closed curated evidence boundaries. It did **not** close:

1. **forge-relational** `presentation/bridge` formats typed IDs into text.
2. **forge-runtime-bridge** wraps them in tagged `Arc<str>`.
3. **forge-query** requires `String` on receipts and `snapshot_token() -> String`.

`9.7` journal and pinning work cannot succeed on this seam.

## Strict Rules (non-negotiable)

This milestone is **break → fix**. Not migrate. Not strangle. Not validate along
the way.

1. **Hard break first.** Phase 2 deletes facade string APIs. Workspace must be
   red before any call-site fix begins. Fixing compile errors in Phase 2 is a
   spec violation.

2. **No incremental refactoring.** Forbidden patterns:
   - leaving string APIs alive “until callers are ready”
   - migrating one call site per PR while the old constructor still compiles
   - `#[cfg]` / feature flags that keep both string and typed paths
   - default trait methods that stringify so “most” adapters keep working
   - touching a slice without deleting its string path in the same change
   - restoring a forbidden API temporarily to unblock a downstream crate

   Each fix phase replaces a whole slice with typed identity — delete the old
   path in the same change. No half-migrated dual stack.

3. **No parity tests as we refactor.** Forbidden patterns:
   - asserting pre-break string output equals post-break typed output
   - round-trip tests that reconstruct `format!("commit-{}", …)` and compare
   - snapshot/golden fixtures of old display conventions
   - “old vs new behavior” or “transition” test suites
   - byte-identical / repeated-lowering equivalence proofs against deleted folklore
   - keeping a deleted string code path alive in tests “for comparison”

   Tests prove the **new typed path** works. They do not prove equivalence to
   deleted string folklore.

4. **No legacy compatibility.** No `#[deprecated]` aliases, `*_str()` escape
   hatches, dual APIs on the same surface, or compat shims for downstream crates.
   Phase 7 addendum: if a struct stores typed evidence internally,
   `*_digest() -> &str` / `reference_identity() -> &str` on the same type is a
   dual API unless renamed to `*_for_reporting` and used only at display/JSON
   edges — never for composition or drift checks.

5. **No incremental validation.** No exposure-report error-count tracking as a
   progress metric, phased scan greening schedules, sabotage matrices, or
   certification suites whose purpose is to validate the refactor incrementally.

6. **No Rust inventory module as coverage proof.** A `bridge_truth_identity_inventory.rs`
   with `include_str!` pattern scans is not authoritative — it will miss paths and
   lie by curation. The **Collapse Matrix** in this doc (filled by the **agent in
   Phase 1**) is the single source of truth for what exists and what must be fixed.

### API bans (forever after Phase 2)

- `From<&str>` / `From<String>` on truth identity types
- Public `Truth*Identity::new(&str)` / `as_str()` on bridge facade
- Public `commit_identity: String` / `snapshot_token: String` on Query receipts
- `snapshot_token(&self) -> String` on adapter traits

## Fix Order

Work top-down. Do not restore any forbidden API to unblock a downstream crate.

```
1  → agent comprehensive scan → Collapse Matrix in this doc (authoritative inventory)
2  → HARD BREAK — workspace RED
3  → forge-relational presentation/bridge lawful mint
4  → forge-runtime-bridge sealed identity
5  → forge-query adapters + harnesses
6  → forge-query receipts + write surfaces + shared_read_pins
7  → forge-query evidence + intent + signal + Phase 7 feeder spines (QA CLEARED)
8  → worth-topo production adapters (blocked until Phase 7 QA CLEARED)
9  → hostile certification (typed journal position; no rsplit parse)
10 → workspace GREEN + closeout
```

### Phase 1 — agent collapse scan (matrix in this doc; do not fix anything)

**What this phase is:** the **implementing agent** runs a comprehensive codebase
scan of the truth-routing collapse chain and records every in-scope path as a row
in the **Collapse Matrix** at the end of this doc. The matrix is the authoritative
inventory for the whole milestone — not a Rust scan module, not grep output dumped
without reading files.

**What this phase is not:**
- Not the hard break (Phase 2).
- Not migrating call sites or deleting APIs.
- Not making `cargo check --workspace` red.
- Not a Rust `include_str!` inventory module — that approach is explicitly banned.
- Not grep-only: the agent must read files and trace call chains.

**Agent scan method (required)**

The agent cannot load the whole codebase. Use **slice passes**: `rg`/search to
triage, then **read only hit files + their callers/callees** until the chain is
understood. Multiple agent iterations within Phase 1 is expected (one slice per
pass). Do not add matrix rows from `rg` output alone.

**What is banned as inventory machinery**

- Rust modules that `include_str!` entire files and regex-scan them
- Curated path allowlists that hide directories (base `9.6` `EXCLUDED_FOLKLORE_PATHS` style)
- Pasting raw `rg` output into the matrix without a `Pattern` sentence per row

**What the agent should use instead**

| Technique | Purpose |
|-----------|---------|
| `rg` / grep on patterns below | Triage — find candidate files per slice |
| Read hit file + trace callers (`rg` reverse, read imports) | Understand collapse chain |
| Read `facade.rs` / `mod.rs` re-exports | Find public API surfaces Phase 2 will break |
| Read struct/trait definitions | Find authoritative `String` fields and `-> String` methods |
| Follow type names (`TruthCommitIdentity`, `ForgeQueryMutationReceipt`) | Find mint, store, compare sites |
| Slice order below | Bounded context — do not scan entire workspace in one pass |

### Scan slices (work in this order)

1. **Relational birth** — `forge-relational/src/presentation/bridge/`, `facade.rs`
2. **Bridge carry** — `forge-runtime-bridge/src/identity.rs`, `facade/`, `exports_core.rs`
3. **Query intake** — `forge-query/.../contracts.rs`, adapter harnesses, `bridge_backed.rs`
4. **Query store** — `memory_workspace/`, `runtime_identity.rs`, live patch / delta structs
5. **Query consume** — write receipt, `shared_read_pins/`, `shared_read.rs`, `state.rs`
6. **Query lower** — `from_bridge`, `runtime_helpers.rs`, `bridge_mutation_lowering.rs`, `*_evidence.rs`
7. **Query route** — `intent/receipt.rs`, `backend/receipts.rs`, signal invalidation receipts
8. **Production topo** — `worth-topo/.../write_authority.rs`, `bridge_source.rs`, signal sink
9. **Cert / hostile** — `hostile_certification*`, causal materialization, journal gap helpers
10. **Downstream** — `worth-kernel`, `forge-server`, `hadwiger-research`, any crate importing `Truth*Identity` or building `ForgeQueryMutationReceipt`

### Search targets (grep triage — then read)

**Display-string mint (relational + adapters)**

- `format!("commit-` / `format!("commit:` / `format!("patch-` / `format!("patch:`
- `format!("entity:` / `format!("relation:` / `format!("relational-snapshot`
- `record_ref_identity` / `record_identity` returning or feeding `String`

**Bridge identity construction and exposure**

- `TruthCommitIdentity::new` / `TruthSnapshotIdentity::new` / `TruthPatchIdentity::new` / `TruthBranchIdentity::new`
- `BridgeIdentity::new` / `Into<Arc<str>>` on truth tags
- `impl From<` for truth identity types
- `.as_str()` on `Truth*Identity` (especially followed by `.to_string()`)
- `parse_bridge_commit` / `parse_bridge_snapshot` / `parse_bridge_*`

**Query authoritative storage (Phase 2 gate surfaces)**

- `commit_identity: String` / `snapshot_token: String` / `entity_identity: String` on structs
- `commit_identity()` / `snapshot_token()` returning `&str` or `String`
- `snapshot_token_from_runtime` / `snapshot_token(&self)`

**Query adapter contracts**

- `fn snapshot_token(&self) -> String`
- trait methods taking `_snapshot_token: &str` or `commit_identity: &str` for routing
- `.as_str().to_string()` at adapter boundaries

**Query consumption and lowering**

- `from_bridge` functions copying bridge identity to `String`
- `write_receipt` / inspection digests taking string commit/snapshot
- `shared_read_pins` / generation handles keyed on `snapshot_token: String`
- `hash_parts` on signal or invalidation receipts **without** `RuntimeBridge::route`

**Text parsing and comparison (journal / certification)**

- `rsplit('-').parse()` / `split('-')` on `commit_identity`
- `assert_eq!(…commit_identity…, "` string literal comparisons
- `TruthPatchIdentity::new(format!(…commit_identity`

**Routing and signal**

- `TopologyStaticSignalSink` / `StaticSignalSink`
- `RuntimeBridge::route` — note call sites that **don't** use it but should
- `SignalInvalidationRoutingReceipt` / invalidation receipt construction

**Cross-crate import fan-out (after spine passes)**

- `use forge_runtime_bridge::…Truth*Identity`
- `use forge_query::…ForgeQueryMutationReceipt` / `memory_workspace`
- `ForgeQueryRuntimeSourceAdapter` / `ForgeQueryRuntimeBackend` impl blocks

### Structural types to read (not just string patterns)

Even when grep is quiet, read definitions and public fields on:

- `TruthCommitIdentity`, `TruthSnapshotIdentity`, `TruthPatchIdentity`, `TruthBranchIdentity`, record/target identity types
- `ForgeQueryMutationReceipt`, `ForgeQueryMutationDelta`, `ForgeQueryLivePatch`
- `ForgeQueryWriteReceipt` and preview write receipt types
- `ForgeQueryRuntimeSourceAdapter`, `ForgeQueryRuntimeBackend`, bridge-backed adapter traits
- Relational `CommitId`, `EntityId`, `SnapshotId`, `VersionId` → bridge envelope lowering functions

### Matrix `Pattern` column (what to write per row)

One short sentence: **what collapses typed identity to string here**. Examples:

- `format!("commit-{}", commit_id.0)` at envelope export
- `pub commit_identity: String` authoritative field on mutation receipt
- `snapshot_token(&self) -> String` on source adapter trait
- `TruthCommitIdentity::new("commit-a")` at worth-topo write path
- `commit_identity().rsplit('-').parse()` for journal gap count
- `hash_parts` receipt without `RuntimeBridge::route`

**Matrix rules**

- One row per path (file or named function/struct within a file if multiple
  distinct collapse patterns).
- Multi-file rows are allowed only when every listed path has the same fix phase,
  the same collapse pattern class, and must be flipped to `Fixed` together; split
  the row instead of partially fixing a bundle.
- `Fix Phase` = which Fix Order phase owns the fix (3–9).
- `Status`: `Open` → `Fixed` when that phase lands; `Deferred` only with named
  owner milestone from **Out of scope**.
- **Two row kinds:** (1) *path rows* — one file or named function; (2) *feeder
  bundle rows* (Phase 7 rows 572–577) — whole upstream spine. A path row may be
  `Fixed` while its feeder bundle stays `Open` until hostile Phase 7 QA returns
  `CLEARED` for that bundle. Do not mark a feeder bundle `Fixed` from row-scoped
  scans or path-row closure alone.
- Update the matrix in the **same PR** as each fix phase — no drift between doc
  and code.
- Phase 1 is incomplete while any in-scope spine row is missing or still `TBD`.

**Done when**
- [x] **Collapse Matrix** (below) populated: every in-scope path has a concrete
  row — no `TBD` placeholders on the truth-routing spine
- [x] Matrix `Phase 1 scan` row or header records scan date and `Closed`
- [x] Deferred paths explicitly marked `Deferred` with owner milestone, not omitted
- [x] Workspace still green; **no** public API deleted; **no** code changes
  required in Phase 1 identity-lowering deliverable (doc-only)

**Common mistakes**
- Agent dumps `rg` results into the matrix without reading call chains.
- Agent builds a Rust inventory module instead of updating this doc — spec violation.
- Agent starts Phase 2 before the matrix covers relational export + bridge facade +
  query contracts + receipts (the break will be blind).

---

### Phase 2 — hard break (delete facade string APIs; workspace RED)

**What this phase is:** delete the **public string entrypoints** at the two
facade choke points so downstream crates cannot compile against folklore
constructors or string receipt fields. Capture every resulting error in the
exposure report. **Success = workspace red.**

**What this phase is not:**
- Not fixing any call site (worth-topo, harnesses, relational export, etc.).
- Not adding typed replacements for deleted APIs.
- Not making `cargo check --workspace` green.
- Not incremental refactoring (“fix bridge tests while we're here”).
- Not parity tests proving old string behavior still works somewhere.

**The two gates (only these surfaces change in Phase 2)**

| Gate | Crate / paths | Delete or privatize |
|------|---------------|---------------------|
| **Bridge facade** | `forge-runtime-bridge/src/facade/`, `identity.rs`, `exports_core.rs` | Public `TruthCommitIdentity::new(&str)` / `::new(String)` and siblings for `TruthSnapshotIdentity`, `TruthPatchIdentity`, `TruthBranchIdentity`. Public `as_str()`. `From<&str>` / `From<String>`. `BridgeIdentity::new` for truth tags at public boundary. |
| **Query facade** | `forge-query/src/memory_workspace/mod.rs`, `runtime/backend/contracts.rs`, live-patch/mutation-delta structs | Public `commit_identity: String`, `snapshot_token: String`, `entity_identity: String`. `ForgeQueryRuntimeSourceAdapter::snapshot_token(&self) -> String`. `ForgeQueryRuntimeBackend::snapshot_token(&self) -> String`. `&str` accessors that expose folklore. |

**Allowed inside gate crates only (so the crate itself can still build tests):**
- `pub(crate)` mint paths inside `forge-runtime-bridge` / `forge-query` — **not**
  callable from `worth-topo`, `worth-kernel`, `forge-server`, etc.
- Internal `#[doc(hidden)]` helpers if needed for in-crate compile-fail tests.

**Dependency-frontier rule**

If the bridge hard gate makes `forge-relational` red before `forge-query`
can compile, Phase 2 still must install the `forge-query` compile-fail fixtures
and expected stderr for the query facade gates. The report must state that the
fixtures are not executable until the compile frontier advances through Phase 3
relational lawful mint and the Phase 5/6 query adapter/internal impl drift. This
is not a deferral of the query gate: the query public fields and adapter trait
methods must already be deleted/privatized in Phase 2.

**Exposure report (required artifact)**

After gates land, run `cargo check --workspace 2>&1` and write
`_docs/forge-query/bridge_truth_identity_exposure_report.md`:
- group errors by crate (`worth-topo`, `forge-query` harnesses, `forge-relational`, …)
- note error kind: `E0603` private field, `E0599` missing method, struct literal field
  mismatch, etc.
- **cross-check against Collapse Matrix** — any compile error not already a matrix
  row gets added to the matrix; the matrix stays authoritative
- this is a **one-time break catalog**, not a progress scoreboard

**Done when**
 - [x] Bridge gate: external crate cannot call `TruthCommitIdentity::new("commit-1")`
  — compile-fail ui test passes
- [x] Query gate: external/harness struct literals with `commit_identity: String`
  fail compile — compile-fail ui test passes
  - Phase 2 installed query trybuild fixtures and expected stderr behind the
    intentional `forge-relational` red dependency frontier.
  - "Passes" here means the Phase 2 gate contract is installed; execution is
    the Phase 5/6 hard follow-up gate once `forge-query` can run trybuild.
 - [x] Query gate: `snapshot_token() -> String` removed from adapter traits
 - [x] Exposure report written with full workspace error listing
 - [x] `test-requirements.md` row: `Bridge Truth Identity Hard Exposure Gate Test`
- [x] **`cargo check --workspace` is red** — if green, gates are too weak

**Phase 2 → Phase 3 handoff**

Phase 3 begins only after Phase 2 done criteria are met. Query trybuild
fixtures are intentionally installed but not executable while `forge-relational`
is red; executing them is a hard follow-up gate before Phase 5/6 certification,
not a Phase 2 completion condition. Phase 3 fixes
**relational lawful mint** first because that is where typed identity is born;
do not “fix worth-topo first” to get green early — that reintroduces incremental
refactoring.

**Common mistakes**
- Fixing compile errors in the same PR as the gates → spec violation.
- Adding `commit_identity_typed()` beside `commit_identity: String` → dual API.
- Making gates `#[cfg(test)]` only → downstream production stays stringly.
- Weakening gates because harness count is high → exposure report should be large.

### Phases 3–9 — fix (whole-slice replacement only)

Per phase: **delete** the slice's string path and **replace** with typed
identity in one change. Run crate tests for that slice.

**Required every fix phase:** string folklore removed from matrix rows owned by
that phase; set those rows to `Fixed` in the Collapse Matrix in the same PR.

**Forbidden every fix phase:** incremental refactoring, parity tests, round-trip
string comparisons, keeping old behavior behind `cfg`, “prove identical envelopes
across repeated export,” or landing typed accessors beside authoritative string
fields.

**Phase 3** — `forge-relational/src/presentation/bridge/`, `facade.rs`

**Phase 4** — `forge-runtime-bridge/src/identity.rs`, facade exports

**Phase 5** — `contracts.rs`, harness adapters under `forge-query`

- [x] Query bridge-truth compile-fail lane executes and passes once the crate can
  compile far enough to run trybuild.
- [x] Bridge-backed runtime assembly requires a typed current-snapshot authority
  (`ForgeQueryRuntimeSnapshotIdentityAdapter`) instead of relying on erased source
  adapter tokens or a silent unavailable snapshot basis.
- [x] Preview stale-basis proof exercises backend-owned snapshot authority, not
  the removed `ForgeQueryRuntimeSourceAdapter::snapshot_token()` folklore path.
- [x] Signal routing fail-closed proof exercises the ordinary
  bridge-backed `write -> signal_sink.route_write_receipt` path with an
  authority-less mutation receipt.

**Phase 6** — `memory_workspace/`, write receipt surfaces, `shared_read_pins/`

**Phase 7** — evidence `from_bridge`, intent/receipt/inspection surfaces, runtime/backend receipts, causal inspection/materialization, live subscription/runtime session feeders, workflow/domain-capability/effect-lifecycle feeders, and bridge causal retained-mapping feeders

Phase 7 is not row-local. It is closed only when every ordinary covered surface
and every same-class upstream or adjacent feeder that can mint, lower, inspect,
retain, route, or report bridge truth/query evidence uses the canonical typed
identity boundary. This includes `subscription/`, `workflow/`,
`domain_capabilities/`, `effect_lifecycle/`, `runtime/inspection/causal/`,
`runtime/backend/receipts.rs`, `runtime/live_subscription.rs`,
`runtime/runtime_sessions.rs`, and `forge-runtime-bridge` causal-envelope
retained mapping/receipt feeders. No Phase 7 work may be certified complete by
row-scoped scans alone.

Phase 7 acceptance criteria:

- No authoritative Phase 7 production surface stores bridge truth/query evidence
  as `String`, `Arc<str>`, or `&str` while also treating it as identity.
- Digest/reporting accessors are allowed only as explicitly named projections
  such as `*_for_reporting`, never as the internal source of composition.
- Runtime/session/live-subscription/workflow/domain-capability/effect-lifecycle
  feeders must pass typed `ForgeQueryEvidenceIdentity`,
  `ForgeQuerySnapshotIdentity`, `BridgeIdentityEvidence`, or domain-specific
  typed artifact handles into downstream constructors instead of re-wrapping
  display strings.
- Bridge causal retained mapping must compose retained record evidence through
  typed retained-mapping evidence parts, not `hash_parts(...)` or external
  authority strings disguised as typed identity.
- Cursor Phase 7 QA (see **Phase 7 QA gate** below) must return `CLEARED` before
  Phase 7 is done. Phase 8 milestone work may not begin until Phase 7 QA is
  `CLEARED`, even if Phase 8 path rows are already code-complete.

**Phase 7 feeder bundles** (fix order within Phase 7; may run in parallel if paths
do not overlap):

| Bundle row | Primary paths |
|------------|---------------|
| subscription / live / sessions | `runtime/live_subscription.rs`, `runtime/backend/receipts.rs` (`SubscriptionActivationReceipt`), `runtime/runtime_sessions.rs`, `subscription/` |
| workflow / domain_capabilities | `workflow/lowering/writeback.rs`, `domain_capabilities/canonical_runtime/workflow/`, `domain_capabilities/payloads/workflow_semantics.rs`, `domain_capabilities/authoring/workflow.rs` |
| canonical_runtime artifacts | `domain_capabilities/canonical_runtime/continuity.rs`, `support.rs`, `artifacts.rs`, `invariant_capability.rs` |
| effect_lifecycle | `effect_lifecycle/` (normalization → lowering → `execution_bridge.rs`) |
| causal inspection (query) | `runtime/inspection/causal/request.rs`, `identity.rs`, `materialization/` |
| bridge causal envelope | `forge-runtime-bridge/src/diagnostics/causal_envelope/evidence_reference.rs`, `binding.rs`, `retained_mapping/digest_basis.rs`, `retained_mapping/retained_artifact_digest/` |

**Phase 7 done when**

- [x] All matrix rows with `Fix Phase = 7` are `Fixed`, including feeder bundles
  572–577 (not path rows alone)
- [x] Local gates clean for Phase 7 surfaces (at minimum):
  `cargo check -p forge-query --lib`, `cargo check -p forge-runtime-bridge --lib`,
  targeted tests for causal envelope, lower_runtime, identity_boundary as touched
- [ ] Phase 7 QA gate returns **`CLEARED`** (hostile review — see below)
- [ ] Matrix header records Phase 7 QA date and `CLEARED`

**Phase 8** — `worth-topo/.../write_authority.rs`, `bridge_source.rs`,
`TopologyStaticSignalSink` must call `RuntimeBridge::route` with typed commit
identity

Phase 8 path rows in the matrix may show `Fixed` when worth-topo code is
landed, but **Phase 8 is milestone-blocked** until Phase 7 QA is `CLEARED`.
Do not treat Phase 8 as complete for sequencing or closeout until then.

**Phase 9** — `hostile_certification*`, causal support; zero
`commit_identity().rsplit('-').parse()`; downstream crates and harnesses that
consume the typed receipt/query facade must compile without string shims before
Phase 10 closeout.

### Phase 10 — closure

- [ ] Phase 7 QA gate `CLEARED` (recorded in matrix header)
- [ ] `cargo check --workspace` green
- [ ] Compile-fail gate tests still pass
- [ ] Collapse Matrix: all in-scope rows `Fixed`; deferred rows have named owners
- [ ] `milestone-9.6-bridge-truth-identity-closeout.md`
- [ ] Status → `Closed`

## Phase 7 QA gate

Phase 7 closes only on **`CLEARED`** from a hostile QA pass — not when path rows
flip to `Fixed` or when `identity_boundary` regex scans report zero residue.

Run QA only after feeder-bundle local fixes and gates are clean. Use a separate
agent or `composer-2.5-fast` with **code inspection**, not tests alone.

**Hard bar (summary):** every ordinary Phase 7 surface and every same-class
upstream/adjacent feeder uses the canonical typed boundary internally. No strings
disguised as types, no string-first constructors/accessors as authoritative paths,
no `hash_parts` / string-join / `format!` digest folklore on covered ordinary
paths, no dual string/typed authoritative lanes, no production `cfg` escape hatch.
String output is allowed only as explicitly named `*_for_reporting` projections
backed by typed internal fields.

**Return format (exactly one):**

- `CLEARED:` — concise evidence Phase 7 satisfies the hard bar.
- `NOT CLEARED:` — numbered blockers with file paths and violation kind.

**QA prompt (copy verbatim):**

```text
You are doing skeptical QA for Forge Query milestone 9.6 Phase 7 only. Use the
current workspace as authoritative.

Authoritative spec:
_docs/forge-query/milestone-9.6-bridge-truth-identity-lowering.md
(Phase 7 section + feeder bundle table + matrix rows Fix Phase = 7)

Hard bar: Phase 7 is CLOSED only when every ordinary covered surface and every
same-class upstream/adjacent feeder uses the canonical typed boundary internally.
No strings disguised as types, no string-first constructors/accessors as
authoritative paths, no hash_parts/string join/format digest folklore in covered
ordinary paths, no dual string/typed authoritative lanes, no cfg/test escape
hatch in production. String projections acceptable only as *_for_reporting (or
equivalent explicit projection names) backed by typed internal fields.

Inspect these feeder bundles (matrix rows 572–577):
- subscription/live/session/backend receipts
- workflow + domain_capabilities workflow lowering
- domain_capabilities canonical_runtime continuity/support/artifacts
- effect_lifecycle spine
- runtime/inspection/causal request + materialization
- forge-runtime-bridge diagnostics/causal_envelope retained_mapping

Inspect code, not just tests. Be hostile. Return exactly one of:
- CLEARED: with concise evidence
- NOT CLEARED: numbered blockers with paths and why each violates the hard bar

Do not complain about Phase 9 harness/fixture folklore unless it is a
same-class upstream feeder for Phase 7 production surfaces.
```

**After QA:** if `NOT CLEARED`, add or update matrix rows for any new blockers,
fix feeder bundles, re-run local gates, re-run QA. Do not mark feeder bundles
572–577 `Fixed` until QA is `CLEARED`.

## Closure Gate

Closed only when:

1. No relational export formats truth IDs to display strings at ordinary paths.
2. No Query adapter or receipt surface accepts or exposes truth identity as `String`.
3. Production signal sink routes through bridge with typed commit identity.
4. Journal-order helpers do not parse commit identity text.
5. Phase 2 hard gates still enforced — no restored public string constructors.
6. No incremental-refactor debt: no dual string/typed paths, no `cfg` folklore,
   no parity fixtures left in tree.
7. Collapse Matrix in this doc is complete: no `Open` in-scope rows remain.
8. Phase 7 QA gate returned **`CLEARED`** and is recorded in the matrix header.

## Out of scope (other milestones)

`subscription/`, `workflow/`, `domain_capabilities/`, `effect_lifecycle/`, live
subscription/runtime session feeders, and causal inspection/materialization
feeders are **not** out of scope when they feed or consume a Phase 7 bridge
truth/query evidence boundary. Same-class upstream or adjacent feeders are Phase
7 work. Fix `worth-kernel`, `forge-server`, etc. only when matrix lists them; no
compat shims.

## Goal-Mode Loop Prompt

```text
Spec: _docs/forge-query/milestone-9.6-bridge-truth-identity-lowering.md

Read Strict Rules first. They override convenience.

1. Find first incomplete phase from repo state.
   - Phase 1 incomplete: Collapse Matrix has TBD/missing in-scope rows, or
     matrix header not Closed — agent must finish scan and update this doc first.
   - Phase 2 incomplete: gates not landed, or workspace is green — gates too weak.
   - Phase 7 incomplete: any Fix Phase = 7 feeder bundle row Open (572–577), or
     Phase 7 QA not CLEARED — path rows Fixed alone is insufficient.
   - Phase 8 blocked: do not start Phase 9 downstream harness cleanup claiming
     Phase 8 done until Phase 7 QA is CLEARED (Phase 8 path rows may already
     show Fixed in matrix).
   - Phase 3–6, 9+: fix matrix rows per phase; workspace may be red until Phase 10.
2. Phase 1: agent scan in slice passes (rg triage → read hit files + callers).
   Fill Collapse Matrix using Scan targets section — not include_str module, not
   grep-only rows. Doc-only. Workspace GREEN. No API deletes.
3. Phase 2: bridge + query facade gates only. Write exposure report. Workspace
   RED = success. Do not fix any compile errors. Do not add typed replacements.
4. Phases 3–6, 9: one whole slice per iteration — delete string path + typed
   replacement in the same change. No incremental refactoring. No parity tests.
5. Phase 7: fix feeder bundles (table in spec); path rows may be parallel across
   non-overlapping bundles. Run Phase 7 QA gate; stay on Phase 7 until CLEARED.
   Do not mark feeder bundles Fixed until QA CLEARED.
6. Phase 8: only after Phase 7 QA CLEARED (worth-topo may already be code-complete).
7. Phase 10: workspace green + compile-fail gates + closeout doc.

Banned always: incremental refactoring, parity/round-trip tests during refactor,
deprecated shims, dual APIs, cfg folklore, incremental validation suites,
restoring public string ctors, closing Phase 7 from row-scoped scans alone.
```

## Collapse Matrix (Phase 1 deliverable — authoritative inventory)

> **Phase 1 scan status:** `Closed` — agent scan completed on 2026-06-11;
> Cursor QA omissions corrected on 2026-06-11.
>
> **Phase 7 QA status:** `NOT CLEARED` — 2026-06-09 fifth hostile pass (code inspection). Pass 4 closed blockers 1–9 on the targeted spine but uneven depth remains across feeder bundles (blockers 1–9 below). Local test gates green (forge-runtime-bridge causal_envelope 26, subscription 155, workflow 76, effect_lifecycle 74, domain_capabilities 127, causal_inspection 61).
>
> **Phase 7 QA blockers (2026-06-09 pass 5):**
> 1. `subscription/evidence_identities.rs::lifecycle_certification_bundle_identity` — lifecycle delivery auxiliaries (performance, attachment, delivery_window, work_packet, closeout, etc.) still composed via `field_identity(&str)` from certification sequence-projection strings, not typed handles.
> 2. `subscription/evidence_identities.rs::{active_lane_identity,certification_activation_bundle_identity}` — `query_declaration_for_reporting` embedded via `field_identity` while typed `query_declaration_identity` exists upstream (dual string/typed lane on same field).
> 3. `runtime/live_subscription.rs::live_subscription_source_digest_evidence` — installation/counter evidence still wraps `counters.digest()` and other string sources through `field_identity(source_digest, …)` after typed counter `evidence_identity()` exists on the subscription spine.
> 4. `domain_capabilities/canonical_runtime/{artifacts,support,invariant_capability}.rs` — `canonical_runtime_request_identity` / `support_request_identity` still string-wrap `request_digest` with `field_identity` while target/binding use `field_evidence_identity` on the same materialization compose.
> 5. `domain_capabilities/canonical_runtime/workflow/{lowering,preview}.rs` — denial/preview materialization paths still use `field_identity(target, target_digest)` beside typed binding paths.
> 6. `effect_lifecycle/{planning,receipt,batch,authoring_basis}.rs` — production spine retains `hash_parts` compatibility digests and `field_identity` on `admitted_digest()` / `counters.digest()` / receipt strings beyond the normalized/lowering slice that pass 4 fixed.
> 7. `runtime/inspection/causal/materialization/artifacts/bridge_backed.rs` — dual authoritative accessors on typed fields (`artifact_identity()` + `artifact_digest()`, `causal_identity` + `causal_identity_digest()`, `bridge_envelope_digest()` beside `_for_reporting` siblings).
> 8. `runtime/inspection/causal/materialization/exploration.rs` — exploration path still calls `query_admission_digest()` / `bridge_envelope_digest()` after receipt API rename to `*_for_reporting()`, leaving mixed projection dialect.
> 9. `workflow/foundation.rs::{workflow_scope_digest_identity,preview bind helpers}` — binding-scope and preview-session evidence still compose via `field_identity` on raw string digests; only primary source/query/basis helpers were elevated to typed `field_evidence_identity`.
>
> **Phase 7 QA blockers (2026-06-09 pass 3–4 — resolved on targeted spine):**
> 1. ~~`subscription/performance_receipt.rs`~~ — typed `performance_receipt_identity`; reporting via `performance_receipt_for_reporting()`.
> 2. ~~`subscription/evidence_identities.rs`~~ — auxiliaries use `field_evidence_identity` for diagnostics, support, counters, future_selection, performance.
> 3. ~~`SubscriptionLifecycleCertificationBundle`~~ — dual typed `*_digest()` aliases removed; `_for_reporting` + `*_identity()` accessors only.
> 4. ~~`causal/materialization/receipt.rs`, `bridge_backed.rs`~~ — `*_for_reporting()` projection API aligned with proof.rs.
> 5. ~~`domain_capabilities/payloads/*`~~ — payload composition via `ForgeQueryEvidenceIdentity::compose`.
> 6. ~~`canonical_runtime/{artifacts,support,invariant_capability}.rs`~~ — target/request wired with `field_evidence_identity`.
> 7. ~~`effect_lifecycle/normalized.rs`~~ — capability/scoped-basis from typed `EffectAuthoringBasis` identities.
> 8. ~~`effect_lifecycle/lowering.rs`~~ — plan/artifact via `field_evidence_identity`.
> 9. ~~`workflow/foundation.rs`~~ — context source/query/basis accept typed identities.
>
> **Phase 8 milestone status:** `Blocked on Phase 7 QA` — Phase 7 QA is `NOT CLEARED` (pass 5); Phase 8 may not proceed per sequencing rules.
>
> **Last updated:** 2026-06-09
>
> This matrix is the **only** authoritative inventory for this milestone. The
> implementing agent fills it in Phase 1 by reading the codebase — not via a Rust
> `include_str!` module and not via grep-only. Each fix phase sets owned rows to
> `Fixed` in the same PR. Feeder bundle rows (572–577) require Phase 7 QA
> `CLEARED` before `Fixed`.
>
> **Slice-10 clean scan note:** `worth-kernel` and `forge-kernel` were scanned
> on 2026-06-11; no in-scope ordinary bridge truth-routing `Truth*Identity` or
> `ForgeQueryMutationReceipt` string collapse path was found. If the Phase 2
> exposure report surfaces a `worth-kernel` or `forge-kernel` compile break, add
> the concrete row before continuing the fix phases.

| Fix Phase | Crate | Path | Pattern | Status | Notes |
|-----------|-------|------|---------|--------|-------|
| 2 | forge-runtime-bridge | `src/identity.rs`, `src/facade/exports_core.rs`, `input/envelope/core.rs`, `snapshot/token.rs` | `TruthCommitIdentity`, `TruthPatchIdentity`, `TruthBranchIdentity`, and `TruthSnapshotIdentity` are public aliases of `BridgeIdentity<Tag>`, whose public `new`, `as_str`, `Display`, and `PartialEq<&str>` expose typed truth IDs as arbitrary text. | Fixed | Gate landed in Phase 2; internal lawful mint and typed storage remain Phase 4 work |
| 2 | forge-query | `memory_workspace/mod.rs` | `ForgeQueryMutationReceipt`, `ForgeQueryMutationDelta`, and `ForgeQueryLivePatch` expose `commit_identity`, `snapshot_token`, and `entity_identity` as public `String` fields. | Fixed | Gate landed in Phase 2; internal typed receipt replacement remains Phase 6 work |
| 2 | forge-query | `runtime/backend/contracts.rs` | `ForgeQueryRuntimeBackend::snapshot_token()` and `ForgeQueryRuntimeSourceAdapter::snapshot_token()` return `String`, and initialization helpers accept `snapshot_token: &str`. | Fixed | Gate landed in Phase 2; adapter implementation drift remains Phase 5/6 work |
| 3 | forge-relational | `presentation/bridge/identities.rs` | `record_ref_identity` formats relational `EntityId`/`RelationId` as `entity:*`/`relation:*`, `bridge_snapshot_identity_for_binding` formats snapshots as `relational-snapshot:*:version:*`, and parse helpers recover native IDs by splitting those strings. | Fixed | Relational birth now returns typed `RelationalBridgeRecordIdentityParts`; snapshot/commit recovery uses bridge-owned typed extractors |
| 3 | forge-relational | `presentation/bridge/patch_envelopes.rs` | `publication_patch_to_bridge_envelope` mints `TruthCommitIdentity::new(format!("commit-*"))`, `TruthPatchIdentity::new(format!("patch-*"))`, and takes branch/snapshot identities as `impl Into<String>`. | Fixed | Publication mint now accepts native `BranchId` and typed snapshot identity, and uses bridge-owned relational constructors |
| 3 | forge-relational | `presentation/bridge/patch_envelopes.rs` | `publication_bundle_to_bridge_envelope` and `commit_envelope_to_bridge_envelope` call `bridge_snapshot_identity_for_*().as_str().to_string()` before rewrapping the value as `TruthSnapshotIdentity`. | Fixed | Bundle/commit envelope lowering carries `TruthSnapshotIdentity` directly |
| 3 | forge-relational | `presentation/bridge/runtime_source/branch_heads.rs` | `TruthBranchIdentity.as_str().to_string()` becomes relational `BranchId`, and branch-head errors compare/report string branch identity. | Fixed | Branch recovery uses `relational_branch_id()` and diagnostics avoid opaque bridge payload access |
| 3 | forge-relational | `presentation/bridge/runtime_source/committed_patches.rs` | `request.commit_identity().as_str()` feeds `parse_bridge_commit_identity`, which strips `commit-` and parses `CommitId`. | Fixed | Commit recovery uses `TruthCommitIdentity::relational_commit_id()` through the relational bridge helper |
| 3 | forge-relational | `presentation/bridge/runtime_source/snapshot_authority.rs` | `parse_bridge_snapshot_identity` splits `TruthSnapshotIdentity.as_str()` on `:` to recover `SnapshotId` and `VersionId`. | Fixed | Snapshot authority uses `relational_snapshot_parts()` and reports native snapshot/version values |
| 3 | forge-relational | `presentation/bridge/snapshot_reading.rs` | Snapshot reader calls `parse_bridge_record_identity(read.entity_identity())` and reports snapshot/record identity through string accessors. | Fixed | Read path requires bridge-carried typed relational record parts and reports native record labels after typed conversion |
| 3 | forge-relational | `presentation/bridge/runtime_source/continuity_lineage.rs` | Continuity lineage converts branch identity with `.as_str().to_string()`, parses prior slice `entity_identity()` text, formats `lineage:*`, and reuses `record_ref_identity` for resolved records. | Fixed | Branch/lineage/resolved record minting now uses bridge-owned relational typed constructors |
| 3 | forge-relational | `presentation/bridge/test_catalog.rs` | `PublicationBridgeCatalog` accepts branch/snapshot identities as `impl Into<String>`, indexes committed patches and snapshots by `Truth*Identity.as_str().to_string()`, and services requests by erased commit/branch/snapshot identity text. | Fixed | Catalog accepts native branch plus typed snapshot and indexes by typed bridge identities |
| 3 | forge-relational | `grouped_truth/canonical_digest.rs` | Grouped-truth row-set and grouped-projection digests encode `TruthSnapshotIdentity.as_str()` into canonical digest bytes, treating typed snapshot identity as arbitrary display text. | Fixed | Canonical digest basis encodes typed relational snapshot parts and typed relational row identities instead of bridge payload text |
| 3 | forge-relational | `facade.rs` | Public `bridge` facade re-exports the string-collapsing bridge helpers as the supported relational bridge API. | Fixed | Public bridge helpers now expose typed relational signatures and typed snapshot identity constructors |
| 3 | forge-runtime-bridge | `relational_identity.rs`, `input/envelope/canonical.rs`, `snapshot/packet.rs` | Relational bridge export needed a typed carrier through bridge patch items and snapshot reads, otherwise relational consumers would keep parsing `entity_identity()` text. | Fixed | Bridge carries `RelationalBridgeRecordIdentityParts` beside compatibility entity text for Phase 3 ordinary relational paths; Phase 4 owns removing/further sealing generic bridge storage |
| 3 | forge-runtime-bridge | `routing/surfaces.rs`, `routing/lowering/slices.rs`, `routing/planning/canonical.rs`, `continuity/requests/prior_slice.rs`, `facade/runtime/continuity_planning.rs` | Route planning and continuity planning previously carried only erased entity text into snapshot reads and prior slices. | Fixed | Planned route, subscription slice, snapshot read, and continuity prior-slice surfaces preserve typed relational record parts through the relational spine |
| 4 | forge-runtime-bridge | `src/identity.rs` | Generic `BridgeIdentity<Tag>` stores only `Arc<str>` and permits arbitrary text construction/exposure for every truth identity tag. | Fixed | Core constructor/exposure are crate-private, debug is opaque, and equality/hash/order use typed payload semantics instead of display text when typed payloads exist |
| 4 | forge-runtime-bridge | `input/envelope/core.rs`, `snapshot/token.rs` | Truth identity type aliases inherit `BridgeIdentity<Tag>` public text constructors rather than nominal constructors from relational artifacts. | Fixed | Public truth identity construction is via typed relational bridge constructors/extractors; public string mint/access is compile-fail guarded |
| 4 | forge-runtime-bridge | `src/source/async_declaration/writeback/staging.rs` | Async writeback staging falls back to `TruthCommitIdentity::new("bridge-async-writeback-missing-truth")` and `TruthSnapshotIdentity::new("bridge-async-writeback-missing-snapshot")` in production staging when admitted writeback input omits truth identities. | Fixed | Staging now fails closed on missing authoritative truth basis instead of minting placeholder truth identities |
| 4 | forge-runtime-bridge | `src/facade/exports_core.rs` | Facade re-exports truth identity aliases and request/standard-path types so downstream crates can keep constructing truth IDs from strings. | Fixed | Facade can still name typed truth handles but downstream crates cannot mint/expose them through string constructors/accessors |
| 4 | forge-runtime-bridge | `src/facade/tests/`, `src/harness/`, `src/builder/tests/`, `src/input/envelope/construction_tests.rs` | Bridge-internal tests and harnesses construct `Truth*Identity::new("...")` and `Truth*Identity::new(format!(...))` as ordinary fixture setup. | Fixed | Ordinary bridge fixtures use typed fixture/relational helpers and typed in-memory source keys; the only raw constructor is a named malformed validation artifact quarantine |
| 5 | forge-query | `runtime/backend/contracts.rs` | Phase 2 removed the public `snapshot_token() -> String` and declaration-initialization `snapshot_token: &str` gates; Phase 5 must replace stale adapter implementations and call sites with typed snapshot routing rather than restoring string methods. | Fixed | Runtime internals now route through `current_snapshot_identity()`; the default is an explicit unavailable authority marker, while concrete test/runtime backends that own state provide typed snapshot evidence. Text projection remains only at Phase 6-owned compatibility surfaces |
| 5 | forge-query | `runtime/backend/contracts.rs::ForgeQueryRuntimeSignalSinkAdapter` | Signal sink adapter default methods build `SignalInvalidationRoutingReceipt::from_mutation_receipt(receipt)` and `SignalInvalidationBoundaryReceipt::from_mutation_receipt(...)`, making string commit/snapshot receipt routing the ordinary adapter fallback. | Fixed | Default routing now returns `Result` and fails closed unless the mutation receipt carries bridge-authored authority |
| 5 | forge-query | `runtime/backend/bridge_backed.rs` | Bridge-backed backend still contains stale call sites such as `&self.snapshot_token()` after Phase 2 removed the string trait method; Phase 5 must route typed snapshot authority instead. | Fixed | Bridge-backed backend no longer implements or calls the erased snapshot-token trait method; bridge-backed assembly now requires the typed `ForgeQueryRuntimeSnapshotIdentityAdapter` seam instead of silently installing an unavailable snapshot authority |
| 5 | forge-query | `runtime/tests/support/adapters/`, `runtime/tests/support/stateful_bridge_runtime/`, bridge-backed assembly fixtures | Adapter fakes and fixtures construct `Truth*Identity::new("...")`, return string snapshot tokens, and compare receipt string fields; known feeder paths include `runtime/tests/support/stateful_bridge_runtime/` and bridge-backed fixture adapters. | Fixed | Phase 5 adapter fixtures no longer restore deleted adapter snapshot-token methods or string-fed bridge-authority helper calls; stale-basis proof now uses backend-owned typed snapshot authority instead of source-adapter folklore, write-path signal routing rejects authority-less receipts, and broader harness receipt/envelope folklore remains explicitly owned by Phase 9 rows |
| 6 | forge-query | `basis/mod.rs` | `ResolvedSnapshotIdentity` stores `snapshot_token: String`, exposes `snapshot_token() -> &str`, and hashes `format!("snapshot:{}", self.snapshot_token)`. | Fixed | Resolved basis proof now derives from `ResolvedSnapshotIdentity` typed evidence identity and `BasisDigest::from_evidence_identity(...)`; no production `BasisDigest::from_parts(...)` remains |
| 6 | forge-query | `query_basis_lifecycle/intent.rs` | `RawBasisSelector` carries branch, snapshot, commit, and preview identities as `String`, and `compute_raw_digest` formats `commit_identity:*` / `snapshot_identity:*`. | Fixed | Raw basis selectors carry `RawBasisIdentity` typed handles backed by `ForgeQueryEvidenceIdentity` or `BridgeIdentityEvidence`; raw digest composition uses `ForgeQueryEvidenceIdentity` scope fields |
| 6 | forge-query | `query_basis_lifecycle/binding.rs` | Bridge lower-runtime evidence references store record, selector, route, continuity, subscription, and snapshot identities as `String` and format them into digest parts. | Fixed | Bridge lower-runtime evidence references store `BridgeIdentityEvidence`; binding digest composition is isolated in `binding_evidence.rs` and composes typed evidence identity |
| 6 | forge-query | `memory_workspace/workspace.rs` | `snapshot_token()` returns `String`, receipts use `format!("commit-*")`, and delete/update APIs accept `entity_identity: &str`. | Fixed | Memory workspace now exposes `snapshot_identity() -> ForgeQuerySnapshotIdentity`, receipts carry `ForgeQueryCommitIdentity`/`ForgeQuerySnapshotIdentity`, and update/delete APIs require `ForgeQueryEntityIdentity` |
| 6 | forge-query | `memory_workspace/runtime_identity.rs` | `snapshot_token_from_runtime` stringifies `TruthSnapshotIdentity`, uses a string sentinel for empty state, formats entity IDs as `entity:*`, and parses entity strings back to `EntityId`. | Fixed | Runtime identity helpers now construct typed relational snapshot/entity handles and recover native `EntityId` from typed relational record parts |
| 6 | forge-query | `runtime/surface/mutation/write_receipt/mod.rs` | `ForgeQueryWriteReceipt` wraps `ForgeQueryMutationReceipt` and stores declared/target entity identity as `Option<String>`. | Fixed | Write receipt wraps typed mutation receipts, caches typed evidence identities, and stores declared/target entity handles as `ForgeQueryEntityIdentity` |
| 6 | forge-query | `runtime/surface/mutation/write_receipt/accessors.rs` | Public `commit_identity() -> &str`, `snapshot_token() -> &str`, `declared_entity_identity() -> Option<&str>`, and `target_entity_identity() -> Option<&str>` expose truth identity as text. | Fixed | Write receipt accessors expose `ForgeQueryCommitIdentity`, `ForgeQuerySnapshotIdentity`, `ForgeQueryEntityIdentity`, and explicit evidence identities instead of string truth IDs |
| 6 | forge-query | `runtime/surface/mutation/write_receipt/helpers.rs` | Retained assertion evidence takes `snapshot_token: &str`, so assertion verification is still keyed by an erased snapshot token. | Fixed | Existing-truth assertion verification now passes `ForgeQuerySnapshotIdentity` into `ForgeQueryVerifiedExistingTruthAssertion` and `ForgeQueryVerifiedAssumptionSet` |
| 6 | forge-query | `runtime/surface/mutation/write_receipt/preview.rs` | Preview write receipts accept `snapshot_token: String`, synthesize `preview_write_receipt_identity(...)` strings as commit/entity identities, and copy binding resolved entity identities by `to_string()`. | Fixed | Preview write receipts now accept typed snapshot evidence, compose `ForgeQueryCommitIdentity::preview`, and build preview/entity delta handles through `ForgeQueryEntityIdentity` |
| 6 | forge-query | `runtime/surface/mutation/batch_receipt.rs` | Batch receipts derive `write_commit_identity` by iterating `ForgeQueryWriteReceipt::commit_identity` and seal a string-derived batch digest. | Fixed | Batch receipt digest composes `ForgeQueryEvidenceScope::BatchWriteReceipt` from each component receipt's typed commit evidence identity |
| 6 | forge-query | `runtime/surface/mutation/command.rs` | Mutation commands store insert/update/delete `entity_identity` and verification `resolved_entity_identity` as `String`, then expose declared entity identity as owned text. | Fixed | Mutation command entity fields and declared identity accessors use `ForgeQueryEntityIdentity`; graph/batch builders require typed entity handles |
| 6 | forge-query | `runtime/mutation/binding/existing_truth.rs` | Existing-truth bindings store resolved entity identity as `String` and seal binding/denial digests from formatted authoritative/resolved/collection text. | Fixed | Existing-truth target/binding constructors require `ForgeQueryEntityIdentity`; resolved target accessors return typed handles and digests use typed evidence projection |
| 6 | forge-query | `runtime/shared_read.rs`, `runtime/shared_read_pins/`, `runtime/runtime_authoritative_mutation_routing.rs`, `runtime/read_composition_runtime.rs` | Shared read/generation paths capture, retire, and compare `snapshot_token` strings from write receipts and runtime snapshot tokens, including generation digests formatted as `shared-read-generation:{snapshot_token}`. | Fixed | Shared-read generation IDs, registry capture/retire, stale-basis checks, authoritative mutation routing, and read-composition runtime materialization checks now carry `ForgeQuerySnapshotIdentity`; generation digests use `ForgeQueryEvidenceScope::SharedReadGeneration` |
| 6 | forge-query | `runtime/error.rs::SharedReadStaleBasis` | Shared-read stale-basis errors carry `snapshot_token: String` and render that erased snapshot token into runtime error messages. | Fixed | `SharedReadStaleBasis` carries `ForgeQuerySnapshotIdentity`, and stale-basis construction preserves the captured typed handle |
| 6 | forge-query | `runtime/surface/live_read_receipt.rs`, `runtime/surface/unified_inspection_receipt.rs`, `runtime/surface/derived_inspection_receipt.rs`, `runtime/surface/derived_materialization_receipt.rs`, `runtime/surface/existing_truth_probe_receipt.rs` | Read, inspection, materialization, and probe receipts store `snapshot_token: String` and expose it through `snapshot_token() -> &str`. | Fixed | Receipt spine now stores typed snapshot identity/evidence handles and exposes typed snapshot evidence; Phase 7 intent/inspection adapters remain separately tracked |
| 6 | forge-query | `runtime/surface/live_artifact_bundle.rs`, `runtime/surface/derived_materialization_bundle.rs`, `runtime/workspace_queries.rs` | Live and derived bundles store `snapshot_token: String`; workspace query aggregation collects `receipt.snapshot_token().to_string()` from read results and builds bundle digests from `format!("snapshot:{snapshot_token}")`. | Fixed | Live/derived bundles retain `ForgeQuerySnapshotIdentity`; bundle digests compose typed snapshot evidence instead of formatted token text |
| 6 | forge-query | `runtime/surface/read_receipt_construction.rs`, `runtime/surface/read_composition.rs` | Read receipt construction and composition pass `snapshot_token: String`, expose snapshot tokens by `&str`, and mix snapshot text into read/composition digests. | Fixed | Read construction/composition routes typed snapshot identity through read receipts and removes `snapshot_token()` compatibility accessors |
| 6 | forge-query | `runtime/surface/verified_assumption_set.rs` | Verified assumptions accept `snapshot_token: &str`, store `assumption_snapshot_token: String`, and seal assumption digests from formatted snapshot text. | Fixed | Verified assumptions store `ForgeQuerySnapshotIdentity`, cache only canonical evidence projection for legacy access, and derive assumption snapshot digest from the typed evidence identity |
| 6 | forge-query | `runtime/runtime_reads_programs.rs` | Runtime read programs expose `snapshot_token(&self) -> String`, record write receipt commit identities as strings, and format replay/live/derived trace identifiers from text. | Fixed | Runtime snapshot token API removed; traces record typed commit identities and only project evidence strings at trace-label boundaries |
| 6 | forge-query | `projection_consumption/extraction/write_receipt.rs`, `projection_consumption/extraction/mod.rs` | Projection consumption validates source identity against `receipt.commit_identity()`, records write fact source identity as `receipt.commit_identity().to_string()`, copies resolved target entity identity by `str::to_string`, and passes read receipt `snapshot_token()` text into extraction context. | Fixed | Write/read extraction validates against typed commit/snapshot evidence, carries target/resolved identities as `ForgeQueryEntityIdentity`, and removes receipt snapshot-token extraction |
| 6 | forge-query | `projection_consumption/source/constructors.rs`, `projection_consumption/source/mod.rs`, `projection_consumption/consumed/facts.rs`, `projection_consumption/contracts.rs` | Projection source and consumed-fact contracts preserve `source_identity`, `entity_identity`, and receipt-derived identities as strings, then seal projection contract/fact digests from formatted source/entity identity text. | Fixed | Projection source identities use typed source handles; consumed entity/target/relation identity facts carry typed entity handles and digest from typed evidence identities |
| 6 | forge-query | `runtime/computed/surface.rs`, `runtime/computed/routing.rs`, `runtime/computed/refresh_context.rs` | Computed view patches and refresh contexts store `commit_identity` / `snapshot_token` as `String` and bind receipt commit strings into derived patch state. | Fixed | Computed patch/refresh surfaces retain `ForgeQueryCommitIdentity` and typed snapshot evidence; no Phase 6 `commit_identity: String` / `snapshot_token: String` remains |
| 6 | forge-query | `runtime/effect/delivery.rs`, `runtime/effect/routing.rs`, `runtime/runtime_intents.rs` | Effect deliveries and routing compare or clone receipt `commit_identity` strings into delivery state and pending intent matching. | Fixed | Effect delivery/routing uses typed commit identities for matching and delivery state; no receipt commit string clone remains in the Phase 6 slice |
| 6 | forge-query | `runtime/delivery.rs`, `runtime/state.rs` | Delivery and runtime state format patch identities and status details from receipt `commit_identity`, `declared_entity_identity`, and downstream string digests. | Fixed | Delivery/state surfaces route typed receipt/entity identities and reserve formatted details for display-only diagnostics |
| 6 | forge-query | `runtime/workspace.rs`, `runtime/workspace_submission.rs`, `runtime/runtime_declarations.rs` | Workspace-facing mutation APIs accept `entity_identity: impl Into<String>` and expose `snapshot_token() -> String` through runtime boundaries. | Fixed | Workspace mutation APIs require typed entity identities and runtime/workspace `snapshot_token()` APIs were removed |
| 6 | forge-query | `declarative_live.rs` | Public declarative live query session APIs accept `snapshot_token: impl Into<String>` and feed that erased token into `ResolvedSnapshotIdentity::new(...)` for live session basis declaration. | Fixed | Declarative live basis intake no longer accepts erased snapshot token strings in the Phase 6 surface |
| 7 | forge-query | `intent_admission/handoffs/bindings/mod.rs` | Intent handoff bindings store `trigger_commit_identity: String`, expose it as `&str`, compare it against pending delivery commit text, and seal handoff binding digests with formatted `commit:{pending_delivery.commit_identity()}` parts. | Fixed | Effect-triggered execution binding stores `ForgeQueryCommitIdentity`, compares typed commit handles, and hashes pending delivery from commit evidence identity |
| 7 | forge-query | `intent_admission/eligibility/seeds/generic_inspection.rs` | Generic inspection seeds build inspection labels and seed digests from `receipt.commit_identity()` / `receipt.snapshot_token()` text for write receipts and other receipt-derived admission evidence. | Fixed | Generic inspection seeds compose `GenericInspectionIntentSeed` evidence identities and use receipt/commit/snapshot evidence handles instead of receipt token text |
| 7 | forge-query | `intent_admission/eligibility/seeds/mutation.rs` | Mutation admission seeds format declared entity identity text as `entity:{declared_entity_identity}` and include binding/resolved symbolic identity strings in authoritative mutation intent input digests. | Fixed | Mutation intent and batch seed digests compose dedicated evidence identity scopes; hostile delimiter tests cover typed entity evidence and component seed composition |
| 7 | forge-query | `application/declaration_bridge_routing/lower.rs` | Declaration bridge lowering mints `TruthBranchIdentity`, `TruthCommitIdentity`, and `TruthSnapshotIdentity` from formatted query declaration/basis digests such as `query-branch:*`, `query-commit:*`, `query-snapshot:*`, and passes them into `BridgeTruthViewSelector` / `BridgeRouteRequest`. | Fixed | Declaration bridge lowering uses bridge-owned typed truth constructors and stable numeric relational commit/snapshot identity derivation instead of raw truth string constructors |
| 7 | forge-query | `application/declaration_bridge_routing/lower.rs::lower_writeback_declaration` | Writeback declaration lowering builds `TruthCommitIdentity::new(format!("query-trigger:{...}"))` and `TruthSnapshotIdentity::new(...)` from query causality/evaluation/basis digests before bridge writeback execution. | Fixed | Writeback declaration lowering routes through typed causality/route identities plus typed query truth commit/snapshot helpers; declaration bridge routing tests cover the path |
| 7 | forge-query | `effect_lifecycle/execution_bridge.rs` | Effect lifecycle bridge execution constructs `TruthCommitIdentity` from causality digest text and `TruthSnapshotIdentity` from evaluation snapshot / basis digest strings before calling `RuntimeBridge::execute_admitted_writeback`. | Fixed | Effect lifecycle bridge writeback execution now uses typed policy/causality/route identity constructors and stable typed truth commit/snapshot derivation |
| 7 | forge-query | `effect_lifecycle/execution_relational_scalar.rs` | Relational scalar execution compares expected `runtime_snapshot_token()` text against `current_branch_snapshot_token()` and parses branch IDs from binding digest text for freshness checks. | Fixed | Workflow mutation bindings now carry typed `ForgeQuerySnapshotIdentity` and `BranchId`; scalar execution consumes those typed handles directly, removes binding-digest branch parsing, and regression tests cover the boundary plus branch-scoped execution |
| 7 | forge-query | `continuation_pipeline/execution/readmission.rs` | Continuation readmission copies `request.commit_identity().to_string()` and bridge selector commit/snapshot identities via `.as_str().to_string()` into `ForgeQueryPreparedContinuationBasisWitness`. | Fixed | Prepared continuation basis witnesses and readmission observations now retain typed `ForgeQueryEvidenceIdentity` handles; bridge commit/snapshot paths derive typed Query evidence identities and tests guard against digest-string helpers returning |
| 7 | forge-query | `runtime/bridge_mutation_lowering.rs` | Bridge lowering APIs accept `resolved_target_entity_identity: Option<&str>` and rebuild continuity/naming evidence from string target identities. | Fixed | Bridge mutation lowering accepts typed `ForgeQueryEntityIdentity`, lowers only relational-record handles through `BridgeHistoricalResolvedRecordIdentity::from_relational_record`, rejects authored Query evidence strings for native bridge target slots, and regression tests cover both paths |
| 7 | forge-query | `runtime/surface/naming_mutation_evidence.rs` | `from_bridge` and `from_intent` copy attachment, prior/target authoritative, resolved entity, and collection identities into `String` fields with `.to_string()`. | Fixed | Naming evidence now stores typed mutation authority/collection handles plus typed resolved entity identity; bridge evidence is enriched with query-native typed target context when bridge-native identity cannot encode it |
| 7 | forge-query | `runtime/surface/continuity_mutation_evidence.rs` | `from_bridge` copies prior/successor authoritative identities, resolved target entity identity, lineage, and continuity digests into `String` fields; `from_intent` still hashes string successors. | Fixed | Continuity evidence now stores typed authority, target collection, resolved entity, and mutation evidence digest handles; intent digests compose `ForgeQueryEvidenceIdentity` instead of `hash_parts(...)` |
| 7 | forge-query | `runtime/surface/symbolic_target_reference_evidence.rs` | Symbolic target reference evidence stores `symbol`, `resolved_entity_identity`, and optional collection as `String` copied from bridge/reference inputs. | Fixed | Symbolic target reference evidence stores typed symbol, resolved entity, and collection handles; bridge bundles require a typed query-context fallback rather than accepting raw resolved-identity strings |
| 7 | forge-query | `runtime/surface/symbolic_aspect_resolution_evidence.rs` | Symbolic aspect resolution evidence stores `resolved_entity_identity: String` via `impl Into<String>` and exposes it as `&str`. | Fixed | Symbolic aspect resolution evidence stores typed symbol/collection handles and a `ForgeQueryEntityIdentity`; batches with symbolic aspect references use query-side typed resolution instead of backend atomic string resolution |
| 7 | forge-query | `runtime/surface/graph_composition_resolution_map.rs`, `runtime/surface/graph_composition_evidence.rs` | Graph composition resolution maps store `resolved_entity_identity: String`, then graph composition evidence seals symbolic-resolution digests from formatted `entry.resolved_entity_identity()` text. | Fixed | Graph composition resolution maps retain typed symbols, target collections, and entity identities; graph symbolic-resolution digests compose typed evidence identities |
| 7 | forge-query | `runtime/surface/mutation_evidence/binding.rs`, `runtime/surface/mutation_evidence/target.rs` | Mutation evidence binding/target artifacts store authoritative and resolved entity identities as `String` copied from bridge binding bundles and expose those values through `&str` accessors. | Fixed | Mutation binding/target evidence stores typed authority, collection, digest, and entity handles and exposes typed accessors; display strings are explicit edge projections only |
| 7 | forge-query | `runtime/surface/mutation_evidence/provenance.rs`, `runtime/surface/mutation_evidence/causality.rs` | Mutation provenance/causality evidence copies bridge contract, writeback, feedback, causality, route, evaluation, and truth-view digests into string fields with `to_string()`. | Fixed | Mutation provenance/causality evidence stores `ForgeQueryMutationEvidenceDigest` handles for bridge digest inputs and exposes typed digest accessors |
| 7 | forge-query | `runtime/surface/mutation_evidence/batch_digest_helpers.rs` | Batch mutation-evidence digest helpers format declared/resolved entity identity text from target, binding, symbolic reference, continuity, and naming evidence into batch digest parts. | Fixed | Batch mutation-evidence helpers compose aggregate `ForgeQueryEvidenceIdentity` values from typed target, binding, symbolic, naming, continuity, provenance, and causality evidence handles |
| 7 | forge-query | `runtime/inspection/unified/write_receipt.rs`, `runtime/inspection/unified/component.rs` | Write receipt inspections copy `commit_identity`, `snapshot_token`, and entity identities into string fields. | Fixed | Write receipt/component inspections retain typed commit, snapshot, declared entity, and target entity handles; text projection is limited to explicit evidence/reporting edges |
| 7 | forge-query | `runtime/inspection/unified/batch_write.rs` | Batch write receipt inspection collects `commit_identities: Vec<String>` from `entry.commit_identity().to_string()` and preserves batch receipt identity as text in the inspection artifact. | Fixed | Batch write inspection retains typed commit identity handles across entries and components instead of string collections |
| 7 | forge-query | `runtime/inspection/unified/write_receipt/digest.rs`, `runtime/inspection/unified/batch_write_digest.rs` | Digest components compose identity fields named `commit_identity`, `snapshot_token`, and `entity_identity` from string accessors. | Fixed | Write/batch receipt digest helpers compose typed receipt evidence identities and only flatten values inside explicit evidence encoder sequences |
| 7 | forge-query | `runtime/intent/branch.rs`, `runtime/inspection/intent.rs` | Authoritative/effect/preview intent receipt inspection identities now compose typed receipt, commit, snapshot, trigger-commit, basis, and admission identities; the remaining intent inspection gap is branch/basis snapshot routing that still carries `basis_snapshot_token` text and adjacent non-receipt inspection surfaces that still lower typed identity too early. | Fixed | Branch intent receipts and inspections carry typed basis snapshot identities and compose branch/basis inspection evidence through `field_evidence_identity` |
| 7 | forge-query | `runtime/inspection/causal/receipt.rs` | Causal inspection receipts copy `inspection.commit_identity()` and `inspection.snapshot_token()` into evidence references/tags as text rather than retaining typed write/read receipt handles. | Fixed | Causal write receipt consumers use typed commit, snapshot, and entity evidence identities until the bridge evidence-reference boundary |
| 7 | forge-query | `runtime/inspection/causal/materialization/` | Causal materialization receipts and proofs seal query admission, anchor, bridge receipt, and materialization identities through string-formatted digest parts, leaving no typed bridge truth handle boundary for receipt-derived evidence. | Fixed | Causal materialization receipt, proof, reference, performance, denial, and temporal digests compose `ForgeQueryEvidenceIdentity`; materialization fixtures preserve requested typed snapshot identity |
| 7 | forge-query | `runtime/intent/receipt.rs`, `runtime/intent/effect_triggered.rs` | Intent receipts now carry typed commit/snapshot evidence identities and effect-triggered receipts compose the nested authoritative intent receipt plus typed trigger commit evidence identity instead of copying write/effect receipt identity strings. | Fixed | Intent route receipts carry typed receipt identity |
| 7 | forge-query | `runtime/intent/provenance.rs`, `runtime/intent/provenance_identity.rs`, `runtime/intent/receipt.rs`, `runtime/intent/receipt_identity.rs`, `runtime/intent/effect_triggered.rs`, `runtime/inspection/preview/intent_receipt.rs`, `runtime/inspection/preview/intent_receipt_identity.rs` | Intent provenance now accepts typed snapshot evidence identities for authoritative/effect-triggered write-backed receipts, shared snapshot-token callers must pass through an explicit typed evidence adapter, authoritative/effect-triggered intent receipt digests compose nested write receipt and provenance identities, and preview intent receipt inspection digests compose typed basis/admission/receipt identities. | Fixed | Intent provenance and receipt identity boundary |
| 7 | forge-query | `runtime/intent/denial.rs` | Intent denial evidence clones `execution.mutation_receipt().snapshot_token` into `Option<String>`, exposes it as `Option<&str>`, and includes it as a `snapshot_token` evidence identity tag. | Fixed | Denial evidence and denial inspection retain `ForgeQuerySnapshotIdentity` plus typed snapshot evidence identity; no `snapshot_token()` denial accessor remains |
| 7 | forge-query | `runtime/intent/failure.rs` | Intent execution failure evidence clones `execution.mutation_receipt().snapshot_token` into a `String`, exposes it as `&str`, and seals failure digests with formatted `snapshot:{snapshot_token}` text. | Fixed | Failure evidence stores typed snapshot identity/evidence and composes `IntentExecutionFailureEvidence` instead of formatted snapshot text |
| 7 | forge-query | `runtime/intent/execution.rs` | Intent execution placeholder outcomes synthesize `ForgeQueryMutationReceipt` with `commit_identity: String::new()` and `snapshot_token: snapshot_token.into()` for invariant-violation executions. | Fixed | Placeholder/noop/invariant execution constructors require typed commit/snapshot handles at the boundary and no longer mint receipt identity from strings |
| 7 | forge-query | `runtime/inspection/feedback.rs` | Feedback inspection stores `trigger_commit_identity: String`, accepts trigger commit as `&str`, exposes it as `&str`, and seals feedback graph digests from formatted `trigger-commit:{trigger_commit_identity}`. | Fixed | Feedback inspection carries typed trigger commit evidence identity through graph and inspection identities; effect intent inspection asserts the wrapper does not collapse to write commit identity |
| 7 | forge-query | `runtime/backend/receipts.rs` | `SignalInvalidationRoutingReceipt` stores `commit_identity`/`snapshot_token` as `String`, formats digest inputs as `commit:{commit_identity}` and `snapshot:{snapshot_token}`, and compares against string receipt fields. | Fixed | Signal invalidation routing receipt stores and drift-checks typed commit/snapshot handles, exposes only typed `receipt_identity()`, and the lower-runtime signal boundary consumes that typed identity rather than a receipt-digest string accessor |
| 7 | forge-query | `runtime/runtime_writes.rs` | Runtime writes use `backend.snapshot_token()` for synthetic receipts and pass `&receipt.commit_identity` / `&receipt.snapshot_token` into intent execution provenance. | Fixed | Runtime writes now feed synthetic assertion receipts from `current_snapshot_identity()` and preserve typed commit/snapshot evidence through write provenance |
| 7 | forge-query | `runtime/runtime_read_intents.rs`, `runtime/runtime_unified_inspection_intents.rs` | Runtime read and unified-inspection intent routers clone `backend.snapshot_token()` into read/inspection receipts and propagate receipt `snapshot_token()` text into intent evidence. | Fixed | Live read/materialized posture now receives typed snapshot evidence identity; unified inspection surfaces are clean under the row erasure scan |
| 7 | forge-query | `runtime/runtime_sessions.rs` | Runtime session setup passes `backend.snapshot_token()` into session basis and subscription setup, carrying erased snapshot text across session boundaries. | Fixed | Runtime session basis setup carries `ForgeQuerySnapshotIdentity`; row scan found no remaining snapshot-token projection in sessions |
| 7 | forge-query | `runtime/effect/inspection.rs` | Effect inspection formats `delivery.commit_identity()` into feedback-phase inspection digests, preserving trigger commit identity as text in effect inspection evidence. | Fixed | Effect inspection identity composition moved to `runtime/effect/inspection_identity.rs` and composes delivery trigger commit evidence identity directly |
| 7 | forge-query | `runtime/preview/mod.rs`, `runtime/preview/basics.rs`, `runtime/preview/workflow_ops.rs`, `runtime/preview/session_execution.rs`, `runtime/preview/mutation_ops.rs` | Preview sessions store `basis_snapshot_token: String`, compare promotion snapshot token strings, create preview write receipts from `runtime.snapshot_token()`, and record preview trace write receipts from `receipt.commit_identity().to_string()`. | Fixed | Preview route, promotion, closeout, and execution paths carry typed snapshot/source evidence identities; trace write receipt recording keeps typed `ForgeQueryCommitIdentity` |
| 7 | forge-query | `preview/scoped.rs` | Scoped preview inspection routes preview observation basis through `basis().identity().snapshot_token()` and uses erased snapshot text as the scoped basis label. | Fixed | Scoped preview now derives `RawBasisIntent::runtime_snapshot` from typed snapshot identity and compares typed `NormalizedBasisSubject`, not scope-label text |
| 7 | forge-query | `runtime/preview/evidence/promotion.rs`, `runtime/preview/evidence/closeout.rs`, `runtime/preview/evidence/execution.rs` | Preview evidence artifacts store basis/promotion snapshot tokens and preview commit identity as `String` and seal them into evidence identities. | Fixed | Promotion/closeout/execution evidence store typed snapshot and source evidence identities; token/commit-string accessors were removed |
| 7 | forge-query | `runtime/inspection/preview/outcome.rs` | Preview outcome inspection stores preview and target basis snapshot tokens as `String`, exposes them as `&str`, and includes them as evidence identity tags. | Fixed | Preview outcome inspection stores typed basis, closeout, residue, rebinding, and snapshot identities; digest getters are report projections only |
| 7 | forge-query | `runtime/branch.rs`, `runtime/intent/branch.rs` | Branch and intent branch sessions store basis snapshot tokens as `String` and expose them by `&str`. | Fixed | Branch basis and branch intent receipts carry typed `ForgeQuerySnapshotIdentity` and typed receipt/evidence identities |
| 7 | forge-query | `runtime/runtime_batch_writes.rs`, `runtime/runtime_helpers.rs`, `runtime/runtime_probe_routing_intents.rs`, `runtime/runtime_inspection_materialization_intents.rs` | Runtime helper paths synthesize aggregate commit identities with `format!` from child receipt commit strings and propagate string snapshot tokens through batch/probe/materialization flows. | Fixed | Batch/helper/materialization flows compose typed commit/snapshot identities; materialization bundle consistency compares typed snapshot handles and helper budget digest uses `RuntimeSubscriptionBudget` evidence scope |
| 7 | forge-query | `view_shape_live/grouped_execution.rs` | Fixed: grouped execution compares bridge snapshot identity through `ForgeQueryEvidenceIdentity`, grouped bridge row-set materialization preserves typed relational record identity for projection parity, and grouped fixtures derive query basis identity from the same typed bridge snapshot instead of matching display labels. | Fixed | View-shape grouped execution snapshot boundary |
| 7 | forge-query | `lower_runtime_routing/adapters/runtime_backend.rs`, `lower_runtime_routing/plans/mod.rs` | Fixed: write-authority lower-runtime routing binds mutation commit evidence as `ForgeQueryEvidenceIdentity`, signal invalidation subjects compose from typed routing receipt/commit/snapshot identities, `ForgeQueryLowerRuntimeCapabilityRequest` requires `ForgeQueryLowerRuntimeSubjectIdentity`, and `ForgeQueryLowerRuntimeRoutePlan` now requires `ForgeQueryLowerRuntimeRouteSubjectIdentity` instead of accepting raw route-subject strings. | Fixed | Production lower-runtime routing adapter |
| 7 | forge-runtime-bridge | `src/diagnostics/records/route_entry.rs`, `src/diagnostics/state/` | Fixed: route diagnostics now carry `BridgeRouteRecordEntityIdentity` (`RelationalRecord` or `TruthSurface`) with a hard-broken constructor/accessor API; route diagnostic state indexes route, invalidation, continuity, and source commit lookups by typed bridge/truth identities; JSON export performs explicit diagnostic-label projection instead of treating canonical identity as `String`. | Fixed | Adjacent bridge diagnostic feeder |
| 7 | forge-query | `subscription/`, `runtime/live_subscription.rs`, `runtime/runtime_sessions.rs`, `runtime/backend/receipts.rs` | Subscription activation/live-installation/runtime-session feeders store activation, declaration, basis, signal, support, counter, and budget identities as strings or digest-only wrappers before feeding runtime receipt and inspection surfaces. | Fixed | Feeder bundle — typed `ForgeQueryEvidenceIdentity` spine via `subscription/evidence_identities.rs`; activation receipt and live installation consume typed identities; drift compares evidence handles |
| 7 | forge-query | `workflow/`, `workflow/lowering/`, `domain_capabilities/payloads/workflow_semantics.rs`, `domain_capabilities/authoring/workflow.rs`, `domain_capabilities/canonical_runtime/workflow/` | Workflow/domain-capability lowering stores authority binding, basis, causality, runtime-preflight scope, preview declaration, and target binding evidence as digest strings before lowering mutation/writeback/preview bridge evidence. | Fixed | Feeder bundle — `WorkflowContextBinding` stores typed binding/source/query/basis identities; writeback lowering and canonical_runtime workflow paths compose evidence identities |
| 7 | forge-query | `domain_capabilities/canonical_runtime/continuity.rs`, `domain_capabilities/canonical_runtime/support.rs`, `domain_capabilities/canonical_runtime/artifacts.rs`, `domain_capabilities/canonical_runtime/invariant_capability.rs` | Canonical runtime materialization and support artifacts derive continuity/support/materialization/invariant identities from target binding digest strings and `hash_parts(...)` instead of typed contribution target evidence. | Fixed | Feeder bundle — artifacts/support/invariant materialization compose typed contribution target evidence |
| 7 | forge-query | `effect_lifecycle/` | Effect lifecycle normalization, lowering, batch lowering, and execution bridge paths preserve lower-runtime authority binding and workflow mutation/writeback evidence as strings before calling workflow lowering and bridge execution. | Fixed | Feeder bundle — normalized/lowering/batch/receipt paths compose typed workflow binding and capability evidence |
| 7 | forge-query | `runtime/inspection/causal/`, `runtime/inspection/causal/materialization/` | Causal inspection request failures, materialization receipts/proofs, bridge references, and receipt-derived evidence can collapse typed query/bridge evidence into formatted strings or value sequences. | Fixed | Feeder bundle — causal identity wrappers drop `AsRef<str>` dual API; composition uses `field_evidence_identity`; reporting via `*_for_reporting()` |
| 7 | forge-runtime-bridge | `src/diagnostics/causal_envelope/`, `src/diagnostics/causal_envelope/retained_mapping/` | Bridge causal evidence references, bindings, retained-record lookup, receipts, and retained mapping helpers expose or compose reference/binding/retained identities through digest strings, external-authority wrappers, or string projections. | Fixed | Feeder bundle — removed `retained_mapping_identity_digest_part` double-wrap; typed bridge/external/evidence parts; envelope identity/receipt accessors renamed to `*_for_reporting()` |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters/write_authority.rs` | Production write authority builds `ForgeQueryMutationReceipt` with `format!("commit-*")` and `bridge_snapshot_identity_for_commit(...).as_str().to_string()` for single and batch writes. | Fixed | Write authority now builds mutation receipts from `ForgeQueryCommitIdentity` / `ForgeQuerySnapshotIdentity` derived from relational commit parts. Milestone-blocked until Phase 7 QA CLEARED |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters/write_support.rs`, `query_rows.rs` | Topology query rows/deltas format and parse `entity:*`/`relation:*` identities as strings for mutation targets and live rows. | Fixed | Live rows and mutation deltas now carry `ForgeQueryEntityIdentity` with relational record parts; endpoint labels are explicit payload projections only |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters/bridge_source_support.rs` | Topology bridge source parses `commit-*`, `relational-snapshot:*:version:*`, and `entity:*`/`relation:*` strings back into relational IDs. | Fixed | Bridge source support extracts relational commit, snapshot, and record parts from typed bridge/query identities |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters/bridge_source.rs` | Bridge source calls `request.commit_identity().as_str()`, compares branch/snapshot identity text, and reads snapshot packets by parsing `read.entity_identity()`. | Fixed | Bridge source now resolves branch/commit/snapshot/record authority through typed relational payload accessors |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters/binding.rs` | `TopologyRuntimeBinding::snapshot_token()` mints erased snapshot text by calling `bridge_snapshot_identity_for_commit/handle(...).as_str().to_string()` and falling back to a string sentinel for empty state. | Fixed | Runtime binding exposes `current_snapshot_identity() -> ForgeQuerySnapshotIdentity` and preserves typed relational snapshot parts |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters/declaration_initialization.rs` | Declaration initialization accepts `snapshot_token: &str`, compares it against `bridge_snapshot_identity_for_handle(...).as_str()`, and reports mismatch details using snapshot identity text. | Fixed | Declaration initialization no longer accepts or compares snapshot-token text; metadata derives from typed read basis |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters.rs::TopologyRuntimeSourceAdapter` | Source adapter implements `snapshot_token() -> String` by forwarding binding snapshot text. | Fixed | Source adapter implements `ForgeQueryRuntimeSnapshotIdentityAdapter` and returns typed snapshot identity |
| 8 | worth-topo | `projection/runtime_boundary/read_execution/basis_context.rs` | Historical read execution stores `HistoricalSnapshot { snapshot_token: String }`, passes snapshot token by `&str` into preflight/materialization, and builds `QueryBasisContextRequest::historical_snapshot(snapshot_token)` from erased text. | Fixed | Historical read execution stores `ForgeQuerySnapshotIdentity`; evidence-label projection is explicit at lower compatibility edges |
| 8 | worth-topo | `projection/read_views/domain/handle_reads.rs` | Read-handle entry copies `workspace.snapshot_token().to_string()` into `TopologyReadExecutionTarget::historical_snapshot(...)` as an owned erased snapshot token. | Fixed | Handle reads pass `workspace.snapshot_identity()` into historical execution targets |
| 8 | worth-topo | `projection/read_views/domain/read_proof/report.rs`, `projection/read_views/domain/read_proof/report_surface.rs` | Read-proof reports store `executed_snapshot_token: Option<String>` from `receipt.snapshot_token().to_string()` and expose the executed snapshot token as `Option<&str>`. | Fixed | Read-proof reports store typed executed snapshot identity and expose diagnostic labels only through an explicit projection accessor |
| 8 | worth-topo | `projection/runtime_boundary/query_runtime/adapters.rs::TopologyStaticSignalSink` | Static signal sink uses the default `ForgeQueryRuntimeSignalSinkAdapter::build_signal_invalidation_routing_receipt` path, which derives `SignalInvalidationRoutingReceipt` from string commit/snapshot receipt fields instead of a typed bridge route identity. | Fixed | Static signal sink now routes typed receipts through bridge route identity construction before boundary receipt lowering |
| 8 | worth-topo | `certification/bridge.rs` | Bridge certification routes with `TruthCommitIdentity::new(format!("commit-{commit_id}"))`, branch identity from raw branch text, and stores route/snapshot/history identities as strings in proof rows. | Fixed | Bridge certification uses relational truth identity constructors and explicit evidence-label projections for proof rows. All Phase 8 rows: code may be landed; milestone sequencing blocked until Phase 7 QA CLEARED |
| 9 | forge-query | `correspondence/`, `historical/`, `view_shape_live/` test bridge fixtures | Test sources minted patch/head identities from request commit or branch evidence text and compared fixture snapshots by evidence label. | Fixed | Fixtures now use typed relational commit/patch fixture positions and typed snapshot-handle comparison; targeted scans reject commit/branch-derived patch/head folklore |
| 9 | forge-query | `harness/` test bridge/effect fixtures | Harness preflight/resolved-basis fixtures no longer accept or lower raw snapshot token text; `runtime_preflight_with_snapshot_identity`, `runtime_basis`, and `store_basis` require `ForgeQuerySnapshotIdentity`, and ordinary harness callers use relational snapshot fixture handles. Row scan confirms remaining harness `snapshot_token`/formatted bridge-harness truth IDs belong to later explicit rows. | Fixed | Harness folklore replacement |
| 9 | forge-query | `harness/fixtures/effect_authorities.rs`, `harness/fixtures/preview_bridge.rs` | Shared harness fixtures compute runtime snapshot tokens as strings and mint patch/head/snapshot/branch truth identities from request commit, branch text, or formatted preview snapshot seeds. | Fixed | Shared harness fixtures now expose typed `ForgeQuerySnapshotIdentity` handles, mint bridge patch/head/snapshot/branch identities through explicit relational constructors, and compare preview snapshots by typed identity rather than evidence-label text |
| 9 | forge-query | `harness/runtime_api_stabilization/transcript_runtime.rs` | Runtime API stabilization transcript fixtures construct mutation receipts with formatted transcript commit/snapshot strings and mint bridge patch/snapshot/branch identities from request commit or raw fixture text. | Open | Runtime API stabilization fixture folklore |
| 9 | forge-query | `harness/runtime_api_stabilization/transcript_runtime/transcript_authority.rs` | Transcript authority fixtures construct `ForgeQueryMutationReceipt` with formatted `transcript-commit:*` and `transcript-snapshot:*` strings and fixture entity identity text. | Open | Runtime API transcript authority fixture folklore |
| 9 | forge-query | `harness/aspect_api_finalization_certification/rows.rs` | Aspect API finalization certification rows use `receipt.commit_identity().to_string()` as receipt digest material for mutation certification rows. | Open | Aspect API certification receipt string consumer |
| 9 | forge-query | `tests/support/public_bridge_runtime/`, `tests/support/public_bridge_runtime/hostile_certification.rs` | Public bridge runtime test support implements string snapshot-token adapters, constructs `ForgeQueryMutationReceipt` with formatted `public-bridge-commit-*` / `public-bridge-snapshot-*` strings, and hostile certification digests embed `first.commit_identity()`, `second.commit_identity()`, and `artifact.snapshot_token()` text directly. | Open | Public bridge runtime test support |
| 9 | forge-query | `lower_runtime_routing/certification/surface/fixtures/` | Lower-runtime routing certification fixtures construct `ForgeQueryMutationReceipt` literals with string commit/snapshot fields and mint `Truth*Identity::new(...)` / patch identities from commit or branch text across core and phase-six fixtures. | Fixed | Lower-runtime certification fixture tree now composes typed evidence/relational truth identities; Phase 7 inventory scans cover the fixture/support feeder paths for `hash_parts(` and bridge harness-label regressions. |
| 9 | forge-query | `runtime/tests/causal_inspection/` | Causal inspection tests and support route with `TruthCommitIdentity::new("...")`, mint patch/head/snapshot identities from commit or branch text, and build writeback support with formatted `query-trigger:*` truth commit strings. | Open | Causal inspection test/support fixture tree |
| 9 | forge-query | `effect_lifecycle/certification/seeded/support.rs` | Seeded effect lifecycle certification support derives patch identity text from `commit_identity.as_str()` and constructs snapshot/branch truth identities from raw fixture strings. | Fixed | Seeded effect lifecycle support returns typed relational snapshot handles and constructs bridge patch/snapshot/branch identities with typed relational constructors |
| 9 | forge-query | `harness/milestone_eight_certification/` | Milestone-eight certification harness mints patch/head/snapshot/branch truth identities from request commit or branch text and raw fixture strings. | Open | Milestone-eight bridge harness folklore |
| 9 | forge-query | `projection_consumption/tests/`, `query_basis_lifecycle/tests/` | Projection consumption and query-basis lifecycle tests construct bridge truth snapshot/branch/commit identities from raw strings and formatted patch/head commit text. | Open | Query projection/basis fixture folklore |
| 9 | forge-query | `intent_admission/certification/fixtures/bridge.rs` | Bridge certification fixtures derive patch identity text from commit identity via `format!("patch:{}", commit_identity.as_str())` or request commit text. | Open | No old string folklore in bridge certification |
| 9 | forge-query | `intent_admission/certification/fixtures/runtime.rs`, `intent_admission/certification/fixtures/read.rs` | Intent admission certification fixtures still contain receipt/read folklore outside the Phase 5 adapter seam: intent-authority placeholder receipts construct formatted `certification-*` commit/snapshot identities, cloned receipt identity strings feed certification digests, and read fixtures pass `snapshot_token: &str`. | Open | Intent admission certification fixtures; the bridge-backed runtime adapter assembly itself was migrated in Phase 5 |
| 9 | forge-query | `runtime/tests/support/bridge/hostile_certification.rs::hostile_journal_gap_count` | Hostile journal helper calls `receipt.commit_identity().rsplit('-').next().and_then(|suffix| suffix.parse::<usize>().ok())`. | Open | Exact Phase 9 journal parse ban |
| 9 | forge-query | `runtime/tests/support/bridge/hostile_certification.rs::hostile_write_receipt_digest`, `runtime/tests/support/bridge/hostile_certification.rs::hostile_published_artifact_digest` | Hostile certification digest helpers seal digest parts from `receipt.commit_identity()`, `receipt.snapshot_token()`, and `artifact.snapshot_token()` string accessors. | Open | Hostile certification receipt/artifact digest folklore |
| 9 | forge-query | `runtime/tests/support/bridge/fixture.rs::native_patch_envelope` | Bridge fixture support derives harness patch identity from commit evidence labels and wraps snapshot, branch, and entity identities from raw fixture string literals. | Open | Bridge fixture native patch folklore |
| 9 | forge-query | `runtime/backend/receipts.rs` tests | Signal routing tests construct `ForgeQueryMutationReceipt { commit_identity: "commit-1".to_string(), snapshot_token: "snapshot-1".to_string(), ... }` and assert string equality. | Open | Compile-fail/typed tests must replace string literals |
| 9 | forge-relational | `presentation/bridge/bridge_source_tests/` | Bridge source tests mint `TruthCommitIdentity::new(format!("commit-*"))`, compare branch/snapshot identities by `as_str()`, and route committed patch requests from formatted commit text. | Fixed | Moved into Phase 3 because relational bridge-source certification is part of the ordinary relational spine; tests now use relational typed commit/branch/snapshot/record constructors and extractors |
| 9 | worth-topo | `projection/runtime_boundary/bridge/tests.rs` | Bridge tests call `.route(TruthCommitIdentity::new(format!(...)))` and compare route/record identities as strings. | Open | Hostile production bridge certification |
| 9 | worth-topo | `certification/support/read_proof_harness.rs`, `certification/projection_closeout/tests/topology_reads/` | Topology read-proof certification harnesses copy `workspace.snapshot_token().to_string()` into historical read execution targets and assert executed snapshot tokens as string values. | Open | Topology read certification snapshot consumers |
| 9 | worth-topo | `certification/projection_closeout/tests/derived_chain.rs` | Derived-chain certification asserts inspection `commit_identity()` against write receipt commit identity strings and carries topology surface identities as string fixtures. | Open | Topology derived-chain certification consumer |
| 9 | hadwiger-research | `tests/research_graph_invariants.rs` | Test write authority builds `ForgeQueryMutationReceipt` with `commit_identity: commit_identity.to_string()` and `snapshot_token: format!("{commit_identity}:snapshot")`. | Open | Downstream harness consumer |
| 9 | forge-ui | `src/todo/truth.rs` | Todo truth state stores `snapshot_token: String`, synthesizes workspace snapshot text from child snapshot tokens, and routes mutations through string entity identities. | Open | Downstream UI consumer |
| 9 | forge-server | `surfaces/compat_http/mutation_execution/request.rs` | Compat HTTP mutation requests parse `entity_identity` and `resolved_entity_identity` as JSON strings and build canonical request digests from formatted identity text. | Open | Downstream compat request feeder |
| 9 | forge-server | `forge_native/direct/mutation.rs`, `surfaces/compat_http/mutation_execution/response.rs` | Server mutation result digests use `receipt.commit_identity()` as a string result digest for single mutations. | Open | Downstream fix after Query typed receipts land |
| 9 | forge-server | `surfaces/compat_http/mutation_execution/query_execution.rs` | Compatibility precondition observes `handoff.workspace().snapshot_token()` as a string basis digest. | Open | Downstream snapshot basis consumer |
| 9 | forge-server | `tests/support/direct_context_runtime.rs`, `tests/support/query_handoff/runtime.rs`, `tests/support/compat_http/phase_three_runtime.rs`, `tests/support/compat_http/phase_four_runtime.rs` | Server test adapters implement `snapshot_token(&self) -> String` and construct `ForgeQueryMutationReceipt` with formatted commit/snapshot strings. | Open | Downstream harness consumer |
| 9 | forge-server | `tests/forge_native/direct_mutation.rs`, `tests/forge_native/direct_projection.rs` | Forge-native integration tests compare result/inspection digests to `receipt.commit_identity()`, assert `inspection.snapshot_token() == receipt.snapshot_token()`, and consume direct projection read receipt snapshot tokens as strings. | Open | Downstream forge-native integration tests |
| — | forge-runtime-bridge | `src/subscription/replay_tests.rs` | Subscription replay tests mint truth identities from string literals/formatted commit and patch text, but subscription replay is outside this milestone's ordinary truth-routing spine. | Deferred | Owner milestone: subscription replay typed identity milestone |

**Phase 1 complete when:** the agent has filled every `Pattern` column and added
all missing paths; header `Phase 1 scan status` → `Closed`; no in-scope row
remains `TBD` unless explicitly `Deferred` with owner milestone.

### Other artifacts (not the matrix)

| Artifact | When | Path |
|----------|------|------|
| Exposure report | Phase 2 | `_docs/forge-query/bridge_truth_identity_exposure_report.md` — compile errors after gates; cross-check against matrix, add surprise rows |
| Compile-fail gates | Phase 2 | `forge-runtime-bridge/tests/ui/`, `forge-query/tests/ui/` |
| Closeout | Phase 10 | `_docs/forge-query/milestone-9.6-bridge-truth-identity-closeout.md` |
