use crate::live::{BridgeChangeSummary, BridgeFieldDelta, BridgeRelationDelta};

pub(super) fn detail_patch_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "identity",
        "id",
        Some("user-1"),
        Some("user-2"),
    ))
}

pub(super) fn ordered_collection_patch_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "profile",
        "display_name",
        Some("Avery"),
        Some("Zoey"),
    ))
}

pub(super) fn bounded_materialization_patch_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_relation_delta(BridgeRelationDelta::new("manager"))
        .with_materialization_scope_transition(false, true)
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Old Manager"),
            Some("New Manager"),
        ))
}
