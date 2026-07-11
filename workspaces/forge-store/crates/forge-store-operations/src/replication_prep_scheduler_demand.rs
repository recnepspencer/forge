use forge_store_contracts::S6BackgroundPressureDeclaration;

pub const fn replication_prep_background_pressure_shape(
    read_ahead_pages: u64,
) -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::replication_prep_read(read_ahead_pages)
}
