use super::*;
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryLivePatch, ForgeQueryLiveViewHandle, ForgeQuerySnapshotIdentity,
};
use crate::runtime::backend::ForgeQueryRuntimeSnapshotIdentityAdapter;

impl ForgeQueryRuntimeSchemaAdapter for TestSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_live_view_declaration_admission_receipt(name, request);
        Ok(self.build_live_view_declaration_boundary_receipt(name, request, receipt))
    }
}

pub(in crate::runtime::tests) struct TestSchemaAdapter;

pub(in crate::runtime::tests) struct DenyingSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for DenyingSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "schema admission denied by test adapter",
        ))
    }
}

pub(in crate::runtime::tests) struct DriftingSchemaReceiptAdapter;

impl ForgeQueryRuntimeSchemaAdapter for DriftingSchemaReceiptAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<LiveViewDeclarationAdmissionBoundaryReceipt, ForgeQueryWorkspaceError> {
        let receipt = self.build_live_view_declaration_admission_receipt("drifted.view", request);
        Ok(self.build_live_view_declaration_boundary_receipt("drifted.view", request, receipt))
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct TestSourceAdapter {
    live_views: BTreeMap<ForgeQueryLiveArtifactTarget, ForgeQueryMutationTargetCollectionIdentity>,
    fail_declare: bool,
}

impl TestSourceAdapter {
    pub(in crate::runtime::tests) fn fail_declare() -> Self {
        Self {
            live_views: BTreeMap::new(),
            fail_declare: true,
        }
    }
}

impl ForgeQueryRuntimeSourceAdapter for TestSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        if self.fail_declare {
            return Err(ForgeQueryWorkspaceError::new(
                "source declaration denied by test adapter",
            ));
        }
        let live_target = ForgeQueryLiveArtifactTarget::from_view_name(name.clone());
        self.live_views
            .insert(live_target, request.target_collection_identity());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities_for_target(
        &self,
        _target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Vec<ForgeQueryLiveArtifactTarget> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| {
                        delta
                            .target_collection_identity()
                            .same_target_collection_as(collection)
                    })
                    .map(|(target, _)| target.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }
}

pub(in crate::runtime::tests) struct CountingSourceAdapter {
    pub(in crate::runtime::tests) declared_live_views: std::rc::Rc<std::cell::Cell<usize>>,
    inner: TestSourceAdapter,
}

impl CountingSourceAdapter {
    pub(in crate::runtime::tests) fn new(
        declared_live_views: std::rc::Rc<std::cell::Cell<usize>>,
    ) -> Self {
        Self {
            declared_live_views,
            inner: TestSourceAdapter::default(),
        }
    }
}

impl ForgeQueryRuntimeSourceAdapter for CountingSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.declared_live_views
            .set(self.declared_live_views.get().saturating_add(1));
        self.inner.declare_live_view(name, request, schema_view)
    }

    fn live_entities_for_target(
        &self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryEntity> {
        self.inner.live_entities_for_target(target)
    }

    fn drain_live_patches_for_target(
        &mut self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryLivePatch> {
        self.inner.drain_live_patches_for_target(target)
    }

    fn affected_live_view_targets(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Vec<ForgeQueryLiveArtifactTarget> {
        self.inner.affected_live_view_targets(receipt)
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct TestSnapshotIdentityAdapter;

impl ForgeQueryRuntimeSnapshotIdentityAdapter for TestSnapshotIdentityAdapter {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        ForgeQuerySnapshotIdentity::preview(
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
                .field_shape(
                    ForgeQueryEvidenceTag::new("test_snapshot_authority"),
                    "stable",
                )
                .field_usize(ForgeQueryEvidenceTag::new("test_snapshot_sequence"), 1)
                .seal(),
        )
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct DriftingSnapshotIdentityAdapter {
    snapshot_sequence: std::cell::Cell<u64>,
}

impl ForgeQueryRuntimeSnapshotIdentityAdapter for DriftingSnapshotIdentityAdapter {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        let snapshot_sequence = self.snapshot_sequence.get().saturating_add(1);
        self.snapshot_sequence.set(snapshot_sequence);
        ForgeQuerySnapshotIdentity::preview(
            ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
                .field_usize(
                    ForgeQueryEvidenceTag::new("drifting_snapshot_sequence"),
                    snapshot_sequence as usize,
                )
                .seal(),
        )
    }
}
