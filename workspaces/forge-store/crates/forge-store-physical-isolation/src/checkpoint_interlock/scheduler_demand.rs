use forge_store_contracts::S6BackgroundPressureDeclaration;

pub const fn checkpoint_flush_scheduler_demand() -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::checkpoint_flush()
}
