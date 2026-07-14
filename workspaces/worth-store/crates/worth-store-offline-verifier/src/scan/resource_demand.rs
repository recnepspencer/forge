use worth_store_contracts::BackgroundPressureDeclaration;

pub const fn offline_repair_scan_background_pressure_shape(
    read_ahead_pages: u64,
) -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::repair_scan(read_ahead_pages)
}

pub const fn offline_verification_pressure_background_pressure_shape(
    read_ahead_pages: u64,
) -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::verification_pressure(read_ahead_pages)
}
