# Foundation Hardening QA Checklist (Phase 1 Follow-up)

Purpose: close the five foundation QA defects found after implementing the
audit/policy base layers, with production-grade contracts and adversarial tests.

Status rule:
- Do not mark an item complete until all sub-items are done.
- "Schema/type exists" is not sufficient.
- Each item needs contract docs, implementation, adversarial tests, and one
  composition proof.

Evidence rule:
- Every checked item must include:
  - `Code:` absolute file path(s)
  - `Tests:` test name(s)
  - `Command:` exact `cargo test` / `cargo check` command(s)
  - `Notes:` what failure mode is now prevented

---

## H1. `ModelingContext::absorb_sub_result` Must Be a True Drain

- [ ] Contract updated (drain semantics explicit, no mixed drain/borrow wording)
- [ ] Implementation drains all absorbed metadata from child `OperationResult`
- [ ] Adversarial test: double-absorb does not double-count warnings/metrics/lineage/budget
- [ ] Adversarial test: child envelope metadata is empty/default after absorb
- [ ] Composition test: `absorb_sub_result` + `take_sub_metadata` + parent envelope fold does not double-count

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## H2. `ModelingContext` Operation Reset Semantics Must Be Explicit and Correct

- [ ] API split/rename implemented (`clear_decision_log_only` vs `reset_for_new_operation`, or equivalent)
- [ ] Full reset contract documented (what resets vs what persists)
- [ ] `reset_for_new_operation` resets decision IDs, trace-drain state, and sub-op metadata sink
- [ ] Adversarial test: decision IDs restart at 1 after full reset
- [ ] Adversarial test: prior `take_decision_log()` does not suppress later error auto-persist after reset
- [ ] Composition test: policy/tolerance config persists across full reset (unless intentionally changed)

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## H3. Policy Trace Payload Numeric Validation/Comparison Semantics Must Be Explicit

- [ ] Contract updated: validator is strict-exact or domain-tolerant (one clearly chosen)
- [ ] Non-finite numeric values are rejected (`margin`, `threshold`, `query_location`)
- [ ] Adversarial test: NaN/Inf payloads rejected with typed errors
- [ ] Adversarial test: tiny delta mismatch behavior documented and tested
- [ ] Adversarial test: `-0.0` vs `0.0` behavior documented and tested
- [ ] Composition test: payload + `TracedDecision` consistency holds for real policy trace fixture

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## H4. `BoundarySegmentProvenance` Invariants Must Be Enforced

- [ ] Invariant strategy implemented (private fields + constructors/builders, or `validate()`)
- [ ] `transport_hash` consistency with endpoints/hash mode enforced
- [ ] `directed` semantics cannot drift silently (structural encoding or validation)
- [ ] Adversarial test: tampered payload is detected (or impossible via API)
- [ ] Adversarial test: endpoint/hash mismatch rejected/detected
- [ ] Composition test: boundary adapter emits provenance payloads that pass invariant checks

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## H5. Audit Field Naming Convention Validator Must Reject False Positives

- [ ] Validator upgraded from substring matching to strict suffix/token rule
- [ ] Contract docs updated with canonical examples
- [ ] Adversarial test: misleading names (`hashmap_*`, `snapshotted_*`, etc.) rejected
- [ ] Positive test: canonical names accepted (`*_snapshot`, `*_persistent`, `*_hash`)
- [ ] Composition test: real audit schema labels used by a feature record pass convention checks

Evidence:
- Code:
- Tests:
- Command:
- Notes:

---

## Final Gate (before Foundation Phase 2)

- [ ] All five items completed with evidence
- [ ] Targeted crates compile cleanly (`forge-core`, `forge-kernel`, `forge-io`)
- [ ] No new TODO/placeholder markers added outside curved allowlist
- [ ] QA pass confirms no mixed drain/borrow semantics remain in envelope/context absorption APIs

Evidence:
- Code:
- Tests:
- Command:
- Notes:
