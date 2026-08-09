use worth_ui_host_contract::{UiMountedPaintCommandIdentity, UiMountedPaintOrderIdentity};

fn sort_commands(
    left: UiMountedPaintCommandIdentity,
    right: UiMountedPaintCommandIdentity,
) -> bool {
    left < right
}

fn sort_order(
    left: UiMountedPaintOrderIdentity,
    right: UiMountedPaintOrderIdentity,
) -> bool {
    left < right
}

fn main() {}
