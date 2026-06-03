use super::*;
use crate::harness::fixtures::BridgeHarnessFixture;
use crate::merge::MergeHistoryDeclarationIdentity;

mod certification_bundle;
mod counter_snapshot;
mod diagnostics_digest;
pub(in crate::harness::adapter::adapter_impl) mod terminal_report_export;

use certification_bundle::MergeHarnessCertificationBundle;

#[cfg(test)]
mod typed_certification_tests;

pub(super) enum MergeHarnessTarget {
    Execute {
        declaration_identity: MergeHistoryDeclarationIdentity,
    },
    Replay {
        declaration_identity: MergeHistoryDeclarationIdentity,
    },
}

pub(super) enum MergeHarnessExecution {
    Execute {
        certification_bundle: MergeHarnessCertificationBundle,
    },
    Replay {
        certification_bundle: MergeHarnessCertificationBundle,
    },
}

impl MergeHarnessExecution {
    fn certification_bundle(&self) -> &MergeHarnessCertificationBundle {
        match self {
            Self::Execute {
                certification_bundle,
            }
            | Self::Replay {
                certification_bundle,
            } => certification_bundle,
        }
    }
}

pub(super) fn execute_merge_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    target: MergeHarnessTarget,
) -> Result<MergeHarnessExecution, BridgeHarnessError> {
    match target {
        MergeHarnessTarget::Execute {
            declaration_identity,
        } => execute_merge_bundle(runtime_bridge, fixture, &declaration_identity),
        MergeHarnessTarget::Replay {
            declaration_identity,
        } => execute_merge_replay(runtime_bridge, fixture, &declaration_identity),
    }
}

fn execute_merge_bundle(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &MergeHistoryDeclarationIdentity,
) -> Result<MergeHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let bundle = runtime_bridge
        .replay_merge_history(&contract)
        .map_err(|error| BridgeHarnessError::new(format!("bridge merge replay failed: {error}")))?;
    let record = runtime_bridge.canonicalize_merge_record(&bundle);
    let explanation = runtime_bridge.diagnostics().explain_merge_record(&record);

    Ok(MergeHarnessExecution::Execute {
        certification_bundle: MergeHarnessCertificationBundle::from_execution(
            contract,
            bundle,
            record,
            explanation,
            None,
        ),
    })
}

fn execute_merge_replay(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &MergeHistoryDeclarationIdentity,
) -> Result<MergeHarnessExecution, BridgeHarnessError> {
    let contract = admitted_contract(runtime_bridge, fixture, declaration_identity)?;
    let bundle = runtime_bridge
        .replay_merge_history(&contract)
        .map_err(|error| BridgeHarnessError::new(format!("bridge merge replay failed: {error}")))?;
    let record = runtime_bridge.canonicalize_merge_record(&bundle);
    let explanation = runtime_bridge.diagnostics().explain_merge_record(&record);
    let replayed = runtime_bridge
        .replay_canonical_merge_record(&record)
        .map_err(|error| BridgeHarnessError::new(format!("bridge merge replay failed: {error}")))?;

    Ok(MergeHarnessExecution::Replay {
        certification_bundle: MergeHarnessCertificationBundle::from_execution(
            contract,
            bundle,
            record,
            explanation,
            Some(replayed),
        ),
    })
}

fn admitted_contract(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    declaration_identity: &MergeHistoryDeclarationIdentity,
) -> Result<crate::facade::AdmittedMergeHistoryContract, BridgeHarnessError> {
    let declaration = fixture
        .merge_declarations()
        .iter()
        .find(|declaration| declaration.declaration_identity() == declaration_identity)
        .cloned()
        .ok_or_else(|| {
            BridgeHarnessError::new(format!(
                "bridge merge fixture does not declare `{}`",
                declaration_identity.as_str()
            ))
        })?;
    runtime_bridge
        .admit_merge_history(declaration)
        .map_err(|error| BridgeHarnessError::new(format!("bridge merge admission failed: {error}")))
}

fn declaration_identity(
    contract: &crate::facade::AdmittedMergeHistoryContract,
) -> &crate::merge::MergeHistoryDeclarationIdentity {
    contract
        .validated_declaration()
        .declaration()
        .declaration_identity()
}
