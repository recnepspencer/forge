use crate::source::{
    WorthUiArtifact, WorthUiArtifactNode, WorthUiBoundCommandProjectionReference,
    WorthUiBoundCommandReference, WorthUiBoundIconReference, WorthUiBoundSurfaceSemantics,
    WorthUiBoundThemeTokenSemantics, WorthUiBoundViewBindingReference,
    WorthUiDurableStateEligibility, WorthUiDurableStateIneligibilityReason,
    WorthUiMosaicMountFacts, WorthUiMosaicRegionFacts, WorthUiMosaicStructureFacts,
};

use super::worth_ui_artifact_descriptor_basis::{
    command_descriptor_basis, component_descriptor_basis, icon_descriptor_basis,
    mosaic_placement_descriptor_basis, mosaic_region_descriptor_basis,
    mosaic_sizing_descriptor_basis, mosaic_state_descriptor_basis, surface_descriptor_basis,
    theme_token_descriptor_basis,
};

pub(super) fn artifact_semantic_basis(artifact: &WorthUiArtifact) -> String {
    let modules = artifact
        .module_ids()
        .iter()
        .map(|module_id| {
            let module = artifact.module(module_id).expect("artifact module");
            let nodes = module
                .nodes()
                .iter()
                .map(node_semantic_basis)
                .collect::<Vec<_>>()
                .join("||");
            format!("module:{}=>[{nodes}]", module_id.as_str())
        })
        .collect::<Vec<_>>()
        .join("###");
    format!("artifact[{modules}]")
}

pub(super) fn node_semantic_basis(node: &WorthUiArtifactNode) -> String {
    match node {
        WorthUiArtifactNode::Import(node) => format!(
            "import|target:{}|seed:{}|durable:{}",
            node.target().authored_text(),
            identity_seed_basis(node.identity_seed()),
            durable_state_basis(node.durable_state_eligibility())
        ),
        WorthUiArtifactNode::Component(node) => format!(
            "component|id:{}|descriptor:{}|seed:{}|durable:{}|structure:{}",
            node.component().id().as_str(),
            component_descriptor_basis(node.descriptor()),
            identity_seed_basis(node.identity_seed()),
            durable_state_basis(node.durable_state_eligibility()),
            structure_basis(node.structure())
        ),
        WorthUiArtifactNode::Surface(node) => format!(
            "surface|id:{}|descriptor:{}|seed:{}|durable:{}|structure:{}|semantics:{}",
            node.surface().id().as_str(),
            surface_descriptor_basis(node.descriptor()),
            identity_seed_basis(node.identity_seed()),
            durable_state_basis(node.durable_state_eligibility()),
            structure_basis(node.structure()),
            surface_semantics_basis(node.semantics())
        ),
        WorthUiArtifactNode::Binding(node) => format!(
            "binding|id:{}|seed:{}|durable:{}|structure:{}|view:{}",
            node.view_binding_reference().view_binding().id().as_str(),
            identity_seed_basis(node.identity_seed()),
            durable_state_basis(node.durable_state_eligibility()),
            structure_basis(node.structure()),
            view_binding_reference_basis(node.view_binding_reference())
        ),
        WorthUiArtifactNode::Token(node) => format!(
            "token|id:{}|entry:{}|seed:{}|durable:{}|semantics:{}",
            node.theme_token().id().as_str(),
            theme_token_entry_basis(node.entry()),
            identity_seed_basis(node.identity_seed()),
            durable_state_basis(node.durable_state_eligibility()),
            theme_token_semantics_basis(node.semantics())
        ),
    }
}

fn identity_seed_basis(seed: &crate::source::WorthUiArtifactIdentitySeed) -> String {
    format!("{:?}:{}", seed.kind(), seed.basis())
}

fn durable_state_basis(eligibility: &WorthUiDurableStateEligibility) -> String {
    match eligibility {
        WorthUiDurableStateEligibility::Ineligible { reason } => match reason {
            WorthUiDurableStateIneligibilityReason::NoDurableStateSurface => {
                "ineligible:no_durable_state_surface".to_owned()
            }
            WorthUiDurableStateIneligibilityReason::NoRestorableStateSlots => {
                "ineligible:no_restorable_state_slots".to_owned()
            }
        },
        WorthUiDurableStateEligibility::Eligible {
            restorable_state_slot_count,
        } => format!("eligible:{restorable_state_slot_count}"),
    }
}

fn structure_basis(structure: &WorthUiMosaicStructureFacts) -> String {
    let regions = structure
        .root_regions()
        .iter()
        .map(region_basis)
        .collect::<Vec<_>>()
        .join("|");
    format!("regions[{regions}]")
}

