use worth_spatial::facade::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingSet;

fn main() {
    let canonical: Option<PlanarBooleanOverlapRegionCanonicalWindingSet> = None;
    let _ = canonical
        .unwrap()
        .mint_overlap_region_identity_lineage();
}
