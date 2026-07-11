use forge_store_contracts::BackgroundPressureDeclaration;

pub const fn repair_background_pressure_shape(
    read_ahead_pages: u64,
) -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::repair_scan(read_ahead_pages)
}
