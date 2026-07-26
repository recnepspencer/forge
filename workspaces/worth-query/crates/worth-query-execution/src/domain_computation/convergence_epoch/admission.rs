use std::sync::Arc;
use std::{panic::catch_unwind, panic::AssertUnwindSafe};

use worth_query_admission::facade::domain_computation::WorthQueryAdmittedConvergenceContract;
use worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority;

use super::core::WorthQueryConvergenceEpochCore;
use super::{
    WorthQueryAdmittedDirectConvergenceEpoch, WorthQueryAdmittedWorkflowConvergenceEpoch,
    WorthQueryConvergenceEpochCounters, WorthQueryConvergenceEpochDenial,
    WorthQueryConvergenceEpochDenialKind as Kind,
};
use crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor;
use crate::domain_computation::{
    WorthQueryAdmittedDirectRun, WorthQueryAdmittedWorkflowRun,
    WorthQueryConvergenceDomainProvider, WorthQueryExecutionBoundOperationAuthority,
    WorthQueryExecutionRuntime,
};

pub struct WorthQueryDirectConvergenceAdmissionRejection {
    denial: WorthQueryConvergenceEpochDenial,
    contract: WorthQueryAdmittedConvergenceContract,
    managed_run: WorthQueryAdmittedDirectRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
}

impl WorthQueryDirectConvergenceAdmissionRejection {
    pub fn denial(&self) -> &WorthQueryConvergenceEpochDenial {
        &self.denial
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryAdmittedConvergenceContract,
        WorthQueryAdmittedDirectRun,
        WorthQueryInstalledGraphParticipationAuthority,
    ) {
        (self.contract, self.managed_run, self.graph)
    }
}

pub struct WorthQueryWorkflowConvergenceAdmissionRejection {
    denial: WorthQueryConvergenceEpochDenial,
    contract: WorthQueryAdmittedConvergenceContract,
    managed_run: WorthQueryAdmittedWorkflowRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
}

impl WorthQueryWorkflowConvergenceAdmissionRejection {
    pub fn denial(&self) -> &WorthQueryConvergenceEpochDenial {
        &self.denial
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryAdmittedConvergenceContract,
        WorthQueryAdmittedWorkflowRun,
        WorthQueryInstalledGraphParticipationAuthority,
    ) {
        (self.contract, self.managed_run, self.graph)
    }
}

