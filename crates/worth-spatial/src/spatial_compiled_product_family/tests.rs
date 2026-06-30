use super::test_support::{real_retained_cancellation_receipt, retained_and_projected_receipts};
use super::{
    admit_evidence_lookup_spatial_compiled_product_family_input,
    admit_retained_cancellation_spatial_compiled_product_family_input,
    admit_retained_replay_spatial_compiled_product_family_input,
    catalog::catalog_from_declarations, current_spatial_compiled_product_family_catalog,
    select_spatial_compiled_product_family, SpatialCompiledProductConsumer,
    SpatialCompiledProductFamilyDeclarationBuilder, SpatialCompiledProductFamilyErrorKind,
    SpatialCompiledProductFamilyIdentity,
};
use crate::workload_platform::evidence_ledger::{
    receipt_backed_touch_authority_for_admission_tests, BooleanEvidenceStageKind,
    SelectedLookupSliceLedgerAssembly, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_family_catalog::{
    current_evidence_lookup_family_catalog, EvidenceLookupQueryImportEvidence,
    EvidenceLookupStageReceiptFamilyIdentity,
};
use crate::workload_platform::evidence_lookup_index_product::admit_evidence_lookup_index_product;
use crate::workload_platform::evidence_lookup_input_admission::{
    admit_evidence_lookup_input, real_projection_consumption_receipt,
    EvidenceLookupInputAdmissionRequest, EvidenceLookupQueryAdmissionEvidenceSet,
    EvidenceLookupStageReceiptAdmission,
};
use crate::workload_platform::evidence_lookup_plan_selection::{
    select_evidence_lookup_plan, EvidenceLookupSelectedPlan,
};

#[test]
fn spatial_family_declaration_applies_to_multiple_matching_products() {
    let catalog = current_spatial_compiled_product_family_catalog();
    let (selected_plan, product) = real_evidence_lookup_basis();
    let evidence_lookup = select_spatial_compiled_product_family(
        &catalog,
        admit_evidence_lookup_spatial_compiled_product_family_input(
            &catalog,
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &product,
        )
        .expect("evidence lookup admitted input"),
    )
    .expect("evidence lookup family selection");
    let public_closeout = select_spatial_compiled_product_family(
        &catalog,
        admit_evidence_lookup_spatial_compiled_product_family_input(
            &catalog,
            SpatialCompiledProductConsumer::EvidenceLookupPublicCloseout,
            &selected_plan,
            &product,
        )
        .expect("public closeout admitted input"),
    )
    .expect("public closeout family selection");
    let evidence_lookup_identity = evidence_lookup
        .compile_product_identity()
        .expect("evidence lookup lowered identity");
    let public_closeout_identity = public_closeout
        .compile_product_identity()
        .expect("public closeout lowered identity");

    assert_eq!(catalog.counters().family_count(), 3);
    assert_eq!(catalog.counters().declared_family_count(), 3);
    assert_eq!(catalog.counters().supported_consumer_count(), 4);
    assert_eq!(
        evidence_lookup.declaration().identity(),
        public_closeout.declaration().identity()
    );
    assert_eq!(
        evidence_lookup
            .declaration()
            .source_authority_digest_basis(),
        public_closeout
            .declaration()
            .source_authority_digest_basis()
    );
    assert_eq!(
        evidence_lookup.declaration().locality_footprint_basis(),
        public_closeout.declaration().locality_footprint_basis()
    );
    assert_eq!(
        evidence_lookup.declaration().prior_proof_role(),
        public_closeout.declaration().prior_proof_role()
    );
    assert_eq!(
        evidence_lookup.declaration().evidence_support_role(),
        public_closeout.declaration().evidence_support_role()
    );
    assert_eq!(
        evidence_lookup.declaration().equivalence_policy(),
        public_closeout.declaration().equivalence_policy()
    );
    assert_eq!(
        evidence_lookup_identity.family_digest(),
        public_closeout_identity.family_digest()
    );
    assert_eq!(
        evidence_lookup_identity
            .compiled_product_identity()
            .identity_digest(),
        public_closeout_identity
            .compiled_product_identity()
            .identity_digest()
    );
    assert_eq!(
        evidence_lookup_identity
            .equivalence_policy_identity()
            .identity_digest(),
        public_closeout_identity
            .equivalence_policy_identity()
            .identity_digest()
    );

    let (retained, projected) =
        retained_and_projected_receipts("phase-four.spatial-family.retained-replay");
    let retained_replay = select_spatial_compiled_product_family(
        &catalog,
        admit_retained_replay_spatial_compiled_product_family_input(
            &catalog,
            &retained
                .historical_replay(&retained.replay_subject())
                .expect("historical replay"),
            &retained,
            &projected,
        )
        .expect("retained replay admitted input"),
    )
    .expect("retained replay family selection");
    assert_eq!(
        retained_replay.declaration().identity(),
        SpatialCompiledProductFamilyIdentity::RetainedReplayDerivedSupport
    );
    assert_eq!(
        evidence_lookup_identity
            .prior_proof_identity()
            .expect("evidence lookup prior proof")
            .role(),
        schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductPriorProofRole::ProductShapingBasis
    );

    let retained_cancellation_receipt =
        real_retained_cancellation_receipt("phase-four-retained-cancellation");
    let retained_cancellation_identity = select_spatial_compiled_product_family(
        &catalog,
        admit_retained_cancellation_spatial_compiled_product_family_input(
            &catalog,
            &retained_cancellation_receipt,
        )
        .expect("retained cancellation admitted input"),
    )
    .expect("retained cancellation family selection")
    .compile_product_identity()
    .expect("retained cancellation lowered identity");
    assert_eq!(
        retained_cancellation_identity
            .prior_proof_identity()
            .expect("retained cancellation prior proof")
            .role(),
        schema::facade::platform::authority::compiled_product_semantic_graph::CompiledProductPriorProofRole::EquivalenceDimension
    );
}

#[test]
fn spatial_family_missing_identity_fields_cannot_enter_catalog() {
    let missing_locality = SpatialCompiledProductFamilyDeclarationBuilder::default()
        .identity(SpatialCompiledProductFamilyIdentity::EvidenceLookupDerivedSupport)
        .supported_consumers(vec![SpatialCompiledProductConsumer::EvidenceLookupIndexProduct])
        .source_authority_digest_basis(
            super::posture::SpatialSourceAuthorityDigestBasisPosture::EvidenceLookupLedgerBasisWithStageReceiptCoordinate,
        )
        .prior_proof_role(super::posture::SpatialPriorProofRolePosture::SelectedPlanTopologyAndQuerySupportBasis)
        .evidence_support_role(super::posture::SpatialEvidenceSupportRolePosture::QueryAndTopologySupportEvidence)
        .equivalence_policy(super::posture::SpatialEquivalencePolicyPosture::EvidenceLookupIndexSemanticParity)
        .equivalence_policy_name("broken")
        .equivalence_dimensions(&["compiled-product-identity"])
        .build()
        .expect_err("missing locality basis must fail");
    assert_eq!(
        missing_locality.kind(),
        super::error::SpatialCompiledProductFamilyErrorKind::MissingLocalityBasis
    );

    let missing_prior_proof = SpatialCompiledProductFamilyDeclarationBuilder::default()
        .identity(SpatialCompiledProductFamilyIdentity::RetainedReplayDerivedSupport)
        .supported_consumers(vec![SpatialCompiledProductConsumer::RetainedReplayParity])
        .source_authority_digest_basis(
            super::posture::SpatialSourceAuthorityDigestBasisPosture::RetainedPlanarHistoricalInspectionDigest,
        )
        .locality_footprint_basis(
            super::posture::SpatialLocalityFootprintBasisPosture::ProjectionConsumptionDigest,
        )
        .evidence_support_role(super::posture::SpatialEvidenceSupportRolePosture::RetainedReplayProjectionEvidence)
        .equivalence_policy(super::posture::SpatialEquivalencePolicyPosture::RetainedReplaySemanticParity)
        .equivalence_policy_name("broken")
        .equivalence_dimensions(&["compiled-product-identity"])
        .build()
        .expect_err("missing prior proof role must fail");
    assert_eq!(
        missing_prior_proof.kind(),
        super::error::SpatialCompiledProductFamilyErrorKind::MissingPriorProofRole
    );

    let missing_equivalence = SpatialCompiledProductFamilyDeclarationBuilder::default()
        .identity(SpatialCompiledProductFamilyIdentity::RetainedCancellationDerivedSupport)
        .supported_consumers(vec![SpatialCompiledProductConsumer::RetainedCancellationChain])
        .source_authority_digest_basis(
            super::posture::SpatialSourceAuthorityDigestBasisPosture::RetainedCancellationChainAuthorityDigest,
        )
        .locality_footprint_basis(
            super::posture::SpatialLocalityFootprintBasisPosture::ProjectionConsumptionDigest,
        )
        .prior_proof_role(super::posture::SpatialPriorProofRolePosture::RetainedCancellationCheckpointHistoryBasis)
        .evidence_support_role(super::posture::SpatialEvidenceSupportRolePosture::RetainedCancellationProjectionEvidence)
        .build()
        .expect_err("missing equivalence posture must fail");
    assert_eq!(
        missing_equivalence.kind(),
        super::error::SpatialCompiledProductFamilyErrorKind::MissingEquivalencePolicy
    );
}

#[test]
fn spatial_identity_changes_with_real_evidence_lookup_authority_change() {
    let catalog = current_spatial_compiled_product_family_catalog();
    let (selected_plan, product) = real_evidence_lookup_basis();
    let (foreign_selected_plan, foreign_product) = real_evidence_lookup_basis_with_world(
        "phase-four-spatial-family-foreign-projection-consumption-receipt",
    );
    let baseline = select_spatial_compiled_product_family(
        &catalog,
        admit_evidence_lookup_spatial_compiled_product_family_input(
            &catalog,
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &selected_plan,
            &product,
        )
        .expect("baseline admitted input"),
    )
    .expect("baseline family selection")
    .compile_product_identity()
    .expect("baseline lowering");

    let changed = select_spatial_compiled_product_family(
        &catalog,
        admit_evidence_lookup_spatial_compiled_product_family_input(
            &catalog,
            SpatialCompiledProductConsumer::EvidenceLookupIndexProduct,
            &foreign_selected_plan,
            &foreign_product,
        )
        .expect("changed admitted input"),
    )
    .expect("changed family selection")
    .compile_product_identity()
    .expect("changed lowering");

    assert_ne!(
        baseline.compiled_product_identity().identity_digest(),
        changed.compiled_product_identity().identity_digest()
    );
}

#[test]
fn spatial_catalog_rejects_ambiguous_consumer_coverage() {
    let duplicate = catalog_from_declarations(vec![
        SpatialCompiledProductFamilyDeclarationBuilder::default()
            .identity(SpatialCompiledProductFamilyIdentity::EvidenceLookupDerivedSupport)
            .supported_consumers(vec![SpatialCompiledProductConsumer::EvidenceLookupIndexProduct])
            .source_authority_digest_basis(
                super::posture::SpatialSourceAuthorityDigestBasisPosture::EvidenceLookupLedgerBasisWithStageReceiptCoordinate,
            )
            .locality_footprint_basis(
                super::posture::SpatialLocalityFootprintBasisPosture::SpatialTouchDigest,
            )
            .prior_proof_role(
                super::posture::SpatialPriorProofRolePosture::SelectedPlanTopologyAndQuerySupportBasis,
            )
            .evidence_support_role(
                super::posture::SpatialEvidenceSupportRolePosture::QueryAndTopologySupportEvidence,
            )
            .equivalence_policy(
                super::posture::SpatialEquivalencePolicyPosture::EvidenceLookupIndexSemanticParity,
            )
            .equivalence_policy_name("duplicate-left")
            .equivalence_dimensions(&["compiled-product-identity"])
            .build()
            .expect("left declaration"),
        SpatialCompiledProductFamilyDeclarationBuilder::default()
            .identity(SpatialCompiledProductFamilyIdentity::RetainedCancellationDerivedSupport)
            .supported_consumers(vec![SpatialCompiledProductConsumer::EvidenceLookupIndexProduct])
            .source_authority_digest_basis(
                super::posture::SpatialSourceAuthorityDigestBasisPosture::RetainedCancellationChainAuthorityDigest,
            )
            .locality_footprint_basis(
                super::posture::SpatialLocalityFootprintBasisPosture::ProjectionConsumptionDigest,
            )
            .prior_proof_role(
                super::posture::SpatialPriorProofRolePosture::RetainedCancellationCheckpointHistoryBasis,
            )
            .evidence_support_role(
                super::posture::SpatialEvidenceSupportRolePosture::RetainedCancellationProjectionEvidence,
            )
            .equivalence_policy(
                super::posture::SpatialEquivalencePolicyPosture::RetainedCancellationSemanticParity,
            )
            .equivalence_policy_name("duplicate-right")
            .equivalence_dimensions(&["compiled-product-identity"])
            .build()
            .expect("right declaration"),
    ])
    .expect_err("duplicate consumer coverage must deny");
    assert_eq!(
        duplicate.kind(),
        SpatialCompiledProductFamilyErrorKind::DuplicateConsumerCoverage
    );
}

fn real_evidence_lookup_basis() -> (
    EvidenceLookupSelectedPlan,
    crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct,
) {
    real_evidence_lookup_basis_with_world(
        "phase-four-spatial-family-projection-consumption-receipt",
    )
}

fn real_evidence_lookup_basis_with_world(
    world: &'static str,
) -> (
    EvidenceLookupSelectedPlan,
    crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct,
) {
    let catalog = current_evidence_lookup_family_catalog().expect("evidence lookup family catalog");
    let authority = receipt_backed_touch_authority_for_admission_tests(
        BooleanEvidenceStageKind::OperandAProjectionConsumption,
        world,
    );
    let request = EvidenceLookupInputAdmissionRequest::from_spatial_touch_authority(&authority)
        .with_stage_receipt_identity(
            EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
                &authority,
                EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
            ),
        )
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
    let selected_plan =
        select_evidence_lookup_plan(&catalog, &admitted).expect("lookup plan selects");
    let ledger = SelectedLookupSliceLedgerAssembly::from_touch_authority(
        &authority,
        &EvidenceLookupStageReceiptAdmission::from_spatial_touch_authority(
            &authority,
            EvidenceLookupStageReceiptFamilyIdentity::boolean_operand_projection_consumption(),
        ),
    )
    .assemble_selected_lookup_slice()
    .expect("selected lookup ledger");
    let product =
        admit_evidence_lookup_index_product(&selected_plan, &ledger).expect("index product admits");
    (selected_plan, product)
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
