use super::signal_decision_reentry::{
    WorthQueryRetainedConditionalDecision, WorthQueryRetainedConditionalWake,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalSignalDecision {
    Eligible,
    DependencyUnchanged,
    RevertedClean,
    Suppressed,
    Deferred,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConditionalExecutionTerminal {
    EligibleRetained,
    SuppressedRetained,
    DeferredRetained,
    Retryable,
    Indeterminate,
    Committed,
    AlreadyCommitted,
    Failed,
}

/// Descriptive, non-authorizing lineage for one temporal wake processed by an
/// observed clock reading.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalExecutionProvenance {
    intent_identity: String,
    intent_revision: u64,
    due_coordinate: u64,
    signal_scheduled_ordinal: u64,
    signal_ready_ordinal: u64,
    signal_decision: Option<WorthQueryConditionalSignalDecision>,
    application_attempt_ordinal: Option<u64>,
    terminal: WorthQueryConditionalExecutionTerminal,
    canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases,
}

impl WorthQueryConditionalExecutionProvenance {
    pub fn intent_identity(&self) -> &str {
        &self.intent_identity
    }
    pub fn intent_revision(&self) -> u64 {
        self.intent_revision
    }
    pub fn due_coordinate(&self) -> u64 {
        self.due_coordinate
    }
    pub fn signal_scheduled_ordinal(&self) -> u64 {
        self.signal_scheduled_ordinal
    }
    pub fn signal_ready_ordinal(&self) -> u64 {
        self.signal_ready_ordinal
    }
    pub fn signal_decision(&self) -> Option<WorthQueryConditionalSignalDecision> {
        self.signal_decision
    }
    pub fn application_attempt_ordinal(&self) -> Option<u64> {
        self.application_attempt_ordinal
    }
    pub fn terminal(&self) -> WorthQueryConditionalExecutionTerminal {
        self.terminal
    }

    pub const fn canonical_work(
        &self,
    ) -> worth_query_installation::facade::WorthQueryCanonicalWorkPhases {
        self.canonical_work
    }
}

pub(super) fn execution_provenance(
    wakes: &[WorthQueryRetainedConditionalWake],
) -> Vec<WorthQueryConditionalExecutionProvenance> {
    wakes
        .iter()
        .map(|wake| WorthQueryConditionalExecutionProvenance {
            intent_identity: wake.due.intent_identity().as_str().to_string(),
            intent_revision: wake.due.revision(),
            due_coordinate: wake.due.due_coordinate(),
            signal_scheduled_ordinal: wake.due.signal_scheduled_ordinal(),
            signal_ready_ordinal: wake.due.signal_ready_ordinal(),
            signal_decision: wake.last_signal_decision,
            application_attempt_ordinal: wake.application_attempted.then_some(wake.attempt),
            terminal: terminal(&wake.decision),
            canonical_work: worth_query_installation::facade::WorthQueryCanonicalWorkPhases::new(
                worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
                wake.application_admission_canonical_work,
                worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
                worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
                worth_query_installation::facade::WorthQueryCanonicalWorkEvidence::zero(),
            ),
        })
        .collect()
}

fn terminal(
    decision: &WorthQueryRetainedConditionalDecision,
) -> WorthQueryConditionalExecutionTerminal {
    match decision {
        WorthQueryRetainedConditionalDecision::Eligible(_) => {
            WorthQueryConditionalExecutionTerminal::EligibleRetained
        }
        WorthQueryRetainedConditionalDecision::Suppressed(_) => {
            WorthQueryConditionalExecutionTerminal::SuppressedRetained
        }
        WorthQueryRetainedConditionalDecision::Deferred(_) => {
            WorthQueryConditionalExecutionTerminal::DeferredRetained
        }
        WorthQueryRetainedConditionalDecision::OperationRetryable(_, _) => {
            WorthQueryConditionalExecutionTerminal::Retryable
        }
        WorthQueryRetainedConditionalDecision::OperationIndeterminate(_, _) => {
            WorthQueryConditionalExecutionTerminal::Indeterminate
        }
        WorthQueryRetainedConditionalDecision::OperationCommitted => {
            WorthQueryConditionalExecutionTerminal::Committed
        }
        WorthQueryRetainedConditionalDecision::OperationAlreadyCommitted => {
            WorthQueryConditionalExecutionTerminal::AlreadyCommitted
        }
        WorthQueryRetainedConditionalDecision::Failed(_) => {
            WorthQueryConditionalExecutionTerminal::Failed
        }
    }
}

pub(super) fn signal_decision(
    class: worth_signal::facade::SignalConditionalDecisionClass,
) -> WorthQueryConditionalSignalDecision {
    use worth_signal::facade::SignalConditionalDecisionClass as Class;
    match class {
        Class::ComputedChanged => WorthQueryConditionalSignalDecision::Eligible,
        Class::DependencyUnchanged => WorthQueryConditionalSignalDecision::DependencyUnchanged,
        Class::ComputedRevertedClean => WorthQueryConditionalSignalDecision::RevertedClean,
        Class::SuppressedBeforeCompute => WorthQueryConditionalSignalDecision::Suppressed,
        Class::DeferredByCondition | Class::DeferredTemporal | Class::DeferredOnDemand => {
            WorthQueryConditionalSignalDecision::Deferred
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_signal::facade::SignalConditionalDecisionClass as Class;

    #[test]
    fn query_provenance_preserves_every_signal_decision_class() {
        for (class, expected) in [
            (
                Class::ComputedChanged,
                WorthQueryConditionalSignalDecision::Eligible,
            ),
            (
                Class::DependencyUnchanged,
                WorthQueryConditionalSignalDecision::DependencyUnchanged,
            ),
            (
                Class::ComputedRevertedClean,
                WorthQueryConditionalSignalDecision::RevertedClean,
            ),
            (
                Class::SuppressedBeforeCompute,
                WorthQueryConditionalSignalDecision::Suppressed,
            ),
            (
                Class::DeferredByCondition,
                WorthQueryConditionalSignalDecision::Deferred,
            ),
            (
                Class::DeferredTemporal,
                WorthQueryConditionalSignalDecision::Deferred,
            ),
            (
                Class::DeferredOnDemand,
                WorthQueryConditionalSignalDecision::Deferred,
            ),
        ] {
            assert_eq!(signal_decision(class), expected);
        }
    }
}
