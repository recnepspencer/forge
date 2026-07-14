use worth_store_physical_certification::{
    ForbiddenShortcutKind, SyntheticHarnessShortcutDenialReceipt,
};

fn main() {
    let _receipt = SyntheticHarnessShortcutDenialReceipt {
        shortcut: ForbiddenShortcutKind::PrivateMutation,
    };
}
