mod foreign_authority;

use std::marker::PhantomData;

use super::edge_splitting_replay_parity_support::{
    build_edge_split_replay_parity_subject, replay_parity_report, EdgeSplitReplayParitySubject,
};
use super::metaboss_support::MetabossEventExtractionSubject;
use foreign_authority::assert_foreign_split_authorities_are_rejected;
use topology::facade::TopologyTouchedGraphBasis;
use worth_kernel::workload_composition::CompletedBooleanSplitHandoff;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanEdgeSplitReplayParityReport,
    PlanarBooleanLoopReconstructionSplitConsumption,
    PlanarBooleanLoopReconstructionSplitConsumptionInput,
};
use worth_spatial::facade::workload_vocabulary::{
    deny_manual_evidence_row_as_spatial_touch_authority,
    deny_query_descriptor_digest_as_spatial_evidence_lookup_authority,
    deny_raw_row_as_spatial_query_lowering_authority,
    deny_topology_touched_basis_as_spatial_query_lowering_authority,
    deny_topology_touched_graph_basis_as_spatial_touch_authority,
    spatial_evidence_surface_deletion_ledger, BooleanEvidenceStageKind,
    SpatialEvidenceLookupDenialKind, SpatialEvidenceQueryLoweringDenialKind,
    SpatialEvidenceSubstitutionDenial, SpatialEvidenceSurfaceDeletionAction,
    SpatialEvidenceTopologySubstitutionSurface, SpatialGeometryEvidenceTouchDenialKind,
    SpatialGeometryEvidenceTouchRequest, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

pub(crate) fn assert_split_public_contract_requires_real_ledger_and_preserves_authority_boundaries()
{
    let subject =
        MetabossEventExtractionSubject::certify("phase7.3 public downstream split consumption");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    assert_completed_split_handoff_admits_spatial_touch_authority(
        &completed_split_handoff,
        &replay_subject,
    );
    let consumption = admit_real_downstream_split_consumption(
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );

    assert_downstream_consumption_preserves_real_split_authority(
        &consumption,
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );
    assert_loop_reconstruction_consumes_downstream_split_product(&consumption);
    assert_foreign_split_authorities_are_rejected(
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );
    assert_lower_authority_substitutes_are_rejected_before_split_consumption(
        &completed_split_handoff,
        &replay_subject,
    );
    assert_split_handoff_dependency_direction_uses_spatial_facade_proof();
    assert_displaced_stage_index_consumer_path_is_deleted_residue();
}

pub(crate) fn assert_split_downstream_migration_uses_spatial_facade_proof_product() {
    let subject = MetabossEventExtractionSubject::certify("phase6 downstream split migration");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let replay_report = replay_parity_report(&replay_subject);
    let completed_split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    assert_completed_split_handoff_admits_spatial_touch_authority(
        &completed_split_handoff,
        &replay_subject,
    );
    let consumption = admit_real_downstream_split_consumption(
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );

    assert_downstream_consumption_preserves_real_split_authority(
        &consumption,
        &replay_subject,
        &replay_report,
        &completed_split_handoff,
    );
    assert_lower_authority_substitutes_are_rejected_before_split_consumption(
        &completed_split_handoff,
        &replay_subject,
    );
    assert_split_handoff_dependency_direction_uses_spatial_facade_proof();
    assert_displaced_stage_index_consumer_path_is_deleted_residue();
}

pub(crate) fn assert_split_handoff_admits_spatial_touch_authority_from_completed_workload() {
    let subject =
        MetabossEventExtractionSubject::certify("phase4 spatial touch split handoff admission");
    let replay_subject = build_edge_split_replay_parity_subject(&subject);
    let completed_split_handoff = completed_split_handoff_for(&subject, &replay_subject);
    assert_completed_split_handoff_admits_spatial_touch_authority(
        &completed_split_handoff,
        &replay_subject,
    );
}

fn assert_completed_split_handoff_admits_spatial_touch_authority(
    completed_split_handoff: &CompletedBooleanSplitHandoff,
    replay_subject: &EdgeSplitReplayParitySubject,
) {
    let authority = completed_split_handoff
        .completed_workload()
        .admit_spatial_geometry_evidence_touch(replay_subject.original_ledger.receipt())
        .expect("real completed split workload must admit spatial touch authority");
    assert_eq!(authority.boolean_stage(), BooleanEvidenceStageKind::Split);
    assert_eq!(
        authority.evidence_identity(),
        replay_subject.original_ledger.receipt().receipt_identity()
    );
    assert_eq!(
        authority.stage_index_identity(),
        completed_split_handoff.workload_stage_index_identity()
    );
    assert_eq!(authority.lookup_counters().indexed_lookup_count(), 1);
    assert_eq!(authority.lookup_counters().raw_row_scan_count(), 0);
}

pub(crate) fn completed_split_handoff_for(
    subject: &MetabossEventExtractionSubject,
    replay_subject: &EdgeSplitReplayParitySubject,
) -> CompletedBooleanSplitHandoff {
    let completed_split_handoff = subject
        .pair()
        .left()
        .workload()
        .complete_boolean_split_handoff(replay_subject.original_ledger.receipt())
        .expect("real workload should produce a proof-bearing split completion handoff");
    completed_split_handoff
        .require_boolean_split()
        .expect("completed split handoff should require the exact split ledger receipt");
    completed_split_handoff
}

fn admit_real_downstream_split_consumption(
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) -> PlanarBooleanDownstreamSplitConsumption {
    completed_split_handoff
        .admit_downstream_split_consumption(
            replay_subject.original_decision_log.receipt(),
            &replay_subject.original_products.validation,
            &replay_subject.original_products.naming,
            replay_report.receipt(),
        )
        .expect("real split ledger receipt should admit downstream split consumption")
}

fn assert_downstream_consumption_preserves_real_split_authority(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) {
    assert!(consumption.certifies_downstream_split_consumption());
    assert_eq!(
        consumption.split_ledger_receipt_identity(),
        replay_subject.original_ledger.receipt().receipt_identity()
    );
    assert_eq!(
        consumption.split_ledger_downstream_identity(),
        replay_subject
            .original_ledger
            .receipt()
            .downstream_consumption_identity()
    );
    assert_eq!(
        consumption.decision_log_receipt_identity(),
        replay_subject
            .original_decision_log
            .receipt()
            .receipt_identity()
    );
    assert_eq!(
        consumption.validation_receipt_identity(),
        replay_subject
            .original_products
            .validation
            .receipt_identity()
    );
    assert_eq!(
        consumption.persistent_naming_receipt_identity(),
        replay_subject.original_products.naming.receipt_identity()
    );
    assert_eq!(
        consumption.replay_parity_receipt_identity(),
        replay_report.receipt().receipt_identity()
    );
    assert_eq!(
        consumption.workload_stage_index_identity(),
        completed_split_handoff.workload_stage_index_identity()
    );
    assert_downstream_consumption_matches_spatial_facade_lookup(
        consumption,
        replay_subject,
        completed_split_handoff,
    );
    assert_downstream_consumption_counters_match_real_split_authority(
        consumption,
        replay_subject,
        replay_report,
    );
}

fn assert_downstream_consumption_matches_spatial_facade_lookup(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
    replay_subject: &EdgeSplitReplayParitySubject,
    completed_split_handoff: &CompletedBooleanSplitHandoff,
) {
    let authority = completed_split_handoff
        .completed_workload()
        .admit_spatial_geometry_evidence_touch(replay_subject.original_ledger.receipt())
        .expect("real split receipt should admit spatial touch authority");
    let lookup = authority
        .spatial_evidence_lookup(
            completed_split_handoff
                .completed_workload()
                .evidence_ledger(),
        )
        .expect("real split authority should admit spatial evidence lookup");
    assert_eq!(authority.boolean_stage(), BooleanEvidenceStageKind::Split);
    assert_eq!(
        consumption.split_ledger_receipt_identity(),
        authority.evidence_identity()
    );
    assert_eq!(
        consumption.spatial_lookup_key(),
        lookup.lookup_key().as_str()
    );
    assert_eq!(
        consumption.spatial_lookup_product_digest().as_str(),
        lookup.product_digest().as_str()
    );
    assert_eq!(consumption.spatial_support(), lookup.support());
    assert_eq!(consumption.spatial_stage_counters(), lookup.counters());
    assert_eq!(
        consumption.spatial_lookup_counters(),
        lookup.lookup_counters()
    );
}

fn assert_downstream_consumption_counters_match_real_split_authority(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
    replay_subject: &EdgeSplitReplayParitySubject,
    replay_report: &PlanarBooleanEdgeSplitReplayParityReport,
) {
    assert_eq!(
        consumption.counters().split_chains_consumed(),
        replay_subject
            .original_ledger
            .receipt()
            .chain_identities()
            .len()
    );
    assert_eq!(
        consumption.counters().fragment_rows_consumed(),
        replay_subject
            .original_products
            .validation
            .fragment_coverage_rows()
            .len()
    );
    assert_eq!(
        consumption.counters().vertex_rows_consumed(),
        replay_subject
            .original_decision_log
            .receipt()
            .counters()
            .coalescence_decisions_recorded()
    );
    assert_eq!(
        consumption.counters().persistent_name_rows_consumed(),
        replay_subject
            .original_products
            .naming
            .persistent_name_rows()
            .len()
    );
    assert_eq!(
        consumption.counters().replay_parity_rows_consumed(),
        replay_report.receipt().parity_rows().len()
    );
    assert_eq!(consumption.counters().spatial_lookup_products_consumed(), 1);
    assert_eq!(consumption.counters().spatial_lookup_indexed_lookups(), 1);
    assert_eq!(consumption.counters().spatial_lookup_raw_row_scans(), 0);
    assert_eq!(consumption.counters().foreign_receipts_rejected(), 0);
    assert_eq!(consumption.counters().missing_receipts_rejected(), 0);
    assert_eq!(consumption.counters().non_receipt_evidence_rejected(), 0);
}

fn assert_loop_reconstruction_consumes_downstream_split_product(
    consumption: &PlanarBooleanDownstreamSplitConsumption,
) {
    let loop_consumption = PlanarBooleanLoopReconstructionSplitConsumption::admit(
        PlanarBooleanLoopReconstructionSplitConsumptionInput::from_downstream_split_consumption(
            consumption,
        ),
    )
    .expect("loop reconstruction should consume only the downstream split-consumption product");
    assert!(loop_consumption.certifies_loop_reconstruction_split_consumption());
    assert_eq!(
        loop_consumption.downstream_consumption_identity(),
        consumption.consumption_identity()
    );
    assert_eq!(
        loop_consumption.split_ledger_receipt_identity(),
        consumption.split_ledger_receipt_identity()
    );
    assert_eq!(
        loop_consumption.split_ledger_downstream_identity(),
        consumption.split_ledger_downstream_identity()
    );
    assert_eq!(
        loop_consumption.split_request_identity(),
        consumption.split_request_identity()
    );
    assert_eq!(
        loop_consumption.workload_stage_index_identity(),
        consumption.workload_stage_index_identity()
    );
    assert_eq!(loop_consumption.counters().downstream_gate_consumed(), 1);
}

fn assert_lower_authority_substitutes_are_rejected_before_split_consumption(
    _completed_split_handoff: &CompletedBooleanSplitHandoff,
    replay_subject: &EdgeSplitReplayParitySubject,
) {
    let manual_row = WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual split");
    let manual_denial = deny_manual_evidence_row_as_spatial_touch_authority(&manual_row)
        .expect_err("manual row cannot satisfy spatial touch authority");
    assert!(matches!(
        manual_denial,
        SpatialEvidenceSubstitutionDenial::ManualEvidenceRow { .. }
    ));

    let receipt_only_denial = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(
        replay_subject.original_ledger.receipt(),
    )
    .receipt_only_preview()
    .pass_closeout()
    .expect_err("receipt-only preview cannot pass public closeout");
    assert_eq!(
        receipt_only_denial.kind(),
        SpatialGeometryEvidenceTouchDenialKind::DiagnosticOnly
    );

    assert_eq!(
        deny_raw_row_as_spatial_query_lowering_authority("split-receipt-id").kind(),
        SpatialEvidenceQueryLoweringDenialKind::RawRowSubstitution
    );
    assert_eq!(
        deny_raw_row_as_spatial_query_lowering_authority("WorkloadEvidenceRow").kind(),
        SpatialEvidenceQueryLoweringDenialKind::RawRowSubstitution
    );
    assert_eq!(
        deny_query_descriptor_digest_as_spatial_evidence_lookup_authority(
            "forge-query-descriptor-digest"
        )
        .kind(),
        SpatialEvidenceLookupDenialKind::QueryDescriptorDigestSubstitution
    );
    assert_eq!(
        deny_topology_touched_basis_as_spatial_query_lowering_authority(
            "TopologyTouchedGraphBasis"
        )
        .kind(),
        SpatialEvidenceQueryLoweringDenialKind::TopologyTouchedBasisSubstitution
    );
    assert!(matches!(
        deny_topology_touched_graph_basis_as_spatial_touch_authority(
            PhantomData::<TopologyTouchedGraphBasis>
        ),
        SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence {
            surface: SpatialEvidenceTopologySubstitutionSurface::TopologyTouchedGraphBasis
        }
    ));
}

fn assert_split_handoff_dependency_direction_uses_spatial_facade_proof() {
    let source =
        include_str!("../../../../workload_composition/worth_workload/boolean_split_handoff.rs");
    assert!(source.contains(".admit_spatial_geometry_evidence_touch(&self.split_ledger_receipt)"));
    assert!(source.contains(".spatial_evidence_lookup(self.completed_workload.evidence_ledger())"));
    assert!(source.contains("&spatial_touch_authority"));
    assert!(source.contains("&spatial_lookup"));
    assert!(
        !source.contains("evidence_ledger().stage_index(),"),
        "migrated split consumer must not pass a raw stage index into downstream consumption"
    );
}

fn assert_displaced_stage_index_consumer_path_is_deleted_residue() {
    let row = spatial_evidence_surface_deletion_ledger()
        .into_iter()
        .find(|row| {
            row.surface_name()
                == "PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(stage_index)"
        })
        .expect("phase 6 displaced stage-index consumer row must be recorded");
    assert_eq!(
        row.deletion_action(),
        SpatialEvidenceSurfaceDeletionAction::Deleted
    );
    assert!(!row.production_reachable());
    assert!(row.replacement_exists());
}
