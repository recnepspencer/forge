use super::super::super::*;
use std::collections::BTreeSet;

#[test]
fn phase_one_anchors_cover_changed_suppressed_denied_preview_and_replay_observations() {
    for (outcome, reason) in [
        (
            CausalObservationOutcome::Changed,
            CausalInspectionReason::ChangedResult,
        ),
        (
            CausalObservationOutcome::Suppressed,
            CausalInspectionReason::SuppressedResult,
        ),
        (
            CausalObservationOutcome::Denied,
            CausalInspectionReason::DeniedResult,
        ),
        (
            CausalObservationOutcome::BranchPreview,
            CausalInspectionReason::BranchPreviewResult,
        ),
        (
            CausalObservationOutcome::Replayed,
            CausalInspectionReason::HistoricalReplayResult,
        ),
    ] {
        let receipt = QueryObservationReceipt::fixture(
            outcome,
            vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    format!("query-inspection-{}", outcome.as_str()),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgeRoute,
                    format!("bridge-route-{}", outcome.as_str()),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::SignalInvalidation,
                    format!("signal-invalidation-{}", outcome.as_str()),
                ),
            ],
        );

        let anchor = anchor_causal_observation(receipt, reason).unwrap();

        assert_eq!(anchor.observation_receipt().outcome(), outcome);
        assert_eq!(anchor.inspection_reason(), reason);
        assert_eq!(anchor.lower_runtime_evidence_family_count(), 3);
        assert_eq!(
            anchor.missing_reference_posture(),
            CausalObservationMissingReferencePosture::Complete
        );
        assert!(!anchor.anchor_digest().as_str().is_empty());
    }
}

#[test]
fn anchor_counters_are_exact_and_width_bound_to_carried_reference_families() {
    let receipt = QueryObservationReceipt::fixture(
        CausalObservationOutcome::Changed,
        vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                "query-inspection-a",
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                "query-inspection-b",
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::RelationalAuthority,
                "relational-authority-a",
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                "bridge-route-a",
            ),
        ],
    );

    let anchor = anchor_causal_observation(receipt, CausalInspectionReason::ChangedResult).unwrap();
    let counters = anchor.counters();

    assert_eq!(counters.source_receipt_family_count(), 1);
    assert_eq!(counters.reference_family_count(), 3);
    assert_eq!(
        counters.missing_reference_posture(),
        CausalObservationMissingReferencePosture::Complete
    );
    assert_eq!(
        counters.anchor_digest_width(),
        anchor.anchor_digest().as_str().len()
    );
    assert_eq!(counters.runtime_graph_scan_count(), 0);
    assert_eq!(counters.diagnostics_retention_scan_count(), 0);
    assert!(!counters.counter_snapshot().is_empty());
}

