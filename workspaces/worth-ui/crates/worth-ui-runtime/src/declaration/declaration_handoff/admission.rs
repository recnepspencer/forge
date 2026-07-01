use crate::declaration::{
    UiAspectContract, UiDeclarationFamily, UiDeclarationGraphHandoff, UiDeclarationIdentity,
    UiDeclarationStructuralSemantics, UiDeclaredAspectPayload, UiDeclaredPostureContract,
    UiDeclaredPosturePayload, UiStructuralDeclarationPayload,
};

pub(crate) fn derive_declaration_graph_handoff(
    identity: &UiDeclarationIdentity,
    aspect_contract: &UiAspectContract,
    family: &UiDeclarationFamily,
    structural_semantics: &UiDeclarationStructuralSemantics,
    declared_posture: &UiDeclaredPostureContract,
) -> UiDeclarationGraphHandoff {
    UiDeclarationGraphHandoff::new(
        identity.clone(),
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
