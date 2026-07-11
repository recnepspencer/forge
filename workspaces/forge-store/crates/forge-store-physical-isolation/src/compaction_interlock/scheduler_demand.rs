use forge_store_contracts::BackgroundPressureDeclaration;

pub const fn compaction_rewrite_scheduler_demand() -> BackgroundPressureDeclaration {
    BackgroundPressureDeclaration::compaction_rewrite()
}
