use crate::spatial_compiled_product_family::{
    current_spatial_compiled_product_family_catalog, select_spatial_compiled_product_family,
    SpatialCompiledProductConsumer,
};
use crate::workload_platform::compiled_product_admission::{
    admit_spatial_compiled_product_input, SpatialCompiledProductAdmissionRequest,
};
use crate::workload_platform::evidence_ledger::{
    receipt_backed_touch_authority_for_admission_tests, BooleanEvidenceRowAuthority,
    BooleanEvidenceStageKind, SelectedLookupSliceLedger, SelectedLookupSliceLedgerAssembly,
    WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupQueryImportEvidence,
    EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, real_projection_consumption_receipt,
    EvidenceLookupInputAdmissionRequest, EvidenceLookupQueryAdmissionEvidenceSet,
    EvidenceLookupStageReceiptAdmission,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    select_evidence_lookup_plan, EvidenceLookupSelectedPlan,
};
use crate::workload_platform::spatial_compiled_product_consumer_cutover::build_retained_replay_parity_report;
use crate::workload_platform::spatial_compiled_product_consumer_cutover::lower_evidence_lookup_index_product;

#[test]
fn spatial_equivalent_inputs_admit_to_same_identity() {
    let catalog = current_spatial_compiled_product_family_catalog();
    let (selected_plan, _ledger, product) =
        real_evidence_lookup_product("phase-7-spatial-admit-stable");
    let left = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &product,
        ),
    )
    .expect("left admitted input");
    let right = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &product,
        ),
    )
    .expect("right admitted input");
    let left_identity =
        select_spatial_compiled_product_family(&catalog, left.family_admitted_input())
            .expect("left family selection")
            .compile_product_identity()
            .expect("left lowering");
    let right_identity =
        select_spatial_compiled_product_family(&catalog, right.family_admitted_input())
            .expect("right family selection")
            .compile_product_identity()
            .expect("right lowering");

    assert_eq!(
        left_identity.compiled_product_identity().identity_digest(),
        right_identity.compiled_product_identity().identity_digest()
    );
}

#[test]
fn spatial_admission_witness_depends_on_boundary_request_shape_not_only_family_input() {
    let catalog = current_spatial_compiled_product_family_catalog();
    let (selected_plan, ledger, product) =
        real_evidence_lookup_product("phase-7-spatial-admit-request-shaped-witness");
    let ledger_admitted = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_ledger(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &ledger,
        ),
    )
    .expect("ledger admission");
    let product_admitted = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &product,
        ),
    )
    .expect("product admission");

    assert_eq!(
        ledger_admitted.family_admitted_input(),
        product_admitted.family_admitted_input(),
        "ledger and product admission should lower to the same family input for the same selected slice"
    );
    assert_eq!(
        ledger_admitted.witness().family_identity(),
        product_admitted.witness().family_identity()
    );
    assert_ne!(
        ledger_admitted.witness().admission_token(),
        product_admitted.witness().admission_token(),
        "admission witness must depend on the real request boundary, not only the lowered family input"
    );
}

