use forge_store_contracts::S6BackgroundPressureDeclaration;

pub const fn backup_prep_background_pressure_shape(
    bytes: u64,
    read_ahead_pages: u64,
) -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::backup_prep_read(bytes, read_ahead_pages)
}
