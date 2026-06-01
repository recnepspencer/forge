use crate::orchestration_inventory::{
    ForgeQueryOrchestrationAspectPosture, ForgeQueryOrchestrationBindingProjection,
    ForgeQueryOrchestrationContributionCompatibility, ForgeQueryOrchestrationProofContract,
    ForgeQueryOrchestrationStrategyAttachment,
    ForgeQueryOrchestrationSurfaceCertificationReference,
    ForgeQueryOrchestrationSurfaceDocReference, ForgeQueryOrchestrationSurfaceInventory,
    ForgeQueryOrchestrationSurfaceRow,
};

pub(super) fn current_row(public_name: &str) -> ForgeQueryOrchestrationSurfaceRow {
    ForgeQueryOrchestrationSurfaceInventory::current()
        .row_for_public_name(public_name)
        .unwrap_or_else(|| panic!("expected inventory row {public_name}"))
        .clone()
}

pub(super) fn inventory_without_public_name(
    public_name: &str,
) -> ForgeQueryOrchestrationSurfaceInventory {
    ForgeQueryOrchestrationSurfaceInventory::new(
        ForgeQueryOrchestrationSurfaceInventory::current()
            .rows()
            .iter()
            .filter(|row| row.public_name() != public_name)
            .cloned()
            .collect(),
    )
}

pub(super) fn inventory_with_replaced_row(
    replacement: ForgeQueryOrchestrationSurfaceRow,
) -> ForgeQueryOrchestrationSurfaceInventory {
    ForgeQueryOrchestrationSurfaceInventory::new(
        ForgeQueryOrchestrationSurfaceInventory::current()
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
    row: &ForgeQueryOrchestrationSurfaceRow,
    binding_projection: ForgeQueryOrchestrationBindingProjection,
) -> ForgeQueryOrchestrationSurfaceRow {
    rebuild_row(
        row,
        binding_projection,
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        row.semantic_profile().clone(),
    )
}

pub(super) fn row_with_doc_reference(
    row: &ForgeQueryOrchestrationSurfaceRow,
    path: &'static str,
    section: &'static str,
) -> ForgeQueryOrchestrationSurfaceRow {
    rebuild_row(
        row,
        row.binding_projection(),
        row.proof_contract(),
        ForgeQueryOrchestrationSurfaceDocReference::new(path, section),
        row.certification_reference(),
        row.semantic_profile().clone(),
    )
}

pub(super) fn row_with_aspect_posture(
    row: &ForgeQueryOrchestrationSurfaceRow,
    aspect_posture: ForgeQueryOrchestrationAspectPosture,
) -> ForgeQueryOrchestrationSurfaceRow {
    let semantics = row.semantic_profile();
    rebuild_row(
        row,
        row.binding_projection(),
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        crate::orchestration_inventory::ForgeQueryOrchestrationSemanticProfile::new(
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
    row: &ForgeQueryOrchestrationSurfaceRow,
    strategy_attachment: ForgeQueryOrchestrationStrategyAttachment,
) -> ForgeQueryOrchestrationSurfaceRow {
    let semantics = row.semantic_profile();
    rebuild_row(
        row,
        row.binding_projection(),
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        crate::orchestration_inventory::ForgeQueryOrchestrationSemanticProfile::new(
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
    row: &ForgeQueryOrchestrationSurfaceRow,
    contribution_compatibility: ForgeQueryOrchestrationContributionCompatibility,
) -> ForgeQueryOrchestrationSurfaceRow {
    let semantics = row.semantic_profile();
    rebuild_row(
        row,
        row.binding_projection(),
        row.proof_contract(),
        row.doc_reference(),
        row.certification_reference(),
        crate::orchestration_inventory::ForgeQueryOrchestrationSemanticProfile::new(
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
    row: &ForgeQueryOrchestrationSurfaceRow,
    binding_projection: ForgeQueryOrchestrationBindingProjection,
    proof_contract: &ForgeQueryOrchestrationProofContract,
    doc_reference: ForgeQueryOrchestrationSurfaceDocReference,
    certification_reference: ForgeQueryOrchestrationSurfaceCertificationReference,
    semantic_profile: crate::orchestration_inventory::ForgeQueryOrchestrationSemanticProfile,
) -> ForgeQueryOrchestrationSurfaceRow {
    ForgeQueryOrchestrationSurfaceRow::new(
        row.public_name(),
        row.canonical_base_name(),
        row.family(),
        row.visibility(),
        row.ordinary_outcome_supported(),
        binding_projection,
        ForgeQueryOrchestrationProofContract::new(
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
