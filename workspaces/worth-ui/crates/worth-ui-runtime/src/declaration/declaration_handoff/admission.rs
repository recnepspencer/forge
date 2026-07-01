use crate::declaration::{
    UiDeclarationFamily, UiDeclarationGraphHandoff, UiDeclarationIdentity,
    UiDeclarationStructuralSemantics, UiDeclaredPostureContract, UiDeclaredPosturePayload,
    UiStructuralDeclarationPayload,
};

pub(crate) fn derive_declaration_graph_handoff(
    identity: &UiDeclarationIdentity,
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
        UiDeclaredPosturePayload::new(declared_posture.clone()),
    )
}