#[test]
fn spatial_wrong_receipt_or_manual_support_is_rejected() {
    let catalog = current_spatial_compiled_product_family_catalog();
    let (selected_plan, ledger, product) =
        real_evidence_lookup_product("phase-7-spatial-admit-baseline");
    let (_foreign_selected_plan, _foreign_ledger, foreign_product) =
        real_evidence_lookup_product("phase-7-spatial-admit-foreign");
    let broad_scan = real_complete_scope_basis("phase-7-spatial-admit-broad-scan");

    let wrong_authority = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &foreign_product,
        ),
    )
    .expect_err("foreign lookup product must deny");
    assert_eq!(
        wrong_authority.kind(),
        super::denial::SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis
    );

    let wrong_receipt = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &rebuild_lookup_product(
                &selected_plan,
                &ledger,
                &product,
                Some("phase-7-wrong-receipt-family"),
                None,
                None,
            ),
        ),
    )
    .expect_err("wrong receipt family must deny");
    assert_eq!(
        wrong_receipt.kind(),
        super::denial::SpatialCompiledProductAdmissionErrorKind::WrongReceiptFamily
    );

    let wrong_support = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_evidence_lookup_product(
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &rebuild_lookup_product(
                &selected_plan,
                &ledger,
                &product,
                None,
                Some("phase-7-wrong-query-support"),
                None,
            ),
        ),
    )
    .expect_err("wrong support posture must deny");
    assert_eq!(
        wrong_support.kind(),
        super::denial::SpatialCompiledProductAdmissionErrorKind::WrongSupportPosture
    );

    assert_eq!(
        broad_scan,
        crate::workload_platform::evidence_ledger::WorkloadEvidenceLedgerError::SelectedLookupSliceExceedsScope(
            WorkloadEvidenceStage::BooleanSharedPlaneIdentity
        )
    );

    let (retained, projected) = retained_and_projected_receipts("phase-7-spatial-retained");
    let (foreign_retained, _) = retained_and_projected_receipts("phase-7-spatial-retained-foreign");
    let foreign_historical = foreign_retained
        .historical_replay(&foreign_retained.replay_subject())
        .expect("foreign historical replay");
    let retained_mismatch = admit_spatial_compiled_product_input(
        &catalog,
        SpatialCompiledProductAdmissionRequest::for_retained_replay(
            &foreign_historical,
            &retained,
            &projected,
        ),
    )
    .expect_err("foreign historical basis must deny");
    assert_eq!(
        retained_mismatch.kind(),
        super::denial::SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis
    );

    let parity = build_retained_replay_parity_report(
        &retained,
        &retained
            .historical_replay(&retained.replay_subject())
            .expect("historical replay"),
        &projected,
    )
    .expect("retained replay parity report");
    assert_eq!(parity.row_count(), 1);
}

fn real_evidence_lookup_product(
    world: &'static str,
) -> (
    EvidenceLookupSelectedPlan,
    SelectedLookupSliceLedger,
    EvidenceLookupIndexProduct,
) {
    let catalog = current_evidence_lookup_family_catalog().expect("evidence lookup family catalog");
    let authority = receipt_backed_touch_authority_for_admission_tests(
        BooleanEvidenceStageKind::OperandAProjectionConsumption,
        world,
    );
    let stage_receipt = EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
        &authority,
        EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
    );
    let request = EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(&authority)
        .with_stage_receipt_identity(stage_receipt.clone())
        .with_query_import_evidence(
            EvidenceLookupQueryAdmissionEvidenceSet::from_projection_consumption_receipt(
                &real_projection_consumption_receipt(),
                query_import_fact_family(
                    &catalog,
                    WorkloadEvidenceStage::BooleanOperandAProjectionConsumption,
                ),
            ),
        );
    let admitted = admit_evidence_lookup_input(&catalog, request).expect("lookup input admits");
    let selected_plan = select_evidence_lookup_plan(&catalog, &admitted).expect("lookup plan");
    let ledger =
        SelectedLookupSliceLedgerAssembly::from_touch_authority(&authority, &stage_receipt)
            .assemble_selected_lookup_slice()
            .expect("selected lookup ledger");
    let product =
        lower_evidence_lookup_index_product(&selected_plan, &ledger).expect("index product");
    (selected_plan, ledger, product)
}

fn real_complete_scope_basis(
    world: &'static str,
) -> crate::workload_platform::evidence_ledger::WorkloadEvidenceLedgerError {
    let authority = receipt_backed_touch_authority_for_admission_tests(
        BooleanEvidenceStageKind::OperandAProjectionConsumption,
        world,
    );
    let unrelated = receipt_backed_touch_authority_for_admission_tests(
        BooleanEvidenceStageKind::SharedPlaneIdentity,
        "phase-7-spatial-admit-unrelated-shared-plane",
    );
    SelectedLookupSliceLedgerAssembly::from_touch_authority(
        &authority,
        &EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            &authority,
            EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
        ),
    )
    .with_additional_boolean_receipt(&UnrelatedBooleanReceipt::from_touch_authority(&unrelated))
    .assemble_selected_lookup_slice()
    .expect_err("complete-ledger scope must deny")
}

