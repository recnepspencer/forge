use worth_query::facade::certification::{
    WorthQueryPublicBridgeReaderLaneInventory, WorthQueryPublicBridgeReaderLaneSabotage,
};

const PUBLIC_BRIDGE_CERTIFICATION_SOURCES: &[(&str, &str)] = &[
    (
        "tests/support/public_bridge_runtime/hostile_certification.rs",
        include_str!("../hostile_certification.rs"),
    ),
    (
        "tests/support/public_bridge_runtime/reader_lane_honesty/projection_reader.rs",
        include_str!("projection_reader.rs"),
    ),
];

const SABOTAGED_PUBLIC_BRIDGE_CERTIFICATION_SOURCES: &[(&str, &str)] = &[
    (
        "tests/support/public_bridge_runtime/hostile_certification.rs",
        include_str!("../hostile_certification.rs"),
    ),
    (
        "tests/support/public_bridge_runtime/reader_lane_honesty/projection_reader.rs",
        r#"
            artifact
                .published_binding()
                .unwrap()
                .materialization_by_name(view)
                .unwrap()
                .rows()
        "#,
    ),
];

pub fn public_bridge_certification_inventory() -> WorthQueryPublicBridgeReaderLaneInventory {
    WorthQueryPublicBridgeReaderLaneInventory::scan(
        PUBLIC_BRIDGE_CERTIFICATION_SOURCES.iter().copied(),
    )
}

pub fn sabotaged_public_bridge_certification_inventory() -> WorthQueryPublicBridgeReaderLaneInventory
{
    WorthQueryPublicBridgeReaderLaneInventory::scan(
        SABOTAGED_PUBLIC_BRIDGE_CERTIFICATION_SOURCES
            .iter()
            .copied(),
    )
}

pub fn public_bridge_direct_materialization_sabotage() -> WorthQueryPublicBridgeReaderLaneSabotage {
    WorthQueryPublicBridgeReaderLaneSabotage::evaluate_direct_materialization_read(
        &sabotaged_public_bridge_certification_inventory(),
    )
}

#[allow(dead_code)]
pub fn direct_materialization_read_count(source: &str) -> usize {
    WorthQueryPublicBridgeReaderLaneInventory::scan([("inline-sabotage", source)])
        .direct_materialization_read_count()
}

#[allow(dead_code)]
pub fn public_bridge_certification_inventory_paths() -> Vec<String> {
    public_bridge_certification_inventory().paths().to_vec()
}
