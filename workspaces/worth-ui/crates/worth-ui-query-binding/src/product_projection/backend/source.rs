use worth_query::facade::{foundation, runtime};

use super::state::SharedSourceState;

pub(super) struct WorthUiScalarProjectionSource {
    state: SharedSourceState,
}

impl WorthUiScalarProjectionSource {
    pub(super) fn new(state: SharedSourceState) -> Self {
        Self { state }
    }
}

impl runtime::WorthQueryRuntimeSourceAdapter for WorthUiScalarProjectionSource {
    fn declare_live_view(
        &mut self,
        name: String,
        request: foundation::DeclarativeLiveQueryRequest,
        _schema_view: runtime::QuerySchemaView,
    ) -> Result<foundation::WorthQueryLiveViewHandle, foundation::WorthQueryWorkspaceError> {
        let target = runtime::WorthQueryLiveArtifactTarget::from_source_adapter_declared_view_name(
            name.clone(),
        );
        self.state
            .borrow_mut()
            .register_live_target(target, request.target_collection_identity());
        Ok(foundation::WorthQueryLiveViewHandle::new(name))
    }

    fn close_live_view(&mut self, name: &str) -> Result<(), foundation::WorthQueryWorkspaceError> {
        let target =
            runtime::WorthQueryLiveArtifactTarget::from_source_adapter_declared_view_name(name);
        self.state.borrow_mut().remove_live_target(&target);
        Ok(())
    }

    fn live_entities_for_target(
        &self,
        target: &runtime::WorthQueryLiveArtifactTarget,
    ) -> Vec<foundation::WorthQueryEntity> {
        let state = self.state.borrow();
        if state.live_collection(target).is_none() {
            return Vec::new();
        }
        state
            .record()
            .cloned()
            .map(super::super::WorthUiScalarProjectionSourceRecord::into_query_entity)
            .into_iter()
            .collect()
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &runtime::WorthQueryLiveArtifactTarget,
    ) -> Vec<foundation::WorthQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        receipt: &foundation::WorthQueryMutationReceipt,
    ) -> Vec<runtime::WorthQueryLiveArtifactTarget> {
        let state = self.state.borrow();
        let mut affected = receipt
            .deltas()
            .iter()
            .flat_map(|delta| {
                state
                    .live_targets()
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
