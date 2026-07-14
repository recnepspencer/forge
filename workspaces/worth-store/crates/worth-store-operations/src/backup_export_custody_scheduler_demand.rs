use worth_store_contracts::BackgroundPressureDeclaration;

pub const fn backup_prep_background_pressure_shape(
    bytes: u64,
    read_ahead_pages: u64,
) -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::backup_prep_read(bytes, read_ahead_pages)
}
