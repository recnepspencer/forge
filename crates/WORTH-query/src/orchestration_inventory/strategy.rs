#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct WorthQueryOrchestrationStrategyAttachment {
    relational_merge_strategy_relevant: bool,
    signal_merge_invalidation_delivery_strategy_relevant: bool,
    runtime_bridge_preview_merge_writeback_strategy_relevant: bool,
    foundational_materialization_profile_relevant: bool,
}

impl WorthQueryOrchestrationStrategyAttachment {
    pub const fn new(
        relational_merge_strategy_relevant: bool,
        signal_merge_invalidation_delivery_strategy_relevant: bool,
        runtime_bridge_preview_merge_writeback_strategy_relevant: bool,
        foundational_materialization_profile_relevant: bool,
    ) -> Self {
        Self {
            relational_merge_strategy_relevant,
            signal_merge_invalidation_delivery_strategy_relevant,
            runtime_bridge_preview_merge_writeback_strategy_relevant,
            foundational_materialization_profile_relevant,
        }
    }

    pub const fn none() -> Self {
        Self::new(false, false, false, false)
    }

    pub const fn foundational_materialization_profile() -> Self {
        Self::new(false, false, false, true)
    }

    pub const fn declaration_entry_foundational() -> Self {
        Self::foundational_materialization_profile()
    }

    pub const fn signal_and_bridge() -> Self {
        Self::new(false, true, true, false)
    }

    pub fn relational_merge_strategy_relevant(self) -> bool {
        self.relational_merge_strategy_relevant
    }

    pub fn signal_merge_invalidation_delivery_strategy_relevant(self) -> bool {
        self.signal_merge_invalidation_delivery_strategy_relevant
    }

    pub fn runtime_bridge_preview_merge_writeback_strategy_relevant(self) -> bool {
        self.runtime_bridge_preview_merge_writeback_strategy_relevant
    }

    pub fn foundational_materialization_profile_relevant(self) -> bool {
        self.foundational_materialization_profile_relevant
    }

    pub fn is_merge_strategy_aware(self) -> bool {
        self.relational_merge_strategy_relevant
            || self.signal_merge_invalidation_delivery_strategy_relevant
            || self.runtime_bridge_preview_merge_writeback_strategy_relevant
    }

    pub fn as_str(self) -> &'static str {
        match (
            self.relational_merge_strategy_relevant,
            self.signal_merge_invalidation_delivery_strategy_relevant,
            self.runtime_bridge_preview_merge_writeback_strategy_relevant,
            self.foundational_materialization_profile_relevant,
        ) {
            (false, false, false, false) => "none",
            (false, false, false, true) => "foundational_materialization_profile",
            (false, true, true, false) => "signal_and_runtime_bridge_strategy",
            (true, false, false, false) => "relational_merge_strategy",
            _ => "mixed_strategy_attachment",
        }
    }
}
