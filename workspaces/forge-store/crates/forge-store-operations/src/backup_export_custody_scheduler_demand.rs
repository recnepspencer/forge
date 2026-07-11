use forge_store_contracts::IoPressureBackgroundPressureDeclaration;

pub const fn backup_prep_background_pressure_shape(
    bytes: u64,
    read_ahead_pages: u64,
) -> IoPressureBackgroundPressureDeclaration {
    IoPressureBackgroundPressureDeclaration::backup_prep_read(bytes, read_ahead_pages)
}
