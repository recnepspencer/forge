use super::worth_ui_structural_admission::admit_structure;
use super::worth_ui_structural_legality_context::WorthUiStructuralLegalityContext;
use crate::source::{
    WorthUiLegallyStructuredArtifactInputBindingNode,
    WorthUiLegallyStructuredArtifactInputComponentNode,
    WorthUiLegallyStructuredArtifactInputSurfaceNode, WorthUiResolvedArtifactInputBindingNode,
    WorthUiResolvedArtifactInputComponentNode, WorthUiResolvedArtifactInputSurfaceNode,
    WorthUiStructuralLegalityDiagnostic,
};

pub(super) fn lower_component_node(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    component_node: &WorthUiResolvedArtifactInputComponentNode,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<
    WorthUiLegallyStructuredArtifactInputComponentNode,
    Vec<WorthUiStructuralLegalityDiagnostic>,
> {
    let structure = admit_structure(
        module_id,
        component_node.structure(),
        component_node.provenance(),
        context,
    )?;
    Ok(WorthUiLegallyStructuredArtifactInputComponentNode::new(
        component_node.component().clone(),
        component_node.descriptor().clone(),
        component_node.authored_identity().map(str::to_owned),
        structure,
        component_node.provenance().clone(),
    ))
}

pub(super) fn lower_surface_node(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    surface_node: &WorthUiResolvedArtifactInputSurfaceNode,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<
    WorthUiLegallyStructuredArtifactInputSurfaceNode,
    Vec<WorthUiStructuralLegalityDiagnostic>,
> {
    let structure = admit_structure(
        module_id,
        surface_node.structure(),
        surface_node.provenance(),
        context,
    )?;
    Ok(WorthUiLegallyStructuredArtifactInputSurfaceNode::new(
        surface_node.surface().clone(),
        surface_node.descriptor().clone(),
        surface_node.authored_identity().map(str::to_owned),
        structure,
        surface_node.provenance().clone(),
    ))
}

pub(super) fn lower_binding_node(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    binding_node: &WorthUiResolvedArtifactInputBindingNode,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<
    WorthUiLegallyStructuredArtifactInputBindingNode,
    Vec<WorthUiStructuralLegalityDiagnostic>,
> {
    let structure = admit_structure(
        module_id,
        binding_node.structure(),
        binding_node.provenance(),
        context,
    )?;
    Ok(WorthUiLegallyStructuredArtifactInputBindingNode::new(
        binding_node.view_binding().clone(),
        binding_node.entry().clone(),
        binding_node.authored_identity().map(str::to_owned),
        structure,
        binding_node.provenance().clone(),
    ))
}
