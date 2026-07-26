use crate::declaration::{
    UiDeclarationArtifact, UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial,
};

pub(crate) fn lower_graph_handoffs(
    declaration_artifacts: &[UiDeclarationArtifact],
) -> Result<Vec<UiDeclarationGraphHandoff>, UiDeclarationGraphHandoffDenial> {
    declaration_artifacts
        .iter()
        .map(UiDeclarationArtifact::graph_handoff)
        .collect()
}
