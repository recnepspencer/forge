use super::{BaselineName, SupplyChainBaseline};
use crate::world::supply_chain::scale::SupplyChainScale;
use crate::world::supply_chain::schema::SchemaVersion;

pub(super) fn build(scale: SupplyChainScale) -> SupplyChainBaseline {
    let mut baseline = super::operating::build(scale);
    baseline.name = BaselineName::VersionBoundary;
    baseline.pre_upgrade_schema = Some(SchemaVersion::V1);
    baseline.post_upgrade_schema = Some(SchemaVersion::V2);
    baseline
}
