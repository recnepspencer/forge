use forge_server::{ForgeServerAdmission, ForgeServerForgeNativePreparedSession};

fn main() {
    let _prepared = ForgeServerForgeNativePreparedSession::new(fake_admission());
}

fn fake_admission() -> ForgeServerAdmission {
    todo!()
}