#[test]
fn anchor_derivation_denies_receipts_without_carried_evidence_identities() {
    let receipt = QueryObservationReceipt::fixture(CausalObservationOutcome::Changed, Vec::new());

    let error =
        anchor_causal_observation(receipt, CausalInspectionReason::ChangedResult).unwrap_err();

    assert_eq!(
        error.kind(),
        CausalObservationAnchorErrorKind::MissingRequiredEvidenceReference
    );
    assert!(error.message().contains("evidence identity"));
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn anchor_derivation_denies_empty_carried_evidence_reference_digests() {
    let receipt = QueryObservationReceipt::fixture(
        CausalObservationOutcome::Changed,
        vec![CausalObservationEvidenceIdentity::new(
            CausalEvidenceFamily::BridgeRoute,
            "",
        )],
    );

    let error =
        anchor_causal_observation(receipt, CausalInspectionReason::ChangedResult).unwrap_err();

    assert_eq!(
        error.kind(),
        CausalObservationAnchorErrorKind::MissingRequiredEvidenceReference
    );
    assert!(error.message().contains("non-empty evidence reference"));
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn anchor_derivation_denies_inspection_reason_outcome_mismatch() {
    let receipt = QueryObservationReceipt::fixture(
        CausalObservationOutcome::Denied,
        vec![CausalObservationEvidenceIdentity::new(
            CausalEvidenceFamily::QueryInspection,
            "denied-inspection",
        )],
    );

    let error =
        anchor_causal_observation(receipt, CausalInspectionReason::ChangedResult).unwrap_err();

    assert_eq!(
        error.kind(),
        CausalObservationAnchorErrorKind::InspectionReasonOutcomeMismatch
    );
    assert!(error.message().contains("reason must match"));
    assert!(!error.failure_digest().is_empty());
}

#[test]
fn evidence_inventory_names_existing_lower_runtime_reference_identities() {
    let rows = causal_evidence_inventory_rows();

    assert!(rows.iter().any(|row| {
        row.owner() == CausalEvidenceOwner::RuntimeBridge
            && row.family() == CausalEvidenceFamily::BridgeRoute
            && row
                .authority_surface()
                .contains("BridgeDiagnosticsFacade::route_records")
            && row.query_reference_identity() == "route_digest"
    }));
    assert!(rows.iter().any(|row| {
        row.owner() == CausalEvidenceOwner::Signal
            && row.family() == CausalEvidenceFamily::SignalForensicAvailability
    }));
    assert!(rows.iter().any(|row| {
        row.owner() == CausalEvidenceOwner::Relational
            && row.family() == CausalEvidenceFamily::RelationalAuthority
    }));
    for family in [
        CausalEvidenceFamily::QueryInspection,
        CausalEvidenceFamily::QueryMutationCausality,
        CausalEvidenceFamily::QueryMutationProvenance,
        CausalEvidenceFamily::RelationalAuthority,
        CausalEvidenceFamily::RelationalDecision,
        CausalEvidenceFamily::BridgeRoute,
        CausalEvidenceFamily::BridgeEvaluation,
        CausalEvidenceFamily::BridgeSourceMaterialization,
        CausalEvidenceFamily::BridgeSourceFailure,
        CausalEvidenceFamily::BridgeContinuity,
        CausalEvidenceFamily::BridgeMerge,
        CausalEvidenceFamily::BridgeStructural,
        CausalEvidenceFamily::BridgeStream,
        CausalEvidenceFamily::BridgePreview,
        CausalEvidenceFamily::BridgeWriteback,
        CausalEvidenceFamily::BridgeMapper,
        CausalEvidenceFamily::BridgeReplay,
        CausalEvidenceFamily::SignalInvalidation,
        CausalEvidenceFamily::SignalEvaluation,
        CausalEvidenceFamily::SignalForensicAvailability,
        CausalEvidenceFamily::SignalReplayCursor,
        CausalEvidenceFamily::SignalLineage,
        CausalEvidenceFamily::Lineage,
        CausalEvidenceFamily::Provenance,
        CausalEvidenceFamily::Policy,
        CausalEvidenceFamily::Redaction,
    ] {
        assert!(
            rows.iter().any(|row| row.family() == family),
            "missing causal evidence inventory row for {family:?}"
        );
    }

    let unique_families = rows
        .iter()
        .map(CausalEvidenceInventoryRow::family)
        .collect::<BTreeSet<_>>();
    assert_eq!(unique_families.len(), rows.len());
}

#[test]
fn evidence_reference_resolution_consumes_anchor_and_emits_sealed_reference_set() {
    let anchor = anchor_causal_observation(
        QueryObservationReceipt::fixture(
            CausalObservationOutcome::Changed,
            vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    "query-inspection-reference",
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::RelationalAuthority,
                    "relational-authority-reference",
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgeRoute,
                    "bridge-route-reference",
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::SignalInvalidation,
                    "signal-invalidation-reference",
                ),
            ],
        ),
        CausalInspectionReason::ChangedResult,
    )
    .unwrap();
    let anchor_digest = anchor.anchor_digest().clone();

    let resolution = resolve_causal_evidence_references(
        anchor,
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::RelationalAuthority,
            CausalEvidenceFamily::SignalInvalidation,
        ],
    );

    let CausalEvidenceReferenceResolution::Resolved {
        reference_set,
        counters,
    } = resolution
    else {
        panic!("expected causal evidence references to resolve");
    };

    assert_eq!(reference_set.anchor().anchor_digest(), &anchor_digest);
    assert_eq!(reference_set.references().len(), 3);
    assert!(reference_set.references().iter().any(|reference| {
        reference.owner() == CausalEvidenceOwner::RuntimeBridge
            && reference.family() == CausalEvidenceFamily::BridgeRoute
            && reference.reference_digest().as_str() == "bridge-route-reference"
    }));
    assert!(reference_set.references().iter().any(|reference| {
        reference.owner() == CausalEvidenceOwner::Relational
            && reference.family() == CausalEvidenceFamily::RelationalAuthority
    }));
    assert!(reference_set.references().iter().any(|reference| {
        reference.owner() == CausalEvidenceOwner::Signal
            && reference.family() == CausalEvidenceFamily::SignalInvalidation
    }));
    assert!(!reference_set.reference_set_digest().as_str().is_empty());
    assert!(!reference_set.receipt().receipt_digest().is_empty());
    assert_eq!(reference_set.receipt().resolved_reference_count(), 3);
    assert_eq!(reference_set.receipt().missing_reference_family_count(), 0);
    assert_eq!(counters.requested_family_count(), 3);
    assert_eq!(counters.anchor_reference_width(), 4);
    assert_eq!(counters.index_lookup_count(), 3);
    assert_eq!(counters.resolved_reference_count(), 3);
    assert_eq!(counters.missing_required_reference_count(), 0);
    assert_eq!(counters.bridge_record_scan_fallback_count(), 0);
    assert_eq!(counters.retained_record_scan_count(), 0);
    assert_eq!(counters.runtime_graph_scan_count(), 0);
}

