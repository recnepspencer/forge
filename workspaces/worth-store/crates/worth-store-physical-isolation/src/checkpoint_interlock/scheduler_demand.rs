use worth_store_contracts::BackgroundPressureDeclaration;

pub const fn checkpoint_flush_scheduler_demand() -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::checkpoint_flush()
}
