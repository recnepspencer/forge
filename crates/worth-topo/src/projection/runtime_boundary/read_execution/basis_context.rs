use forge_query::facade::{
    admit_historical_evaluation_path, admit_query_basis_context, bind_query_basis_context,
    materialization_metadata_from_resolved, preflight_execution_basis,
    resolve_historical_materialization_path, resolve_runtime_current_snapshot_basis,
    AdmittedQueryBasisContext, ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryReadFamily,
    ForgeQueryReadResult, ForgeQuerySnapshotIdentity, ForgeQueryWorkspace,
    HistoricalCapabilityDescriptor, HistoricalEvaluationRequest,
    HistoricalMaterializationDescriptor, HistoricalPathReuseDescriptor, QueryBasisContextRequest,
    QueryContextBindingSource,
};

use crate::projection::read_views::domain::error::TopologyReadError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TopologyReadExecutionTarget {
    CurrentHead,
    HistoricalSnapshot {
        snapshot_identity: ForgeQuerySnapshotIdentity,
    },
}

impl TopologyReadExecutionTarget {
    pub(crate) fn current_head() -> Self {
        Self::CurrentHead
    }

    pub(crate) fn historical_snapshot(snapshot_identity: ForgeQuerySnapshotIdentity) -> Self {
        Self::HistoricalSnapshot { snapshot_identity }
    }

    pub(crate) fn execute_family_with_explicit_graph_access_plan(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        family: &ForgeQueryReadFamily,
    ) -> Result<ForgeQueryReadResult, TopologyReadError> {
        match self {
            Self::CurrentHead => {
                let plan = current_head_graph_access_plan(workspace, family)?;
                workspace.execute_read_family_with_access_plan(family, plan)
            }
            Self::HistoricalSnapshot { snapshot_identity } => {
                let context = historical_context_for_family(family, snapshot_identity)?;
                let plan = basis_context_graph_access_plan(workspace, family, &context)?;
                workspace
                    .execute_read_family_in_basis_context_with_access_plan(family, &context, plan)
            }
        }
        .map_err(TopologyReadError::from_query_runtime_error)
    }
}

fn current_head_graph_access_plan(
    workspace: &mut ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
) -> Result<ForgeQueryAdmittedGraphReadAccessPlan, TopologyReadError> {
    workspace
        .read_family_intent(family)
        .review()
        .and_then(|review| review.graph_read_access_plan())
        .map_err(TopologyReadError::from_query_runtime_error)
}

fn basis_context_graph_access_plan(
    workspace: &mut ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
    context: &AdmittedQueryBasisContext,
) -> Result<ForgeQueryAdmittedGraphReadAccessPlan, TopologyReadError> {
    workspace
        .read_family_in_basis_context_intent(family, context)
        .review()
        .and_then(|review| review.graph_read_access_plan())
        .map_err(TopologyReadError::from_query_runtime_error)
}

fn historical_context_for_family(
    family: &ForgeQueryReadFamily,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) -> Result<AdmittedQueryBasisContext, TopologyReadError> {
    let snapshot_evidence_label = snapshot_identity
        .bridge_admission_evidence()
        .terminal_projection_for_reporting()
        .to_string();
    let snapshot_evidence_label = snapshot_evidence_label.as_str();
    let query_preflight = runtime_preflight_for_family(family, snapshot_identity)?;
    let request = HistoricalEvaluationRequest::retained_snapshot(
        snapshot_evidence_label,
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        snapshot_evidence_label,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability)
        .map_err(|error| TopologyReadError::read_family_execution_denied(format!("{error:?}")))?;
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot(snapshot_evidence_label),
    )
    .map_err(|error| TopologyReadError::read_family_execution_denied(format!("{error:?}")))?;
    let metadata = materialization_metadata_from_resolved(resolved);
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::historical_snapshot(snapshot_evidence_label),
        QueryContextBindingSource::Historical {
            query_preflight: &query_preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .map_err(|error| TopologyReadError::read_family_execution_denied(format!("{error:?}")))?;
    admit_query_basis_context(binding)
        .map_err(|error| TopologyReadError::read_family_execution_denied(format!("{error:?}")))
}

fn runtime_preflight_for_family(
    family: &ForgeQueryReadFamily,
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) -> Result<forge_query::facade::ExecutionPreflightBundle, TopologyReadError> {
    let basis = resolve_runtime_current_snapshot_basis(
        snapshot_identity.evidence_identity(),
        family.read_graph().schema_basis().clone(),
    )
    .map_err(|error| TopologyReadError::read_family_execution_denied(format!("{error:?}")))?;
    preflight_execution_basis(family.read_graph().execution_plan().clone(), basis)
        .map_err(|error| TopologyReadError::read_family_execution_denied(format!("{error:?}")))
}
