use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceRowAuthority, CompleteWorkloadEvidenceLedger,
    SpatialGeometryEvidenceTouchRequest, WorkloadEvidenceLedger, WorkloadEvidenceRow,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    event_ledger_lookup_execution_subject, raw_interval_entry, raw_point_entry, raw_schedule,
    raw_set_from_schedules, recover_source_edge_carriers_for_tests, source_carriers_for_tests,
    split_subject_with_carriers_for_tests, PlanarBooleanCandidateIndexConsumptionGate,
    PlanarBooleanCandidateIndexConsumptionInput, PlanarBooleanDownstreamSplitConsumption,
    PlanarBooleanDownstreamSplitConsumptionInput, PlanarBooleanEdgeSplitReplayParityReceipt,
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEdgeSplitRequestInput,
    PlanarBooleanEdgeSplitScopeAdmission, PlanarBooleanEdgeSplitScopeAdmissionInput,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput, PlanarBooleanMicroIntervalPolicy,
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanSplitDecisionLogQueryDomain,
    PlanarBooleanSplitDecisionLogQueryInput, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitEdgeChainLedgerQueryDomain, PlanarBooleanSplitEdgeChainLedgerQueryInput,
    PlanarBooleanSplitEdgeChainLedgerQueryResult, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitPersistentNamingInput, PlanarBooleanSplitPersistentNamingQueryBasis,
    PlanarBooleanSplitPersistentNamingReceipt, PlanarBooleanSplitSourceEdgeCarrierSet,
    PlanarBooleanSplitVertexIdentitySet, SourceEdgeCarrierRecoverySubject,
};

use super::super::request::{
    PlanarBooleanLoopReconstructionRequest, PlanarBooleanLoopReconstructionRequestInput,
};
use super::replay_parity_subject::replay_parity_receipt_for;
use super::replay_support::retained_replay_receipt_chain;
use crate::workload_platform::retained_replay_workload::ReplayReceiptSet;
use crate::workload_platform::vocabulary::{
    DiagnosticWorkload, GeometryBindingWorkload, ProjectionWorkload, ResponseWorkload,
    RetainedReplayWorkload, SurfaceSupportWorkload, TransformWorkload,
};
use topology::facade::TopologySeed;

pub(crate) enum LoopFixtureEntryOrder {
    Canonical,
    Replayed,
}

pub(crate) struct PreparedLoopReconstructionSubject {
    pub(crate) request_subject: SourceEdgeCarrierRecoverySubject,
    pub(crate) recovered_source_carriers: PlanarBooleanSplitSourceEdgeCarrierSet,
    pub(crate) interval_subdivision: PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    pub(crate) vertices: PlanarBooleanSplitVertexIdentitySet,
    pub(crate) fragments: PlanarBooleanSplitEdgeFragmentSet,
    pub(crate) overlap_chains: PlanarBooleanOverlapEdgeChainSet,
    pub(crate) naming: PlanarBooleanSplitPersistentNamingReceipt,
    pub(crate) decision_log: PlanarBooleanSplitDecisionLogQueryResult,
    pub(crate) split_ledger_result: PlanarBooleanSplitEdgeChainLedgerQueryResult,
    pub(crate) loop_split_consumption: PlanarBooleanLoopReconstructionSplitConsumption,
}

impl PreparedLoopReconstructionSubject {
    pub(crate) fn admit_loop_request(&self) -> PlanarBooleanLoopReconstructionRequest {
        PlanarBooleanLoopReconstructionRequest::admit(
            PlanarBooleanLoopReconstructionRequestInput::from_split_consumption(
                &self.loop_split_consumption,
            ),
        )
        .expect("loop reconstruction request should admit")
    }
}

pub(crate) fn prepared_loop_reconstruction_subject(
    order: LoopFixtureEntryOrder,
) -> PreparedLoopReconstructionSubject {
    prepared_loop_reconstruction_subject_with_tag(order, "phase-3")
}

