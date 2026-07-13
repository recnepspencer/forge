use forge_store_layout_indexes::maintenance::{
    layout_maintenance, VerifierMaintenanceProtocol,
};

fn misuse(protocol: &VerifierMaintenanceProtocol) {
    let _ = layout_maintenance().certify_live_exact(protocol);
}

fn main() {}
