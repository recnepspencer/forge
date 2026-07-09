use worth_store_physical_certification::register_s5_physical_isolation_certification_lane;
use worth_store_physical_isolation::PhysicalIsolationEntryAdmission;

struct CopiedS45ReadinessRows {
    row_count: usize,
}

fn register_copied_rows(
    entry: &PhysicalIsolationEntryAdmission,
    copied_rows: CopiedS45ReadinessRows,
) {
    let _ = register_s5_physical_isolation_certification_lane(entry, copied_rows);
}

fn main() {}
