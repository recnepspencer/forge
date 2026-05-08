use forge_query::facade::{
    admit_historical_evaluation_path, admit_query_basis_context, bind_query_basis_context,
    materialization_metadata_from_resolved, preflight_execution_basis,
    resolve_historical_materialization_path, resolve_runtime_current_snapshot_basis,
    AdmittedQueryBasisContext, ForgeQueryReadFamily, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimeFamilySupportStatus, ForgeQueryWorkspace, HistoricalCapabilityDescriptor,
    HistoricalEvaluationRequest, HistoricalMaterializationDescriptor,
    HistoricalPathReuseDescriptor, QueryBasisContextRequest, QueryContextBindingSource,
};

use super::super::error::TopologyDomainQueryError;
use crate::query::runtime::TOPOLOGY_SNAPSHOT_HISTORICAL_BASIS_EVIDENCE;

pub(super) enum TopologyReadBasisExecutionMode {
    CurrentHead,
    HistoricalSnapshot { context: AdmittedQueryBasisContext },
}

impl TopologyReadBasisExecutionMode {
    pub(super) fn for_workspace(
        workspace: &ForgeQueryWorkspace,
        family: &ForgeQueryReadFamily,
    ) -> Result<Self, TopologyDomainQueryError> {
        if !workspace_uses_snapshot_historical_basis(workspace) {
            return Ok(Self::CurrentHead);
        }
        Ok(Self::HistoricalSnapshot {
            context: historical_context_for_family(family, workspace.snapshot_token().as_str())?,
        })
    }
}

fn workspace_uses_snapshot_historical_basis(workspace: &ForgeQueryWorkspace) -> bool {
    workspace
        .public_api_contract()
        .family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .is_some_and(|contract| {
            contract.status() == ForgeQueryRuntimeFamilySupportStatus::Unsupported
                && contract
                    .evidence()
                    .iter()
                    .any(|evidence| evidence == TOPOLOGY_SNAPSHOT_HISTORICAL_BASIS_EVIDENCE)
        })
}

fn historical_context_for_family(
    family: &ForgeQueryReadFamily,
    snapshot_token: &str,
) -> Result<AdmittedQueryBasisContext, TopologyDomainQueryError> {
    let query_preflight = runtime_preflight_for_family(family, snapshot_token)?;
    let request = HistoricalEvaluationRequest::retained_snapshot(
        snapshot_token,
        1,
        1,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let capability = HistoricalCapabilityDescriptor::retained_snapshot(
        snapshot_token,
        HistoricalPathReuseDescriptor::retained_reuse(),
    );
    let admission = admit_historical_evaluation_path(request, capability).map_err(|error| {
        TopologyDomainQueryError::read_family_execution_denied(format!("{error:?}"))
    })?;
    let resolved = resolve_historical_materialization_path(
        admission.clone(),
        HistoricalMaterializationDescriptor::retained_snapshot(snapshot_token),
    )
    .map_err(|error| {
        TopologyDomainQueryError::read_family_execution_denied(format!("{error:?}"))
    })?;
    let metadata = materialization_metadata_from_resolved(resolved);
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::historical_snapshot(snapshot_token),
        QueryContextBindingSource::Historical {
            query_preflight: &query_preflight,
            admission: &admission,
            metadata: &metadata,
        },
    )
    .map_err(|error| {
        TopologyDomainQueryError::read_family_execution_denied(format!("{error:?}"))
    })?;
    admit_query_basis_context(binding).map_err(|error| {
        TopologyDomainQueryError::read_family_execution_denied(format!("{error:?}"))
    })
}

fn runtime_preflight_for_family(
    family: &ForgeQueryReadFamily,
    snapshot_token: &str,
) -> Result<forge_query::facade::ExecutionPreflightBundle, TopologyDomainQueryError> {
    let basis = resolve_runtime_current_snapshot_basis(
        snapshot_token.to_string(),
        family.read_graph().schema_basis().clone(),
    )
    .map_err(|error| {
        TopologyDomainQueryError::read_family_execution_denied(format!("{error:?}"))
    })?;
    preflight_execution_basis(family.read_graph().execution_plan().clone(), basis).map_err(
        |error| TopologyDomainQueryError::read_family_execution_denied(format!("{error:?}")),
    )
}
