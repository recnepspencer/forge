use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationFamilyKind, UiDeclarationGraphHandoff,
    UiDeclarationGraphHandoffDenial,
};

pub(crate) fn lower_graph_handoffs(
    declaration_artifacts: &[UiDeclarationArtifact],
) -> Result<Vec<UiDeclarationGraphHandoff>, UiDeclarationGraphHandoffDenial> {
    let mut handoffs = Vec::with_capacity(declaration_artifacts.len());
    for artifact in declaration_artifacts {
        let nonstructural = artifact.family().is_ok_and(|family| {
            matches!(
                family.kind(),
                UiDeclarationFamilyKind::Intent | UiDeclarationFamilyKind::QueryBinding
            )
        });
        if !nonstructural {
            handoffs.push(artifact.graph_handoff()?);
        }
    }
    Ok(handoffs)
}