pub(crate) fn prepared_loop_reconstruction_subject_with_tag(
    order: LoopFixtureEntryOrder,
    tag: &str,
) -> PreparedLoopReconstructionSubject {
    let carriers = source_carriers_for_tests();
    let anchor = carriers
        .first()
        .cloned()
        .expect("test support should provide at least one carrier");
    let source_edge_identity = anchor.source_edge_identity().to_string();
    let carrier_identity = anchor.carrier_identity().to_string();
    let replay_receipts = retained_replay_receipt_chain(tag);
    let request_subject = request_subject_with_replay_evidence(
        split_subject_with_carriers_for_tests(carriers),
        &replay_receipts,
    );
    let recovered_source_carriers = recover_source_edge_carriers_for_tests(&request_subject);

    let endpoint_boundary = raw_set_from_schedules(vec![raw_schedule(
        &format!("{tag}-raw-schedule"),
        &source_edge_identity,
        &carrier_identity,
        raw_entries(order, tag, &source_edge_identity, &carrier_identity),
    )])
    .canonicalize_split_schedule_order()
    .expect("raw schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicates should normalize")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint boundary should normalize");
    let interval_subdivision = endpoint_boundary
        .normalize_overlap_interval_subdivisions(
            PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance,
        )
        .expect("interval subdivisions should normalize");
    let vertices = interval_subdivision
        .mint_split_vertex_identities()
        .expect("split vertices should mint");
    let fragments = interval_subdivision
        .build_split_edge_fragments(&vertices)
        .expect("split fragments should build");
    let overlap_chains = interval_subdivision
        .build_overlap_edge_chains(&fragments)
        .expect("overlap chains should build");
    let validation = fragments
        .validate_split_edge_chains(&overlap_chains)
        .expect("split chain validation should admit");
    let naming = PlanarBooleanSplitPersistentNamingReceipt::admit(
        PlanarBooleanSplitPersistentNamingInput::new(
            &validation,
            &fragments,
            &vertices,
            &overlap_chains,
            PlanarBooleanSplitPersistentNamingQueryBasis::from_query_runtime(
                "worth.topology/current_head_authoritative",
                "persistent-name-live-view",
                "naming-attachment-report",
            ),
        ),
    )
    .expect("persistent naming should admit");
    let decision_log = PlanarBooleanSplitDecisionLogQueryDomain::declare(
        PlanarBooleanSplitDecisionLogQueryInput::new(
            &request_subject.request,
            &endpoint_boundary,
            &interval_subdivision,
            &vertices,
            &fragments,
            &validation,
            &naming,
        ),
    )
    .expect("decision log declaration should lower")
    .execute()
    .expect("decision log should execute");
    let split_ledger_result = PlanarBooleanSplitEdgeChainLedgerQueryDomain::declare(
        PlanarBooleanSplitEdgeChainLedgerQueryInput::new(
            &request_subject.request,
            &endpoint_boundary,
            &interval_subdivision,
            &vertices,
            &fragments,
            &overlap_chains,
            &validation,
            &naming,
            &decision_log,
        ),
    )
    .expect("split ledger declaration should lower")
    .execute()
    .expect("split ledger should execute");

    let replay_parity = replay_parity_receipt_for(
        &request_subject,
        &split_ledger_result,
        &decision_log,
        &validation,
        &naming,
        &fragments,
        &overlap_chains,
        &replay_receipts,
    );
    let evidence_ledger =
        complete_loop_test_spatial_touch_ledger(tag, split_ledger_result.receipt());
    let spatial_touch_authority =
        SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(split_ledger_result.receipt())
            .with_complete_ledger(&evidence_ledger)
            .admit()
            .expect("split receipt should admit spatial touch authority");
    let downstream_split_consumption = PlanarBooleanDownstreamSplitConsumption::admit(
        PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(
            split_ledger_result.receipt(),
            decision_log.receipt(),
            &validation,
            &naming,
            &replay_parity,
            &spatial_touch_authority,
        ),
    )
    .expect("downstream split consumption should admit");
    let loop_split_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
            &downstream_split_consumption,
        ),
    )
    .expect("loop split consumption should admit");

    PreparedLoopReconstructionSubject {
        request_subject,
        recovered_source_carriers,
        interval_subdivision,
        vertices,
        fragments,
        overlap_chains,
        naming,
        decision_log,
        split_ledger_result,
        loop_split_consumption,
    }
}

