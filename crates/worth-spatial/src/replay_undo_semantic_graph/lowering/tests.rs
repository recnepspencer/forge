use crate::replay_family_catalog::{
    admit_spatial_replay_family_identity, current_spatial_replay_family_catalog,
    SpatialReplayFamilyIdentityAuthority, SpatialReplayFamilyWorkloadDependencyPosture,
};
use crate::replay_undo_semantic_graph::{
    admit_prepared_spatial_replay_semantic_graph_input, admit_spatial_replay_semantic_graph_input,
    admit_spatial_undo_semantic_graph_input, lower_spatial_replay_equivalence_basis,
    lower_spatial_replay_equivalence_basis_from_admitted_input,
    lower_spatial_replay_equivalence_basis_from_scope_product, lower_spatial_replay_scope_identity,
    lower_spatial_replay_scope_identity_from_admitted_input,
    lower_spatial_replay_scope_identity_from_scope_product,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_admitted_input,
    prepare_spatial_replay_semantic_graph_request, select_spatial_replay_plan,
    SpatialReplaySemanticGraphAdmissionError, SpatialReplaySemanticGraphPreparationRequest,
    SpatialUndoSemanticGraphAdmissionRequest,
};
use crate::undo_family_catalog::SpatialUndoFamilyIdentityAuthority;
use crate::workload_platform::evidence_ledger::{
    SelectedLookupSliceLedgerAssembly, SpatialGeometryEvidenceTouchAuthority,
    WorkloadEvidenceStage, WorkloadEvidenceStageIndexProduct,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_stage_cutover::current_path::admit_current_family_stage_cutover_path;
use crate::workload_platform::evidence_lookup_stage_cutover::current_retained_replay_receipt_for_stage;
use crate::workload_platform::evidence_lookup_workload_cutover::EvidenceLookupConsumedWorkloadHandoff;
use crate::workload_platform::vocabulary::RetainedReplayWorkloadReceipt;

#[test]
fn spatial_replay_selected_plan_tracks_dependency_posture_and_bound_receipts() {
    let boolean_fixture = boolean_event_ledger_fixture();
    let boolean_admitted = admit_boolean_event_ledger_input(&boolean_fixture);
    let boolean_plan = select_spatial_replay_plan(&boolean_admitted).expect("boolean plan");
    let expected_retained_replay_identity = boolean_fixture
        .matching_retained_replay_receipt
        .identity()
        .receipt_identity();

    assert_eq!(
        boolean_plan.workload_dependency_posture(),
        SpatialReplayFamilyWorkloadDependencyPosture::RequiresLookupConsumedWorkloadAndRetainedReplay
    );
    assert_eq!(
        boolean_plan.admitted_input_semantic_graph_identity(),
        boolean_admitted.semantic_graph_identity()
    );
    assert_eq!(
        boolean_plan.retained_replay_receipt_identity(),
        Some(expected_retained_replay_identity.as_str())
    );
    assert!(!boolean_plan
        .lookup_consumed_workload_handoff_identity()
        .is_empty());

    let projection_fixture = projection_receipt_fixture();
    let projection_admitted = admit_projection_receipt_input(&projection_fixture);
    let projection_plan = select_spatial_replay_plan(&projection_admitted).expect("projection");

    assert_eq!(
        projection_plan.workload_dependency_posture(),
        SpatialReplayFamilyWorkloadDependencyPosture::LookupReceiptOnly
    );
    assert_eq!(projection_plan.retained_replay_receipt_identity(), None);
}

#[test]
fn spatial_replay_scope_product_preserves_lookup_and_retained_bindings() {
    let fixture = boolean_event_ledger_fixture();
    let admitted = admit_boolean_event_ledger_input(&fixture);
    let scope_product =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("scope product");

    assert_eq!(
        scope_product
            .lookup_consumed_workload_handoff()
            .stage_receipt_identity(),
        fixture.workload_handoff.stage_receipt_identity()
    );
    assert_eq!(
        scope_product.lookup_consumed_workload_handoff_identity(),
        select_spatial_replay_plan(&admitted)
            .expect("plan")
            .lookup_consumed_workload_handoff_identity()
    );
    assert_eq!(
        scope_product
            .retained_replay_receipt()
            .expect("retained replay receipt")
            .identity()
            .receipt_identity(),
        fixture
            .matching_retained_replay_receipt
            .identity()
            .receipt_identity()
    );
    assert_eq!(
        scope_product.stage_index_identity(),
        admitted.stage_index_identity()
    );
}

#[test]
fn spatial_replay_scope_product_routes_scope_identity_and_equivalence_basis() {
    let fixture = boolean_event_ledger_fixture();
    let admitted = admit_boolean_event_ledger_input(&fixture);
    let scope_product =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("scope product");

    assert_eq!(
        lower_spatial_replay_scope_identity_from_scope_product(&scope_product),
        lower_spatial_replay_scope_identity_from_admitted_input(&admitted).expect("scope identity")
    );
    assert_eq!(
        lower_spatial_replay_equivalence_basis_from_scope_product(&scope_product),
        lower_spatial_replay_equivalence_basis_from_admitted_input(&admitted)
            .expect("equivalence basis")
    );
}

#[test]
fn spatial_replay_scope_product_keeps_firewall_counters_zero() {
    let fixture = boolean_event_ledger_fixture();
    let admitted = admit_boolean_event_ledger_input(&fixture);
    let scope_product =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("scope product");
    let counters = scope_product.counters();

    assert_eq!(counters.raw_row_scan_count(), 0);
    assert_eq!(counters.broad_receipt_scan_count(), 0);
    assert_eq!(counters.caller_owned_scan_count(), 0);
    assert_eq!(counters.retained_replay_binding_count(), 1);
    assert_eq!(
        counters.touched_subject_count(),
        scope_product.equivalence_basis().touched_subjects().len()
    );
    assert!(counters.covered_family_count() > 0);
}

#[test]
fn spatial_replay_scope_product_is_stable_for_same_admitted_input() {
    let fixture = projection_receipt_fixture();
    let admitted = admit_projection_receipt_input(&fixture);

    let first =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("first scope");
    let second =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("second scope");

    assert_eq!(
        first.scope_product_identity().digest(),
        second.scope_product_identity().digest()
    );
    assert_eq!(
        first.scope_identity().digest(),
        second.scope_identity().digest()
    );
    assert_eq!(first.equivalence_basis(), second.equivalence_basis());
}

#[test]
fn spatial_replay_plan_and_scope_product_drift_when_handoff_coverage_drifts() {
    let fixture = projection_receipt_fixture();
    let admitted = admit_projection_receipt_input(&fixture);
    let mut expanded_coverage = fixture
        .workload_handoff
        .covered_family_identities()
        .to_vec();
    expanded_coverage.push("phase-12.synthetic.extra-covered-family".to_string());
    let drifted_handoff = fixture
        .workload_handoff
        .clone()
        .with_test_covered_family_identities(expanded_coverage);
    let drifted_admitted = admit_projection_receipt_input_with_handoff(&fixture, &drifted_handoff);

    let original_plan = select_spatial_replay_plan(&admitted).expect("original plan");
    let drifted_plan = select_spatial_replay_plan(&drifted_admitted).expect("drifted plan");
    let original_scope =
        lower_spatial_replay_scope_product_from_admitted_input(&admitted).expect("original scope");
    let drifted_scope = lower_spatial_replay_scope_product_from_admitted_input(&drifted_admitted)
        .expect("drifted scope");

    assert_ne!(
        original_plan.lookup_consumed_workload_handoff_identity(),
        drifted_plan.lookup_consumed_workload_handoff_identity()
    );
    assert_ne!(
        original_plan.selected_plan_identity(),
        drifted_plan.selected_plan_identity()
    );
    assert_ne!(
        original_scope.scope_product_identity().digest(),
        drifted_scope.scope_product_identity().digest()
    );
}

#[test]
fn spatial_replay_admission_rejects_retained_replay_drift_before_scope_lowering() {
    let fixture = boolean_event_ledger_fixture();
    let foreign_retained_replay_receipt = current_retained_replay_receipt_for_stage(
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
    );

    let error = admit_spatial_replay_semantic_graph_input(
        &current_spatial_replay_family_catalog(),
        SpatialReplaySemanticGraphPreparationRequest::new(
            admit_spatial_replay_family_identity(
                SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
            ),
            &fixture.authority,
            &fixture.execution_receipt,
            &fixture.workload_handoff,
        )
        .with_retained_replay_receipt(&foreign_retained_replay_receipt),
    )
    .expect_err("retained replay drift must fail before scope lowering");

    assert!(matches!(
        error,
        SpatialReplaySemanticGraphAdmissionError::RetainedReplayReceiptMismatch { .. }
    ));
}

#[test]
fn legacy_spatial_replay_lowering_stays_equivalent_for_raw_identity_surface() {
    let fixture = boolean_event_ledger_fixture();
    let stage_index_product = event_ledger_stage_index_product(&fixture.authority);

    assert_eq!(
        lower_spatial_replay_scope_identity(
            &fixture.authority,
            &fixture.execution_receipt,
            &stage_index_product
        )
        .expect("legacy scope")
        .digest(),
        lower_spatial_replay_scope_identity(
            &fixture.authority,
            &fixture.execution_receipt,
            &stage_index_product
        )
        .expect("legacy scope repeat")
        .digest()
    );
    assert_eq!(
        lower_spatial_replay_equivalence_basis(
            &fixture.authority,
            &fixture.execution_receipt,
            &stage_index_product
        )
        .expect("legacy basis"),
        lower_spatial_replay_equivalence_basis(
            &fixture.authority,
            &fixture.execution_receipt,
            &stage_index_product
        )
        .expect("legacy basis repeat")
    );
}

#[test]
fn spatial_undo_scope_product_preserves_lookup_binding_when_required() {
    let fixture = boolean_event_ledger_fixture();
    let stage_index_product = event_ledger_stage_index_product(&fixture.authority);
    let admitted = admit_spatial_undo_semantic_graph_input(
        SpatialUndoSemanticGraphAdmissionRequest::new(
            SpatialUndoFamilyIdentityAuthority::boolean_event_ledger().identity(),
            &fixture.authority,
            &fixture.execution_receipt,
            &stage_index_product,
        )
        .with_lookup_consumed_workload_handoff(&fixture.workload_handoff),
    )
    .expect("undo input should admit");
    let scope_product =
        lower_spatial_undo_scope_product_from_admitted_input(&admitted).expect("scope product");

    assert_eq!(scope_product.family_identity(), admitted.family_identity());
    assert_eq!(
        scope_product
            .lookup_consumed_workload_handoff()
            .expect("lookup handoff")
            .semantic_graph_identity(),
        fixture.workload_handoff.semantic_graph_identity()
    );
    assert_eq!(
        scope_product.stage_index_identity().digest(),
        admitted.stage_index_identity().digest()
    );
}

struct ReplayLoweringFixture {
    authority: SpatialGeometryEvidenceTouchAuthority,
    execution_receipt: EvidenceLookupExecutionReceipt,
    workload_handoff: EvidenceLookupConsumedWorkloadHandoff,
    matching_retained_replay_receipt: RetainedReplayWorkloadReceipt,
}

fn boolean_event_ledger_fixture() -> ReplayLoweringFixture {
    let (authority, execution_receipt, workload_handoff) = current_cutover_replay_components(
        "spatial-touch.boolean.event-ledger-evidence.v1",
        WorkloadEvidenceStage::BooleanEventLedger,
    );
    ReplayLoweringFixture {
        authority,
        execution_receipt,
        workload_handoff,
        matching_retained_replay_receipt: current_retained_replay_receipt_for_stage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ),
    }
}

