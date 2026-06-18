#[path = "public_api_planar_boolean_collinear_relations_support/mod.rs"]
#[allow(dead_code)]
mod collinear_relation_support;
#[path = "public_api_planar_boolean_event_ledger_support.rs"]
#[allow(dead_code)]
mod event_ledger_support;
#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
#[allow(dead_code, unused_imports)]
mod metaboss_support;
#[path = "public_api_planar_boolean_point_events_support/mod.rs"]
#[allow(dead_code)]
mod point_event_support;
#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
#[allow(dead_code)]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use metaboss_support::MetabossEventExtractionSubject;
use worth_kernel::workload_composition::PlanarBooleanOutcomeKind;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanCandidateIndexConsumptionGate, PlanarBooleanCandidateIndexConsumptionInput,
    PlanarBooleanEdgeSplitPolicyOutcomeKind, PlanarBooleanEdgeSplitRequest,
    PlanarBooleanEdgeSplitRequestInput, PlanarBooleanEdgeSplitScopeAdmission,
    PlanarBooleanEdgeSplitScopeAdmissionInput, PlanarBooleanEdgeSplitScopeClass,
};
use worth_spatial::facade::workload_vocabulary::{WorkloadEvidenceLedger, WorkloadEvidenceRow};

#[test]
fn edge_split_scope_admission_preserves_query_indexed_request_lineage() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify("phase7.3 split scope admission");
        let (request, gate) = metaboss_edge_split_request(&subject);
        let admission = PlanarBooleanEdgeSplitScopeAdmission::admit(
            PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&request),
        )
        .expect("metaboss split request should admit into 7.3 edge surgery scope");

        assert_eq!(
            admission.scope_class(),
            PlanarBooleanEdgeSplitScopeClass::PlanarBRepLineSegmentEdgeSurgery
        );
        assert_eq!(
            admission.split_request_identity(),
            request.split_request_identity()
        );
        assert_eq!(
            admission.event_ledger_identity(),
            subject.ledger().event_ledger_identity()
        );
        assert_eq!(
            admission.reduced_pair_identity(),
            subject.ledger().reduced_pair_identity()
        );
        assert_eq!(
            admission.candidate_index_product_identity(),
            gate.candidate_index_product_identity()
        );
        assert_eq!(
            admission.query_index_plan_digest(),
            gate.query_index_plan_digest()
        );
        assert_eq!(
            admission.counters().source_carrier_count(),
            subject.ledger().segment_carriers().len()
        );
    });
}

#[test]
fn edge_split_scope_policy_outcome_maps_to_kernel_machine_kind() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = MetabossEventExtractionSubject::certify("phase7.3 split scope outcome kind");
        let (request, _) = metaboss_edge_split_request(&subject);
        let admission = PlanarBooleanEdgeSplitScopeAdmission::admit(
            PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&request),
        )
        .expect("metaboss split request should admit");

        assert_policy_kind_maps(
            admission.policy_outcome().kind(),
            PlanarBooleanOutcomeKind::Admitted,
        );
        assert_eq!(
            admission.policy_outcome().event_ledger_identity(),
            subject.ledger().event_ledger_identity()
        );
        assert!(admission
            .policy_outcome()
            .is_admitted_for_source_edge_recovery());
    });
}

#[test]
fn edge_split_policy_outcome_taxonomy_preserves_distinct_kernel_machine_kinds() {
    let expected_pairs = [
        (
            PlanarBooleanEdgeSplitPolicyOutcomeKind::Admitted,
            PlanarBooleanOutcomeKind::Admitted,
        ),
        (
            PlanarBooleanEdgeSplitPolicyOutcomeKind::Unsupported,
            PlanarBooleanOutcomeKind::Unsupported,
        ),
        (
            PlanarBooleanEdgeSplitPolicyOutcomeKind::Blocked,
            PlanarBooleanOutcomeKind::Blocked,
        ),
        (
            PlanarBooleanEdgeSplitPolicyOutcomeKind::Denied,
            PlanarBooleanOutcomeKind::Denied,
        ),
        (
            PlanarBooleanEdgeSplitPolicyOutcomeKind::PolicyRequired,
            PlanarBooleanOutcomeKind::PolicyRequired,
        ),
        (
            PlanarBooleanEdgeSplitPolicyOutcomeKind::IntegrityMismatch,
            PlanarBooleanOutcomeKind::IntegrityMismatch,
        ),
    ];

    for (spatial_kind, kernel_kind) in expected_pairs {
        assert_policy_kind_maps(spatial_kind, kernel_kind);
        assert_eq!(
            spatial_kind.stable_name(),
            kernel_outcome_kind_stable_name(kernel_kind)
        );
    }
}

fn assert_policy_kind_maps(
    spatial_kind: PlanarBooleanEdgeSplitPolicyOutcomeKind,
    expected_kernel_kind: PlanarBooleanOutcomeKind,
) {
    let kernel_kind = match spatial_kind {
        PlanarBooleanEdgeSplitPolicyOutcomeKind::Admitted => PlanarBooleanOutcomeKind::Admitted,
        PlanarBooleanEdgeSplitPolicyOutcomeKind::Unsupported => {
            PlanarBooleanOutcomeKind::Unsupported
        }
        PlanarBooleanEdgeSplitPolicyOutcomeKind::Blocked => PlanarBooleanOutcomeKind::Blocked,
        PlanarBooleanEdgeSplitPolicyOutcomeKind::Denied => PlanarBooleanOutcomeKind::Denied,
        PlanarBooleanEdgeSplitPolicyOutcomeKind::PolicyRequired => {
            PlanarBooleanOutcomeKind::PolicyRequired
        }
        PlanarBooleanEdgeSplitPolicyOutcomeKind::IntegrityMismatch => {
            PlanarBooleanOutcomeKind::IntegrityMismatch
        }
    };
    assert_eq!(kernel_kind, expected_kernel_kind);
}

fn kernel_outcome_kind_stable_name(kind: PlanarBooleanOutcomeKind) -> &'static str {
    match kind {
        PlanarBooleanOutcomeKind::Admitted => "admitted",
        PlanarBooleanOutcomeKind::Unsupported => "unsupported",
        PlanarBooleanOutcomeKind::Blocked => "blocked",
        PlanarBooleanOutcomeKind::Denied => "denied",
        PlanarBooleanOutcomeKind::PolicyRequired => "policy-required",
        PlanarBooleanOutcomeKind::IntegrityMismatch => "integrity-mismatch",
        PlanarBooleanOutcomeKind::NoOptions => "no-options",
    }
}

fn metaboss_edge_split_request(
    subject: &MetabossEventExtractionSubject,
) -> (
    PlanarBooleanEdgeSplitRequest,
    PlanarBooleanCandidateIndexConsumptionGate,
) {
    let segment_pairs = &subject.inputs().pair_worklist;
    let ledger = subject.ledger();
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(ledger),
    ])
    .expect("metaboss boolean evidence should index");
    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            ledger,
            segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index gate should admit");
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        ledger,
        &gate,
        evidence.stage_index(),
    ))
    .expect("edge split request should admit from event ledger and candidate-index gate");
    (request, gate)
}
