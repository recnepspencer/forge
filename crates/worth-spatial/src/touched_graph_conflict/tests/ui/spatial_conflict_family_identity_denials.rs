use worth_spatial::touched_graph_conflict::admit_spatial_conflict_family_identity;

fn main() {
    let _ = admit_spatial_conflict_family_identity("shared-plane-overlap");
    let _ = admit_spatial_conflict_family_identity("coplanar-overlap");
}