fn projection_receipt_fixture() -> ReplayLoweringFixture {
    let (authority, execution_receipt, workload_handoff) = current_cutover_replay_components(
        "spatial-touch.boolean.projection-consumption-evidence.v1",
        WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
    );
    ReplayLoweringFixture {
        authority,
        execution_receipt,
        workload_handoff,
        matching_retained_replay_receipt: current_retained_replay_receipt_for_stage(
            WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
        ),
    }
}

fn admit_boolean_event_ledger_input<'a>(
    fixture: &'a ReplayLoweringFixture,
) -> crate::replay_undo_semantic_graph::SpatialReplaySemanticGraphAdmittedInput<'a> {
    let catalog = current_spatial_replay_family_catalog();
    let request = prepare_spatial_replay_semantic_graph_request(
        crate::replay_undo_semantic_graph::SpatialReplaySemanticGraphPreparationRequest::new(
            admit_spatial_replay_family_identity(
                SpatialReplayFamilyIdentityAuthority::boolean_event_ledger(),
            ),
            &fixture.authority,
            &fixture.execution_receipt,
            &fixture.workload_handoff,
        )
        .with_retained_replay_receipt(&fixture.matching_retained_replay_receipt),
    )
    .expect("prepared request");

    admit_prepared_spatial_replay_semantic_graph_input(&catalog, &request).expect("admitted input")
}

