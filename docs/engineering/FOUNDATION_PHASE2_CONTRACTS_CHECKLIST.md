# Foundation Phase 2 Contracts Checklist

Purpose: implement and verify the five Phase 2 contracts from
`FOUNDATION_PHASE2_CONTRACTS_SPEC.md` at production quality.

Rules:
- Do not mark items complete on schema-only changes.
- Each item requires:
  - contract/API implementation
  - integration into at least one real runtime path
  - adversarial tests
  - composition/round-trip tests
- Evidence is mandatory.

Evidence format (per checked item):
- `Code:` absolute file path(s)
- `Tests:` test name(s)
- `Command:` exact command(s)
- `Notes:` what failure mode/invariant is now covered

---

## P2-1. Trace Adjunct / Versioning Strategy

- [ ] Canonical adjunct record type(s) implemented with per-family versioning
- [ ] Deterministic ordering contract implemented for multi-adjunct attachments
- [ ] Adjunct attachment/transport integrated into trace persistence path (or trace store payload)
- [ ] Unknown adjunct kinds/versions preserved round-trip without semantic loss
- [ ] Typed policy adjunct uses adjunct contract (no ad hoc side-channel bypass)
- [ ] Adversarial test: contradiction with `TracedDecision` detected via typed validator
- [ ] Adversarial test: unknown adjunct version survives parse + re-emit
- [ ] Reader/view/store compatibility behavior for unknown adjuncts implemented and tested (`forge-view` or trace store path)

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## P2-2. Operation Finalization Contract

- [ ] `OperationFinalizer` (or equivalent) contract implemented
- [ ] Single-pass drain semantics enforced (re-entry safe / rejected)
- [ ] Separate success/error finalization APIs (no status-string finalization)
- [ ] Collect-vs-emit split implemented (deterministic collect phase, explicit I/O emit phase)
- [ ] Adjunct attachment integrated into finalization path (no split trace/adjunct finalization)
- [ ] State-hash semantics implemented and explicitly labeled (topology hash unless composite hash contract exists)
- [ ] Success-path finalization integrates context drains + adjuncts + envelope merge + trace/audit emission
- [ ] Error-path finalization preserves trace/audit parity with explicit semantics
- [ ] One top-level region-merge or boolean path migrated to finalizer contract
- [ ] Adversarial test: double-finalization cannot double-count metadata
- [ ] Adversarial test: error path still emits deterministic trace/audit summary
- [ ] Adversarial test: emit failure does not corrupt collected finalization summary
- [ ] Adversarial test: finalized adjunct ordering is deterministic

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## P2-3. Policy Registry / Config Source Model

- [ ] Typed source precedence model implemented (non-overridable/op/feature/model/session/default)
- [ ] `ModelingContext` policy resolution API backed by actual registry snapshot
- [ ] Source scope metadata serializable/traced (model key, feature/op IDs where available)
- [ ] Per-op override lifecycle bounded by finalization/reset (no leakage)
- [ ] Adversarial test: overlapping overrides resolve deterministically
- [ ] Adversarial test: missing policy fails closed with typed trace outcome
- [ ] Runtime integration: region merge certifier/policy path uses registry-backed resolution

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## P2-4. Persistent-Name Resolution Result Contract (Typed + Traced)

- [ ] Reusable typed resolution result contract implemented (`Resolved/Ambiguous/Missing`)
- [ ] Typed resolution trace adjunct payload implemented (P2-1 family)
- [ ] Candidate ordering deterministic and documented
- [ ] Snapshot vs persistent identity fields explicitly labeled in result payloads
- [ ] Region merge path consumes the reusable contract (no custom ambiguity enums)
- [ ] Adversarial test: ambiguous name fails closed (no first-match)
- [ ] Adversarial test: generation reuse/topology reorder cannot cause stale snapshot leakage
- [ ] Lineage/re-identification compatibility proven or typed incompatibility surfaced

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## P2-5. Replay / Audit Bridge Contract

- [ ] Replay bridge schema implemented with typed compatibility outcomes
- [ ] Distinguishes exact replay vs counterfactual-only compatibility
- [ ] Typed witness mapping implemented (no string parsing)
- [ ] Region merge audit artifact maps to replay bridge record or typed incompatibility
- [ ] Adversarial test: missing witness vs schema mismatch are distinct typed outcomes
- [ ] Adversarial test: deterministic bridge output for identical audit record
- [ ] Adversarial test: typed error summary preserved in bridge failure path

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## Phase 2 Gate (before broader epic rollout)

- [ ] `P2-1` and `P2-2` implemented + integrated + QA’d before `P2-3`
- [ ] No production path emits policy/provenance side data outside adjunct contract after `P2-1`
- [ ] No top-level migrated path uses ad hoc finalization after `P2-2`
- [ ] Vision alignment review completed (traceability, determinism, fail-closed semantics)
- [ ] Deferred provenance invalid-state note retained in spec until type-level refactor lands

Evidence:
- Code:
- Tests:
- Command:
- Notes:
