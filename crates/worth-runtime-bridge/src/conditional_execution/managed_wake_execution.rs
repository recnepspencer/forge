use std::sync::Arc;

use super::{
    BridgeConditionalDecisionEvidence, BridgeConditionalDenial, BridgeConditionalDenialKind,
    BridgeConditionalExecutionRequest, BridgeInstalledConditionalLowering, BridgeManagedDueWake,
    BridgeOwnedSignalRuntime,
};

/// Exact managed-wake input for one Bridge-owned conditional evaluation.
/// The Signal wake identity remains private to Bridge.
pub struct BridgeManagedConditionalExecutionRequest<'a> {
    pub due_wake: &'a BridgeManagedDueWake,
    pub lowering: &'a Arc<BridgeInstalledConditionalLowering>,
    pub query_binding_identity: &'a str,
    pub query_capability_identity: u64,
    pub snapshot_identity: &'a str,
    pub truth_branch_identity: Option<&'a str>,
    pub bridge_snapshot_identity: Option<&'a crate::snapshot::TruthSnapshotIdentity>,
    pub triggering_correspondence:
        Option<&'a crate::correspondence::BridgeCorrespondenceDeliveryReceipt>,
    pub attempt: u64,
}

impl BridgeOwnedSignalRuntime {
    pub fn execute_managed_due_wake(
        &mut self,
        request: BridgeManagedConditionalExecutionRequest<'_>,
        compute_context: &mut dyn std::any::Any,
    ) -> Result<BridgeConditionalDecisionEvidence, BridgeConditionalDenial> {
        self.validate_managed_due_wake(&request)?;
        self.validate_triggering_correspondence(&request)?;
        let execution_identity = format!(
            "managed-wake:{}:revision={}:signal={}:scheduled={}:ready={}",
            request.due_wake.intent_identity().as_str(),
            request.due_wake.revision(),
            request.due_wake.signal_wake_id.get(),
            request.due_wake.signal_scheduled_ordinal(),
            request.due_wake.signal_ready_ordinal(),
        );
        let mut evidence = self.execute_with_managed_source_record(
            BridgeConditionalExecutionRequest {
                lowering: request.lowering,
                query_binding_identity: request.query_binding_identity,
                query_capability_identity: request.query_capability_identity,
                snapshot_identity: request.snapshot_identity,
                truth_branch_identity: request.truth_branch_identity,
                bridge_snapshot_identity: request.bridge_snapshot_identity,
                execution_identity: &execution_identity,
                attempt: request.attempt,
            },
            Some(request.due_wake.source_record_identity()),
            compute_context,
        )?;
        if let Some(trigger) = request.triggering_correspondence {
            std::sync::Arc::get_mut(&mut evidence.core)
                .expect("new managed decision core is uniquely owned")
                .triggering_change_set = Some(trigger.change_set().clone());
        }
        Ok(evidence)
    }

    fn validate_managed_due_wake(
        &self,
        request: &BridgeManagedConditionalExecutionRequest<'_>,
    ) -> Result<(), BridgeConditionalDenial> {
        let retained = self
            .managed_clock_lanes
            .get(request.due_wake.binding_identity())
            .is_some_and(|lane| lane.retains_due_wake(request.lowering, request.due_wake));
        if retained {
            Ok(())
        } else {
            Err(BridgeConditionalDenial::new(
                BridgeConditionalDenialKind::ManagedWakeMismatch,
                "managed due wake lost its exact clock, intent, or conditional lowering affinity",
            ))
        }
    }

    fn validate_triggering_correspondence(
        &self,
        request: &BridgeManagedConditionalExecutionRequest<'_>,
    ) -> Result<(), BridgeConditionalDenial> {
        let Some(trigger) = request.triggering_correspondence else {
            return Ok(());
        };
        let change_set = trigger.change_set();
        let dependency = change_set.dependency();
        let installed = request
            .lowering
            .correspondences
            .iter()
            .any(|correspondence| correspondence.dependency() == dependency);
        let record = request.due_wake.source_record_identity();
        let touches_wake = change_set
            .changes()
            .iter()
            .any(|change| change.relational_record_identity() == Some(record));
        if installed && touches_wake {
            return Ok(());
        }
        Err(BridgeConditionalDenial::new(
            BridgeConditionalDenialKind::ManagedWakeMismatch,
            "triggering correspondence does not match the installed dependency and due record",
        ))
    }
}
