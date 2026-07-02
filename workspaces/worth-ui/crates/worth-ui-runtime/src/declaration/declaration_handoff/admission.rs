use crate::declaration::{
    stable_text_digest, UiAspectContract, UiDeclarationFamily, UiDeclarationGraphHandoff,
    UiDeclarationIdentity, UiDeclarationProvenance, UiDeclarationStructuralSemantics,
    UiDeclaredAspectPayload, UiDeclaredPostureContract, UiDeclaredPosturePayload,
    UiStructuralDeclarationPayload,
};

pub(crate) fn derive_declaration_graph_handoff(
    identity: &UiDeclarationIdentity,
    provenance: &UiDeclarationProvenance,
    aspect_contract: &UiAspectContract,
    family: &UiDeclarationFamily,
    structural_semantics: &UiDeclarationStructuralSemantics,
    declared_posture: &UiDeclaredPostureContract,
) -> UiDeclarationGraphHandoff {
    UiDeclarationGraphHandoff::new(
        identity.clone(),
        stable_text_digest(provenance.source_provenance().module_path())
            ^ (provenance.source_provenance().declaration_index() as u64).rotate_left(13),
        UiStructuralDeclarationPayload::new(
            family.clone(),
            structural_semantics.role(),
            structural_semantics.containment_intent().clone(),
            structural_semantics.slot_participation_intent().clone(),
            structural_semantics.ordering_guarantee(),
            structural_semantics.repetition_posture(),
        ),
        UiDeclaredAspectPayload::new(aspect_contract.clone()),
        UiDeclaredPosturePayload::new(declared_posture.clone()),
    )
}
