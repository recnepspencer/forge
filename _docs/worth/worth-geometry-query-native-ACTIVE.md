# Active Execution - Query-Native Hard Break

> **Read this first every session.** Full spec:
> [`worth-geometry-query-native-hard-break-spec.md`](worth-geometry-query-native-hard-break-spec.md)
>
> This file is the control board, not a journal. **Overwrite** transient notes
> each turn; do not append changelogs.

## Agent Rules

1. Do not claim the hard break is **100% finished** while any row in **Open
   Proof Debt** is red.
2. `geometry_hard_break_closeout` passing is necessary but **not sufficient**
   for full closure; the wider binding proof net must also be green.
3. Frozen passes stay closed unless a verification command regresses.
4. One slice per turn: fix proof debt, or report blocker with evidence.

---

## Overall Status

**`CLOSED` on current evidence**

| Signal | Result |
|--------|--------|
| Explicit closeout bundle (`geometry_hard_break_closeout`) | **Green** (2/2) |
| Full `binding::tests` net | **Green** (60 pass, 0 fail) |
| Architecture (dual runtime deleted) | **Closed** |
| Proof net completeness | **Closed** |

The hard break is closed on the current proof bundle. Reopen only if one of the
reopen conditions below regresses or a new query-native violation is found.

---

## Pass Progress

| Pass | Status | Note |
|------|--------|------|
| **A** Domain homes | **Closed** | Query domains in `worth-spatial` |
| **B** Typed receipts | **Closed** | Receipt + `from_bound_envelope` attachment |
| **C** Delete dual runtime | **Closed** | Post-`Bound` replay gone; rebinding projection receipt is derived from the declaration only on the bound projection fact path and attached with `from_bound_envelope` |
| **D** Retained views | **Closed** | Payload owner exists; digest-protocol cross-family proof is green |
| **E** Routing/recovery/projection | **Closed** | Route posture green; closeout bundle covers recovery/projection/grouped paths |
| **F** Kernel collapse | **Closed** | Kernel `binding/` is test-only; legacy deletion and boundary compile-fail nets are green |

---

## Current Focus

### Binding digest-protocol proof debt

**Status:** `CLOSED`

**Former failing test:**

- `binding::tests::binding_digest_protocol::canonical_binding_identity_digest_protocol_is_shared_across_kernel_spatial_and_retained_paths`

**Resolution:** rebinding prior identity now uses an explicit
`rebinding.prior.binding_identity` canonical key, so prior/source identity no
longer collides with direct binding identity lanes.

**Primary files:**

- `crates/worth-kernel/src/binding/tests/binding_digest_protocol.rs`
- `crates/worth-spatial/src/bindings/query_native_rebinding_declaration_support.rs`
- `crates/worth-spatial/src/bindings/query_native_rebinding_authoring.rs`
- `crates/worth-spatial/src/bindings/query_native_rebinding_projection_logic.rs`
- `crates/worth-spatial/src/bindings/query_native_retained_view_payload.rs`
- `crates/worth-spatial/src/bindings/query_native_historical_geometry_inspection.rs`

**Exit criteria:**

- [x] `cargo test -p worth-kernel binding::tests::binding_digest_protocol -- --nocapture` green
- [x] `cargo test -p worth-kernel binding::tests -- --nocapture` fully green (60 pass, 0 fail)
- [x] No change reintroduces post-`Bound` local semantic replay

---

## Frozen Proof Locks

### Pass C - dual runtime deleted

- `workflow_transport.rs` deleted
- `primitive_rebinding_projection_facts`: single orchestrate ->
  `declaration.projection_receipt()` -> `from_bound_envelope`
- `build_rebinding_fact_receipt` / `evaluate_replacement_candidates`: `#[cfg(test)]` only
- `PrimitiveRebindingDeclarationEntry` stores intent plus neighborhood seed, not
  a precomputed projection receipt sidecar

### Pass D/E - closeout bundle evidence

