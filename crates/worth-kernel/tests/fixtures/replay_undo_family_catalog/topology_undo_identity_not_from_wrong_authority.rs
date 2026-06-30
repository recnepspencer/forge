use topology::facade::{
    admit_topology_undo_family_identity, TopologyReplayFamilyIdentityAuthority,
};

fn main() {
    let authority = TopologyReplayFamilyIdentityAuthority::traversal_views();
    let _ = admit_topology_undo_family_identity(authority);
}
