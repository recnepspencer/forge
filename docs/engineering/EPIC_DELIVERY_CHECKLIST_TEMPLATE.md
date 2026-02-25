# Epic Delivery Checklist (Template)

Use this checklist for any feature/epic before claiming completion.

Rules:
- Every checked item must include an indented `Evidence:` line directly below it.
- Evidence must reference concrete tests, files, or commands (no `TODO`/`TBD`).
- Leave non-applicable items unchecked and explain in the epic spec/review notes.
- Curved placeholders are allowed only in explicitly scoped curved modules during the current milestone.

## Contract

- [ ] Spec sections implemented and cross-checked against architecture inventory
  Evidence: <spec sections + code references>
- [ ] Hard invariants preserved (generational handles, fail-closed ambiguity, transactional updates)
  Evidence: <tests + code references>
- [ ] No raw phase-boundary escape hatches introduced (no split-brain topo/geom APIs)
  Evidence: <API diff / grep / review note>

## Policy / Tracing / Envelope

- [ ] Policy semantics are explicit (no policy-shaped trace labels without a real policy decision path, unless explicitly documented as temporary and scoped)
  Evidence: <code references + tests>
- [ ] Decision traces propagate to `ModelingContext` and returned envelope without loss
  Evidence: <tests>
- [ ] Sub-operation envelopes are absorbed before `.into_value()` / error mapping
  Evidence: <code references + tests>
- [ ] Warnings/metrics/lineage/error-budget propagation verified where applicable
  Evidence: <tests or invariants>

## Precision / Tolerance / Provenance

- [ ] Tolerance handling is explicit and fail-closed at ambiguity boundaries
  Evidence: <tests + trace assertions>
- [ ] Provenance is stable enough for audit (no perimeter-order or first-match pseudo-provenance)
  Evidence: <tests>
- [ ] Snapshot-scoped vs persistent identity is documented correctly
  Evidence: <schema/docs references>

## Tests (Adversarial Coverage)

- [ ] Happy path coverage exists
  Evidence: <test names>
- [ ] Ambiguity path coverage exists (selector/trace/policy ambiguity)
  Evidence: <test names>
- [ ] Fallback/slow path or internal planner path coverage exists (if applicable)
  Evidence: <test names>
- [ ] Transaction rollback / fail-fast path coverage exists
  Evidence: <test names>
- [ ] Trace propagation path coverage exists
  Evidence: <test names>
- [ ] Typed error assertions exist for key rejection paths (not string-only)
  Evidence: <test names>

## Storage / Audit (if feature emits records)

- [ ] Serializable artifact schema is versioned and round-trip tested
  Evidence: <tests>
- [ ] Snapshot-scoped fields are labeled; persistent fields are labeled
  Evidence: <schema refs>
- [ ] Error summaries are typed and serializable (no `Display`-only collapse)
  Evidence: <schema/tests>

## Final Verification

- [ ] Focused integration suite executed for touched subsystems
  Evidence: <commands + pass result>
- [ ] CI delivery guard passes for this epic checklist
  Evidence: `python3 scripts/ci/check_delivery_guards.py --checklist <path>`
