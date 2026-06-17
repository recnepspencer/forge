use forge_signal::facade::{bridge_signal_branch_basis_trust_boundary, PartitionToken};

fn main() {
    let token = PartitionToken::new("host-partition");
    let _bridged = bridge_signal_branch_basis_trust_boundary(token);
}
