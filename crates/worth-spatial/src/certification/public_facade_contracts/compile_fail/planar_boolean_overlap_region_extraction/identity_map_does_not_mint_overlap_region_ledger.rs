use worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionIdentityMap;

fn main() {
    let identity_map: Option<PlanarBooleanOverlapRegionIdentityMap> = None;
    let _ = identity_map.unwrap().mint_overlap_region_ledger();
}
