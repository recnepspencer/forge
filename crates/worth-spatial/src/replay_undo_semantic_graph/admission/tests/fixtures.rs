use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, current_spatial_replay_family_catalog,
    SpatialReplayFamilyIdentityAuthority,
};
use crate::replay_undo_semantic_graph::admission::SpatialReplaySemanticGraphPreparationRequest;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceStageIndexProduct;
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path;
use crate::workload_platform::evidence_lookup_stage_cutover::current_retained_replay_receipt_for_stage;
use crate::workload_platform::evidence_lookup_stage_cutover::EvidenceLookupCoveredStageCutoverProof;
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use crate::workload_platform::vocabulary::{
    GeometryBindingWorkload, ProjectionWorkload, RetainedReplayWorkload,
    RetainedReplayWorkloadReceipt, SurfaceSupportWorkload, TransformWorkload,
};
use topology::facade::TopologySeed;

pub(super) struct ReplayAdmissionFixture {
    pub family_catalog: crate::replay_family_catalog::SpatialReplayFamilyCatalog,
    pub authority: crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
    pub execution_receipt: EvidenceLookupExecutionReceipt,
    pub workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
    pub matching_retained_replay_receipt: RetainedReplayWorkloadReceipt,
    pub foreign_retained_replay_receipt: RetainedReplayWorkloadReceipt,
}

pub(super) fn boolean_event_ledger_fixture() -> ReplayAdmissionFixture {
    let (authority, execution_receipt, workload_handoff) = current_cutover_replay_components(
        "spatial-touch.boolean.event-ledger-evidence.v1",
        crate::workload_platform::evidence_ledger::WorkloadEvidenceStage::BooleanEventLedger,
    );
    let matching_retained_replay_receipt = current_retained_replay_receipt_for_stage(
        crate::workload_platform::evidence_ledger::WorkloadEvidenceStage::BooleanEventLedger,
    );
    let foreign_retained_replay_receipt =
        retained_replay_receipt("phase-12 spatial replay admission foreign retained replay");

    ReplayAdmissionFixture {
        family_catalog: current_spatial_replay_family_catalog(),
        authority,
        execution_receipt,
        workload_handoff,
        matching_retained_replay_receipt,
        foreign_retained_replay_receipt,
    }
}

pub(super) fn projection_receipt_fixture() -> ReplayAdmissionFixture {
    let (authority, execution_receipt, workload_handoff) = current_cutover_replay_components(
        "spatial-touch.boolean.projection-consumption-evidence.v1",
        crate::workload_platform::evidence_ledger::WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
    );
    let matching_retained_replay_receipt = current_retained_replay_receipt_for_stage(
        crate::workload_platform::evidence_ledger::WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
    );
    let foreign_retained_replay_receipt =
        retained_replay_receipt("phase-12 projection replay foreign retained replay");

    ReplayAdmissionFixture {
        family_catalog: current_spatial_replay_family_catalog(),
        authority,
        execution_receipt,
        workload_handoff,
        matching_retained_replay_receipt,
        foreign_retained_replay_receipt,
    }
}

pub(super) fn boolean_event_ledger_request<'a>(
    fixture: &'a ReplayAdmissionFixture,
) -> SpatialReplaySemanticGraphPreparationRequest<'a> {
    SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        ),
        &fixture.authority,
        &fixture.execution_receipt,
        &fixture.workload_handoff,
    )
    .with_retained_replay_receipt(&fixture.matching_retained_replay_receipt)
}

pub(super) fn projection_receipt_request<'a>(
    fixture: &'a ReplayAdmissionFixture,
) -> SpatialReplaySemanticGraphPreparationRequest<'a> {
    SpatialReplaySemanticGraphPreparationRequest::new(
        admit_spatial_replay_family_identity(
            SpatialReplayFamilyIdentityAuthority::projection_receipt(),
        ),
        &fixture.authority,
        &fixture.execution_receipt,
        &fixture.workload_handoff,
    )
}

pub(super) fn boolean_event_ledger_stage_proof() -> EvidenceLookupCoveredStageCutoverProof {
    current_cutover_replay_stage_proof(
        "spatial-touch.boolean.event-ledger-evidence.v1",
        crate::workload_platform::evidence_ledger::WorkloadEvidenceStage::BooleanEventLedger,
    )
}

pub(super) fn event_ledger_stage_index_product(
    authority: &crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
) -> WorkloadEvidenceStageIndexProduct {
    crate::workload_platform::evidence_ledger::SelectedLookupSliceLedgerAssembly::from_touch_authority(
        authority,
        &crate::workload_platform::evidence_lookup_input_admission::EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            authority,
            EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        ),
    )
    .assemble()
    .expect("assembled lookup ledger closes")
    .stage_index()
    .clone()
}

fn current_cutover_replay_components(
    family_identity: &str,
    stage: crate::workload_platform::evidence_ledger::WorkloadEvidenceStage,
) -> (
    crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
    EvidenceLookupExecutionReceipt,
    EvidenceLookupConsumedWorkloadHandoff,
) {
    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    let family = catalog
        .family_by_identity(family_identity)
        .expect("covered family declaration");
    let path = admit_current_family_stage_cutover_path(&catalog, family, stage)
        .expect("current cutover path");
    let proof = path
        .prove_for_family(family.identity().as_str())
        .expect("covered family proof");
    (
        path.spatial_touch_authority().clone(),
        path.execution_receipt().clone(),
        EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof).expect("handoff"),
    )
}

fn current_cutover_replay_stage_proof(
    family_identity: &str,
    stage: crate::workload_platform::evidence_ledger::WorkloadEvidenceStage,
) -> EvidenceLookupCoveredStageCutoverProof {
    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    let family = catalog
        .family_by_identity(family_identity)
        .expect("covered family declaration");
    let path = admit_current_family_stage_cutover_path(&catalog, family, stage)
        .expect("current cutover path");
    path.prove_for_family(family.identity().as_str())
        .expect("covered family proof")
}

fn retained_replay_receipt(label: &'static str) -> RetainedReplayWorkloadReceipt {
    let topology = TopologySeed::cube()
        .with_declaration(label)
        .build()
        .expect("topology seed should certify");
    let geometry = GeometryBindingWorkload::for_topology_receipt(
        topology.query_receipts().declaration_receipt(),
    )
    .declared(format!("{label} geometry binding"))
    .admit()
    .expect("geometry binding should admit");
    let support = SurfaceSupportWorkload::for_geometry_binding(&geometry)
        .declared(format!("{label} surface support"))
        .admit()
        .expect("surface support should admit");
    let projection = ProjectionWorkload::for_surface_support(&support)
        .declared(format!("{label} projection"))
        .admit()
        .expect("projection should admit");
    let transform = TransformWorkload::for_projection(&projection)
        .declared(format!("{label} transform"))
        .admit()
        .expect("transform should admit");
    RetainedReplayWorkload::for_transform(&transform)
        .declared(format!("{label} retained replay"))
        .admit()
        .expect("retained replay should admit")
}
