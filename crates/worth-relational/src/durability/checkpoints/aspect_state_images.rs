use worth_foundational::facade::{
    export_portable_record_aspect_state, readmit_portable_record_aspect_state, AspectContract,
    AspectKey, AuthoritativeRecordAspectState, PortableAspectContract,
    PortableAspectContractLookup, PortableRecordAspectState,
};
use worth_proof::TransitionOutcome;

use crate::durability::data::{DurabilityError, RecoveryFailureClass};
use crate::schema::data::LoweredAspectContractPlan;

pub(super) fn export_state(
    state: Option<AuthoritativeRecordAspectState>,
    plan: Option<&LoweredAspectContractPlan>,
) -> Result<Option<PortableRecordAspectState>, DurabilityError> {
    state
        .map(|state| {
            let plan = plan.ok_or_else(|| {
                DurabilityError::new(
                    RecoveryFailureClass::SchemaMismatch,
                    "checkpoint aspect state belongs to a slot without a kind plan",
                )
            })?;
            export_portable_record_aspect_state(&state, plan).map_err(|denial| {
                DurabilityError::new(
                    RecoveryFailureClass::SchemaMismatch,
                    format!("checkpoint aspect export denied: {denial:?}"),
                )
            })
        })
        .transpose()
}

pub(super) fn readmit_state(
    state: Option<PortableRecordAspectState>,
    contracts: &CheckpointAspectContractCatalog,
) -> Result<Option<AuthoritativeRecordAspectState>, DurabilityError> {
    state
        .map(
            |state| match readmit_portable_record_aspect_state(state, contracts) {
                TransitionOutcome::Success(artifact) => Ok(artifact.into_parts().into_parts().0),
                TransitionOutcome::Denied(denial) => Err(DurabilityError::new(
                    RecoveryFailureClass::CorruptCheckpoint,
                    format!("checkpoint aspect readmission denied: {denial:?}"),
                )),
            },
        )
        .transpose()
}

pub(crate) struct CheckpointAspectContractCatalog {
    contracts: std::collections::BTreeMap<AspectKey, AspectContract>,
}

impl CheckpointAspectContractCatalog {
    pub(crate) fn readmit(candidates: &[PortableAspectContract]) -> Result<Self, DurabilityError> {
        let mut contracts = std::collections::BTreeMap::new();
        for candidate in candidates {
            let contract = candidate.readmit().map_err(|denial| {
                DurabilityError::new(
                    RecoveryFailureClass::CorruptCheckpoint,
                    format!("checkpoint aspect contract readmission denied: {denial:?}"),
                )
            })?;
            match contracts.get(contract.key()) {
                Some(existing) if existing == &contract => continue,
                Some(_) => {
                    return Err(DurabilityError::new(
                        RecoveryFailureClass::CorruptCheckpoint,
                        format!(
                            "checkpoint carries conflicting contracts for aspect `{}`",
                            contract.key().as_str()
                        ),
                    ));
                }
                None => {
                    contracts.insert(contract.key().clone(), contract);
                }
            }
        }
        Ok(Self { contracts })
    }
}

impl PortableAspectContractLookup for CheckpointAspectContractCatalog {
    fn contract_for(&self, key: &AspectKey) -> Option<AspectContract> {
        self.contracts.get(key).cloned()
    }
}
