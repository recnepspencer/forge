use crate::capability::MosaicStatePersistencePolicy;
use crate::source::{
    WorthUiArtifactIdentitySeed, WorthUiBoundArtifactInputBindingNode,
    WorthUiBoundArtifactInputComponentNode, WorthUiBoundArtifactInputSurfaceNode,
    WorthUiBoundArtifactInputThemeTokenNode, WorthUiDurableStateEligibility,
    WorthUiDurableStateIneligibilityReason, WorthUiMosaicRegionFacts, WorthUiMosaicStructureFacts,
    WorthUiSourceModuleId,
};

pub(super) fn component_seed(
    module_id: &WorthUiSourceModuleId,
    node: &WorthUiBoundArtifactInputComponentNode,
) -> WorthUiArtifactIdentitySeed {
    authored_or_structural_seed(
        "component",
        module_id,
        node.authored_identity(),
        &format!(
            "{}|{}",
            node.component().id().as_str(),
            structure_digest_basis(node.structure())
        ),
    )
}

pub(super) fn surface_seed(
    module_id: &WorthUiSourceModuleId,
    node: &WorthUiBoundArtifactInputSurfaceNode,
) -> WorthUiArtifactIdentitySeed {
    authored_or_structural_seed(
        "surface",
        module_id,
        node.authored_identity(),
        &format!(
            "{}|{}",
            node.surface().id().as_str(),
            structure_digest_basis(node.structure())
        ),
    )
}

pub(super) fn binding_seed(
    module_id: &WorthUiSourceModuleId,
    node: &WorthUiBoundArtifactInputBindingNode,
) -> WorthUiArtifactIdentitySeed {
    authored_or_structural_seed(
        "binding",
        module_id,
        node.authored_identity(),
        node.view_binding_reference().view_binding().id().as_str(),
    )
}

pub(super) fn token_seed(
    module_id: &WorthUiSourceModuleId,
    node: &WorthUiBoundArtifactInputThemeTokenNode,
) -> WorthUiArtifactIdentitySeed {
    authored_or_structural_seed(
        "token",
        module_id,
        node.authored_identity(),
        node.theme_token().id().as_str(),
    )
}

pub(super) fn import_seed(
    module_id: &WorthUiSourceModuleId,
    import: &crate::source::WorthUiArtifactInputImportNode,
) -> WorthUiArtifactIdentitySeed {
    WorthUiArtifactIdentitySeed::structural_fallback(format!(
        "import|module:{}|target:{}",
        module_id.as_str(),
        import.target().authored_text()
    ))
}

pub(super) fn classify_durable_state(
    structure: &WorthUiMosaicStructureFacts,
) -> WorthUiDurableStateEligibility {
    let count = count_restorable_state_slots(structure);
    if count == 0 {
        WorthUiDurableStateEligibility::Ineligible {
            reason: WorthUiDurableStateIneligibilityReason::NoRestorableStateSlots,
        }
    } else {
        WorthUiDurableStateEligibility::Eligible {
            restorable_state_slot_count: count,
        }
    }
}

pub(super) fn no_durable_state_surface() -> WorthUiDurableStateEligibility {
    WorthUiDurableStateEligibility::Ineligible {
        reason: WorthUiDurableStateIneligibilityReason::NoDurableStateSurface,
    }
}

fn authored_or_structural_seed(
    kind: &str,
    module_id: &WorthUiSourceModuleId,
    authored_identity: Option<&str>,
    fallback_identity: &str,
) -> WorthUiArtifactIdentitySeed {
    match authored_identity {
        Some(authored_identity) => WorthUiArtifactIdentitySeed::authored(format!(
            "{kind}|module:{}|authored:{authored_identity}",
            module_id.as_str()
        )),
        None => WorthUiArtifactIdentitySeed::structural_fallback(format!(
            "{kind}|module:{}|identity:{fallback_identity}",
            module_id.as_str()
        )),
    }
}

fn count_restorable_state_slots(structure: &WorthUiMosaicStructureFacts) -> usize {
    structure
        .root_regions()
        .iter()
        .map(count_restorable_region_slots)
        .sum()
}

fn count_restorable_region_slots(region: &WorthUiMosaicRegionFacts) -> usize {
    let region_slot = count_slot(region.state_slot().map(|(_, descriptor)| descriptor));
    let mount_slots = region
        .mounts()
        .iter()
        .map(|mount| count_slot(mount.state_slot().map(|(_, descriptor)| descriptor)))
        .sum::<usize>();
    let child_slots = region
        .child_regions()
        .iter()
        .map(count_restorable_region_slots)
        .sum::<usize>();
    region_slot + mount_slots + child_slots
}

fn count_slot(slot: Option<&crate::capability::MosaicStateSlotDescriptor>) -> usize {
    match slot.and_then(|descriptor| descriptor.persistence_policy()) {
        Some(MosaicStatePersistencePolicy::RestoreAcrossHotReload)
        | Some(MosaicStatePersistencePolicy::PersistAcrossRuntimeRestart) => 1,
        _ => 0,
    }
}

fn structure_digest_basis(structure: &WorthUiMosaicStructureFacts) -> String {
    let mut basis = String::from("regions[");
    for region in structure.root_regions() {
        push_region_digest_basis(&mut basis, region);
        basis.push('|');
    }
    basis.push(']');
    basis
}

fn push_region_digest_basis(basis: &mut String, region: &WorthUiMosaicRegionFacts) {
    basis.push_str("region(");
    basis.push_str(region.region().id().as_str());
    basis.push(')');
    if let Some((sizing_contract, _)) = region.sizing_contract() {
        basis.push_str("sizing(");
        basis.push_str(sizing_contract.id().as_str());
        basis.push(')');
    }
    if let Some((state_slot, _)) = region.state_slot() {
        basis.push_str("state(");
        basis.push_str(state_slot.id().as_str());
        basis.push(')');
    }
    basis.push_str("children[");
    for child_region in region.child_regions() {
        push_region_digest_basis(basis, child_region);
        basis.push('|');
    }
    basis.push(']');
    basis.push_str("mounts[");
    for mount in region.mounts() {
        basis.push_str("mount(");
        basis.push_str(mount.surface().id().as_str());
        basis.push(')');
        if let Some((placement_policy, _)) = mount.placement_policy() {
            basis.push_str("placement(");
            basis.push_str(placement_policy.id().as_str());
            basis.push(')');
        }
        if let Some((state_slot, _)) = mount.state_slot() {
            basis.push_str("state(");
            basis.push_str(state_slot.id().as_str());
            basis.push(')');
        }
        basis.push('|');
    }
    basis.push(']');
}
