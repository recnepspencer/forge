use super::RelationalRuntime;

impl RelationalRuntime {
    /// Select the execution model this runtime uses from now on.
    ///
    /// Owner authority, so it takes the owner's exclusive handle. The change
    /// itself lands in the runtime's owned configuration, which is what lets it
    /// succeed while independently borrowable services are bound to this exact
    /// runtime.
    pub fn set_execution_model(
        &mut self,
        execution_model: crate::config::data::RelationalExecutionModel,
    ) {
        self.reconfigure(|configuration| configuration.set_execution_model(execution_model));
    }

    /// Put a durability mode in force for a runtime this crate is rebuilding.
    ///
    /// Recovery replays against the in-memory canonical log and restores the
    /// configured mode once the rebuilt runtime is finalized. It is owner
    /// authority like any other reconfiguration, so it goes through the same
    /// single installation route rather than reaching into the configuration.
    pub(crate) fn set_durability_mode(&mut self, mode: crate::durability::data::DurabilityMode) {
        self.reconfigure(|configuration| configuration.set_durability_mode(mode));
    }
}