fn admit_projection_receipt_input<'a>(
    fixture: &'a ReplayLoweringFixture,
) -> crate::replay_undo_semantic_graph::SpatialReplaySemanticGraphAdmittedInput<'a> {
    admit_projection_receipt_input_with_handoff(fixture, &fixture.workload_handoff)
}

fn admit_projection_receipt_input_with_handoff<'a>(
    fixture: &'a ReplayLoweringFixture,
    workload_handoff: &'a EvidenceLookupConsumedWorkloadHandoff,
) -> crate::replay_undo_semantic_graph::SpatialReplaySemanticGraphAdmittedInput<'a> {
    let catalog = current_spatial_replay_family_catalog();
    let request = prepare_spatial_replay_semantic_graph_request(
        crate::replay_undo_semantic_graph::SpatialReplaySemanticGraphPreparationRequest::new(
            admit_spatial_replay_family_identity(
                SpatialReplayFamilyIdentityAuthority::projection_receipt(),
            ),
            &fixture.authority,
            &fixture.execution_receipt,
            workload_handoff,
        ),
    )
    .expect("prepared request");

    admit_prepared_spatial_replay_semantic_graph_input(&catalog, &request).expect("admitted input")
}

fn current_cutover_replay_components(
    family_identity: &str,
    stage: WorkloadEvidenceStage,
) -> (
    SpatialGeometryEvidenceTouchAuthority,
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

fn event_ledger_stage_index_product(
    authority: &SpatialGeometryEvidenceTouchAuthority,
) -> WorkloadEvidenceStageIndexProduct {
    SelectedLookupSliceLedgerAssembly::from_touch_authority(
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
