use topology::facade::{NmtTopologyScopeKind, NmtTopologyScopeReceipt};

fn main() {
    let _receipt = NmtTopologyScopeReceipt {
        parent_construction_identity: "construction".to_string(),
        pattern_identity: "pattern".to_string(),
        scope_identity: "scope".to_string(),
        kind: NmtTopologyScopeKind::OpenRadialFan,
        layer_index: None,
        face_identities: Vec::new(),
        edge_identities: Vec::new(),
        loop_identities: Vec::new(),
        topology_posture: todo!(),
        open_boundary_identity: "boundary".to_string(),
        radial_adjacency_identity: "radial".to_string(),
        counters: todo!(),
    };
}
