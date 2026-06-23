use forge_query::facade::{ForgeQueryNativeRow, ForgeQueryWorkspace};

fn main() {
    let mut workspace = workspace_fixture();

    let _ = workspace.state_live_by_name("live.example");
    let _ = workspace.read_live_by_name("live.example");
    let _ = workspace.subscription_basis_digest_by_name("live.example");
    let _ = workspace.inspect_live_by_name("live.example");
}

fn workspace_fixture() -> ForgeQueryWorkspace {
    panic!("fixture only")
}

fn _row_marker() -> ForgeQueryNativeRow {
    ForgeQueryNativeRow
}