#[test]
fn evidence_reference_resolution_denies_requested_family_missing_from_anchor() {
    let anchor = anchor_causal_observation(
        QueryObservationReceipt::fixture(
            CausalObservationOutcome::Denied,
            vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                "query-denial-reference",
            )],
        ),
        CausalInspectionReason::DeniedResult,
    )
    .unwrap();
    let anchor_digest = anchor.anchor_digest().clone();

    let resolution = resolve_causal_evidence_references(
        anchor,
        &[
            CausalEvidenceFamily::QueryInspection,
            CausalEvidenceFamily::BridgeRoute,
        ],
    );

    let CausalEvidenceReferenceResolution::MissingRequiredEvidence { denial, counters } =
        resolution
    else {
        panic!("expected missing bridge route evidence to deny resolution");
    };

    assert_eq!(denial.anchor_digest(), &anchor_digest);
    assert_eq!(
        denial.missing_families(),
        &[CausalEvidenceFamily::BridgeRoute]
    );
    assert!(!denial.failure_digest().is_empty());
    assert_eq!(counters.requested_family_count(), 2);
    assert_eq!(counters.anchor_reference_width(), 1);
    assert_eq!(counters.index_lookup_count(), 2);
    assert_eq!(counters.resolved_reference_count(), 1);
    assert_eq!(counters.missing_required_reference_count(), 1);
    assert_eq!(counters.bridge_record_scan_fallback_count(), 0);
    assert_eq!(counters.retained_record_scan_count(), 0);
    assert_eq!(counters.runtime_graph_scan_count(), 0);
}

#[test]
fn evidence_reference_resolution_defaults_to_anchor_carried_families() {
    let anchor = anchor_causal_observation(
        QueryObservationReceipt::fixture(
            CausalObservationOutcome::Replayed,
            vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    "replay-query-inspection-reference",
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgeReplay,
                    "bridge-replay-reference",
                ),
            ],
        ),
        CausalInspectionReason::HistoricalReplayResult,
    )
    .unwrap();

    let resolution = resolve_causal_evidence_references(anchor, &[]);

    let CausalEvidenceReferenceResolution::Resolved {
        reference_set,
        counters,
    } = resolution
    else {
        panic!("expected carried anchor families to resolve by default");
    };

    assert_eq!(reference_set.references().len(), 2);
    assert!(reference_set.references().iter().any(|reference| {
        reference.owner() == CausalEvidenceOwner::RuntimeBridge
            && reference.family() == CausalEvidenceFamily::BridgeReplay
    }));
    assert_eq!(counters.requested_family_count(), 2);
    assert_eq!(counters.anchor_reference_width(), 2);
    assert_eq!(counters.index_lookup_count(), 2);
    assert_eq!(counters.resolved_reference_count(), 2);
    assert_eq!(counters.missing_required_reference_count(), 0);
}
