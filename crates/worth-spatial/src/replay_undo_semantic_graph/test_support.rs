use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, SpatialReplayFamilyIdentity,
    SpatialReplayFamilyIdentityAuthority,
};
use crate::workload_platform::evidence_ledger::{
    SelectedLookupSliceLedgerAssembly, SpatialGeometryEvidenceTouchAuthority,
    WorkloadEvidenceStage, WorkloadEvidenceStageIndexProduct,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupFamilyDeclaration,
    EvidenceLookupFamilyIdentity, EvidenceLookupStageReceiptFamilyIdentity, TestCatalogCloseout,
};
use crate::workload_platform::evidence_lookup_input_admission::EvidenceLookupStageReceiptAdmission;
use crate::workload_platform::evidence_lookup_stage_cutover::{
    admit_current_family_stage_cutover_path,
    admit_current_family_stage_cutover_path_with_query_evidence,
    current_retained_replay_receipt_for_stage,
};
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use crate::workload_platform::evidence_lookup_input_admission::EvidenceLookupQueryAdmissionEvidenceSet;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[derive(Clone)]
pub struct ReplayUndoSpatialBoundaryFixture {
    replay_family_identity: SpatialReplayFamilyIdentity,
    authority: SpatialGeometryEvidenceTouchAuthority,
    execution_receipt: EvidenceLookupExecutionReceipt,
    workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
    retained_replay_receipt: Option<RetainedReplayWorkloadReceipt>,
    stage_index_product: WorkloadEvidenceStageIndexProduct,
}

impl ReplayUndoSpatialBoundaryFixture {
    pub fn replay_family_identity(&self) -> SpatialReplayFamilyIdentity {
        self.replay_family_identity
    }

    pub fn authority(&self) -> &SpatialGeometryEvidenceTouchAuthority {
        &self.authority
    }

    pub fn execution_receipt(&self) -> &EvidenceLookupExecutionReceipt {
        &self.execution_receipt
    }

    pub fn workload_handoff(&self) -> &EvidenceLookupConsumedWorkloadHandoff {
        &self.workload_handoff
    }

    pub fn workload_handoff_with_test_stage_receipt_identity(
        &self,
        stage_receipt_identity: impl Into<String>,
    ) -> EvidenceLookupConsumedWorkloadHandoff {
        self.workload_handoff
            .clone()
            .with_test_stage_receipt_identity(stage_receipt_identity)
    }

    pub fn workload_handoff_with_test_raw_row_scan_count(
        &self,
        raw_row_scan_count: usize,
    ) -> EvidenceLookupConsumedWorkloadHandoff {
        self.workload_handoff
            .clone()
            .with_test_raw_row_scan_count(raw_row_scan_count)
    }

    pub fn workload_handoff_with_test_broad_receipt_scan_count(
        &self,
        broad_receipt_scan_count: usize,
    ) -> EvidenceLookupConsumedWorkloadHandoff {
        self.workload_handoff
            .clone()
            .with_test_broad_receipt_scan_count(broad_receipt_scan_count)
    }

    pub fn workload_handoff_with_test_caller_owned_scan_count(
        &self,
        caller_owned_scan_count: usize,
    ) -> EvidenceLookupConsumedWorkloadHandoff {
        self.workload_handoff
            .clone()
            .with_test_caller_owned_scan_count(caller_owned_scan_count)
    }

    pub fn retained_replay_receipt(&self) -> Option<&RetainedReplayWorkloadReceipt> {
        self.retained_replay_receipt.as_ref()
    }

    pub fn stage_index_product(&self) -> &WorkloadEvidenceStageIndexProduct {
        &self.stage_index_product
    }
}

pub fn boolean_event_ledger_spatial_boundary_fixture() -> ReplayUndoSpatialBoundaryFixture {
    replay_undo_spatial_boundary_fixture(
        SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        "spatial-touch.boolean.event-ledger-evidence.v1",
        WorkloadEvidenceStage::BooleanEventLedger,
        Some(current_retained_replay_receipt_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
        )),
    )
}

pub fn projection_receipt_spatial_boundary_fixture() -> ReplayUndoSpatialBoundaryFixture {
    replay_undo_spatial_boundary_fixture(
        SpatialReplayFamilyIdentityAuthority::projection_receipt(),
        "spatial-touch.boolean.projection-consumption-evidence.v1",
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        None,
    )
}

