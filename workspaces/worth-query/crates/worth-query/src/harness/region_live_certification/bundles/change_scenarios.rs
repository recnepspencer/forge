use crate::live::{BridgeChangeSummary, BridgeFieldDelta, BridgeRelationDelta};

pub(super) fn detail_in_region_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
}

pub(super) fn detail_off_region_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-b")
}

pub(super) fn detail_region_widening_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
        .with_region_slice("assembly-b")
}

pub(super) fn detail_without_locality_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default().with_field_delta(BridgeFieldDelta::new(
        "identity",
        "id",
        Some("user-1"),
        Some("user-2"),
    ))
}

pub(super) fn partition_coarse_fallback_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_coarse_fallback_slice("tenant-a")
}

pub(super) fn duplicate_region_slice_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_region_slice("assembly-a")
        .with_region_slice("assembly-a")
}

pub(super) fn single_field_partition_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_partition_slice("tenant-a")
}

pub(super) fn two_field_partition_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_field_delta(BridgeFieldDelta::new(
            "identity",
            "id",
            Some("user-1"),
            Some("user-2"),
        ))
        .with_partition_slice("tenant-a")
}

pub(super) fn bounded_in_region_change() -> BridgeChangeSummary {
    BridgeChangeSummary::default()
        .with_field_delta(BridgeFieldDelta::new(
            "profile",
            "display_name",
            Some("Esther"),
            Some("Ess"),
        ))
        .with_relation_delta(BridgeRelationDelta::new("manager"))
        .with_materialization_scope_transition(false, true)
        .with_region_slice("assembly-a")
}
