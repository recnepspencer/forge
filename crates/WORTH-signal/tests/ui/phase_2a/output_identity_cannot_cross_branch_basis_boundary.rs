use worth_signal::facade::{bridge_signal_branch_basis_trust_boundary, OutputIdentity};

fn main() {
    let token = OutputIdentity::new("host-output");
    let _bridged = bridge_signal_branch_basis_trust_boundary(token);
}
