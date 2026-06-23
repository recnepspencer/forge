use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use forge_foundational::facade::AspectValue;

use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey,
};
use crate::authorized_projection::{
    derive_authorized_projection, AuthorizedProjectionArtifact, PolicyAspectMask,
    PolicyInfluenceSet,
};
use crate::canonicalization::CanonicalResultShapeArtifact;
use crate::projection_consumption::{ProjectMaterializedFacts, ProjectionFactConsumptionAttempt};

use super::support::*;

#[derive(Clone)]
pub(super) struct SharedReadPublishingMaintainer {
    pub(super) invocations: Arc<AtomicUsize>,
    pub(super) mode: SharedReadPublicationMode,
}

#[derive(Clone)]
pub(super) enum SharedReadPublicationMode {
    RefreshTitle(&'static str),
    EmptyRefresh,
    SequencedRefresh(&'static [&'static str]),
    IncrementalTitle(&'static str),
}

impl ForgeQueryDerivedViewMaintainer for SharedReadPublishingMaintainer {
    fn maintain(
        &mut self,
        view: &crate::program::ForgeQueryDerivedView,
        _delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let next = self.invocations.fetch_add(1, Ordering::SeqCst) + 1;
        match self.mode {
            SharedReadPublicationMode::RefreshTitle(title) => {
                materialization.replace_retained_rows([published_title_retained_row(title)]);
                ForgeQueryDerivedPatch::whole_refresh_materialized(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-refresh-{next}"
                    )),
                    test_aspect_touches(["title.value"]),
                    ForgeQueryDerivedPatchPayload::from_retained_row(published_title_retained_row(
                        title,
                    )),
                    format!("shared-read-publication-{next}"),
                )
            }
            SharedReadPublicationMode::EmptyRefresh => {
                materialization.replace_retained_rows(std::iter::empty());
                ForgeQueryDerivedPatch::whole_refresh_materialized(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-empty-{next}"
                    )),
                    test_aspect_touches(["title.value"]),
                    ForgeQueryDerivedPatchPayload::empty(),
                    format!("shared-read-empty-publication-{next}"),
                )
            }
            SharedReadPublicationMode::SequencedRefresh(titles) => {
                let title = titles
                    .get(next.saturating_sub(1))
                    .copied()
                    .unwrap_or_else(|| titles[titles.len() - 1]);
                materialization.replace_retained_rows([published_title_retained_row(title)]);
                ForgeQueryDerivedPatch::whole_refresh_materialized(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-sequenced-{next}"
                    )),
                    test_aspect_touches(["title.value"]),
                    ForgeQueryDerivedPatchPayload::from_retained_row(published_title_retained_row(
                        title,
                    )),
                    format!("shared-read-sequenced-publication-{next}"),
                )
            }
            SharedReadPublicationMode::IncrementalTitle(title) => {
                materialization.replace_retained_rows([published_title_retained_row(title)]);
                ForgeQueryDerivedPatch::incremental(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-stale-{next}"
                    )),
                    crate::memory_workspace::admit_authored_entity_label(format!("entity-{next}")),
                    test_aspect_touches(["title.value"]),
                    ForgeQueryDerivedPatchPayload::from_retained_row(published_title_retained_row(
                        title,
                    )),
                )
            }
        }
    }
}

pub(super) fn shared_read_workspace(name: &str) -> ForgeQueryWorkspace {
    stateful_bridge_task_runtime()
        .workspace(name)
        .expect("workspace should build")
}

pub(super) fn declare_shared_read_derived(
    workspace: &mut ForgeQueryWorkspace,
    suffix: &str,
    maintainer: SharedReadPublishingMaintainer,
) -> ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> {
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view_request(
            &format!("tasks.{suffix}"),
            task_live_request(),
            task_schema(),
        )
        .expect("live view should declare");
    workspace
        .computed_view(
            crate::program::ForgeQueryDerivedView::new(
                format!("derived.{suffix}"),
                test_aspect_touches(["title.value"]),
            )
            .depends_on_live(&live),
            maintainer,
        )
        .expect("derived view should declare")
}

pub(super) fn insert_task(workspace: &mut ForgeQueryWorkspace, id: &str, title: &str) {
    workspace
        .insert("Task", |builder| {
            builder
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value(id),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value(title),
                )
        })
        .expect("task insert should succeed");
}

pub(super) fn consume_display_title_attempt(
    artifact: &ForgeQueryPublishedDerivedArtifactHandle,
) -> ForgeQueryPublishedProjectionConsumption {
    let (result_shape, authorized_projection) = projection_artifacts();
    artifact
        .consume_projection_facts(
            &result_shape,
            &authorized_projection,
            ProjectMaterializedFacts::declare().display_field_path(
                crate::projection_consumption::projection_fact_field_path_from_segments([
                    forge_foundational::facade::FieldKey::new("title")
                        .expect("projection fact field segment should admit"),
                    forge_foundational::facade::FieldKey::new("value")
                        .expect("projection fact field segment should admit"),
                ]),
            ),
        )
        .expect("projection consumption should stay on the typed artifact lane")
}

pub(super) fn consume_display_title(artifact: &ForgeQueryPublishedDerivedArtifactHandle) -> String {
    let completed = match consume_display_title_attempt(artifact) {
        ForgeQueryPublishedProjectionConsumption::Current(
            ProjectionFactConsumptionAttempt::Admitted(completed),
        ) => completed,
        other => panic!("expected admitted published consumption, got {other:?}"),
    };
    completed
        .facts()
        .display_fields()
        .first()
        .and_then(|fact| match fact.value() {
            AspectValue::String(forge_foundational::facade::InternedString::Raw(value)) => {
                Some(value.clone())
            }
            _ => None,
        })
        .expect("display-field title should be present")
}

fn projection_artifacts() -> (CanonicalResultShapeArtifact, AuthorizedProjectionArtifact) {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("task").unwrap())
        .project(AspectFieldSelector::new("title", "value").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("title", "value", "title.value").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap();
    let authorized_projection = derive_authorized_projection(
        canonical.query(),
        canonical.result_shape(),
        "policy:test",
        "schema:test",
        &PolicyAspectMask::allow_all(),
        &PolicyInfluenceSet::none(),
        8,
        8,
    )
    .unwrap();
    (canonical.result_shape().clone(), authorized_projection)
}

fn published_title_retained_row(title: &str) -> ForgeQueryRetainedMaterializedRow {
    retained_string_test_row("title.value", title)
}
