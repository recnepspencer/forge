use std::collections::HashMap;

use worth_ui_host_contract::{UiMountedPaintCommandIdentity, UiMountedPaintOrderIdentity};

fn correlate_without_ordering(
    command: UiMountedPaintCommandIdentity,
    order: UiMountedPaintOrderIdentity,
) -> bool {
    let mut authored_position = HashMap::new();
    authored_position.insert(command, 0_usize);
    authored_position.get(&order.command()) == Some(&0)
}

fn main() {}
