use forge_store_physical_certification::ForbiddenShortcutSet;

fn requires_shortcut_set(_: ForbiddenShortcutSet) {}

fn main() {
    requires_shortcut_set("private-mutation");
}
