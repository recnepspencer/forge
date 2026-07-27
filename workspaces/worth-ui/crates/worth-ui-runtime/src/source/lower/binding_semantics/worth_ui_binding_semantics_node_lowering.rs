use crate::source::{
    WorthUiBindingDiagnostic, WorthUiBindingDiagnosticCode, WorthUiBoundArtifactInputBindingNode,
    WorthUiBoundArtifactInputComponentNode, WorthUiBoundArtifactInputSurfaceNode,
    WorthUiBoundArtifactInputThemeTokenNode, WorthUiBoundCommandReference,
    WorthUiBoundCommandSemantics, WorthUiBoundIconReference, WorthUiBoundSurfaceSemantics,
    WorthUiBoundThemeTokenSemantics, WorthUiBoundViewBindingReference,
    WorthUiLegallyStructuredArtifactInputBindingNode,
    WorthUiLegallyStructuredArtifactInputComponentNode,
    WorthUiLegallyStructuredArtifactInputSurfaceNode,
    WorthUiLegallyStructuredArtifactInputThemeTokenNode,
};

use super::worth_ui_binding_semantics_context::WorthUiBindingSemanticsContext;
use super::worth_ui_query_binding_semantics::bind_query_view_semantics;

pub(super) fn lower_component_node(
    component_node: &WorthUiLegallyStructuredArtifactInputComponentNode,
) -> WorthUiBoundArtifactInputComponentNode {
    WorthUiBoundArtifactInputComponentNode::new(
        component_node.component().clone(),
        component_node.descriptor().clone(),
        component_node.authored_identity().map(str::to_owned),
        component_node.structure().clone(),
        component_node.provenance().clone(),
    )
}

