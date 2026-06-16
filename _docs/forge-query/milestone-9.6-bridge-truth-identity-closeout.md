# Forge Query Milestone 9.6 Bridge Truth Identity Closeout

> **Status:** Closed
>
> **Date:** 2026-06-15
>
> **Governing law:** Architecture Law 42 and Forge Foundational identity lifecycle categories.

## Closure Summary

Milestone 9.6 closed the bridge truth identity seam by making identity authority a typed lifecycle property instead of a representation convention. Relational, signal, runtime bridge, Query, server, worth-topo, hadwiger-research, and forge-ui in-scope paths now either preserve typed authority/evidence, quarantine terminal projections, or are explicitly deferred to an owner milestone.

The remaining allowed strings are terminal reporting, diagnostics, JSON compatibility input, or historical certification labels. They do not satisfy current authority APIs.

## Authority Model

- Current authority must be carried by the owning typed identity or admitted owner witness.
- Boundary-crossed values must be bridged or revalidated before they can satisfy current authority.
- External tokens, digest evidence, projection labels, and raw representations cannot enter authority APIs.
- Reporting accessors are terminal and may not feed composition, lookup, admission, routing, or coherence checks.
- Bridge writeback/naming/route identities no longer expose raw `from_external_authority_evidence` constructors; callers must provide typed bridge evidence.

## Final QA Findings

1. **Certification row digests still used reporting references.** Fixed by hashing retained `reference_receipt_evidence_identity()` values in causal certification row-digest collection and named-slot composition.
2. **Bridge writeback/naming/route facade constructors still accepted raw evidence text.** Fixed by deleting public `from_external_authority_evidence` constructors and requiring `BridgeIdentityEvidence`.
3. **Mutation batch tests used fake raw bridge evidence and one non-relational resolved-target label.** Fixed by deriving test evidence from typed Query identities and using relational bridge record identities where the test asserts resolved-target variation.
4. **Closeout state was stale.** Fixed by adding this closeout and updating the milestone status/checklists.

## Post-Closeout QA Correction

The 2026-06-15 follow-up QA found additional projection re-entry in the
causal/subscription certification edge after the initial closeout. The fix
keeps report strings terminal and moves certification composition back onto
typed evidence identities:

- `subscription/runtime_certification/scope.rs` compares support, parity,
  diagnostic, lifecycle, and coverage handles through retained
  `ForgeQueryEvidenceIdentity` values and seals the scope digest through the
  evidence encoder.
- `runtime/inspection/causal/certification/*` now seals hostile rows,
  boundary audit, representative matrix rows, named evidence slots,
  reference collections, row-digest sets, proof-shape digests, certification
  scope, and certification bundle through `ForgeQueryEvidenceIdentity`
  composition instead of `hash_parts` over report strings.
- `query_basis_lifecycle/binding_evidence.rs` uses `field_bridge_identity`
  for retained bridge evidence rather than projecting `BridgeIdentityEvidence`
  through `as_str()`.

## Eight-Point QA Closure

1. `subscription/runtime_certification/scope.rs` no longer composes scope
   identity from `certification_bundle_for_reporting()` or `hash_parts`; it
   compares and seals retained typed evidence identities.
2. `SubscriptionLifecycleCertificationBundle` no longer stores parallel
   `*_for_reporting: String` fields for artifacts that already have typed
   lifecycle identities. Reporting accessors project from typed identities, and
   the bundle itself carries a `QuerySubscriptionAuthorityIdentity`.
3. Causal hostile rows no longer consume `artifact_for_reporting()`; artifact
   rows seal retained artifact evidence identities.
4. Causal row-digest slots and reference collections no longer join
   `reference_receipt_evidence_identity().as_str()` strings; they seal retained
   reference identities through the evidence encoder.
5. Query lower-runtime binding evidence no longer stringifies
   `BridgeIdentityEvidence`; both `binding_evidence.rs` and `raw_identity.rs`
   use `field_bridge_identity`.
6. Forge Foundational authority categories are wired into production Query
   carriers: subscription certification bundles carry
   `QuerySubscriptionAuthorityIdentity`, causal certification bundles carry
   `QueryCausalInspectionAuthorityIdentity`, and lower-runtime basis bindings
   carry `QueryFeederAuthorityIdentity`.
7. The named Phase-7-adjacent residuals were cleaned: query-basis lifecycle
   digest construction no longer uses `hash_parts`; subscription certification
   errors and runtime-certification errors seal evidence identities; declaration
   bridge routing digest projections use value fields; digest wrappers no
   longer re-enter digest text as identity fields.
8. Matrix row 1055 no longer contradicts itself by marking the row `Fixed`
   while preserving an open "remaining risk" note.

## Verification Gates

- `cargo check --workspace`
- `cargo check -p forge-runtime-bridge -p forge-query --lib`
- `cargo test -p forge-runtime-bridge --test phase_boundaries_compile_fail`
- `cargo test -p forge-runtime-bridge --test phase_boundaries_bridge_truth_identity_compile_fail`
- `cargo test -p forge-runtime-bridge --test phase_boundaries_bridge_truth_identity_digest`
- `cargo test -p forge-query --test phase_boundaries_bridge_truth_identity_compile_fail`
- `cargo test -p forge-relational --test phase_boundaries_compile_fail`
- `cargo test -p forge-signal --test phase_2a_signal_boundaries`
- `cargo test -p forge-query runtime::surface::mutation_evidence::batch --lib`
- `cargo test -p forge-query runtime::tests::causal_inspection::certification::row_digest --lib`
- `cargo test -p forge-query runtime::tests::causal_inspection::certification --lib`
- `cargo test -p forge-query subscription::tests::runtime_certification --lib`
- `cargo test -p forge-query query_basis_lifecycle --lib`
- `cargo fmt --check --all`

Evidence files for the final pass are stored under `_docs/forge-query/goal_mode_final_*` and `_docs/forge-query/goal_mode_bridge_*`.

## Deferred Scope

- Subscription replay typed identity fixture cleanup remains deferred to the subscription replay typed identity milestone.
- Out-of-scope local/display formatting in worth-kernel and forge-kernel remains ordinary projection text unless a future trace finds an authority path.
