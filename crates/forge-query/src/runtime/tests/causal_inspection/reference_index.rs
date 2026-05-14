use super::super::super::inspection::{
    causal_evidence_reference_index, causal_evidence_reference_index_record,
};
use super::super::super::*;

fn changed_anchor_with_route_signal_and_inspection() -> CausalObservationAnchor {
    anchor_causal_observation(
        QueryObservationReceipt::fixture(
            CausalObservationOutcome::Changed,
            vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    "indexed-query-inspection-reference",
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgeRoute,
                    "indexed-bridge-route-reference",
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::SignalInvalidation,
                    "indexed-signal-invalidation-reference",
                ),
            ],
        ),
        CausalInspectionReason::ChangedResult,
    )
    .unwrap()
}

#[test]
fn indexed_evidence_reference_resolution_uses_index_records_without_scan_fallback() {
    let anchor = changed_anchor_with_route_signal_and_inspection();
    let anchor_digest = anchor.anchor_digest().clone();
    let index = causal_evidence_reference_index([
        causal_evidence_reference_index_record(
            CausalEvidenceOwner::RuntimeBridge,
            CausalEvidenceFamily::BridgeRoute,
            "indexed-bridge-route-reference",
        )
        .unwrap(),
        causal_evidence_reference_index_record(
            CausalEvidenceOwner::Signal,
            CausalEvidenceFamily::SignalInvalidation,
            "indexed-signal-invalidation-reference",
        )
        .unwrap(),
    ]);

    let resolution = resolve_indexed_causal_evidence_references(
        anchor,
        &[
            CausalEvidenceFamily::BridgeRoute,
            CausalEvidenceFamily::SignalInvalidation,
        ],
        &index,
    );

    let CausalEvidenceReferenceResolution::Resolved {
        reference_set,
        counters,
    } = resolution
    else {
        panic!("expected indexed causal references to resolve");
    };

    assert_eq!(reference_set.anchor().anchor_digest(), &anchor_digest);
    assert_eq!(reference_set.references().len(), 2);
    assert!(reference_set.references().iter().any(|reference| {
        reference.owner() == CausalEvidenceOwner::RuntimeBridge
            && reference.family() == CausalEvidenceFamily::BridgeRoute
            && reference.reference_digest().as_str() == "indexed-bridge-route-reference"
    }));
    assert!(reference_set.references().iter().any(|reference| {
        reference.owner() == CausalEvidenceOwner::Signal
            && reference.family() == CausalEvidenceFamily::SignalInvalidation
            && reference.reference_digest().as_str() == "indexed-signal-invalidation-reference"
    }));
    assert_eq!(index.record_count(), 2);
    assert_eq!(index.family_count(), 2);
    assert!(!index.index_digest().is_empty());
    assert_eq!(counters.requested_family_count(), 2);
    assert_eq!(counters.anchor_reference_width(), 3);
    assert_eq!(counters.indexed_record_count(), 2);
    assert_eq!(counters.index_lookup_count(), 2);
    assert_eq!(counters.resolved_reference_count(), 2);
    assert_eq!(counters.missing_required_reference_count(), 0);
    assert_eq!(counters.bridge_record_scan_fallback_count(), 0);
    assert_eq!(counters.retained_record_scan_count(), 0);
    assert_eq!(counters.runtime_graph_scan_count(), 0);
}

#[test]
fn indexed_reference_resolution_denies_missing_index_record_for_anchor_carried_family() {
    let anchor = changed_anchor_with_route_signal_and_inspection();
    let anchor_digest = anchor.anchor_digest().clone();
    let index = causal_evidence_reference_index([causal_evidence_reference_index_record(
        CausalEvidenceOwner::Query,
        CausalEvidenceFamily::QueryInspection,
        "indexed-query-inspection-reference",
    )
    .unwrap()]);

    let resolution = resolve_indexed_causal_evidence_references(
        anchor,
        &[CausalEvidenceFamily::BridgeRoute],
        &index,
    );

    let CausalEvidenceReferenceResolution::MissingRequiredEvidence { denial, counters } =
        resolution
    else {
        panic!("expected missing indexed bridge record to deny resolution");
    };

    assert_eq!(denial.anchor_digest(), &anchor_digest);
    assert!(denial.missing_families().is_empty());
    assert_eq!(denial.missing_indexed_reference_count(), 1);
    assert_eq!(counters.requested_family_count(), 1);
    assert_eq!(counters.anchor_reference_width(), 3);
    assert_eq!(counters.indexed_record_count(), 1);
    assert_eq!(counters.index_lookup_count(), 1);
    assert_eq!(counters.resolved_reference_count(), 0);
    assert_eq!(counters.missing_required_reference_count(), 1);
    assert_eq!(counters.bridge_record_scan_fallback_count(), 0);
    assert_eq!(counters.retained_record_scan_count(), 0);
    assert_eq!(counters.runtime_graph_scan_count(), 0);
}

