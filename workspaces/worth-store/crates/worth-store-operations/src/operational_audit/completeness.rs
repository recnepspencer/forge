use std::collections::BTreeSet;

use crate::{OperationalControlRecord, OperationalOperationId, OperationalTransitionId};

use super::{
    assemble_operational_audit_records, derive_operational_audit_records, OperationalAuditRecord,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpectedAuditTransitionSet {
    operation_id: OperationalOperationId,
    transitions: BTreeSet<String>,
    terminal_transition: String,
}

impl ExpectedAuditTransitionSet {
    pub(crate) fn new(
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

    pub fn from_durable_control_records(
        operation_id: OperationalOperationId,
        durable_records: &[OperationalControlRecord],
    ) -> Result<Self, AuditCompletenessDenial> {
        let audit = derive_operational_audit_records(durable_records)
            .map_err(|_| AuditCompletenessDenial::InvalidDurableControlHistory)?;
        let operation_records = audit
            .iter()
            .filter(|record| record.operation_id() == &operation_id)
            .collect::<Vec<_>>();
        let terminal = operation_records
            .last()
            .ok_or(AuditCompletenessDenial::InvalidExpectedTransitionSet)?
            .transition_id()
            .clone();
        Self::new(
            operation_id,
            operation_records
                .iter()
                .map(|record| record.transition_id().clone()),
            terminal,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditCompletenessDenial {
    InvalidExpectedTransitionSet,
    MissingTransition(String),
    UnexpectedTransition(String),
    TerminalTransitionMissing,
    CausalParentMismatch,
    InvalidDeliverySet,
    InvalidDurableControlHistory,
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

    pub const fn terminal_record_identity(&self) -> [u8; 32] {
        self.terminal_record_identity
    }
}

impl ExpectedAuditTransitionSet {
    pub fn verify(
        self,
        records: &[OperationalAuditRecord],
    ) -> Result<AuditCompletenessReceipt, AuditCompletenessDenial> {
        let delivered_transitions = records
            .iter()
            .filter(|record| record.operation_id() == &self.operation_id)
            .map(|record| record.transition_id().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        if let Some(missing) = self.transitions.difference(&delivered_transitions).next() {
            return Err(AuditCompletenessDenial::MissingTransition(missing.clone()));
        }
        if let Some(unexpected) = delivered_transitions.difference(&self.transitions).next() {
            return Err(AuditCompletenessDenial::UnexpectedTransition(
                unexpected.clone(),
            ));
        }
        let assembled = assemble_operational_audit_records(records.iter().cloned())
            .map_err(|_| AuditCompletenessDenial::InvalidDeliverySet)?;
        let operation_records = assembled
            .iter()
            .filter(|record| record.operation_id() == &self.operation_id)
            .collect::<Vec<_>>();
        let observed = operation_records
            .iter()
            .map(|record| record.transition_id().as_str().to_owned())
            .collect::<BTreeSet<_>>();
        debug_assert_eq!(observed, self.transitions);
        for pair in operation_records.windows(2) {
            if pair[1]
                .causal_parent()
                .map(|parent| parent.record_identity())
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
