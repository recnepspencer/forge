use std::sync::Arc;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;
use crate::runtime::async_result_state::runtime_async_causality_identity;
use crate::runtime::computed::ForgeQueryDerivedViewRuntime;
use crate::runtime::evidence_identities::{
    shared_read_bind_retained_artifact_label_identity, shared_read_republishing_causality_identity,
    shared_read_unpublished_causality_identity,
};
use crate::runtime::{
    ForgeQueryDerivedArtifactBinding, ForgeQueryDerivedMaterializationBundle,
    ForgeQueryDerivedMaterializationReceipt, ForgeQueryDerivedMaterializationResult,
    ForgeQueryDerivedMaterializationTarget, ForgeQueryRuntimeAsyncResultState,
    ForgeQueryRuntimeAsyncResultStateKind, ForgeQueryRuntimeError,
};

#[derive(Clone, Debug, PartialEq)]
pub(in crate::runtime) struct ForgeQueryPublishedArtifactEntry {
    view_name: String,
    published_binding: Option<Arc<ForgeQueryDerivedArtifactBinding>>,
    async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
}

impl ForgeQueryPublishedArtifactEntry {
    pub(in crate::runtime) fn published(
        view_name: impl Into<String>,
        binding: ForgeQueryDerivedArtifactBinding,
        async_result_state: Option<ForgeQueryRuntimeAsyncResultState>,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            published_binding: Some(Arc::new(binding)),
            async_result_state,
        }
    }

    pub(in crate::runtime) fn unpublished(
        view_name: impl Into<String>,
        async_result_state: ForgeQueryRuntimeAsyncResultState,
    ) -> Self {
        Self {
            view_name: view_name.into(),
            published_binding: None,
            async_result_state: Some(async_result_state),
        }
    }

    pub(in crate::runtime) fn from_runtime_view(
        snapshot_identity: &ForgeQuerySnapshotIdentity,
        view_name: &str,
        runtime_view: &ForgeQueryDerivedViewRuntime,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        let evidence =
            crate::runtime::ForgeQueryComputedInspectionEvidence::from_runtime(runtime_view);
        let receipt = ForgeQueryDerivedMaterializationReceipt::from_evidence(
            &evidence,
            snapshot_identity.clone(),
        );
        let materialization = ForgeQueryDerivedMaterializationResult::from_retained_rows(
            runtime_view.materialization.retained_rows().to_vec(),
            receipt,
        );
        let async_result_state = async_result_state_for_shared_read_entry(
            snapshot_identity,
            view_name,
            evidence.pending_patch_count(),
            evidence.pending_refresh_fallback_count(),
        );
        if runtime_view.materialization.is_published() {
            return Ok(Self::published(
                view_name.to_string(),
                bind_shared_read_artifact(snapshot_identity, view_name, materialization)?,
                async_result_state,
            ));
        }
        Ok(Self::unpublished(
            view_name.to_string(),
            unpublished_async_result_state(snapshot_identity, view_name),
        ))
    }

    pub(in crate::runtime) fn published_binding(
        &self,
    ) -> Option<Arc<ForgeQueryDerivedArtifactBinding>> {
        self.published_binding.clone()
    }

    pub(in crate::runtime) fn async_result_state(
        &self,
    ) -> Option<ForgeQueryRuntimeAsyncResultState> {
        self.async_result_state.clone()
    }
}

fn bind_shared_read_artifact(
    snapshot_identity: &ForgeQuerySnapshotIdentity,
    view_name: &str,
    materialization: ForgeQueryDerivedMaterializationResult,
) -> Result<ForgeQueryDerivedArtifactBinding, ForgeQueryRuntimeError> {
    let bundle = ForgeQueryDerivedMaterializationBundle::new(
        snapshot_identity.clone(),
        std::collections::BTreeMap::from([(view_name.to_string(), materialization)]),
    );
    bundle.bind_retained_artifact_identity(
        shared_read_bind_retained_artifact_label_identity(
            view_name,
            &snapshot_identity.evidence_identity(),
        ),
        [ForgeQueryDerivedMaterializationTarget::new(view_name)],
    )
}

fn async_result_state_for_shared_read_entry(
    snapshot_identity: &ForgeQuerySnapshotIdentity,
    view_name: &str,
    pending_patch_count: usize,
    pending_refresh_fallback_count: usize,
) -> Option<ForgeQueryRuntimeAsyncResultState> {
    if pending_patch_count == 0 {
        return None;
    }

    let kind = if pending_refresh_fallback_count > 0 {
        ForgeQueryRuntimeAsyncResultStateKind::Revalidating
    } else {
        ForgeQueryRuntimeAsyncResultStateKind::Stale
    };

    Some(ForgeQueryRuntimeAsyncResultState::new(
        kind,
        &runtime_async_causality_identity(&shared_read_republishing_causality_identity(
            view_name,
            kind,
            &snapshot_identity.evidence_identity(),
        )),
        &snapshot_identity.evidence_identity(),
        &shared_read_generation_identity(snapshot_identity),
    ))
}

fn unpublished_async_result_state(
    snapshot_identity: &ForgeQuerySnapshotIdentity,
    view_name: &str,
) -> ForgeQueryRuntimeAsyncResultState {
    ForgeQueryRuntimeAsyncResultState::new(
        ForgeQueryRuntimeAsyncResultStateKind::Pending,
        &runtime_async_causality_identity(&shared_read_unpublished_causality_identity(
            view_name,
            &snapshot_identity.evidence_identity(),
        )),
        &snapshot_identity.evidence_identity(),
        &shared_read_generation_identity(snapshot_identity),
    )
}

fn shared_read_generation_identity(
    snapshot_identity: &ForgeQuerySnapshotIdentity,
) -> crate::evidence_identity::ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::SharedReadGeneration)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_identity"),
            &snapshot_identity.evidence_identity(),
        )
        .seal()
}