fn region_basis(region: &WorthUiMosaicRegionFacts) -> String {
    let child_regions = region
        .child_regions()
        .iter()
        .map(region_basis)
        .collect::<Vec<_>>()
        .join("|");
    let mounts = region
        .mounts()
        .iter()
        .map(mount_basis)
        .collect::<Vec<_>>()
        .join("|");
    [
        format!("region_id:{}", region.region().id().as_str()),
        format!(
            "region_descriptor:{}",
            mosaic_region_descriptor_basis(region.descriptor())
        ),
        option_basis(region.sizing_contract().map(|(contract, descriptor)| {
            format!(
                "{}:{}",
                contract.id().as_str(),
                mosaic_sizing_descriptor_basis(descriptor)
            )
        })),
        option_basis(region.state_slot().map(|(slot, descriptor)| {
            format!(
                "{}:{}",
                slot.id().as_str(),
                mosaic_state_descriptor_basis(descriptor)
            )
        })),
        format!("children:[{child_regions}]"),
        format!("mounts:[{mounts}]"),
    ]
    .join("|")
}

fn mount_basis(mount: &WorthUiMosaicMountFacts) -> String {
    [
        format!("surface_id:{}", mount.surface().id().as_str()),
        format!(
            "surface_descriptor:{}",
            surface_descriptor_basis(mount.descriptor())
        ),
        option_basis(mount.placement_policy().map(|(policy, descriptor)| {
            format!(
                "{}:{}",
                policy.id().as_str(),
                mosaic_placement_descriptor_basis(descriptor)
            )
        })),
        option_basis(mount.state_slot().map(|(slot, descriptor)| {
            format!(
                "{}:{}",
                slot.id().as_str(),
                mosaic_state_descriptor_basis(descriptor)
            )
        })),
    ]
    .join("|")
}

fn surface_semantics_basis(semantics: &WorthUiBoundSurfaceSemantics) -> String {
    let commands = semantics
        .command_slots()
        .iter()
        .map(command_reference_basis)
        .collect::<Vec<_>>()
        .join("|");
    [
        option_basis(semantics.icon().map(icon_reference_basis)),
        format!("commands:[{commands}]"),
        option_basis(semantics.view_binding().map(view_binding_reference_basis)),
    ]
    .join("|")
}

fn icon_reference_basis(icon: &WorthUiBoundIconReference) -> String {
    format!(
        "{}:{}",
        icon.icon().id().as_str(),
        icon_descriptor_basis(icon.descriptor())
    )
}

fn command_reference_basis(command: &WorthUiBoundCommandReference) -> String {
    [
        format!("command_id:{}", command.command().id().as_str()),
        format!(
            "command_descriptor:{}",
            command_descriptor_basis(command.descriptor())
        ),
        option_basis(command.semantics().icon().map(icon_reference_basis)),
        format!(
            "readiness:{}",
            command.semantics().readiness().digest_basis()
        ),
        option_basis(
            command
                .semantics()
                .runtime_intent_binding()
                .map(|binding| binding.digest_basis().to_owned()),
        ),
        option_basis(
            command
                .semantics()
                .projection_eligibility()
                .map(command_projection_basis),
        ),
    ]
    .join("|")
}

fn command_projection_basis(projection: &WorthUiBoundCommandProjectionReference) -> String {
    format!(
        "{}:{}",
        projection.command_projection().id().as_str(),
        projection.descriptor().id().as_str()
    )
}

fn view_binding_reference_basis(view_binding: &WorthUiBoundViewBindingReference) -> String {
    let query = view_binding.query_semantics();
    [
        format!(
            "view_binding_id:{}",
            view_binding.view_binding().id().as_str()
        ),
        format!(
            "query_binding_key:{}",
            view_binding.entry().query_binding_key().as_str()
        ),
        format!("query_definition:{}", query.definition().digest().as_u64()),
        format!("query_view:{}", query.definition().identity().as_str()),
        format!("view_shape:{:?}", query.definition().shape()),
        format!("lifecycle:{:?}", query.definition().lifecycle()),
        format!(
            "denial_presentation:{}",
            query.denial_presentation().digest_basis()
        ),
    ]
    .join("|")
}

fn theme_token_semantics_basis(semantics: &WorthUiBoundThemeTokenSemantics) -> String {
    format!(
        "{}:{}:{}",
        semantics.resolved_target_theme_token().id().as_str(),
        semantics.resolved_target_entry().key().projection_basis(),
        semantics
            .resolved_target_entry()
            .resolved_target_id()
            .as_str()
    )
}

fn theme_token_entry_basis(entry: &crate::capability::FrozenThemeTokenEntry) -> String {
    [
        theme_token_descriptor_basis(entry.descriptor()),
        format!("key:{}", entry.key().projection_basis()),
        format!("resolved_target:{}", entry.resolved_target_id().as_str()),
    ]
    .join("|")
}

fn option_basis(value: Option<impl Into<String>>) -> String {
    value
        .map(|value| format!("some:{}", value.into()))
        .unwrap_or_else(|| "none".to_owned())
}