fn complete_loop_test_spatial_touch_ledger<T>(
    tag: &str,
    receipt: &T,
) -> CompleteWorkloadEvidenceLedger
where
    T: BooleanEvidenceRowAuthority + 'static,
{
    let label = format!("{tag} loop-test spatial touch authority");
    let topology = TopologySeed::cube()
        .with_declaration(label.clone())
        .build()
        .expect("topology seed should certify");
    let topology_workload = topology.query_receipts().declaration_receipt();
    let geometry = GeometryBindingWorkload::for_topology_receipt(topology_workload)
        .declared(format!("{label} geometry binding"))
        .admit()
        .expect("geometry binding should certify");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .declared(format!("{label} surface support"))
        .admit()
        .expect("surface support should certify");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .declared(format!("{label} projection"))
        .admit()
        .expect("projection should certify");
    let transform = TransformWorkload::for_projection(&projection)
        .declared(format!("{label} transform"))
        .admit()
        .expect("transform should certify");
    let replay = RetainedReplayWorkload::for_transform(&transform)
        .declared(format!("{label} retained replay"))
        .admit()
        .expect("retained replay should certify");
    let diagnostics = DiagnosticWorkload::for_retained_replay(&replay)
        .declared(format!("{label} diagnostics"))
        .admit()
        .expect("diagnostics should certify");
    let response = ResponseWorkload::for_diagnostics(&diagnostics)
        .declared(format!("{label} response"))
        .admit()
        .expect("response should certify");

    WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_topology_seed_receipt(&topology),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::GeometryBinding,
            geometry.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::binding(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::SurfaceSupport,
            support.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::surface_support(1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Projection,
            projection.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::projection(1, 1),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::Transform,
            transform.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::transform(1, 1, 0),
        ),
        WorkloadEvidenceRow::receipt_backed(
            WorkloadEvidenceStage::RetainedReplay,
            replay.identity().receipt_identity(),
            WorkloadEvidenceStageCounters::retained_replay(1, 1),
        ),
        WorkloadEvidenceRow::from_diagnostic_receipt(&diagnostics),
        WorkloadEvidenceRow::from_response_receipt(&response),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(receipt),
    ])
    .expect("loop test spatial touch rows should index")
    .certify_complete()
    .expect("loop test spatial touch rows should complete")
}

fn request_subject_with_replay_evidence(
    subject: SourceEdgeCarrierRecoverySubject,
    replay_receipts: &ReplayReceiptSet,
) -> SourceEdgeCarrierRecoverySubject {
    let evidence = WorkloadEvidenceLedger::from_rows(vec![
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
        WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.ledger),
        WorkloadEvidenceRow::from_replay_receipt_set(replay_receipts),
    ])
    .expect("replay-backed receipt evidence should index");
    let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
        PlanarBooleanCandidateIndexConsumptionInput::new(
            &subject.ledger,
            &subject.segment_pairs,
            evidence.stage_index(),
        ),
    )
    .expect("candidate-index gate should admit with replay-backed evidence");
    let event_ledger_lookup = event_ledger_lookup_execution_subject(
        "loop-runtime",
        &subject.ledger,
        vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(&subject.ledger),
            WorkloadEvidenceRow::from_replay_receipt_set(replay_receipts),
        ],
    );
    let retained_replay_links = event_ledger_lookup
        .complete_ledger
        .stage_index()
        .link_required_stages(&[WorkloadEvidenceStage::RetainedReplay])
        .expect("retained replay stage link should admit");
    let request = PlanarBooleanEdgeSplitRequest::admit(PlanarBooleanEdgeSplitRequestInput::new(
        &subject.ledger,
        &gate,
        &event_ledger_lookup.witness,
        Some(&retained_replay_links),
    ))
    .expect("split request should admit with replay-backed evidence");
    let scope = PlanarBooleanEdgeSplitScopeAdmission::admit(
        PlanarBooleanEdgeSplitScopeAdmissionInput::from_split_request(&request),
    )
    .expect("scope should admit with replay-backed request");
    SourceEdgeCarrierRecoverySubject {
        segment_pairs: subject.segment_pairs,
        ledger: subject.ledger,
        request,
        scope,
    }
}

fn raw_entries(
    order: LoopFixtureEntryOrder,
    tag: &str,
    source_edge_identity: &str,
    carrier_identity: &str,
) -> Vec<
    crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanRawEdgeSplitScheduleEntry,
> {
    let point_a = raw_point_entry(
        &format!("{tag}-point-a"),
        source_edge_identity,
        carrier_identity,
        &format!("{tag}-event:a"),
        0.25,
    );
    let interval = raw_interval_entry(
        &format!("{tag}-interval"),
        source_edge_identity,
        carrier_identity,
        &format!("{tag}-event:interval"),
        0.5,
    );
    let point_b = raw_point_entry(
        &format!("{tag}-point-b"),
        source_edge_identity,
        carrier_identity,
        &format!("{tag}-event:b"),
        0.75,
    );
    match order {
        LoopFixtureEntryOrder::Canonical => vec![point_a, interval, point_b],
        LoopFixtureEntryOrder::Replayed => vec![point_b, interval, point_a],
    }
}
