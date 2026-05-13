use crate::basis_lifecycle::BasisFamily;
use crate::effect_lifecycle::{
    effect_lifecycle_family_inventory, effect_lifecycle_family_row_for_key,
    effect_lifecycle_public_surface_inventory, effect_lifecycle_support_matrix,
    effect_lifecycle_support_row_matches_inventory, effect_lifecycle_supported_basis_families,
    EffectAuthorityOwner, EffectDeferredNeighborFamily, EffectFamily, EffectLifecycleFamilyKey,
    EffectLoweredArtifactKind, EffectPublicSurfaceAvailability, EffectPublicSurfaceKind,
    EffectReceiptArtifactKind, EffectSupportCause, EffectSupportPosture,
};

#[test]
fn family_inventory_names_authority_basis_lowering_and_receipt_contracts() {
    let inventory = effect_lifecycle_family_inventory();

    let mutation = inventory
        .rows()
        .iter()
        .find(|row| row.family_key() == EffectLifecycleFamilyKey::Mutation)
        .expect("mutation family row should exist");
    assert_eq!(
        mutation.authority_owner(),
        EffectAuthorityOwner::ForgeRelational
    );
    assert_eq!(
        mutation.lowered_artifact_kind(),
        EffectLoweredArtifactKind::LoweredMutationIntentDeclaration
    );
    assert_eq!(
        mutation.receipt_artifact_kind(),
        EffectReceiptArtifactKind::ForgeQueryIntentExecution
    );
    assert_eq!(
        mutation.admitted_basis_families(),
        &[
            BasisFamily::CurrentHead,
            BasisFamily::BranchHead,
            BasisFamily::TenantScoped,
            BasisFamily::PolicyScoped,
        ]
    );
}

#[test]
fn support_rows_match_the_family_inventory_contract() {
    let matrix = effect_lifecycle_support_matrix();

    assert!(matrix
        .rows()
        .iter()
        .all(effect_lifecycle_support_row_matches_inventory));
}

#[test]
fn batch_family_inventory_is_first_class_and_not_hidden_as_scalar_mutation() {
    let batch = effect_lifecycle_family_row_for_key(EffectLifecycleFamilyKey::OrderedBatch)
        .expect("ordered batch family row should exist");

    assert_eq!(
        batch.authority_owner(),
        EffectAuthorityOwner::ForgeRelational
    );
    assert_eq!(
        batch.lowered_artifact_kind(),
        EffectLoweredArtifactKind::LoweredEffectBatchExecutionPlan
    );
    assert_eq!(
        batch.receipt_artifact_kind(),
        EffectReceiptArtifactKind::ForgeQueryBatchWriteReceipt
    );
    assert_eq!(
        batch.admitted_basis_families(),
        &[
            BasisFamily::CurrentHead,
            BasisFamily::BranchHead,
            BasisFamily::TenantScoped,
            BasisFamily::PolicyScoped,
        ]
    );
}

#[test]
fn supported_basis_family_lookup_matches_inventory_rows() {
    let families = effect_lifecycle_supported_basis_families(EffectFamily::Writeback);

    assert_eq!(
        families,
        vec![
            BasisFamily::CurrentHead,
            BasisFamily::BranchHead,
            BasisFamily::TenantScoped,
            BasisFamily::PolicyScoped,
        ]
    );
}

#[test]
fn public_surface_inventory_names_batch_and_hidden_runtime_boundaries() {
    let inventory = effect_lifecycle_public_surface_inventory();

    let batch = inventory
        .rows()
        .iter()
        .find(|row| row.surface_kind() == EffectPublicSurfaceKind::BatchExecution)
        .expect("batch surface row should exist");
    assert_eq!(
        batch.primary_artifact_kind(),
        Some(EffectReceiptArtifactKind::ForgeQueryBatchWriteReceipt)
    );
    assert!(batch
        .entrypoint()
        .expect("batch execution surface should have a concrete entrypoint")
        .contains("effect_batch()"));
    assert_eq!(
        batch.availability(),
        EffectPublicSurfaceAvailability::Implemented
    );
    assert!(batch.lower_runtime_visibility_hidden());

    let hidden = inventory
        .rows()
        .iter()
        .find(|row| row.surface_kind() == EffectPublicSurfaceKind::HiddenLowerRuntimeTypes)
        .expect("hidden lower-runtime row should exist");
    assert!(hidden.lower_runtime_visibility_hidden());
    assert_eq!(hidden.primary_artifact_kind(), None);

    let diagnostics = inventory
        .rows()
        .iter()
        .find(|row| row.surface_kind() == EffectPublicSurfaceKind::DiagnosticsEnvelope)
        .expect("diagnostics/envelope row should exist");
    assert_eq!(
        diagnostics.availability(),
        EffectPublicSurfaceAvailability::Implemented
    );
    assert!(diagnostics
        .entrypoint()
        .expect("diagnostics/envelope surface should have a concrete entrypoint")
        .contains("materialize_diagnostics"));
}

#[test]
fn support_matrix_retains_deferred_and_rebind_postures_after_enrichment() {
    let matrix = effect_lifecycle_support_matrix();

    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::PreviewDerived
            && row.effect_family() == EffectFamily::Mutation
            && row.posture() == EffectSupportPosture::Advisory
            && row.cause() == EffectSupportCause::AdvisoryOnlyExecution
            && row.authority_owner() == EffectAuthorityOwner::ForgeRelational
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::Preview
            && row.effect_family() == EffectFamily::Mutation
            && row.posture() == EffectSupportPosture::RebindRequired
            && row.cause() == EffectSupportCause::PreviewRebindRequired
            && row.authority_owner() == EffectAuthorityOwner::ForgeRelational
    }));
    assert!(matrix.rows().iter().any(|row| {
        row.basis_family() == BasisFamily::StoreBacked
            && row.effect_family() == EffectFamily::Writeback
            && row.posture() == EffectSupportPosture::Deferred
            && row.cause() == EffectSupportCause::StoreBackedExecutionDeferred
            && row.receipt_artifact_kind() == EffectReceiptArtifactKind::ForgeQueryWriteReceipt
    }));
}

#[test]
fn support_rows_expose_denial_rebind_and_deferred_neighbor_contracts() {
    let matrix = effect_lifecycle_support_matrix();

    let preview_mutation = matrix
        .rows()
        .iter()
        .find(|row| {
            row.basis_family() == BasisFamily::Preview
                && row.effect_family() == EffectFamily::Mutation
        })
        .expect("preview mutation support row should exist");
    assert!(preview_mutation.requires_rebind());
    assert_eq!(
        preview_mutation.denial_kinds(),
        &[crate::effect_lifecycle::DeniedEffectEligibilityKind::PreviewRebindRequired]
    );

    let current_writeback = matrix
        .rows()
        .iter()
        .find(|row| {
            row.basis_family() == BasisFamily::CurrentHead
                && row.effect_family() == EffectFamily::Writeback
        })
        .expect("current-head writeback support row should exist");
    assert_eq!(
        current_writeback.deferred_neighbors(),
        &[
            EffectDeferredNeighborFamily::StoreBackedExecutionParity,
            EffectDeferredNeighborFamily::DurableReplayAndRestartStableEnvelope,
        ]
    );
}
