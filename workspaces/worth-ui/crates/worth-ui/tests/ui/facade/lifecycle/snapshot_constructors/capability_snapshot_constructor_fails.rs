use worth_ui::facade::{CapabilitySnapshot, RegisteredCapabilitySet};

fn main() {
    let _ = CapabilitySnapshot::from_registered_capabilities(registered_capabilities());
}

fn registered_capabilities() -> RegisteredCapabilitySet {
    panic!("fixture never runs")
}
