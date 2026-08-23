use crate::basis_lifecycle::BasisOperationLane;

use super::{
    evaluate_owned_conditional_node, WorthQueryConditionalEvaluationPass,
    WorthQueryConditionalEvaluationScope, WorthQueryConditionalEvaluationStop,
    WorthQueryConditionalProvenance,
};

pub struct WorthQueryOwnedConditionalExecutionReport {
    provenance: WorthQueryConditionalProvenance,
    counters: crate::domain_installation::WorthQueryOperationExecutionCounters,
    resource_admission_identity: String,
}

#[derive(Debug)]
pub enum WorthQueryOwnedConditionalExecutionDenial {
    Instance(crate::runtime::WorthQueryOwnedConditionalInstanceDenial),
    Bridge {
        kind: worth_runtime_bridge::facade::BridgeConditionalDenialKind,
        detail: String,
    },
    Reentry(super::WorthQueryConditionalAdmissionDenial),
    UnexpectedDeferred,
}

impl WorthQueryOwnedConditionalExecutionReport {
    pub fn provenance(&self) -> &WorthQueryConditionalProvenance {
        &self.provenance
    }

    pub const fn counters(
        &self,
    ) -> crate::domain_installation::WorthQueryOperationExecutionCounters {
        self.counters
    }

    pub fn resource_admission_identity(&self) -> &str {
        &self.resource_admission_identity
    }

    pub fn performed_signal_invalidation(
        &self,
    ) -> Option<&worth_signal::facade::adapters::SignalInvalidationExecutionReceipt> {
        self.provenance
            .bridge_evidence()
            .performed_signal_invalidation()
    }
}

impl<D: 'static, O, F: 'static, L: BasisOperationLane>
    crate::domain_installation::WorthQueryAdmittedDirectOperation<D, O, F, L>
where
    O: crate::domain_installation::WorthQueryExecutableDomainOperation<
        D,
        F,
        Execution = crate::domain_installation::WorthQueryDirectOperation,
    >,
{
    pub fn execute_owned_conditional_instance(
        self,
        instance: &crate::runtime::WorthQueryInstalledOwnedConditionalInstance,
        attempt: std::num::NonZeroU64,
        workspace: &mut crate::runtime::WorthQueryWorkspace,
    ) -> Result<WorthQueryOwnedConditionalExecutionReport, WorthQueryOwnedConditionalExecutionDenial>
    {
        let node = workspace
            .resolve_owned_conditional_instance::<D, O, F>(instance)
            .map_err(WorthQueryOwnedConditionalExecutionDenial::Instance)?;
        let snapshot = workspace.snapshot_identity();
        let execution_identity = format!(
            "owned-conditional:{}:{}",
            instance.instance_identity(),
            self.resource_attempt.evidence().identity()
        );
        let resource_admission_identity = self.resource_attempt.resources().identity().to_owned();
        let mut counters =
            crate::domain_installation::WorthQueryOperationExecutionCounters::default();
        let evaluation = WorthQueryConditionalEvaluationPass {
            workspace,
            snapshot: &snapshot,
            execution_identity: &execution_identity,
            scope: WorthQueryConditionalEvaluationScope::Operation,
            workflow_run_identity: None,
            attempt: attempt.get(),
            resources: self.resource_attempt.resources(),
            resource_evidence: self.resource_attempt.evidence(),
            counters: &mut counters,
        };
        let mut evaluation = evaluation;
        let provenance =
            evaluate_owned_conditional_node(&self.bound, node.as_ref(), &mut evaluation)
                .map_err(map_evaluation_stop)?;
        Ok(WorthQueryOwnedConditionalExecutionReport {
            provenance,
            counters,
            resource_admission_identity,
        })
    }
}

fn map_evaluation_stop(
    stop: WorthQueryConditionalEvaluationStop,
) -> WorthQueryOwnedConditionalExecutionDenial {
    match stop {
        WorthQueryConditionalEvaluationStop::Failed { kind, detail } => {
            WorthQueryOwnedConditionalExecutionDenial::Bridge { kind, detail }
        }
        WorthQueryConditionalEvaluationStop::Reentry(denial) => {
            WorthQueryOwnedConditionalExecutionDenial::Reentry(denial)
        }
        WorthQueryConditionalEvaluationStop::Deferred(_) => {
            WorthQueryOwnedConditionalExecutionDenial::UnexpectedDeferred
        }
    }
}
