#![allow(dead_code)]

use super::edge_splitting_persistent_naming_support::typed_topology_query_basis;
use super::metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanAdmittedIntervalSplitCandidateSet, PlanarBooleanCandidateIndexConsumptionGate,
    PlanarBooleanCandidateIndexConsumptionInput, PlanarBooleanEdgeSplitRequest,
    PlanarBooleanEdgeSplitRequestInput, PlanarBooleanEdgeSplitScopeAdmission,
    PlanarBooleanEdgeSplitScopeAdmissionInput,
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanMicroIntervalPolicy,
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanPointSplitPostureSet,
    PlanarBooleanRawEdgeSplitScheduleSet, PlanarBooleanSplitChainValidationReceipt,
    PlanarBooleanSplitDecisionLogQueryDomain, PlanarBooleanSplitDecisionLogQueryInput,
    PlanarBooleanSplitDecisionLogQueryResult, PlanarBooleanSplitDecisionReason,
    PlanarBooleanSplitEdgeFragmentSet, PlanarBooleanSplitEventParticipationIndex,
    PlanarBooleanSplitPersistentNamingInput, PlanarBooleanSplitPersistentNamingReceipt,
    PlanarBooleanSplitSourceEdgeCarrierRecoveryInput, PlanarBooleanSplitSourceEdgeCarrierSet,
    PlanarBooleanSplitVertexIdentitySet,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

pub(crate) fn assert_split_decision_log_matches_metaboss(subject: &MetabossEventExtractionSubject) {
    let products = build_decision_log_products_for_metaboss(subject);
    let result = PlanarBooleanSplitDecisionLogQueryDomain::declare(
        PlanarBooleanSplitDecisionLogQueryInput::new(
            &products.request,
            &products.endpoint_boundary,
            &products.interval_subdivision,
            &products.vertices,
            &products.fragments,
            &products.validation,
            &products.naming,
        ),
    )
    .expect("metaboss decision-log Query declaration should lower")
    .execute()
    .expect("metaboss decision-log Query plan should execute");

    assert!(result.certifies_query_owned_decision_log());
    assert_eq!(
        result.coverage().observed_rows(),
        expected_decision_row_count(&products)
    );
    assert_decision_rows_are_machine_typed(&result);
    assert_eq!(
        result.receipt().split_chain_validation_receipt_identity(),
        products.validation.receipt_identity()
    );
    assert_eq!(
        result.receipt().split_persistent_naming_receipt_identity(),
        products.naming.receipt_identity()
    );
}

pub(crate) struct DecisionLogMetabossProducts {
    pub(crate) request: PlanarBooleanEdgeSplitRequest,
    pub(crate) endpoint_boundary: PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    pub(crate) interval_subdivision: PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    pub(crate) vertices: PlanarBooleanSplitVertexIdentitySet,
    pub(crate) fragments: PlanarBooleanSplitEdgeFragmentSet,
    pub(crate) chains: PlanarBooleanOverlapEdgeChainSet,
    pub(crate) validation: PlanarBooleanSplitChainValidationReceipt,
    pub(crate) naming: PlanarBooleanSplitPersistentNamingReceipt,
}

pub(crate) fn build_decision_log_products_for_metaboss(
    subject: &MetabossEventExtractionSubject,
) -> DecisionLogMetabossProducts {
    let request = split_request_for_metaboss(subject);
    let recovered = recovered_carriers_from_request(subject, &request);
    let participation = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
        &recovered,
        subject.ledger(),
    )
    .expect("metaboss participation index should derive from recovered carriers");
    let raw = raw_schedule_from_participation(&participation);
    let ordered = raw
        .canonicalize_split_schedule_order()
        .expect("raw metaboss split schedules should canonicalize before normalization");
    let normalized = ordered
        .collapse_duplicate_split_points()
        .expect("metaboss ordered split schedules should normalize duplicate point cuts");
    let endpoint_boundary = normalized
        .normalize_endpoint_boundary_splits()
        .expect("metaboss endpoint-boundary schedules should normalize before decision logging");
    let interval_subdivision = endpoint_boundary
        .normalize_overlap_interval_subdivisions(
            PlanarBooleanMicroIntervalPolicy::RequireExplicitDecision,
        )
        .expect("metaboss interval subdivisions should normalize before decision logging");
    let vertices = interval_subdivision
        .mint_split_vertex_identities()
        .expect("metaboss split vertices should mint before decision logging");
    let fragments = interval_subdivision
        .build_split_edge_fragments(&vertices)
        .expect("metaboss fragments should build before decision logging");
    let chains = interval_subdivision
        .build_overlap_edge_chains(&fragments)
        .expect("metaboss overlap chains should build before decision logging");
    let validation = fragments
        .validate_split_edge_chains(&chains)
        .expect("metaboss split-chain validation should certify before decision logging");
    let naming = PlanarBooleanSplitPersistentNamingReceipt::admit(
        PlanarBooleanSplitPersistentNamingInput::new(
            &validation,
            &fragments,
            &vertices,
            &chains,
            typed_topology_query_basis(),
        ),
    )
    .expect("metaboss persistent naming should admit before decision logging");
    DecisionLogMetabossProducts {
        request,
        endpoint_boundary,
        interval_subdivision,
        vertices,
        fragments,
        chains,
        validation,
        naming,
    }
}

