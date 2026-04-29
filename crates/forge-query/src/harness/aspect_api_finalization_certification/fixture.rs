use crate::program::ForgeQueryDerivedView;
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
        let row = Value::String(delta.entity_identity.clone());
        materialization.push_row(row.clone());
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "aspect-api-cert-derived-commit",
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

pub(super) fn task_runtime() -> ForgeQueryRuntime {
    ForgeQueryRuntime::builder()
        .compatibility_in_memory_collections([crate::memory_workspace::ForgeQueryCollection::new(
            "Task",
            [
                crate::memory_workspace::ForgeQueryAspect::new("identity.id", "identity.id"),
                crate::memory_workspace::ForgeQueryAspect::new("title.value", "title.value"),
                crate::memory_workspace::ForgeQueryAspect::new(
                    "description.value",
                    "description.value",
                ),
            ],
        )])
        .build()
        .expect("runtime should build")
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
