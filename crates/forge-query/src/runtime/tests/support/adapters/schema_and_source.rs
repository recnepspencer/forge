use super::*;
use crate::memory_workspace::{ForgeQueryLivePatch, ForgeQueryLiveViewHandle};

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
    live_views: BTreeMap<String, String>,
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
        self.live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| *collection == &delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }

    fn snapshot_token(&self) -> String {
        "external-snapshot".to_string()
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

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        self.inner.live_entities(view_name)
    }

    fn drain_live_patches(&mut self, view_name: &str) -> Vec<ForgeQueryLivePatch> {
        self.inner.drain_live_patches(view_name)
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        self.inner.affected_live_view_ids(receipt)
    }

    fn snapshot_token(&self) -> String {
        self.inner.snapshot_token()
    }
}

#[derive(Default)]
pub(in crate::runtime::tests) struct DriftingSnapshotSourceAdapter {
    snapshot_sequence: std::cell::Cell<u64>,
}

impl ForgeQueryRuntimeSourceAdapter for DriftingSnapshotSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        _request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, _view_name: &str) -> Vec<ForgeQueryEntity> {
        Vec::new()
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, _receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        Vec::new()
    }

    fn snapshot_token(&self) -> String {
        let next = self.snapshot_sequence.get() + 1;
        self.snapshot_sequence.set(next);
        format!("drifting-snapshot-{next}")
    }
}
