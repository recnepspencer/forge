use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

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
struct SharedReadPublishingMaintainer {
    invocations: Arc<AtomicUsize>,
    mode: SharedReadPublicationMode,
}

#[derive(Clone)]
enum SharedReadPublicationMode {
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

#[test]
fn shared_read_context_consumes_published_projection_facts_through_typed_artifact_handles() {
    let mut workspace = shared_read_workspace("shared-read.current");
    let invocations = Arc::new(AtomicUsize::new(0));
    let derived = declare_shared_read_derived(
        &mut workspace,
        "shared-read",
        SharedReadPublishingMaintainer {
            invocations: Arc::clone(&invocations),
            mode: SharedReadPublicationMode::RefreshTitle("Task One"),
        },
    );
    insert_task(&mut workspace, "task-1", "Task One");

    let artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("published artifact handle should mint");
    let consumption = consume_display_title_attempt(&artifact);

    match consumption {
        ForgeQueryPublishedProjectionConsumption::Current(
            ProjectionFactConsumptionAttempt::Admitted(completed),
        ) => {
            assert_eq!(completed.receipt().extracted_fact_count(), 1);
            assert_eq!(
                artifact
                    .inspect_projection_consumption()
                    .async_result_state()
                    .expect("republishing posture should stay visible")
                    .kind(),
                ForgeQueryRuntimeAsyncResultStateKind::Revalidating
            );
            assert_eq!(consume_display_title(&artifact), "Task One");
        }
        other => panic!("expected published fact consumption, got {other:?}"),
    }

    assert_eq!(
        invocations.load(Ordering::SeqCst),
        1,
        "shared-read consumption must not trigger derived reevaluation"
    );
}

#[test]
fn shared_read_unpublished_artifact_fails_closed_with_typed_pending_state() {
    let mut workspace = shared_read_workspace("shared-read.pending");
    let derived = declare_shared_read_derived(
        &mut workspace,
        "shared-read.pending",
        SharedReadPublishingMaintainer {
            invocations: Arc::new(AtomicUsize::new(0)),
            mode: SharedReadPublicationMode::RefreshTitle("Task One"),
        },
    );
    let unpublished = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("declared unpublished derived view should still mint a typed handle");

    match consume_display_title_attempt(&unpublished) {
        ForgeQueryPublishedProjectionConsumption::ResultState(state) => {
            assert_eq!(state.kind(), ForgeQueryRuntimeAsyncResultStateKind::Pending);
            assert_eq!(
                state.basis_for_reporting(),
                unpublished
                    .snapshot_identity()
                    .evidence_identity()
                    .terminal_projection_for_reporting()
            );
        }
        other => panic!("expected pending async posture, got {other:?}"),
    }
}

#[test]
fn shared_read_foreign_derived_handle_denies_instead_of_masking_as_pending() {
    let workspace = shared_read_workspace("shared-read.foreign");
    let read_ctx = workspace
        .shared_read_context()
        .expect("shared read context should mint");
    let mut foreign_workspace = shared_read_workspace("shared-read.foreign-source");
    let foreign = declare_shared_read_derived(
        &mut foreign_workspace,
        "shared-read.foreign",
        SharedReadPublishingMaintainer {
            invocations: Arc::new(AtomicUsize::new(0)),
            mode: SharedReadPublicationMode::RefreshTitle("Task One"),
        },
    );

    let error = read_ctx
        .published_derived_artifact(&foreign)
        .expect_err("foreign derived handles must not masquerade as pending publication");
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingDerivedView(_)
    ));
}

#[test]
fn shared_read_empty_published_artifact_stays_published_instead_of_pending() {
    let mut workspace = shared_read_workspace("shared-read.empty-published");
    let derived = declare_shared_read_derived(
        &mut workspace,
        "shared-read.empty",
        SharedReadPublishingMaintainer {
            invocations: Arc::new(AtomicUsize::new(0)),
            mode: SharedReadPublicationMode::EmptyRefresh,
        },
    );
    insert_task(&mut workspace, "task-1", "Task One");

    let artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("published empty artifact handle should mint");
    let inspection = artifact.inspect_projection_consumption();

    assert!(inspection.published());
    assert!(artifact.published_binding().is_some());
    assert_ne!(
        inspection
            .async_result_state()
            .expect("published empty artifact should still retain republication posture")
            .kind(),
        ForgeQueryRuntimeAsyncResultStateKind::Pending
    );
}