#[test]
fn indexed_reference_resolution_ignores_unrelated_retained_index_records() {
    let anchor = changed_anchor_with_route_signal_and_inspection();
    let index = causal_evidence_reference_index([
        causal_evidence_reference_index_record(
            CausalEvidenceOwner::RuntimeBridge,
            CausalEvidenceFamily::BridgeRoute,
            "stale-bridge-route-reference",
        )
        .unwrap(),
        causal_evidence_reference_index_record(
            CausalEvidenceOwner::RuntimeBridge,
            CausalEvidenceFamily::BridgeRoute,
            "indexed-bridge-route-reference",
        )
        .unwrap(),
        causal_evidence_reference_index_record(
            CausalEvidenceOwner::Signal,
            CausalEvidenceFamily::SignalEvaluation,
            "unrelated-signal-evaluation-reference",
        )
        .unwrap(),
    ]);

    let resolution = resolve_indexed_causal_evidence_references(
        anchor,
        &[CausalEvidenceFamily::BridgeRoute],
        &index,
    );

    let CausalEvidenceReferenceResolution::Resolved {
        reference_set,
        counters,
    } = resolution
    else {
        panic!(
            "expected exact indexed bridge record to resolve despite unrelated retained records"
        );
    };

    assert_eq!(reference_set.references().len(), 1);
    assert_eq!(index.record_count(), 3);
    assert_eq!(counters.requested_family_count(), 1);
    assert_eq!(counters.anchor_reference_width(), 3);
    assert_eq!(counters.indexed_record_count(), 1);
    assert_eq!(counters.index_lookup_count(), 1);
    assert_eq!(counters.resolved_reference_count(), 1);
    assert_eq!(counters.missing_required_reference_count(), 0);
    assert_eq!(counters.bridge_record_scan_fallback_count(), 0);
    assert_eq!(counters.retained_record_scan_count(), 0);
    assert_eq!(counters.runtime_graph_scan_count(), 0);
}

#[test]
fn indexed_reference_resolution_lookup_cost_follows_anchor_reference_width_not_retention_width() {
    for unrelated_retained_record_count in [0, 8, 64] {
        let anchor = changed_anchor_with_route_signal_and_inspection();
        let mut index_records = vec![causal_evidence_reference_index_record(
            CausalEvidenceOwner::RuntimeBridge,
            CausalEvidenceFamily::BridgeRoute,
            "indexed-bridge-route-reference",
        )
        .unwrap()];
        for retained_index in 0..unrelated_retained_record_count {
            index_records.push(
                causal_evidence_reference_index_record(
                    CausalEvidenceOwner::RuntimeBridge,
                    CausalEvidenceFamily::BridgeRoute,
                    format!("unrelated-retained-bridge-route-{retained_index}"),
                )
                .unwrap(),
            );
        }
        let index = causal_evidence_reference_index(index_records);

        let resolution = resolve_indexed_causal_evidence_references(
            anchor,
            &[CausalEvidenceFamily::BridgeRoute],
            &index,
        );

        let CausalEvidenceReferenceResolution::Resolved { counters, .. } = resolution else {
            panic!("expected exact indexed bridge route reference to resolve");
        };

        assert_eq!(index.record_count(), 1 + unrelated_retained_record_count);
        assert_eq!(counters.requested_family_count(), 1);
        assert_eq!(counters.anchor_reference_width(), 3);
        assert_eq!(counters.indexed_record_count(), 1);
        assert_eq!(counters.index_lookup_count(), 1);
        assert_eq!(counters.resolved_reference_count(), 1);
        assert_eq!(counters.missing_required_reference_count(), 0);
        assert_eq!(counters.bridge_record_scan_fallback_count(), 0);
        assert_eq!(counters.retained_record_scan_count(), 0);
        assert_eq!(counters.runtime_graph_scan_count(), 0);
    }
}

#[test]
fn reference_index_record_denies_owner_mismatch_and_empty_digest() {
    let owner_mismatch = causal_evidence_reference_index_record(
        CausalEvidenceOwner::Signal,
        CausalEvidenceFamily::BridgeRoute,
        "bridge-route-reference",
    )
    .unwrap_err();

    assert_eq!(
        owner_mismatch.kind(),
        CausalEvidenceReferenceIndexErrorKind::EvidenceOwnerMismatch
    );
    assert_eq!(owner_mismatch.family(), CausalEvidenceFamily::BridgeRoute);
    assert_eq!(owner_mismatch.supplied_owner(), CausalEvidenceOwner::Signal);
    assert_eq!(
        owner_mismatch.expected_owner(),
        CausalEvidenceOwner::RuntimeBridge
    );
    assert!(!owner_mismatch.failure_digest().is_empty());

    let empty_digest = causal_evidence_reference_index_record(
        CausalEvidenceOwner::RuntimeBridge,
        CausalEvidenceFamily::BridgeRoute,
        "",
    )
    .unwrap_err();

    assert_eq!(
        empty_digest.kind(),
        CausalEvidenceReferenceIndexErrorKind::EmptyReferenceDigest
    );
    assert_eq!(empty_digest.family(), CausalEvidenceFamily::BridgeRoute);
}