pub fn boolean_event_ledger_query_required_sibling_spatial_boundary_fixture(
) -> ReplayUndoSpatialBoundaryFixture {
    let catalog = event_ledger_catalog_with_query_required_sibling();
    let family = catalog
        .family_by_identity("spatial-touch.boolean.event-ledger-query-required-sibling.v1")
        .expect("covered sibling family declaration");
    let projection_family = catalog
        .family_by_identity("spatial-touch.boolean.projection-consumption-evidence.v1")
        .expect("projection family declaration");
    let query_evidence = EvidenceLookupQueryAdmissionEvidenceSet::from_query_import_evidence(
        projection_family
            .query_posture()
            .imported_evidence()
            .expect("projection family requires query evidence"),
    );
    let path = admit_current_family_stage_cutover_path_with_query_evidence(
        &catalog,
        family,
        WorkloadEvidenceStage::BooleanEventLedger,
        Some(&query_evidence),
        Some(&crate::workload_platform::evidence_lookup_input_admission::current_projection_consumption_receipt()),
    )
    .expect("current cutover path with sibling query support");
    replay_undo_fixture_from_path(
        SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
        family.identity().as_str(),
        path,
        Some(current_retained_replay_receipt_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
        )),
    )
}

fn replay_undo_spatial_boundary_fixture(
    replay_family_identity_authority: SpatialReplayFamilyIdentityAuthority,
    family_identity: &str,
    stage: WorkloadEvidenceStage,
    retained_replay_receipt: Option<RetainedReplayWorkloadReceipt>,
) -> ReplayUndoSpatialBoundaryFixture {
    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    let family = catalog
        .family_by_identity(family_identity)
        .expect("covered family declaration");
    let path =
        admit_current_family_stage_cutover_path(&catalog, family, stage).expect("current cutover path");
    replay_undo_fixture_from_path(
        replay_family_identity_authority,
        family.identity().as_str(),
        path,
        retained_replay_receipt,
    )
}

fn replay_undo_fixture_from_path(
    replay_family_identity_authority: SpatialReplayFamilyIdentityAuthority,
    family_identity: &str,
    path: crate::workload_platform::evidence_lookup_stage_cutover::current_path::EvidenceLookupCurrentCoveredStageCutoverPath,
    retained_replay_receipt: Option<RetainedReplayWorkloadReceipt>,
) -> ReplayUndoSpatialBoundaryFixture {
    let proof = path.prove_for_family(family_identity).expect("covered family proof");
    let authority = path.spatial_touch_authority().clone();
    let execution_receipt = path.execution_receipt().clone();
    let workload_handoff =
        EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(&proof).expect("handoff");
    let stage_index_product = SelectedLookupSliceLedgerAssembly::from_touch_authority(
        &authority,
        &EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            &authority,
            EvidenceLookupStageReceiptFamilyIdentity::boolean_event_ledger(),
        ),
    )
    .assemble()
    .expect("assembled lookup ledger closes")
    .stage_index()
    .clone();

    ReplayUndoSpatialBoundaryFixture {
        replay_family_identity: admit_spatial_replay_family_identity(
            replay_family_identity_authority,
        ),
        authority,
        execution_receipt,
        workload_handoff,
        retained_replay_receipt,
        stage_index_product,
    }
}

fn event_ledger_catalog_with_query_required_sibling() -> TestCatalogCloseout {
    let catalog = current_evidence_lookup_family_catalog().expect("catalog closes");
    let event_family = catalog
        .family_by_identity("spatial-touch.boolean.event-ledger-evidence.v1")
        .expect("event family exists")
        .clone();
    let projection_family = catalog
        .family_by_identity("spatial-touch.boolean.projection-consumption-evidence.v1")
        .expect("projection family exists")
        .clone();
    let query_required_event_sibling =
        copy_family_with_identity_and_query(&event_family, &projection_family);
    TestCatalogCloseout::from_declarations(vec![
        event_family,
        projection_family,
        query_required_event_sibling,
    ])
    .expect("custom sibling catalog closes")
}

fn copy_family_with_identity_and_query(
    event_family: &EvidenceLookupFamilyDeclaration,
    query_family: &EvidenceLookupFamilyDeclaration,
) -> EvidenceLookupFamilyDeclaration {
    EvidenceLookupFamilyDeclaration::builder()
        .identity(EvidenceLookupFamilyIdentity::declared(
            "spatial-touch.boolean.event-ledger-query-required-sibling.v1",
        ))
        .spatial_touch_authority(event_family.spatial_touch_authority())
        .topology_input_posture(event_family.topology_input_posture().clone())
        .stage_applicability(event_family.stage_applicability().clone())
        .evidence_classes(event_family.evidence_classes().clone())
        .lookup_product_posture(event_family.lookup_product_posture())
        .index_posture(query_family.index_posture().clone())
        .query_posture(query_family.query_posture().clone())
        .diagnostic_witness(event_family.diagnostic_witness().clone())
        .source_inventory_pressure(event_family.source_inventory_pressure().clone())
        .build()
        .expect("custom event sibling declaration builds")
}
