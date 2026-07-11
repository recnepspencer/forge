use forge_store_contracts::IoPressureBackgroundPressureDeclaration;

pub const fn repair_background_pressure_shape(
    read_ahead_pages: u64,
) -> IoPressureBackgroundPressureDeclaration {
    IoPressureBackgroundPressureDeclaration::repair_scan(read_ahead_pages)
}
