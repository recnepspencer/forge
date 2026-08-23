use std::{collections::BTreeMap, sync::Arc};

use worth_query_installation::facade::WorthQueryTemporalIntentLifecycle;
use worth_runtime_bridge::facade::{
    BridgeManagedClockBinding, BridgeManagedTemporalIntentIdentity,
    BridgeManagedTemporalIntentLifecycle, BridgeManagedTemporalIntentReconciliation,
    BridgeManagedTemporalIntentReconciliationParts, BridgeOwnedSignalRuntime,
};

use super::{
    bridge_reconstruction_denial, WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryReconstructedTemporalIntent,
};

pub(in crate::domain_computation::primary_graph::conditional_operation) fn reconcile_refreshed_temporal_intents<
    Clock,
    Input,
>(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    previous: &BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
    refreshed: &mut BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
    for (identity, reconstructed) in refreshed.iter() {
        let candidate = reconstructed.candidate();
        let outcome = reconcile_candidate(bridge, clock, reconstructed)?;
        let expected = match candidate.lifecycle() {
            WorthQueryTemporalIntentLifecycle::Active => matches!(
                outcome,
                BridgeManagedTemporalIntentReconciliation::Installed
                    | BridgeManagedTemporalIntentReconciliation::Duplicate
                    | BridgeManagedTemporalIntentReconciliation::Superseded
            ),
            WorthQueryTemporalIntentLifecycle::Cancelled
            | WorthQueryTemporalIntentLifecycle::Completed => matches!(
                outcome,
                BridgeManagedTemporalIntentReconciliation::Retired
                    | BridgeManagedTemporalIntentReconciliation::TerminalNoop
            ),
        };
        if !expected {
            return Err(bridge_reconstruction_denial(format!(
                "current temporal intent `{identity}` could not reconcile its authoritative revision"
            )));
        }
    }
    retire_removed_intents(bridge, clock, previous, refreshed)?;
    refreshed.retain(|_, intent| {
        intent.candidate().lifecycle() == WorthQueryTemporalIntentLifecycle::Active
    });
    Ok(())
}

fn retire_removed_intents<Clock, Input>(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    previous: &BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
    refreshed: &BTreeMap<String, WorthQueryReconstructedTemporalIntent<Clock, Input>>,
) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
    for (identity, old) in previous
        .iter()
        .filter(|(identity, _)| !refreshed.contains_key(*identity))
    {
        let candidate = old.candidate();
        let revision = candidate
            .revision()
            .checked_add(1)
            .ok_or_else(|| bridge_reconstruction_denial("temporal intent revision overflowed"))?;
        let outcome = bridge
            .reconcile_managed_temporal_intent(BridgeManagedTemporalIntentReconciliationParts {
                binding: clock,
                identity: managed_identity(candidate.identity().as_str())?,
                revision,
                due_coordinate: candidate.due().nanoseconds(),
                idempotency_identity: Arc::from(candidate.idempotency().as_str()),
                source_record_identity: old.source_record(),
                lifecycle: BridgeManagedTemporalIntentLifecycle::Completed,
            })
            .map_err(|denial| bridge_reconstruction_denial(denial.detail()))?;
        if !matches!(
            outcome,
            BridgeManagedTemporalIntentReconciliation::Retired
                | BridgeManagedTemporalIntentReconciliation::TerminalNoop
        ) {
            return Err(bridge_reconstruction_denial(format!(
                "removed temporal intent `{identity}` did not retire its derived wake"
            )));
        }
    }
    Ok(())
}

fn reconcile_candidate<Clock, Input>(
    bridge: &mut BridgeOwnedSignalRuntime,
    clock: &BridgeManagedClockBinding,
    reconstructed: &WorthQueryReconstructedTemporalIntent<Clock, Input>,
) -> Result<BridgeManagedTemporalIntentReconciliation, WorthQueryConditionalRuntimeInstallationDenial>
{
    let candidate = reconstructed.candidate();
    let lifecycle = match candidate.lifecycle() {
        WorthQueryTemporalIntentLifecycle::Active => BridgeManagedTemporalIntentLifecycle::Active,
        WorthQueryTemporalIntentLifecycle::Cancelled => {
            BridgeManagedTemporalIntentLifecycle::Cancelled
        }
        WorthQueryTemporalIntentLifecycle::Completed => {
            BridgeManagedTemporalIntentLifecycle::Completed
        }
    };
    bridge
        .reconcile_managed_temporal_intent(BridgeManagedTemporalIntentReconciliationParts {
            binding: clock,
            identity: managed_identity(candidate.identity().as_str())?,
            revision: candidate.revision(),
            due_coordinate: candidate.due().nanoseconds(),
            idempotency_identity: Arc::from(candidate.idempotency().as_str()),
            source_record_identity: reconstructed.source_record(),
            lifecycle,
        })
        .map_err(|denial| bridge_reconstruction_denial(denial.detail()))
}

fn managed_identity(
    identity: &str,
) -> Result<BridgeManagedTemporalIntentIdentity, WorthQueryConditionalRuntimeInstallationDenial> {
    BridgeManagedTemporalIntentIdentity::declare(Arc::<str>::from(identity))
        .map_err(|denial| bridge_reconstruction_denial(denial.detail()))
}
