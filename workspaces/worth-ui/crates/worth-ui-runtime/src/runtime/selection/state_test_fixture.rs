use super::*;

pub(super) fn key(value: u64) -> UiSelectionStableKey {
    UiSelectionStableKey::new(crate::runtime::UiApplicationItemKey::new(
        item_key_family(),
        core::num::NonZeroU64::new(value).unwrap(),
    ))
}

pub(super) fn item_key_family() -> crate::runtime::UiApplicationItemKeyFamily {
    crate::runtime::UiApplicationItemKeyFamily::new(core::num::NonZeroU64::new(3).unwrap())
}

pub(super) fn owner() -> UiSelectionOwnerIdentity {
    UiSelectionOwnerIdentity::new(
        worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().expect("surface"),
        crate::graph::UiGraphNodeIdentity::new(71),
        item_key_family(),
    )
}

pub(super) fn incarnation() -> UiSelectionOwnerIncarnation {
    UiSelectionOwnerIncarnation::new(7).unwrap()
}

pub(super) fn registration(
    owner: UiSelectionOwnerIdentity,
    policy: UiSelectionPolicy,
    catalog: Vec<UiSelectionStableKey>,
    posture: UiSelectionCatalogPosture,
) -> UiSelectionRegistration {
    UiSelectionRegistration::new(owner, incarnation(), policy, catalog, posture).unwrap()
}
