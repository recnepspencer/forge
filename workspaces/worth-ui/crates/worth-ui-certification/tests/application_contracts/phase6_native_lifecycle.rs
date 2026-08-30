#[path = "../application_contracts/phase6_native_lifecycle/oracle.rs"]
mod oracle;
#[path = "../application_contracts/phase6_native_lifecycle/production_world.rs"]
mod production_world;
#[path = "../application_contracts/phase6_native_lifecycle/protocol_events.rs"]
mod protocol_events;
#[path = "../application_contracts/phase6_native_lifecycle/protocol_world.rs"]
mod protocol_world;
#[path = "../application_contracts/phase6_native_lifecycle/schedule_inventory.rs"]
mod schedule_inventory;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NativeFaultContractEvidence {
    pub(crate) qualified_schedules: usize,
    pub(crate) state_event_pairs: usize,
    pub(crate) exact_capacity_preserved_sequence: bool,
    pub(crate) over_capacity_stopped_before_retention: bool,
    pub(crate) invalid_ime_range_stopped_before_retention: bool,
}

pub(crate) fn verify_native_fault_contract() -> NativeFaultContractEvidence {
    protocol_world::verify_native_fault_contract()
}
