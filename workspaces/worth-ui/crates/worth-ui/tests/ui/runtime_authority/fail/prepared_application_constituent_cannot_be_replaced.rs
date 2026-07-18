use worth_ui::facade::app::WorthUiPreparedApplicationAuthority;
use worth_ui::facade::registry::CapabilitySnapshot;

fn replace_capabilities(
    authority: &mut WorthUiPreparedApplicationAuthority,
    replacement: CapabilitySnapshot,
) {
    authority.capability_snapshot = replacement;
}

fn main() {}
