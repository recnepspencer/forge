#![allow(invalid_value)]

use worth_spatial::facade::evidence_lookup_inventory::EvidenceLookupInventoryCloseout;

fn main() {
    let _ = EvidenceLookupInventoryCloseout {
        guard_report: unsafe { std::mem::zeroed() },
        rows: Vec::new(),
        counters: unsafe { std::mem::zeroed() },
        catalog_validation_report: unsafe { std::mem::zeroed() },
        closeout_digest: String::new(),
    };
}
