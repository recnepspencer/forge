use forge_store_contracts::IoPressureBackgroundPressureDeclaration;

pub const fn replication_prep_background_pressure_shape(
    read_ahead_pages: u64,
) -> IoPressureBackgroundPressureDeclaration {
    IoPressureBackgroundPressureDeclaration::replication_prep_read(read_ahead_pages)
}
