use forge_store_contracts::IoPressureBackgroundPressureDeclaration;

pub const fn offline_repair_scan_background_pressure_shape(
    read_ahead_pages: u64,
) -> IoPressureBackgroundPressureDeclaration {
    IoPressureBackgroundPressureDeclaration::repair_scan(read_ahead_pages)
}

pub const fn offline_verification_pressure_background_pressure_shape(
    read_ahead_pages: u64,
) -> IoPressureBackgroundPressureDeclaration {
    IoPressureBackgroundPressureDeclaration::verification_pressure(read_ahead_pages)
}
