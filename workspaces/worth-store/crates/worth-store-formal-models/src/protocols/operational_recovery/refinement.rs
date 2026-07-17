use std::collections::BTreeSet;

use worth_store_operations::OperationalControlRecord;

use super::{
    map_operational_control_record, OperationalRecoveryActionKind,
    OperationalRecoveryControlledDefect, OperationalRecoveryCounterexample,
    OperationalRecoveryModel,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryRefinementReceipt {
    concrete_transition_count: u64,
    reached_model_transitions: BTreeSet<OperationalRecoveryActionKind>,
    operation_identities: BTreeSet<String>,
    refinement_identity: [u8; 32],
}

impl OperationalRecoveryRefinementReceipt {
    pub const fn concrete_transition_count(&self) -> u64 {
        self.concrete_transition_count
    }
    pub fn reached_model_transitions(&self) -> &BTreeSet<OperationalRecoveryActionKind> {
        &self.reached_model_transitions
    }
    pub fn operation_identities(&self) -> &BTreeSet<String> {
        &self.operation_identities
    }
    pub const fn refinement_identity(&self) -> [u8; 32] {
        self.refinement_identity
    }
}

pub fn check_operational_recovery_refinement(
    records: &[OperationalControlRecord],
    controlled_defect: Option<OperationalRecoveryControlledDefect>,
) -> Result<OperationalRecoveryRefinementReceipt, OperationalRecoveryCounterexample> {
    let mut model = OperationalRecoveryModel::default();
    let mut operation_identities = BTreeSet::new();
    let mut digest = sha2::Sha256::new();
    use sha2::Digest;
    digest.update(b"worth-store-operational-recovery-refinement-v1");
    for record in records {
        let action = map_operational_control_record(record);
        operation_identities.insert(action.operation_identity().to_owned());
        digest.update(action.evidence_identity());
        model.apply(&action, controlled_defect)?;
    }
    Ok(OperationalRecoveryRefinementReceipt {
        concrete_transition_count: records.len() as u64,
        reached_model_transitions: model.reached_transitions().clone(),
        operation_identities,
        refinement_identity: digest.finalize().into(),
    })
}
