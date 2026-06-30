use crate::capability::MosaicChildRule;
use crate::source::{
    WorthUiLegallyStructuredArtifactInputBindingNode,
    WorthUiLegallyStructuredArtifactInputComponentNode,
    WorthUiLegallyStructuredArtifactInputSurfaceNode, WorthUiMosaicMountFacts,
    WorthUiMosaicRegionFacts, WorthUiMosaicStructureFacts, WorthUiResolvedArtifactInputBindingNode,
    WorthUiResolvedArtifactInputComponentNode, WorthUiResolvedArtifactInputSurfaceNode,
    WorthUiStructuralLegalityDiagnostic, WorthUiStructuralLegalityDiagnosticCode,
};

use super::worth_ui_structural_body_parser::{
    WorthUiAuthoredMount, WorthUiAuthoredRegion, WorthUiStructuralBodyParser,
};
use super::worth_ui_structural_legality_context::WorthUiStructuralLegalityContext;
use super::worth_ui_structural_semantics::{
    mount_state_slot_is_legal, placement_policy_matches_mount, region_state_slot_is_legal,
    sizing_contract_matches_region,
};

pub(super) fn lower_component_node(
    module_id: &crate::source::WorthUiSourceModuleId,
    component_node: &WorthUiResolvedArtifactInputComponentNode,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<
    WorthUiLegallyStructuredArtifactInputComponentNode,
    Vec<WorthUiStructuralLegalityDiagnostic>,
> {
    let structure = lower_structure(
        module_id,
        component_node.body_atoms(),
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
    module_id: &crate::source::WorthUiSourceModuleId,
    surface_node: &WorthUiResolvedArtifactInputSurfaceNode,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<
    WorthUiLegallyStructuredArtifactInputSurfaceNode,
    Vec<WorthUiStructuralLegalityDiagnostic>,
> {
    let structure = lower_structure(
        module_id,
        surface_node.body_atoms(),
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
    module_id: &crate::source::WorthUiSourceModuleId,
    binding_node: &WorthUiResolvedArtifactInputBindingNode,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<
    WorthUiLegallyStructuredArtifactInputBindingNode,
    Vec<WorthUiStructuralLegalityDiagnostic>,
> {
    let structure = lower_structure(
        module_id,
        binding_node.body_atoms(),
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

fn lower_structure(
    module_id: &crate::source::WorthUiSourceModuleId,
    body_atoms: &[crate::source::WorthUiArtifactInputBodyAtom],
    provenance: &crate::source::WorthUiArtifactInputProvenance,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<WorthUiMosaicStructureFacts, Vec<WorthUiStructuralLegalityDiagnostic>> {
    let authored_body = match WorthUiStructuralBodyParser::parse(body_atoms) {
        Ok(body) => body,
        Err(failure) => {
            return Err(vec![WorthUiStructuralLegalityDiagnostic::new(
                failure.code,
                module_id.clone(),
                failure.authored_text,
                failure.structural_locus,
                provenance.clone(),
            )]);
        }
    };
    let mut diagnostics = Vec::new();
    let mut root_regions = Vec::new();

    for (index, region) in authored_body.root_regions().iter().enumerate() {
        let locus = format!("root[{index}]");
        match lower_region(module_id, region, &locus, provenance, context) {
            Ok(region) => root_regions.push(region),
            Err(mut region_diagnostics) => diagnostics.append(&mut region_diagnostics),
        }
    }

    if diagnostics.is_empty() {
        Ok(WorthUiMosaicStructureFacts::new(root_regions))
    } else {
        Err(diagnostics)
    }
}

fn lower_region(
    module_id: &crate::source::WorthUiSourceModuleId,
    region: &WorthUiAuthoredRegion,
    structural_locus: &str,
    provenance: &crate::source::WorthUiArtifactInputProvenance,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<WorthUiMosaicRegionFacts, Vec<WorthUiStructuralLegalityDiagnostic>> {
    let mut diagnostics = Vec::new();
    let (resolved_region, descriptor) = match context.resolve_region(
        module_id,
        &region.region_id_text,
        structural_locus,
        provenance,
    ) {
        Ok(value) => value,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };

    let sizing_contract = region
        .sizing_contract_id_text
        .as_ref()
        .map(|authored_text| {
            context.resolve_sizing(module_id, authored_text, structural_locus, provenance)
        })
        .transpose();
    let sizing_contract = match sizing_contract {
        Ok(value) => value,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };

    if let Some((_, sizing_descriptor)) = sizing_contract.as_ref() {
        let sizing_matches = descriptor.sizing_behavior().is_some_and(|behavior| {
            sizing_contract_matches_region(behavior, sizing_descriptor.kind())
        });
        if !sizing_matches {
            diagnostics.push(WorthUiStructuralLegalityDiagnostic::new(
                WorthUiStructuralLegalityDiagnosticCode::IllegalSizingContractForRegion,
                module_id.clone(),
                sizing_descriptor.id().as_str(),
                structural_locus,
                provenance.clone(),
            ));
        }
    }

    let state_slot = region
        .state_slot_id_text
        .as_ref()
        .map(|authored_text| {
            context.resolve_state_slot(module_id, authored_text, structural_locus, provenance)
        })
        .transpose();
    let state_slot = match state_slot {
        Ok(value) => value,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };

    if let (Some((_, state_descriptor)), Some(scroll_ownership)) =
        (state_slot.as_ref(), descriptor.scroll_ownership())
    {
        if let Err(code) = region_state_slot_is_legal(
            descriptor.id(),
            descriptor.role(),
            scroll_ownership,
            state_descriptor,
        ) {
            diagnostics.push(WorthUiStructuralLegalityDiagnostic::new(
                code,
                module_id.clone(),
                state_descriptor.id().as_str(),
                structural_locus,
                provenance.clone(),
            ));
        }
    }

    let mut child_regions = Vec::new();
    for (index, child_region) in region.child_regions.iter().enumerate() {
        match lower_region(
            module_id,
            child_region,
            &format!("{structural_locus}/region[{index}]"),
            provenance,
            context,
        ) {
            Ok(child) => child_regions.push(child),
            Err(mut child_diagnostics) => diagnostics.append(&mut child_diagnostics),
        }
    }

    let mut mounts = Vec::new();
    for mount in &region.mounts {
        match lower_mount(
            module_id,
            mount,
            descriptor.role(),
            structural_locus,
            provenance,
            context,
        ) {
            Ok(mount) => mounts.push(mount),
            Err(mut mount_diagnostics) => diagnostics.append(&mut mount_diagnostics),
        }
    }

    match descriptor.child_rule() {
        Some(MosaicChildRule::AcceptsSurfaces) if !child_regions.is_empty() => {
            diagnostics.push(WorthUiStructuralLegalityDiagnostic::new(
                WorthUiStructuralLegalityDiagnosticCode::IllegalRegionChildMix,
                module_id.clone(),
                descriptor.id().as_str(),
                structural_locus,
                provenance.clone(),
            ))
        }
        Some(MosaicChildRule::AcceptsRegions | MosaicChildRule::AcceptsRegionStack)
            if !mounts.is_empty() =>
        {
            diagnostics.push(WorthUiStructuralLegalityDiagnostic::new(
                WorthUiStructuralLegalityDiagnosticCode::IllegalSurfaceMountInRegion,
                module_id.clone(),
                descriptor.id().as_str(),
                structural_locus,
                provenance.clone(),
            ))
        }
        Some(MosaicChildRule::LeafOnly) if !mounts.is_empty() || !child_regions.is_empty() => {
            diagnostics.push(WorthUiStructuralLegalityDiagnostic::new(
                WorthUiStructuralLegalityDiagnosticCode::IllegalLeafRegionChildren,
                module_id.clone(),
                descriptor.id().as_str(),
                structural_locus,
                provenance.clone(),
            ))
        }
        _ => {}
    }

    if diagnostics.is_empty() {
        Ok(WorthUiMosaicRegionFacts::new(
            resolved_region,
            descriptor,
            sizing_contract,
            state_slot,
            child_regions,
            mounts,
        ))
    } else {
        Err(diagnostics)
    }
}

fn lower_mount(
    module_id: &crate::source::WorthUiSourceModuleId,
    mount: &WorthUiAuthoredMount,
    region_role: &crate::capability::MosaicRegionRole,
    structural_locus: &str,
    provenance: &crate::source::WorthUiArtifactInputProvenance,
    context: &mut WorthUiStructuralLegalityContext<'_>,
) -> Result<WorthUiMosaicMountFacts, Vec<WorthUiStructuralLegalityDiagnostic>> {
    let mount_locus = format!("{structural_locus}/mount:{}", mount.surface_id_text);
    let (surface, descriptor) = match context.resolve_surface(
        module_id,
        &mount.surface_id_text,
        &mount_locus,
        provenance,
    ) {
        Ok(value) => value,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };
    let placement_policy = mount
        .placement_policy_id_text
        .as_ref()
        .map(|authored_text| {
            context.resolve_placement(module_id, authored_text, &mount_locus, provenance)
        })
        .transpose();
    let placement_policy = match placement_policy {
        Ok(value) => value,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };
    let state_slot = mount
        .state_slot_id_text
        .as_ref()
        .map(|authored_text| {
            context.resolve_state_slot(module_id, authored_text, &mount_locus, provenance)
        })
        .transpose();
    let state_slot = match state_slot {
        Ok(value) => value,
        Err(diagnostic) => return Err(vec![diagnostic]),
    };

    let mut diagnostics = Vec::new();
    if let Some((_, policy_descriptor)) = placement_policy.as_ref() {
        if !placement_policy_matches_mount(&descriptor, region_role, policy_descriptor) {
            diagnostics.push(WorthUiStructuralLegalityDiagnostic::new(
                WorthUiStructuralLegalityDiagnosticCode::IllegalPlacementPolicyForMount,
                module_id.clone(),
                policy_descriptor.id().as_str(),
                &mount_locus,
                provenance.clone(),
            ));
        }
    }

    if let Some((_, state_descriptor)) = state_slot.as_ref() {
        if let Err(code) = mount_state_slot_is_legal(descriptor.id(), &descriptor, state_descriptor)
        {
            diagnostics.push(WorthUiStructuralLegalityDiagnostic::new(
                code,
                module_id.clone(),
                state_descriptor.id().as_str(),
                &mount_locus,
                provenance.clone(),
            ));
        }
    }

    if diagnostics.is_empty() {
        Ok(WorthUiMosaicMountFacts::new(
            surface,
            descriptor,
            placement_policy,
            state_slot,
        ))
    } else {
        Err(diagnostics)
    }
}
