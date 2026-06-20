use crate::query_context::AdmittedQueryBasisContext;
use crate::runtime::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryAuthoritativeMutationObligationDispatch,
    ForgeQueryLiveGraphReadAccessPlan, ForgeQueryReadFamily,
    ForgeQueryRuntimeLiveSubscriptionInstallation,
};

use super::handoff_execution_binding_identity;
use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily, ForgeQueryLiveReadExecutionHandoff,
    ForgeQueryReadExecutionHandoff,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryReadExecutionBinding {
    handoff: ForgeQueryReadExecutionHandoff,
    graph_obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
    graph_read_access_plan: ForgeQueryAdmittedGraphReadAccessPlan,
    binding_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryLiveReadExecutionBinding {
    handoff: ForgeQueryLiveReadExecutionHandoff,
    graph_obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
    live_graph_read_access_plan: ForgeQueryLiveGraphReadAccessPlan,
    binding_digest: String,
}

impl ForgeQueryReadExecutionBinding {
    pub(crate) fn from_handoff(
        handoff: ForgeQueryReadExecutionHandoff,
        graph_obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
        graph_read_access_plan: ForgeQueryAdmittedGraphReadAccessPlan,
    ) -> Self {
        let binding_digest = handoff_execution_binding_identity(
            "read-execution",
            &format!(
                "{}:{}",
                handoff.handoff_digest(),
                graph_read_access_plan.digest()
            ),
        );
        Self {
            handoff,
            graph_obligation_dispatch,
            graph_read_access_plan,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn read_family(&self) -> &ForgeQueryReadFamily {
        self.handoff.read_family()
    }

    pub fn basis_context(&self) -> Option<&AdmittedQueryBasisContext> {
        self.handoff.basis_context()
    }

    pub fn handoff(&self) -> &ForgeQueryReadExecutionHandoff {
        &self.handoff
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.graph_obligation_dispatch.as_ref()
    }

    pub fn graph_read_access_plan(&self) -> &ForgeQueryAdmittedGraphReadAccessPlan {
        &self.graph_read_access_plan
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

impl ForgeQueryLiveReadExecutionBinding {
    pub(crate) fn from_handoff(
        handoff: ForgeQueryLiveReadExecutionHandoff,
        graph_obligation_dispatch: Option<ForgeQueryAuthoritativeMutationObligationDispatch>,
        live_graph_read_access_plan: ForgeQueryLiveGraphReadAccessPlan,
    ) -> Self {
        let binding_digest = handoff_execution_binding_identity(
            "live-read-execution",
            &format!(
                "{}:{}",
                handoff.handoff_digest(),
                live_graph_read_access_plan.digest()
            ),
        );
        Self {
            handoff,
            graph_obligation_dispatch,
            live_graph_read_access_plan,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        self.handoff.installation()
    }

    pub fn handoff(&self) -> &ForgeQueryLiveReadExecutionHandoff {
        &self.handoff
    }

    pub fn graph_obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.graph_obligation_dispatch.as_ref()
    }

    pub fn live_graph_read_access_plan(&self) -> &ForgeQueryLiveGraphReadAccessPlan {
        &self.live_graph_read_access_plan
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}
