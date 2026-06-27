use topology::facade::{
    admit_topology_replay_family_identity, TopologyUndoFamilyIdentityAuthority,
};

fn main() {
    let authority = TopologyUndoFamilyIdentityAuthority::traversal_views();
    let _ = admit_topology_replay_family_identity(authority);
}
