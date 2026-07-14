use worth_store_contracts::BackgroundPressureDeclaration;

pub const fn scrub_scan_scheduler_demand() -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::scrub_scan()
}
