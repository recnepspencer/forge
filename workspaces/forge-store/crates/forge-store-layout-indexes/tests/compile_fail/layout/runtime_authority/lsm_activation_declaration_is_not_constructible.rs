use forge_store_lsm_authority::LsmMembershipActivationDeclaration;

fn forge() -> LsmMembershipActivationDeclaration {
    LsmMembershipActivationDeclaration {
        selected_key: panic!(),
        selected_identities: panic!(),
        selected_base: None,
        selected_version: 0,
        store_binding: String::new(),
        output: panic!(),
        scope: panic!(),
        bytes: Vec::new(),
    }
}

fn main() {
    let _ = forge();
}
