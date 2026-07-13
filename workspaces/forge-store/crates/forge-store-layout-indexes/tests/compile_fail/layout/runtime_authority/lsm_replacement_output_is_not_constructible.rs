use forge_store_lsm_authority::AdmittedLsmReplacementOutput;

fn forge() -> AdmittedLsmReplacementOutput {
    AdmittedLsmReplacementOutput {
        envelope: panic!(),
        scope: panic!(),
        persisted_path: panic!(),
        persisted_bytes: 4096,
        key: panic!(),
        selected_identities: panic!(),
        membership_version: 1,
        store_binding: String::new(),
        physical: panic!(),
    }
}

fn main() {
    let _ = forge();
}
