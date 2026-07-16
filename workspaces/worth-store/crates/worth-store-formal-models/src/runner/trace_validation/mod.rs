mod compaction;
mod durability;
mod import;
mod lease;
mod quarantine;
mod replication;
mod source;

use crate::{ProtocolFamily, SharedFrontierDenial, SharedFrontierModel};

use super::{CanonicalProtocolAction, CanonicalProtocolTrace};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProtocolTraceValidationReceipt {
    protocol: ProtocolFamily,
    action_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolTraceValidationDenial {
    Durability {
        action_index: usize,
        denial: crate::DurabilityRecoveryDenial,
    },
    SourcePrecedence {
        action_index: usize,
        denial: source::SourceTraceDenial,
    },
    CompactionVisibility {
        action_index: usize,
        denial: compaction::CompactionTraceDenial,
    },
    LeaseReclaim {
        action_index: usize,
        denial: lease::LeaseTraceDenial,
    },
    QuarantineReadmission {
        action_index: usize,
        denial: quarantine::QuarantineTraceDenial,
    },
    ImportPublication {
        action_index: usize,
        denial: import::ImportTraceDenial,
    },
    ReplicationAdmission {
        action_index: usize,
        denial: replication::ReplicationTraceDenial,
    },
    SharedFrontiers {
        action_index: usize,
        denial: SharedFrontierDenial,
    },
    ActionFamilyMismatch {
        action_index: usize,
    },
}

pub fn validate_canonical_protocol_trace(
    trace: &CanonicalProtocolTrace,
) -> Result<ProtocolTraceValidationReceipt, ProtocolTraceValidationDenial> {
    match trace.protocol() {
        ProtocolFamily::DurabilityRecovery => durability::validate(trace.actions())?,
        ProtocolFamily::RecoverySourcePrecedence => source::validate(trace.actions())?,
        ProtocolFamily::CompactionVisibility => compaction::validate(trace.actions())?,
        ProtocolFamily::LeaseReclaim => lease::validate(trace.actions())?,
        ProtocolFamily::QuarantineReadmission => quarantine::validate(trace.actions())?,
        ProtocolFamily::ImportPublication => import::validate(trace.actions())?,
        ProtocolFamily::ReplicationAdmission => replication::validate(trace.actions())?,
        ProtocolFamily::SharedFrontiers => validate_shared_frontier(trace.actions())?,
    }
    Ok(ProtocolTraceValidationReceipt {
        protocol: trace.protocol(),
        action_count: trace.actions().len(),
    })
}

fn validate_shared_frontier(
    actions: &[CanonicalProtocolAction],
) -> Result<(), ProtocolTraceValidationDenial> {
    let mut model = SharedFrontierModel::initial();
    for (action_index, action) in actions.iter().copied().enumerate() {
        let CanonicalProtocolAction::SharedFrontier(action) = action else {
            return Err(ProtocolTraceValidationDenial::ActionFamilyMismatch { action_index });
        };
        model
            .apply(action)
            .map_err(|denial| ProtocolTraceValidationDenial::SharedFrontiers {
                action_index,
                denial,
            })?;
    }
    Ok(())
}

impl ProtocolTraceValidationReceipt {
    pub const fn protocol(self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn action_count(self) -> usize {
        self.action_count
    }
}
