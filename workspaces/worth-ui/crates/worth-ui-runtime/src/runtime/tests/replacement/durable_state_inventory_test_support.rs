use crate::capability::{
    FrozenMosaicStateCapabilities, MosaicRegionKindId, MosaicStateOwnerIdentity,
    MosaicStatePersistencePolicy, MosaicStateReplacementRule, MosaicStateSlotDescriptor,
    MosaicStateSlotId, MosaicStateSlotKind, MosaicStateTruthPosture,
};
use crate::runtime::{
    WorthUiDurableStateInventory, WorthUiDurableStateInventoryDenial, WorthUiNodeReplacementPlan,
};

use super::identity_match_graph_test_support::{
    artifact_from_nodes, component_node, identity_match_app, runtime_and_narrowing,
    splitter_surface_node,
};
use super::node_replacement_classification_test_support::{narrowing_for, no_op_impact_for};

pub(super) struct ProductionStateInventoryFixture {
    admitted_state_capabilities: FrozenMosaicStateCapabilities,
}

impl ProductionStateInventoryFixture {
    pub(super) fn build_for_replacement(
        &self,
        plan: &WorthUiNodeReplacementPlan,
    ) -> Result<WorthUiDurableStateInventory, WorthUiDurableStateInventoryDenial> {
        WorthUiDurableStateInventory::assemble_for_replacement(
            plan,
            &self.admitted_state_capabilities,
        )
    }
}

pub(super) fn deterministic_replacement_plan() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiNodeReplacementPlan,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:dashboard", 0),
            splitter_surface_node(
                "surface:main",
                "workspace.surface.main",
                "workspace.sizing.splitter.main",
                1,
            ),
        ],
    )]);
    let candidate = active.clone();

    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = no_op_impact_for(&identity_report);
    let narrowing = narrowing_for(&identity_report);
    let plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("replacement plan builds");
    (runtime, plan)
}

pub(super) fn platform_inventory(
    _runtime: &crate::runtime::WorthUiRuntime,
) -> ProductionStateInventoryFixture {
    state_inventory_fixture([])
}

pub(super) fn admitted_state_inventory(
    descriptors: impl IntoIterator<Item = MosaicStateSlotDescriptor>,
) -> ProductionStateInventoryFixture {
    state_inventory_fixture(descriptors)
}

pub(super) fn state_slot(
    id: &str,
    kind: MosaicStateSlotKind,
    persistence_policy: MosaicStatePersistencePolicy,
    replacement_rule: MosaicStateReplacementRule,
) -> MosaicStateSlotDescriptor {
    MosaicStateSlotDescriptor::new(
        MosaicStateSlotId::new(id).expect("valid test state-slot id"),
        kind,
    )
    .with_owner_identity(MosaicStateOwnerIdentity::mosaic_region_kind(
        MosaicRegionKindId::new("workspace.region.sidebar").expect("valid test mosaic-region id"),
    ))
    .with_persistence_policy(persistence_policy)
    .with_replacement_rule(replacement_rule)
    .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state())
}

fn state_inventory_fixture(
    descriptors: impl IntoIterator<Item = MosaicStateSlotDescriptor>,
) -> ProductionStateInventoryFixture {
    let app = descriptors.into_iter().fold(
        crate::facade::WorthUi::app()
            .bind_certification_host()
            .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse()),
        |builder, descriptor| builder.register_mosaic_state_slot(descriptor),
    );
    let prepared = app.freeze().expect("state fixture application freezes");
    ProductionStateInventoryFixture {
        admitted_state_capabilities: prepared.capabilities().mosaic_state_slots().clone(),
    }
}
