# Audit / Policy Base Layers QA Checklist (BRUTAL)

Scope:
- `/Users/spenstar/Documents/programming/Forge/docs/engineering/AUDIT_POLICY_BASE_LAYERS_SPEC.md` sections 1–7
- Foundation contracts only (not the later Phase 2 integrations yet)

Standard:
- Grade against Forge `VISION.md` (traceability/auditability as product features, explicit policy semantics, deterministic behavior, fail-closed ambiguity handling)
- Assume this foundation will be used for aerospace-grade audit trails and agent-facing workflows

Rules:
- Every checked item must include concrete evidence (tests, commands, code refs)
- “Seems fine” is not evidence
- String-based behavior checks do not count where typed semantics are required

## 0. Review Setup

- [ ] Reviewed `VISION.md` and `ARCHITECTURE_COMPONENT_INVENTORY.md` before QA
  Evidence: <review notes + date>
- [ ] Reviewed `/Users/spenstar/Documents/programming/Forge/docs/engineering/AUDIT_POLICY_BASE_LAYERS_SPEC.md` (sections 1–7) before QA
  Evidence: <review notes + date>

## 1. Typed Error Summary Layer (`forge-core::errors::summary`)

- [ ] `KernelErrorSummary` preserves typed `MergeError` variants (no string collapse)
  Evidence: <tests + code refs>
- [ ] `MergeErrorSummary::BoundaryCertificationFailed` preserves `reason` and `witness`
  Evidence: <tests>
- [ ] `TopologyErrorSummary` preserves structured fields for at least one adversarially important variant (`RadialEdgeInconsistency` or equivalent)
  Evidence: <tests>
- [ ] `ErrorSummary.human_message` is documented/treated as non-authoritative
  Evidence: <docs/code refs>
- [ ] No audit-critical logic in this layer depends on `Display` parsing
  Evidence: <grep/review note>
- [ ] JSON round-trip tests exist for top-level error summaries
  Evidence: <tests>

## 2. Structured Provenance Payload Layer (`forge-core::provenance`)

- [ ] `SnapshotHandleRef` explicitly encodes `(kind, index, generation)` and is labeled snapshot-scoped
  Evidence: <code refs>
- [ ] Boundary provenance includes both endpoint handles (not just one vertex / ordinal)
  Evidence: <code refs>
- [ ] Transport hash is deterministic and direction-sensitive
  Evidence: <tests>
- [ ] Transport hash changes on generation reuse (ABA-sensitive)
  Evidence: <tests>
- [ ] Provenance payloads are serializable and round-trip tested
  Evidence: <tests>
- [ ] Provenance schema does not imply persistent identity
  Evidence: <docs/code refs>

## 3. Policy Resolution Trace Payload Layer (`forge-core::tracing::policy_trace`)

- [ ] Policy payload captures required semantics:
  Evidence: fields for `policy_kind`, `margin`, `threshold`, `candidate_summary`, `outcome`, `source`, `default_used`
- [ ] Payload is serializable and round-trip tested
  Evidence: <tests>
- [ ] Payload/decision consistency validator enforces `DecisionKind`/`DecisionTier` compatibility
  Evidence: <tests + code refs>
- [ ] Escalated policy outcomes cannot masquerade as `PolicyApplied`
  Evidence: <negative test>
- [ ] This layer is staged as side-channel without silently mutating `TracedDecision` semantics
  Evidence: <code refs + docs>

## 4. ModelingContext Sub-Operation Metadata Lifecycle Contract

- [ ] `ModelingContext` supports accumulate + read + take-and-reset semantics
  Evidence: <API refs>
- [ ] `take_sub_metadata()` drains warnings/metrics/lineage/budget and resets sink state
  Evidence: <tests>
- [ ] Repeated drains are idempotent after reset (no phantom carry-over)
  Evidence: <tests>
- [ ] Folding drained metadata into an `OperationResult` can be done without double-counting
  Evidence: <tests>
- [ ] Sub-op metadata sink semantics are documented as operation-boundary infrastructure (not ad hoc convenience)
  Evidence: <docs/code refs>

## 5. Versioned Audit Schema Conventions (`forge-io::audit`)

- [ ] Versioned audit envelope enforces non-zero `schema_version` and `operation_version`
  Evidence: <tests>
- [ ] `operation_type` convention is validated (snake_case / stable identifier constraints)
  Evidence: <tests + code refs>
- [ ] Snapshot/persistent/hash field-name conventions are codified and validated in code (not just docs)
  Evidence: <tests + code refs>
- [ ] Deterministic serialization test exists for audit envelope records
  Evidence: <tests>