fn query_import_fact_family(
    catalog: &crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyCatalogCloseout,
    stage: WorkloadEvidenceStage,
) -> crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupProjectionFactFamily {
    match catalog
        .declarations()
        .iter()
        .find(|family| family.stage_applicability().stages().contains(&stage))
        .and_then(|family| family.query_posture().imported_evidence())
    {
        Some(EvidenceLookupQueryImportEvidence::ProjectionConsumptionReceipt {
            fact_family,
            ..
        }) => *fact_family,
        other => panic!("unexpected query import evidence for stage {stage:?}: {other:?}"),
    }
}

fn retained_and_projected_receipts(
    world: &'static str,
) -> (
    crate::facade::planar_retained_facts::RetainedPlanarFactsReceipt,
    crate::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt,
) {
    crate::spatial_compiled_product_family::retained_and_projected_receipts(world)
}

fn rebuild_lookup_product(
    selected_plan: &EvidenceLookupSelectedPlan,
    ledger: &SelectedLookupSliceLedger,
    product: &EvidenceLookupIndexProduct,
    stage_receipt_digest: Option<&str>,
    query_support_digest: Option<&str>,
    topology_support_digest: Option<&str>,
) -> EvidenceLookupIndexProduct {
    let admitted_family =
        crate::workload_platform::evidence_lookup_index_product::admit_and_lower_index_family_identity(
            selected_plan,
            ledger,
        )
        .expect("rebuild basis admits");
    EvidenceLookupIndexProduct::new(
        admitted_family.lowered_identity(),
        admitted_family.selected_equivalence_family(),
        product.selected_plan_digest().to_string(),
        product.spatial_touch_digest().to_string(),
        stage_receipt_digest
            .unwrap_or(product.stage_receipt_digest())
            .to_string(),
        admitted_family.evidence_ledger_basis_digest().to_string(),
        topology_support_digest
            .unwrap_or(admitted_family.topology_support_digest())
            .to_string(),
        query_support_digest
            .unwrap_or(admitted_family.query_support_digest())
            .to_string(),
        product
            .reuse_decision_identity_digest()
            .map(ToString::to_string),
        product.query_surface_contract_rows().to_vec(),
        product.lifecycle_posture(),
        product.disposal_posture(),
        *product.counters(),
        product.rows().to_vec(),
    )
}

struct UnrelatedBooleanReceipt {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_identity: String,
    support: crate::workload_platform::evidence_ledger::WorkloadEvidenceSupport,
    counters: crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters,
}

impl UnrelatedBooleanReceipt {
    fn from_touch_authority(
        authority: &crate::workload_platform::evidence_ledger::SpatialGeometryEvidenceTouchAuthority,
    ) -> Self {
        Self {
            boolean_stage: authority.boolean_stage(),
            evidence_identity: authority.evidence_identity().to_string(),
            support: authority.support(),
            counters: authority.evidence_counters(),
        }
    }
}

impl crate::workload_platform::evidence_ledger::BooleanEvidenceReceipt for UnrelatedBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    fn evidence_support(
        &self,
    ) -> crate::workload_platform::evidence_ledger::WorkloadEvidenceSupport {
        self.support
    }

    fn evidence_counters(
        &self,
    ) -> crate::workload_platform::evidence_ledger::WorkloadEvidenceStageCounters {
        self.counters
    }
}

impl crate::trusted_boolean_evidence_authority::Seal for UnrelatedBooleanReceipt {}

impl BooleanEvidenceRowAuthority for UnrelatedBooleanReceipt {}
