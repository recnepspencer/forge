use std::collections::BTreeSet;

use crate::{OperationalOperationId, OperationalTransitionId};

use super::OperationalAuditRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedAuditTransitionSet {
    operation_id: OperationalOperationId,
    transitions: BTreeSet<String>,
    terminal_transition: String,
}

impl ExpectedAuditTransitionSet {
    pub fn new(
        operation_id: OperationalOperationId,
        transitions: impl IntoIterator<Item = OperationalTransitionId>,
        terminal_transition: OperationalTransitionId,
    ) -> Result<Self, AuditCompletenessDenial> {
        let transitions = transitions
            .into_iter()
            .map(|transition| transition.as_str().to_owned())
            .collect::<BTreeSet<_>>();
        if transitions.is_empty() || !transitions.contains(terminal_transition.as_str()) {
            return Err(AuditCompletenessDenial::InvalidExpectedTransitionSet);
        }
        Ok(Self {
            operation_id,
            transitions,
            terminal_transition: terminal_transition.as_str().to_owned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditCompletenessDenial {
    InvalidExpectedTransitionSet,
    MissingTransition(String),
    UnexpectedTransition(String),
    TerminalTransitionMissing,
    CausalParentMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditCompletenessReceipt {
    operation_id: OperationalOperationId,
    transition_count: u64,
    terminal_record_identity: [u8; 32],
}

impl AuditCompletenessReceipt {
    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }

    pub const fn transition_count(&self) -> u64 {
        self.transition_count
    }
}

impl ExpectedAuditTransitionSet {
    pub fn verify(
        self,
        records: &[OperationalAuditRecord],
    ) -> Result<AuditCompletenessReceipt, AuditCompletenessDenial> {
        let operation_records = records
            .iter()
            .filter(|record| record.operation_id() == &self.operation_id)
            .collect::<Vec<_>>();
        let observed = operation_records
            .iter()
            .map(|record| record.transition_id().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        if let Some(missing) = self.transitions.difference(&observed).next() {
            return Err(AuditCompletenessDenial::MissingTransition(missing.clone()));
        }
        if let Some(unexpected) = observed.difference(&self.transitions).next() {
            return Err(AuditCompletenessDenial::UnexpectedTransition(unexpected.clone()));
        }
        for pair in operation_records.windows(2) {
            if pair[1].causal_parent().map(|parent| parent.record_identity())
                != Some(pair[0].record_identity())
            {
                return Err(AuditCompletenessDenial::CausalParentMismatch);
            }
        }
        let terminal = operation_records
            .iter()
            .find(|record| record.transition_id().as_str() == self.terminal_transition)
            .ok_or(AuditCompletenessDenial::TerminalTransitionMissing)?;
        Ok(AuditCompletenessReceipt {
            operation_id: self.operation_id,
            transition_count: operation_records.len() as u64,
            terminal_record_identity: terminal.record_identity(),
        })
    }
}