- [ ] Convention helpers are reusable by feature-specific audit records (`RegionMergeAuditRecord`)
  Evidence: <API refs / review note>

## 6. Deterministic Trace Fingerprinting (`forge-core::tracing::fingerprint`)

- [ ] Fingerprint helper hashes semantic trace content deterministically
  Evidence: <code refs>
- [ ] Fingerprint excludes span timing by default
  Evidence: <code refs + tests>
- [ ] Fingerprint changes on semantic decision changes
  Evidence: <tests>
- [ ] Fingerprint includes ordered `decision_ids` for downstream audit/replay linkage
  Evidence: <code refs + tests>
- [ ] Fingerprint implementation avoids non-deterministic map iteration in hashed content
  Evidence: <review note>

## 7. Reusable Fixture Builders (`forge-test::region_merge_fixtures`)

- [ ] Fixture module covers at least:
  Evidence: simple, weakly-simple, rejected-crossing boundaries + deterministic group hash helper
- [ ] Fixtures are deterministic and test-covered
  Evidence: <tests>
- [ ] Fixture names make certifier outcome intent explicit (simple/weakly/rejected)
  Evidence: <code refs>
- [ ] Group hash helper is membership-set deterministic (not insertion-order dependent)
  Evidence: <tests>
- [ ] Fixtures are suitable for upcoming policy/provenance tests (not overfit to one test file)
  Evidence: <review note>

## 8. Cross-Layer Composition QA (Most Important)

- [ ] A realistic hypothetical `RegionMergeAuditRecord` can be sketched using only these base layers without inventing new ad hoc core types
  Evidence: <design sketch or review note>
- [ ] No contradictory naming/semantics between error summaries, policy payloads, provenance payloads, and audit conventions
  Evidence: <cross-file review note>
- [ ] Snapshot-scoped identity is labeled consistently across all new layers
  Evidence: <code refs>
- [ ] No hidden stringly fallback remains in the newly added foundation layers for critical semantics
  Evidence: <grep/review note>
- [ ] No hidden mutable lifecycle trap (double-drain/double-merge) remains in newly added metadata paths
  Evidence: <tests + review note>

## 9. Architecture & Vision Alignment (Brutal Gate)

- [ ] Foundation advances “specification graph is the product” (typed, serializable, auditable semantics)
  Evidence: <review note linked to sections 1/3/5>
- [ ] Foundation advances “every decision traced” (typed policy payload path, not just generic labels)
  Evidence: <review note linked to section 3>
- [ ] Foundation advances “tolerance exists but is never silent” (preconditions for policy/precision integration are explicit)
  Evidence: <review note linked to sections 1/3/4>
- [ ] Foundation advances deterministic replay/debug capability (audit conventions + fingerprint + provenance)
  Evidence: <review note linked to sections 2/5/6>
- [ ] Nothing here is “temporary architecture debt” disguised as a foundation (except explicitly staged trace side-channel and curved placeholders)
  Evidence: <explicit exceptions list>

## 10. Verification Commands (Record Actual Runs)

- [ ] `cargo test -q -p forge-core errors::tests:: -- --nocapture`
  Evidence: <pass/fail + date>
- [ ] `cargo test -q -p forge-core provenance::tests:: -- --nocapture`
  Evidence: <pass/fail + date>
- [ ] `cargo test -q -p forge-core policy_trace_payload_ -- --nocapture`
  Evidence: <pass/fail + date>
- [ ] `cargo test -q -p forge-core trace_fingerprint_ -- --nocapture`
  Evidence: <pass/fail + date>
- [ ] `cargo test -q -p forge-io audit:: -- --nocapture`
  Evidence: <pass/fail + date>
- [ ] `cargo test -q -p forge-test region_merge_fixtures:: -- --nocapture`
  Evidence: <pass/fail + date>
- [ ] `cargo check -q -p forge-core -p forge-io -p forge-test`
  Evidence: <pass/fail + date>
- [ ] `python3 /Users/spenstar/Documents/programming/Forge/scripts/ci/check_delivery_guards.py --checklist docs/engineering/AUDIT_POLICY_BASE_LAYERS_CHECKLIST.md`
  Evidence: <pass/fail + date>

## 11. Exit Criteria (Must All Be True)

- [ ] All items in sections 1–10 are checked with concrete evidence
  Evidence: <review sign-off>
- [ ] Any known limitations are explicitly documented as staged (not accidental omissions)
  Evidence: <limitations list>
- [ ] Approved to begin Foundation Phase 2 contracts:
  Evidence: <decision + reviewer/date>
  - Policy registry/config source model
  - Persistent-name resolution result contract
  - Operation finalization contract
  - Trace adjunct/versioning strategy
  - Replay/audit bridge contract
