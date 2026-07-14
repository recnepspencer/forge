use crate::declaration::{
    authored_source_provenance_digest, UiAspectContract, UiDeclarationFamily,
    UiDeclarationGraphHandoff, UiDeclarationIdentity, UiDeclarationProvenance,
    UiDeclarationStructuralDigest, UiDeclarationStructuralSemantics, UiDeclaredAspectPayload,
    UiDeclaredPostureContract, UiDeclaredPosturePayload, UiStructuralDeclarationPayload,
};

pub(crate) fn derive_declaration_graph_handoff(
    identity: &UiDeclarationIdentity,
    provenance: &UiDeclarationProvenance,
    aspect_contract: &UiAspectContract,
    family: &UiDeclarationFamily,
    structural_digest: UiDeclarationStructuralDigest,
    structural_semantics: &UiDeclarationStructuralSemantics,
    declared_posture: &UiDeclaredPostureContract,
) -> UiDeclarationGraphHandoff {
    UiDeclarationGraphHandoff::new(
        identity.clone(),
        authored_source_provenance_digest(
            provenance.source_provenance().module_path(),
            provenance.source_provenance().declaration_index(),
        ),
        UiStructuralDeclarationPayload::new(
            family.clone(),
            structural_digest,
            structural_semantics.role(),
            structural_semantics.operator_kind(),
            structural_semantics.mosaic_sizing_contract_id().cloned(),
            structural_semantics.containment_intent().clone(),
            structural_semantics.slot_participation_intent().clone(),
            structural_semantics.ordering_guarantee(),
            structural_semantics.repetition_posture(),
        ),
        UiDeclaredAspectPayload::new(aspect_contract.clone()),
        UiDeclaredPosturePayload::new(declared_posture.clone()),
    )
}