fn split_request_for_metaboss(
    subject: &MetabossEventExtractionSubject,
) -> PlanarBooleanEdgeSplitRequest {
    let segment_pairs = &subject.inputs().pair_worklist;
    let ledger = subject.ledger();
    let mut evidence_rows = vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(ledger),
    ];
    if let Some(replay_receipts) = subject.pair().left().replay_receipts() {
        evidence_rows.push(WorkloadEvidenceRow::from_replay_receipt_set(
            replay_receipts,
        ));
    }
    let evidence = WorkloadEvidenceLedger::from_rows(evidence_rows)
        .expect("metaboss boolean receipts should build evidence for split request");
    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            ledger,
            segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("metaboss candidate-index gate should admit before split request");
    let event_ledger_lookup = evidence
        .require_boolean_receipt_lookup(ledger)
        .expect("typed event-ledger lookup should admit before split request");
    let retained_replay_stage_links = subject.pair().left().replay_receipts().map(|_| {
        evidence
            .stage_index()
            .link_required_stages(&[WorkloadEvidenceStage::RetainedReplay])
            .expect("retained replay stage link should admit when replay evidence is present")
    });
    PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        ledger,
        &gate,
        &event_ledger_lookup,
        retained_replay_stage_links.as_ref(),
    ))
    .expect("metaboss split request should admit before decision logging")
}

fn recovered_carriers_from_request(
    subject: &MetabossEventExtractionSubject,
    request: &PlanarBooleanEdgeSplitRequest,
) -> PlanarBooleanSplitSourceEdgeCarrierSet {
    let scope = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(request),
    )
    .expect("split scope should admit before source-edge carrier recovery");
    PlanarBooleanSplitSourceEdgeCarrierSet::recover(
        PlanarBooleanSplitSourceEdgeCarrierRecoveryInput::from_scope_and_event_ledger(
            &scope,
            subject.ledger(),
        ),
    )
    .expect("source-edge carriers should recover from scoped metaboss ledger")
}

fn raw_schedule_from_participation(
    participation: &PlanarBooleanSplitEventParticipationIndex,
) -> PlanarBooleanRawEdgeSplitScheduleSet {
    let point_candidates = participation
        .extract_point_split_candidates()
        .expect("metaboss point candidates should extract from the participation index");
    let admitted_points = point_candidates
        .admit_parameter_domain()
        .expect("metaboss point candidates should admit in domain");
    let postures = admitted_points
        .classify_point_split_postures()
        .expect("metaboss point postures should classify");
    let interval_candidates = participation
        .extract_interval_split_candidates()
        .expect("metaboss interval candidates should extract from the participation index");
    let admitted_intervals = interval_candidates
        .admit_parameter_domain()
        .expect("metaboss interval candidates should admit in domain");

    assert_eq!(
        postures.participation_index_identity(),
        admitted_intervals.participation_index_identity()
    );
    assemble_raw_schedule(&postures, &admitted_intervals)
}

fn assemble_raw_schedule(
    postures: &PlanarBooleanPointSplitPostureSet,
    admitted_intervals: &PlanarBooleanAdmittedIntervalSplitCandidateSet,
) -> PlanarBooleanRawEdgeSplitScheduleSet {
    PlanarBooleanRawEdgeSplitScheduleSet::assemble_from_admitted_candidates(
        postures,
        admitted_intervals,
    )
    .expect("raw schedules should assemble from same-index candidate products")
}

fn expected_decision_row_count(products: &DecisionLogMetabossProducts) -> usize {
    1 + products
        .endpoint_boundary
        .endpoint_contact_decisions()
        .count()
        + interval_subdivision_count(&products.interval_subdivision)
        + products.vertices.coalescence_decisions().count()
        + products.fragments.fragments().count()
        + products.validation.fragment_coverage_rows().len()
        + products.validation.overlap_coverage_rows().len()
        + products.naming.persistent_name_rows().len()
}

fn interval_subdivision_count(
    interval_subdivision: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
) -> usize {
    interval_subdivision
        .schedules()
        .iter()
        .map(|schedule| schedule.interval_subdivisions().len())
        .sum()
}

fn assert_decision_rows_are_machine_typed(result: &PlanarBooleanSplitDecisionLogQueryResult) {
    assert!(result.receipt().decision_rows().iter().all(|row| {
        !row.decision_reason().reason_name().is_empty()
            && (!reason_carries_legacy_detail(row.decision_reason())
                || row.decision_reason().detail() == row.policy_or_denial_kind())
    }));
    assert!(result.receipt().decision_rows().iter().any(|row| matches!(
        row.decision_reason(),
        PlanarBooleanSplitDecisionReason::PersistentNamePropagated
    )));
}

fn reason_carries_legacy_detail(reason: &PlanarBooleanSplitDecisionReason) -> bool {
    matches!(
        reason,
        PlanarBooleanSplitDecisionReason::SplitVertexCoalesced(_)
            | PlanarBooleanSplitDecisionReason::SplitPhaseDenied(_)
    )
}
