use super::*;
use crate::memory_workspace::{ForgeQueryLivePatch, ForgeQueryLiveViewHandle};

impl ForgeQueryRuntimeSchemaAdapter for TestSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        _request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
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
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "schema admission denied by test adapter",
        ))
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