#[test]
fn shared_read_incremental_republication_preserves_stale_async_posture() {
    let mut workspace = shared_read_workspace("shared-read.stale");
    let invocations = Arc::new(AtomicUsize::new(0));
    let derived = declare_shared_read_derived(
        &mut workspace,
        "shared-read.stale",
        SharedReadPublishingMaintainer {
            invocations: Arc::clone(&invocations),
            mode: SharedReadPublicationMode::IncrementalTitle("Task One"),
        },
    );
    insert_task(&mut workspace, "task-1", "Task One");

    let artifact = workspace
        .shared_read_context()
        .expect("shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("stale artifact handle should mint");

    assert_eq!(
        artifact
            .inspect_projection_consumption()
            .async_result_state()
            .expect("incremental republication should surface async posture")
            .kind(),
        ForgeQueryRuntimeAsyncResultStateKind::Stale
    );
    assert_eq!(consume_display_title(&artifact), "Task One");
    assert_eq!(invocations.load(Ordering::SeqCst), 1);
}

#[test]
fn shared_read_republication_keeps_old_context_on_old_artifact_and_new_context_on_new_artifact() {
    let mut workspace = shared_read_workspace("shared-read.republication");
    let invocations = Arc::new(AtomicUsize::new(0));
    let derived = declare_shared_read_derived(
        &mut workspace,
        "shared-read.republication",
        SharedReadPublishingMaintainer {
            invocations: Arc::clone(&invocations),
            mode: SharedReadPublicationMode::SequencedRefresh(&["Task One", "Task Two"]),
        },
    );
    insert_task(&mut workspace, "task-1", "Task One");
    let old_artifact = workspace
        .shared_read_context()
        .expect("old shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("old artifact handle should mint");

    insert_task(&mut workspace, "task-2", "Task Two");
    let new_artifact = workspace
        .shared_read_context()
        .expect("new shared read context should mint")
        .published_derived_artifact(&derived)
        .expect("new artifact handle should mint");

    assert_eq!(consume_display_title(&old_artifact), "Task One");
    assert_eq!(consume_display_title(&new_artifact), "Task Two");
    assert_ne!(
        old_artifact
            .published_binding()
            .expect("old artifact should stay published")
            .binding_for_reporting(),
        new_artifact
            .published_binding()
            .expect("new artifact should stay published")
            .binding_for_reporting()
    );
    assert_eq!(invocations.load(Ordering::SeqCst), 2);
}

#[test]
fn shared_read_context_fails_closed_when_runtime_retires_its_snapshot_generation() {
    let mut workspace = shared_read_workspace("shared-read.stale-basis");
    let derived = declare_shared_read_derived(
        &mut workspace,
        "shared-read.stale-basis",
        SharedReadPublishingMaintainer {
            invocations: Arc::new(AtomicUsize::new(0)),
            mode: SharedReadPublicationMode::RefreshTitle("Task One"),
        },
    );
    insert_task(&mut workspace, "task-1", "Task One");

    let read_ctx = workspace
        .shared_read_context()
        .expect("shared read context should mint");
    let expected_snapshot_identity = read_ctx.snapshot_identity().clone();
    workspace
        .runtime
        .force_retire_shared_read_snapshot_for_tests(&expected_snapshot_identity);

    let error = read_ctx
        .published_derived_artifact(&derived)
        .expect_err("retired shared-read basis must fail closed");
    match error.stop_class() {
        ForgeQueryStopClass::SharedReadStaleBasis { snapshot_identity } => {
            assert_eq!(snapshot_identity, &expected_snapshot_identity);
        }
        other => panic!("expected shared-read stale-basis stop class, got {other:?}"),
    }
}

#[test]
fn shared_read_pin_counters_return_to_exact_zero_after_context_and_artifact_drop() {
    let mut workspace = shared_read_workspace("shared-read.pin-counts");
    let derived = declare_shared_read_derived(
        &mut workspace,
        "shared-read.pin-counts",
        SharedReadPublishingMaintainer {
            invocations: Arc::new(AtomicUsize::new(0)),
            mode: SharedReadPublicationMode::RefreshTitle("Task One"),
        },
    );
    insert_task(&mut workspace, "task-1", "Task One");

    {
        let read_ctx = workspace
            .shared_read_context()
            .expect("shared read context should mint");
        let artifact = read_ctx
            .published_derived_artifact(&derived)
            .expect("published artifact should mint");
        assert!(artifact.published_binding().is_some());
        assert_eq!(
            workspace
                .runtime
                .shared_read_counters()
                .unretired_pin_count(),
            2
        );
    }

    let counters = workspace.runtime.shared_read_counters();
    assert_eq!(counters.unretired_pin_count(), 0);
    assert_eq!(counters.orphaned_generation_count(), 0);
}

fn shared_read_workspace(name: &str) -> ForgeQueryWorkspace {
    stateful_bridge_task_runtime()
        .workspace(name)
        .expect("workspace should build")
}

fn declare_shared_read_derived(
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

fn insert_task(workspace: &mut ForgeQueryWorkspace, id: &str, title: &str) {
    workspace
        .insert("Task", |builder| {
            builder
                .aspect("identity.id", id)
                .aspect("title.value", title)
        })
        .expect("task insert should succeed");
}

fn consume_display_title_attempt(
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

fn consume_display_title(artifact: &ForgeQueryPublishedDerivedArtifactHandle) -> String {
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
