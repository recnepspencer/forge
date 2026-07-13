use forge_store_physical_format::{
    AdmittedPhysicalAccess, PhysicalAccessCounterReceipt, PhysicalLayoutAccessCounterSnapshot,
};

fn forge(
    access: AdmittedPhysicalAccess,
    counters: PhysicalLayoutAccessCounterSnapshot,
) -> PhysicalAccessCounterReceipt {
    PhysicalAccessCounterReceipt { access, counters }
}

fn main() {}