`geometry_hard_break_closeout.rs` proves one admitted Query-native story across
retained source, mutation evidence, neighborhood replacement, projection
consumption, historical/branch-local inspection, certification bundle, signal,
and continuation, plus typed denied-path recovery.

### Pass F - kernel production surface

- `crates/worth-kernel/src/binding/mod.rs`: test root only
- `crates/worth-kernel/src/lib.rs`: `binding` + `construction` under `#[cfg(test)]`
- Legacy deletion + compile-fail nets exist and are in the verification bundle
- `crates/worth-kernel/src/construction/phase_chain/admitted_scaffold/birth_proof_support.rs`
  remains `#[cfg(test)]`; it is not a production geometry runtime surface.

---

## Acceptance Matrix

Do not mark a row **closed** while its proof is red.

| Row | Status |
|-----|--------|
| Public entry | closed |
| Spatial runtime shape | closed |
| Workflow transport | closed |
| Ordinary outcomes | closed |
| Retained history | closed |
| Basis lifecycle | closed |
| Grouped locality | closed on closeout evidence |
| Contributions | closed on closeout evidence |
| Routing | closed (`route_posture` green) |
| Recovery | closed on closeout evidence |
| Mutation evidence | closed on closeout evidence |
| Projection consumption | closed on closeout evidence |
| Signal and continuation | closed on closeout evidence |
| Kernel role | closed |
| Legacy deletion | closed |
| **Full binding proof net** | **closed** (60 passing tests) |

---

## Verification Bundle

**Green closure gate:**

```text
cargo test -p worth-kernel binding::tests -- --nocapture
cargo test -p worth-kernel geometry_hard_break_closeout -- --nocapture
```

**Closeout + boundary net:**

```text
cargo check -p worth-spatial -p worth-kernel -p worth-topo
cargo test -p worth-kernel --test public_api_contract -- --nocapture
cargo test -p worth-kernel --test ui -- --nocapture
cargo test -p worth-kernel binding::tests::legacy_deletion -- --nocapture
cargo test -p worth-kernel binding::tests::rebinding::outcome_transport -- --nocapture
cargo test -p worth-kernel binding::tests::rebinding::diagnostics -- --nocapture
cargo test -p worth-kernel binding::tests::recovery_action -- --nocapture
cargo test -p worth-kernel binding::tests::inspection::historical_inspection -- --nocapture
cargo test -p worth-kernel binding::tests::inspection::branch_local_inspection -- --nocapture
cargo test -p worth-kernel binding::tests::inspection::replay_parity -- --nocapture
cargo test -p worth-kernel binding::tests::rebinding::grouped_workflow -- --nocapture
cargo test -p worth-kernel binding::tests::rebinding::contribution_workflow -- --nocapture
cargo test -p worth-kernel binding::tests::route_posture -- --nocapture
cargo test -p worth-kernel binding::tests::rebinding_projection_consumption_receipt -- --nocapture
cargo test -p worth-spatial --test public_api_contract -- --nocapture
cargo test -p worth-spatial --test ui -- --nocapture
cargo test -p worth-topo --test public_api_contract -- --nocapture
cargo test -p worth-topo --test ui -- --nocapture
```

**Former red row, now green:**

```text
cargo test -p worth-kernel binding::tests::binding_digest_protocol -- --nocapture
```

Expected: 2 passed, 0 failed.

---

## Reopen Conditions

Reopen a frozen pass only if:

- production geometry reintroduces post-progression local semantic replay
- production geometry reintroduces ordinary-outcome remapping from local replay
- retained views stop depending only on retained Query truth
- grouped/recovery/projection/signal/continuation fall back to shadow runtime
- `worth-kernel` regains production binding/construction runtime modules
- compile-fail fixtures on disk are not enforced
- `geometry_hard_break_closeout` regresses or is deleted

---

## One-Line Truth

Architecture closeout is real, the explicit Phase 9 bundle is green, and the
full binding proof net is green. The hard break is closed on current evidence.
