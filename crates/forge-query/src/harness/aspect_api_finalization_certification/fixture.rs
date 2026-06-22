use crate::program::ForgeQueryDerivedView;
use crate::runtime::tests::support::stateful_bridge_task_runtime as build_stateful_bridge_task_runtime;
use crate::runtime::{
    ForgeQueryDerivedPatch, ForgeQueryDerivedPatchPayload, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryLiveView, ForgeQueryRetainedFieldPath,
    ForgeQueryRuntime, ForgeQueryWorkspace,
};
use forge_foundational::facade::{AspectValue, CanonicalFieldPath, FieldKey};

pub(super) struct CertificationTitleListMaintainer;

impl ForgeQueryDerivedViewMaintainer for CertificationTitleListMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let retained_scalar = (
            retained_field_path("value"),
            AspectValue::String(
                delta
                    .entity_identity
                    .terminal_projection_for_reporting()
                    .to_string()
                    .into(),
            ),
        );
        materialization
            .push_retained_scalar_row([retained_scalar.clone()])
            .expect("certification title row should admit native scalar values");
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            crate::memory_workspace::admit_external_commit_label("aspect-api-cert-derived-commit"),
            delta.entity_identity.clone(),
            if view.produced_aspect_touches().is_empty() {
                delta.admitted_touched_aspects().to_vec()
            } else {
                view.produced_aspect_touches().to_vec()
            },
            ForgeQueryDerivedPatchPayload::from_retained_scalar_values([retained_scalar])
                .expect("certification title payload should admit native scalar values"),
        )
    }
}

fn retained_field_path(path: &str) -> ForgeQueryRetainedFieldPath {
    let fields = path
        .split('.')
        .map(|segment| FieldKey::new(segment.to_string()))
        .collect::<Option<Vec<_>>>()
        .expect("certification retained field path should admit");
    let path = CanonicalFieldPath::new(fields)
        .expect("certification retained field path should not be empty");
    ForgeQueryRetainedFieldPath::from_canonical_field_path(path)
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
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("aspect-api-cert-task")
        })
        .expect("live view should declare")
}
