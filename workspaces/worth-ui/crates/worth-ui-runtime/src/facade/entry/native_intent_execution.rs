use super::WorthUiNativeApplicationShell;

#[derive(Debug)]
pub enum WorthUiNativeManagedIntentConsequencePublicationDenial {
    ManagedRebindAlreadyInFlight,
    ManagedRebindSessionMismatch,
}

pub enum WorthUiNativeManagedIntentConsequencePublicationOutcome {
    NoConsequences(crate::runtime::intent_execution::UiIntentConsequenceCompletionReceipt),
    Published(crate::runtime::rebind::UiRebindReceipt),
    Pending,
    Stopped(super::native_managed_rebind::WorthUiNativeManagedRebindStop),
}

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

    pub fn begin_managed_native_intent_consequence_publication(
        &mut self,
        handle: crate::facade::intent::UiIntentConsequenceHandle,
        now_tick: u64,
    ) -> Result<
        WorthUiNativeManagedIntentConsequencePublicationOutcome,
        WorthUiNativeManagedIntentConsequencePublicationDenial,
    > {
        if self.pending_managed_rebind.is_some() {
            return Err(
                WorthUiNativeManagedIntentConsequencePublicationDenial::
                    ManagedRebindAlreadyInFlight,
            );
        }
        let outcome = self.publish_native_intent_consequences(handle, now_tick);
        match super::native_managed_rebind::normalize_managed_intent_consequence(outcome) {
            super::native_managed_rebind::ManagedIntentConsequenceNormalization::NoConsequences(
                receipt,
            ) => Ok(
                WorthUiNativeManagedIntentConsequencePublicationOutcome::NoConsequences(receipt),
            ),
            super::native_managed_rebind::ManagedIntentConsequenceNormalization::Published(
                receipt,
            ) => Ok(WorthUiNativeManagedIntentConsequencePublicationOutcome::Published(receipt)),
            super::native_managed_rebind::ManagedIntentConsequenceNormalization::Pending(
                pending,
            ) => {
                if pending.session_identity() != self.session.session_identity() {
                    return Err(
                        WorthUiNativeManagedIntentConsequencePublicationDenial::
                            ManagedRebindSessionMismatch,
                    );
                }
                self.pending_managed_rebind = Some(
                    super::native_managed_rebind::WorthUiNativePendingManagedRebind::
                        IntentConsequence(pending),
                );
                Ok(WorthUiNativeManagedIntentConsequencePublicationOutcome::Pending)
            }
            super::native_managed_rebind::ManagedIntentConsequenceNormalization::Stopped(stop) => {
                Ok(WorthUiNativeManagedIntentConsequencePublicationOutcome::Stopped(stop))
            }
        }
    }
}
