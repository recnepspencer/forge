use forge_query::facade::{
    admit_historical_evaluation_path, admit_query_basis_context, bind_query_basis_context,
    materialization_metadata_from_resolved, preflight_execution_basis,
    resolve_historical_materialization_path, resolve_runtime_current_snapshot_basis,
    AdmittedQueryBasisContext, ForgeQueryReadFamily, ForgeQueryReadResult,
    ForgeQuerySnapshotIdentity, ForgeQueryWorkspace, HistoricalCapabilityDescriptor,
    HistoricalEvaluationRequest, HistoricalMaterializationDescriptor,
    HistoricalPathReuseDescriptor, QueryBasisContextRequest, QueryContextBindingSource,
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

    pub(crate) fn execute_family(
        &self,
        workspace: &mut ForgeQueryWorkspace,
        family: &ForgeQueryReadFamily,
    ) -> Result<ForgeQueryReadResult, TopologyReadError> {
        match self {
            Self::CurrentHead => workspace.execute_read_family(family),
            Self::HistoricalSnapshot { snapshot_identity } => workspace
                .execute_read_family_in_basis_context(
                    family,
                    &historical_context_for_family(family, snapshot_identity)?,
                ),
        }
        .map_err(|error| TopologyReadError::read_family_execution_denied(format!("{error:?}")))
    }
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
