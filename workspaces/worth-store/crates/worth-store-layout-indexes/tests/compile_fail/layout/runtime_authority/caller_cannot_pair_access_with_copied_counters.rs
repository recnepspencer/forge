use worth_store_physical_format::{
    AdmittedPhysicalAccess, PhysicalAccessCounterReceipt, PhysicalLayoutAccessCounterSnapshot,
};

fn worth(
    access: AdmittedPhysicalAccess,
    counters: PhysicalLayoutAccessCounterSnapshot,
) -> PhysicalAccessCounterReceipt {
    PhysicalAccessCounterReceipt { access, counters }
}

fn main() {}
