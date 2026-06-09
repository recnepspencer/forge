use forge_server::{ForgeServerAdmission, ForgeServerForgeNativeSession};

fn main() {
    let _session = ForgeServerForgeNativeSession::new(fake_admission());
}

fn fake_admission() -> ForgeServerAdmission {
    todo!()
}
