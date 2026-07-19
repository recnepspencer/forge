use crate::orchestration_inventory::{
    WorthQueryOrchestrationAspectPosture, WorthQueryOrchestrationBindingProjection,
    WorthQueryOrchestrationContributionCompatibility, WorthQueryOrchestrationProofContract,
    WorthQueryOrchestrationStrategyAttachment,
    WorthQueryOrchestrationSurfaceCertificationReference,
    WorthQueryOrchestrationSurfaceDocReference, WorthQueryOrchestrationSurfaceInventory,
    WorthQueryOrchestrationSurfaceRow,
};

pub(super) fn current_row(public_name: &str) -> WorthQueryOrchestrationSurfaceRow {
    WorthQueryOrchestrationSurfaceInventory::current()
        .row_for_public_name(public_name)
        .unwrap_or_else(|| panic!("expected inventory row {public_name}"))
        .clone()
}

pub(super) fn inventory_without_public_name(
    public_name: &str,
) -> WorthQueryOrchestrationSurfaceInventory {
    WorthQueryOrchestrationSurfaceInventory::new(
        WorthQueryOrchestrationSurfaceInventory::current()
            .rows()
            .iter()
            .filter(|row| row.public_name() != public_name)
            .cloned()
            .collect(),
    )
}

pub(super) fn inventory_with_replaced_row(
    replacement: WorthQueryOrchestrationSurfaceRow,
) -> WorthQueryOrchestrationSurfaceInventory {
    WorthQueryOrchestrationSurfaceInventory::new(
        WorthQueryOrchestrationSurfaceInventory::current()
            .rows()
            .iter()
            .map(|row| {
                if row.public_name() == replacement.public_name() {
                    replacement.clone()
                } else {
                    row.clone()
                }
            })
            .collect(),
    )
}

pub(super) fn row_with_binding_projection(
    row: &WorthQueryOrchestrationSurfaceRow,
    binding_projection: WorthQueryOrchestrationBindingProjection,
) -> WorthQueryOrchestrationSurfaceRow {
    rebuild_row(
        row,
        binding_projection,
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        row.semantic_profile().clone(),
    )
}

pub(super) fn row_with_aspect_posture(
    row: &WorthQueryOrchestrationSurfaceRow,
    aspect_posture: WorthQueryOrchestrationAspectPosture,
) -> WorthQueryOrchestrationSurfaceRow {
    let semantics = row.semantic_profile();
    rebuild_row(
        row,
        row.binding_projection(),
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        crate::orchestration_inventory::WorthQueryOrchestrationSemanticProfile::new(
            aspect_posture,
            semantics.basis_posture(),
            semantics.policy_tenant_posture(),
            semantics.lower_authority_attachment(),
            semantics.strategy_attachment(),
            semantics.contribution_compatibility().clone(),
            semantics.collaborative_extension_posture(),
        ),
    )
}

pub(super) fn row_with_strategy_attachment(
    row: &WorthQueryOrchestrationSurfaceRow,
    strategy_attachment: WorthQueryOrchestrationStrategyAttachment,
) -> WorthQueryOrchestrationSurfaceRow {
    let semantics = row.semantic_profile();
    rebuild_row(
        row,
        row.binding_projection(),
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        crate::orchestration_inventory::WorthQueryOrchestrationSemanticProfile::new(
            semantics.aspect_posture(),
            semantics.basis_posture(),
            semantics.policy_tenant_posture(),
            semantics.lower_authority_attachment(),
            strategy_attachment,
            semantics.contribution_compatibility().clone(),
            semantics.collaborative_extension_posture(),
        ),
    )
}

pub(super) fn row_with_contribution_compatibility(
    row: &WorthQueryOrchestrationSurfaceRow,
    contribution_compatibility: WorthQueryOrchestrationContributionCompatibility,
) -> WorthQueryOrchestrationSurfaceRow {
    let semantics = row.semantic_profile();
    rebuild_row(
        row,
        row.binding_projection(),
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        crate::orchestration_inventory::WorthQueryOrchestrationSemanticProfile::new(
            semantics.aspect_posture(),
            semantics.basis_posture(),
            semantics.policy_tenant_posture(),
            semantics.lower_authority_attachment(),
            semantics.strategy_attachment(),
            contribution_compatibility,
            semantics.collaborative_extension_posture(),
        ),
    )
}

fn rebuild_row(
    row: &WorthQueryOrchestrationSurfaceRow,
    binding_projection: WorthQueryOrchestrationBindingProjection,
    proof_contract: &WorthQueryOrchestrationProofContract,
    doc_reference: WorthQueryOrchestrationSurfaceDocReference,
    certification_reference: WorthQueryOrchestrationSurfaceCertificationReference,
    semantic_profile: crate::orchestration_inventory::WorthQueryOrchestrationSemanticProfile,
) -> WorthQueryOrchestrationSurfaceRow {
    WorthQueryOrchestrationSurfaceRow::new(
        row.public_name(),
        row.canonical_base_name(),
        row.family(),
        row.visibility(),
        row.ordinary_outcome_supported(),
        binding_projection,
        WorthQueryOrchestrationProofContract::new(
            proof_contract.checked_type_name(),
            proof_contract.proof_type_name(),
            proof_contract.transcript_family(),
            proof_contract.checked_topology_kind(),
            proof_contract.support_surface(),
        ),
        semantic_profile,
        doc_reference,
        certification_reference,
    )
}
