use super::WorthUiNativeApplicationShell;

impl WorthUiNativeApplicationShell {
    /// Advance application-owned providers without exposing the execution
    /// coordinator or its retained authority to the native composition root.
    pub fn advance_native_intent_executions(
        &mut self,
        reading: crate::facade::intent::UiIntentExecutionClockReading,
    ) -> crate::facade::intent::UiIntentExecutionAdvanceOutcome {
        self.session.advance_intent_executions(reading)
    }

    /// Publish one completed provider handoff through the canonical ordinary
    /// observation/rebind turn.
    pub fn publish_native_intent_consequences(
        &mut self,
        handle: crate::facade::intent::UiIntentConsequenceHandle,
        now_tick: u64,
    ) -> super::UiIntentConsequencePublicationOutcome<'_> {
        self.session.publish_intent_consequences(
            handle,
            crate::runtime::rebind::UiRebindExecutionPolicy::ordinary(),
            crate::runtime::rebind::UiRebindExecutionRequest::new(now_tick),
        )
    }
}
