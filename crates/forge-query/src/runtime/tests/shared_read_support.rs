use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use serde_json::{json, Value};

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
                materialization.replace_rows([published_title_row(title)]);
                ForgeQueryDerivedPatch::whole_refresh_materialized(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-refresh-{next}"
                    )),
                    ["title.value".to_string()],
                    json!({"published": true, "title": title}),
                    format!("shared-read-publication-{next}"),
                )
            }
            SharedReadPublicationMode::EmptyRefresh => {
                materialization.replace_rows(std::iter::empty());
                ForgeQueryDerivedPatch::whole_refresh_materialized(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-empty-{next}"
                    )),
                    ["title.value".to_string()],
                    json!({"published": true, "rows": 0}),
                    format!("shared-read-empty-publication-{next}"),
                )
            }
            SharedReadPublicationMode::SequencedRefresh(titles) => {
                let title = titles
                    .get(next.saturating_sub(1))
                    .copied()
                    .unwrap_or_else(|| titles[titles.len() - 1]);
                materialization.replace_rows([published_title_row(title)]);
                ForgeQueryDerivedPatch::whole_refresh_materialized(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-sequenced-{next}"
                    )),
                    ["title.value".to_string()],
                    json!({"published": true, "title": title}),
                    format!("shared-read-sequenced-publication-{next}"),
                )
            }
            SharedReadPublicationMode::IncrementalTitle(title) => {
                materialization.replace_rows([published_title_row(title)]);
                ForgeQueryDerivedPatch::incremental(
                    view.name(),
                    crate::memory_workspace::admit_external_commit_label(format!(
                        "shared-read-stale-{next}"
                    )),
                    crate::memory_workspace::admit_authored_entity_label(format!("entity-{next}")),
                    ["title.value".to_string()],
                    json!({"published": true, "title": title}),
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
) -> ForgeQueryDerivedViewHandle<Value> {
    let live: ForgeQueryLiveView<Value> = workspace
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
                ["title.value".to_string()],
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
                .aspect("identity.id", id)
                .aspect("title.value", title)
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
            ProjectMaterializedFacts::declare().display_field("title.value"),
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
        .and_then(|fact| fact.value().as_str())
        .expect("display-field title should be present")
        .to_string()
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

fn published_title_row(title: &str) -> Value {
    json!({ "title": { "value": title } })
}
