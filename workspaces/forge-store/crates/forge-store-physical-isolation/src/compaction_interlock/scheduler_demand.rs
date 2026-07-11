use forge_store_contracts::S6BackgroundPressureDeclaration;

pub const fn compaction_rewrite_scheduler_demand() -> S6BackgroundPressureDeclaration {
    S6BackgroundPressureDeclaration::compaction_rewrite()
}
