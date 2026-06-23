use crate::program::ForgeQueryDerivedView;
use crate::runtime::tests::support::stateful_bridge_task_runtime as build_stateful_bridge_task_runtime;
use crate::runtime::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization,
    ForgeQueryLiveView, ForgeQueryRuntime, ForgeQueryWorkspace,
};
use serde_json::Value;

pub(super) struct CertificationTitleListMaintainer;

impl ForgeQueryDerivedViewMaintainer for CertificationTitleListMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let row = Value::String(
            delta
                .entity_identity
                .terminal_projection_for_reporting()
                .to_string(),
        );
        materialization.push_row(row.clone());
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::memory_workspace::admit_external_commit_label("aspect-api-cert-derived-commit"),
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            row,
        )
    }
}

pub(super) fn stateful_bridge_task_runtime() -> ForgeQueryRuntime {
    build_stateful_bridge_task_runtime()
}

pub(super) fn task_live_view<T>(
    workspace: &mut ForgeQueryWorkspace,
    name: &str,
) -> ForgeQueryLiveView<T> {
    workspace
        .live_view(name, |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("aspect-api-cert-task")
        })
        .expect("live view should declare")
}