pub(super) fn lower_surface_node(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    surface_node: &WorthUiLegallyStructuredArtifactInputSurfaceNode,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundArtifactInputSurfaceNode, Vec<WorthUiBindingDiagnostic>> {
    let mut diagnostics = Vec::new();
    let surface_icon =
        surface_node.descriptor().icon().and_then(|icon_id| {
            match bind_surface_icon(module_id, surface_node, icon_id, context) {
                Ok(icon) => Some(icon),
                Err(diagnostic) => {
                    diagnostics.push(diagnostic);
                    None
                }
            }
        });
    let mut command_slots = Vec::new();
    for (index, command_id) in surface_node.descriptor().command_slots().iter().enumerate() {
        let locus = format!(
            "surface:{}/command_slot[{index}]",
            surface_node.surface().id().as_str()
        );
        match context.resolve_command(
            module_id,
            command_id.as_str(),
            &locus,
            surface_node.provenance(),
        ) {
            Ok((command, descriptor)) => {
                match bind_command_semantics(module_id, surface_node, index, &descriptor, context) {
                    Ok(semantics) => {
                        command_slots.push(WorthUiBoundCommandReference::new(
                            command, descriptor, semantics,
                        ));
                    }
                    Err(mut command_diagnostics) => diagnostics.append(&mut command_diagnostics),
                }
            }
            Err(diagnostic) => diagnostics.push(diagnostic),
        }
    }

    let view_binding = surface_node
        .descriptor()
        .view_binding()
        .map(|view_binding_id| {
            let locus = format!(
                "surface:{}/view_binding",
                surface_node.surface().id().as_str()
            );
            let (view_binding, entry) = context.resolve_view_binding(
                module_id,
                view_binding_id.as_str(),
                &locus,
                surface_node.provenance(),
            )?;
            let query_semantics = bind_query_view_semantics(
                module_id,
                &entry,
                &locus,
                surface_node.provenance(),
                context,
            )?;
            Ok(WorthUiBoundViewBindingReference::new(
                view_binding,
                entry,
                query_semantics,
            ))
        })
        .transpose();
    let view_binding = match view_binding {
        Ok(view_binding) => view_binding,
        Err(diagnostic) => {
            diagnostics.push(diagnostic);
            None
        }
    };

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    Ok(WorthUiBoundArtifactInputSurfaceNode::new(
        surface_node.surface().clone(),
        surface_node.descriptor().clone(),
        surface_node.authored_identity().map(str::to_owned),
        surface_node.structure().clone(),
        WorthUiBoundSurfaceSemantics::new(surface_icon, command_slots, view_binding),
        surface_node.provenance().clone(),
    ))
}

pub(super) fn lower_binding_node(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    binding_node: &WorthUiLegallyStructuredArtifactInputBindingNode,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundArtifactInputBindingNode, Vec<WorthUiBindingDiagnostic>> {
    let semantic_locus = format!("binding:{}", binding_node.view_binding().id().as_str());
    let query_semantics = match bind_query_view_semantics(
        module_id,
        binding_node.entry(),
        &semantic_locus,
        binding_node.provenance(),
        context,
    ) {
        Ok(query_semantics) => query_semantics,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };
    Ok(WorthUiBoundArtifactInputBindingNode::new(
        WorthUiBoundViewBindingReference::new(
            binding_node.view_binding().clone(),
            binding_node.entry().clone(),
            query_semantics,
        ),
        binding_node.authored_identity().map(str::to_owned),
        binding_node.structure().clone(),
        binding_node.provenance().clone(),
    ))
}

pub(super) fn lower_theme_token_node(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    token_node: &WorthUiLegallyStructuredArtifactInputThemeTokenNode,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundArtifactInputThemeTokenNode, Vec<WorthUiBindingDiagnostic>> {
    let semantic_locus = format!("token:{}", token_node.theme_token().id().as_str());
    let (resolved_target_theme_token, resolved_target_entry) = match context.resolve_theme_token(
        module_id,
        token_node.binding_target().reference_text(),
        &semantic_locus,
        token_node.provenance(),
    ) {
        Ok(resolution) => resolution,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };
    Ok(WorthUiBoundArtifactInputThemeTokenNode::new(
        token_node.theme_token().clone(),
        token_node.entry().clone(),
        token_node.authored_identity().map(str::to_owned),
        WorthUiBoundThemeTokenSemantics::new(resolved_target_theme_token, resolved_target_entry),
        token_node.provenance().clone(),
    ))
}

fn bind_surface_icon(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    surface_node: &WorthUiLegallyStructuredArtifactInputSurfaceNode,
    icon_id: &crate::capability::IconId,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundIconReference, WorthUiBindingDiagnostic> {
    let locus = format!("surface:{}/icon", surface_node.surface().id().as_str());
    let (icon, descriptor) = context.resolve_icon(
        module_id,
        icon_id,
        &locus,
        surface_node.provenance(),
        [
            WorthUiBindingDiagnosticCode::MissingSemanticSurfaceIconReference,
            WorthUiBindingDiagnosticCode::DeferredSemanticSurfaceIconReference,
            WorthUiBindingDiagnosticCode::UnsupportedSemanticSurfaceIconReference,
            WorthUiBindingDiagnosticCode::PlatformInternalSemanticSurfaceIconReference,
        ],
    )?;
    Ok(WorthUiBoundIconReference::new(icon, descriptor))
}

fn bind_command_semantics(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    surface_node: &WorthUiLegallyStructuredArtifactInputSurfaceNode,
    command_slot_index: usize,
    command_descriptor: &crate::capability::CommandDescriptor,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundCommandSemantics, Vec<WorthUiBindingDiagnostic>> {
    let mut diagnostics = Vec::new();
    let command_icon = command_descriptor.icon().and_then(|icon_id| {
        match bind_command_icon(
            module_id,
            surface_node,
            command_slot_index,
            icon_id,
            context,
        ) {
            Ok(icon) => Some(icon),
            Err(diagnostic) => {
                diagnostics.push(diagnostic);
                None
            }
        }
    });
    let projection_eligibility =
        command_descriptor
            .projection_eligibility()
            .and_then(|command_projection_id| {
                match bind_command_projection(
                    module_id,
                    surface_node,
                    command_slot_index,
                    command_projection_id,
                    context,
                ) {
                    Ok(projection) => Some(projection),
                    Err(diagnostic) => {
                        diagnostics.push(diagnostic);
                        None
                    }
                }
            });

    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }

    Ok(WorthUiBoundCommandSemantics::new(
        command_icon,
        command_descriptor.readiness().clone(),
        command_descriptor.runtime_intent_binding().cloned(),
        projection_eligibility,
    ))
}

fn bind_command_icon(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    surface_node: &WorthUiLegallyStructuredArtifactInputSurfaceNode,
    command_slot_index: usize,
    icon_id: &crate::capability::IconId,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<WorthUiBoundIconReference, WorthUiBindingDiagnostic> {
    let locus = format!(
        "surface:{}/command_slot[{command_slot_index}]/icon",
        surface_node.surface().id().as_str()
    );
    let (icon, descriptor) = context.resolve_icon(
        module_id,
        icon_id,
        &locus,
        surface_node.provenance(),
        [
            WorthUiBindingDiagnosticCode::MissingSemanticCommandIconReference,
            WorthUiBindingDiagnosticCode::DeferredSemanticCommandIconReference,
            WorthUiBindingDiagnosticCode::UnsupportedSemanticCommandIconReference,
            WorthUiBindingDiagnosticCode::PlatformInternalSemanticCommandIconReference,
        ],
    )?;
    Ok(WorthUiBoundIconReference::new(icon, descriptor))
}

fn bind_command_projection(
    module_id: &worth_ui_dsl::WorthUiSourceModuleId,
    surface_node: &WorthUiLegallyStructuredArtifactInputSurfaceNode,
    command_slot_index: usize,
    command_projection_id: &crate::capability::CommandProjectionId,
    context: &mut WorthUiBindingSemanticsContext<'_>,
) -> Result<crate::source::WorthUiBoundCommandProjectionReference, WorthUiBindingDiagnostic> {
    let locus = format!(
        "surface:{}/command_slot[{command_slot_index}]/projection_eligibility",
        surface_node.surface().id().as_str()
    );
    let (command_projection, descriptor) = context.resolve_command_projection(
        module_id,
        command_projection_id,
        &locus,
        surface_node.provenance(),
    )?;
    Ok(crate::source::WorthUiBoundCommandProjectionReference::new(
        command_projection,
        descriptor,
    ))
}