impl WorthQueryExecutionRuntime {
    pub fn admit_direct_convergence_epoch(
        &self,
        operation: &WorthQueryExecutionBoundOperationAuthority,
        contract: WorthQueryAdmittedConvergenceContract,
        managed_run: WorthQueryAdmittedDirectRun,
        graph: WorthQueryInstalledGraphParticipationAuthority,
    ) -> Result<
        WorthQueryAdmittedDirectConvergenceEpoch,
        WorthQueryDirectConvergenceAdmissionRejection,
    > {
        let mut counters = WorthQueryConvergenceEpochCounters::default();
        counters.checked_operation_authority();
        if !operation.belongs_to(self) {
            return Err(direct_rejection(
                Kind::ForeignQueryRuntime,
                "convergence operation belongs to another Query runtime",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        if !operation.belongs_to_current_installation(self) {
            return Err(direct_rejection(
                Kind::StaleInstallationGeneration,
                "convergence operation belongs to a stale installation generation",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        counters.checked_contract_authority();
        if !operation.admits_convergence_contract(&contract) {
            return Err(direct_rejection(
                Kind::ContractOperationMismatch,
                "admitted convergence contract does not belong to the bound operation",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        counters.checked_managed_run_authority();
        if !managed_run.belongs_to_operation(operation) {
            return Err(direct_rejection(
                Kind::ManagedRunOperationMismatch,
                "managed direct run does not belong to the bound convergence operation",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        counters.checked_graph_authority();
        let provider = match bind_convergence_provider(operation, &contract, graph) {
            Ok(provider) => provider,
            Err((kind, detail, graph)) => {
                return Err(direct_rejection(
                    kind,
                    detail,
                    counters,
                    contract,
                    managed_run,
                    graph,
                ))
            }
        };
        let core = WorthQueryConvergenceEpochCore::new(
            operation.binding_identity(),
            managed_run.logical_run_identity(),
            managed_run.identity(),
            provider.graph.authority_identity(),
            contract,
            counters,
        );
        Ok(WorthQueryAdmittedDirectConvergenceEpoch::new(
            core,
            managed_run,
            provider.graph,
            provider.provider,
        ))
    }

    pub fn admit_workflow_convergence_epoch(
        &self,
        operation: &WorthQueryExecutionBoundOperationAuthority,
        contract: WorthQueryAdmittedConvergenceContract,
        managed_run: WorthQueryAdmittedWorkflowRun,
        graph: WorthQueryInstalledGraphParticipationAuthority,
    ) -> Result<
        WorthQueryAdmittedWorkflowConvergenceEpoch,
        WorthQueryWorkflowConvergenceAdmissionRejection,
    > {
        let mut counters = WorthQueryConvergenceEpochCounters::default();
        counters.checked_operation_authority();
        if !operation.belongs_to(self) {
            return Err(workflow_rejection(
                Kind::ForeignQueryRuntime,
                "convergence operation belongs to another Query runtime",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        if !operation.belongs_to_current_installation(self) {
            return Err(workflow_rejection(
                Kind::StaleInstallationGeneration,
                "convergence operation belongs to a stale installation generation",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        counters.checked_contract_authority();
        if !operation.admits_convergence_contract(&contract) {
            return Err(workflow_rejection(
                Kind::ContractOperationMismatch,
                "admitted convergence contract does not belong to the bound operation",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        counters.checked_managed_run_authority();
        if !managed_run.belongs_to_operation(operation) {
            return Err(workflow_rejection(
                Kind::ManagedRunOperationMismatch,
                "managed workflow run does not belong to the bound convergence operation",
                counters,
                contract,
                managed_run,
                graph,
            ));
        }
        counters.checked_graph_authority();
        let provider = match bind_convergence_provider(operation, &contract, graph) {
            Ok(provider) => provider,
            Err((kind, detail, graph)) => {
                return Err(workflow_rejection(
                    kind,
                    detail,
                    counters,
                    contract,
                    managed_run,
                    graph,
                ))
            }
        };
        let core = WorthQueryConvergenceEpochCore::new(
            operation.binding_identity(),
            managed_run.logical_run_identity(),
            managed_run.identity(),
            provider.graph.authority_identity(),
            contract,
            counters,
        );
        Ok(WorthQueryAdmittedWorkflowConvergenceEpoch::new(
            core,
            managed_run,
            provider.graph,
            provider.provider,
        ))
    }
}

fn direct_rejection(
    kind: Kind,
    detail: &'static str,
    counters: WorthQueryConvergenceEpochCounters,
    contract: WorthQueryAdmittedConvergenceContract,
    managed_run: WorthQueryAdmittedDirectRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
) -> WorthQueryDirectConvergenceAdmissionRejection {
    WorthQueryDirectConvergenceAdmissionRejection {
        denial: WorthQueryConvergenceEpochDenial::new(kind, detail, counters),
        contract,
        managed_run,
        graph,
    }
}

fn workflow_rejection(
    kind: Kind,
    detail: &'static str,
    counters: WorthQueryConvergenceEpochCounters,
    contract: WorthQueryAdmittedConvergenceContract,
    managed_run: WorthQueryAdmittedWorkflowRun,
    graph: WorthQueryInstalledGraphParticipationAuthority,
) -> WorthQueryWorkflowConvergenceAdmissionRejection {
    WorthQueryWorkflowConvergenceAdmissionRejection {
        denial: WorthQueryConvergenceEpochDenial::new(kind, detail, counters),
        contract,
        managed_run,
        graph,
    }
}

struct WorthQueryBoundConvergenceProvider {
    graph: WorthQueryInstalledGraphParticipationAuthority,
    provider: Arc<dyn WorthQueryConvergenceDomainProvider>,
}

fn bind_convergence_provider(
    operation: &WorthQueryExecutionBoundOperationAuthority,
    contract: &WorthQueryAdmittedConvergenceContract,
    graph: WorthQueryInstalledGraphParticipationAuthority,
) -> Result<
    WorthQueryBoundConvergenceProvider,
    (
        Kind,
        &'static str,
        WorthQueryInstalledGraphParticipationAuthority,
    ),
> {
    if !operation.admits_convergence_graph(contract, &graph) {
        return Err((
            Kind::GraphOperationMismatch,
            "installed convergence graph does not belong to the operation evidence topology",
            graph,
        ));
    }
    let Some(anchor) = graph.retain_provider_anchor::<WorthQueryGraphProviderAnchor>() else {
        return Err((
            Kind::MissingConvergenceProvider,
            "installed graph authority does not retain the execution provider anchor",
            graph,
        ));
    };
    let Some(provider) = anchor.convergence_provider() else {
        return Err((
            Kind::MissingConvergenceProvider,
            "installed graph provider does not supply convergence semantics",
            graph,
        ));
    };
    let families_match = match catch_unwind(AssertUnwindSafe(|| {
        provider.convergence_families().matches(contract)
    })) {
        Ok(matches) => matches,
        Err(_) => {
            return Err((
                Kind::ConvergenceProviderFamilyInspectionPanicked,
                "installed graph provider panicked while exposing convergence families",
                graph,
            ))
        }
    };
    if !families_match {
        return Err((
            Kind::ConvergenceProviderFamilyMismatch,
            "installed graph provider semantic families do not match the convergence contract",
            graph,
        ));
    }
    Ok(WorthQueryBoundConvergenceProvider { graph, provider })
}
