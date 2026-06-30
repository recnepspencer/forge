use topology::touched_graph_conflict::admit_topology_conflict_family_identity;

fn main() {
    let _ = admit_topology_conflict_family_identity("loop-overlap");
    let _ = admit_topology_conflict_family_identity("coplanar-overlap");
}
