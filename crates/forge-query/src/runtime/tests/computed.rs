use super::support::*;
use crate::memory_workspace::ForgeQueryCommitIdentity;

fn external_commit(label: &str) -> ForgeQueryCommitIdentity {
    crate::memory_workspace::admit_external_commit_label(label)
}

fn touch(path: &str) -> ForgeQueryAspectTouch {
    test_aspect_touch(path)
}

fn touches<const N: usize>(paths: [&str; N]) -> [ForgeQueryAspectTouch; N] {
    paths.map(test_aspect_touch)
}

fn update_string_aspect(
    entity_identity: crate::memory_workspace::ForgeQueryEntityIdentity,
    aspect_path: &str,
    value: &str,
) -> ForgeQueryWriteCommand {
    ForgeQueryWriteCommand::UpdateAspect {
        entity_identity,
        aspect: ForgeQueryAspectValue::new_set(touch(aspect_path), test_string_aspect_value(value))
            .expect("test aspect update should build"),
    }
}

fn read_derived_value_aspects<T>(
    runtime: &ForgeQueryRuntime,
    view: &ForgeQueryDerivedViewHandle<T>,
) -> Vec<AspectValue> {
    retained_value_aspects(read_derived(runtime, view).retained_rows())
}

fn read_derived<T>(
    runtime: &ForgeQueryRuntime,
    view: &ForgeQueryDerivedViewHandle<T>,
) -> ForgeQueryDerivedMaterializationResult {
    runtime
        .read_derived_result(view)
        .expect("test derived materialization should execute")
}

fn retained_value_aspects(rows: &[ForgeQueryRetainedMaterializedRow]) -> Vec<AspectValue> {
    let value_path =
        retained_test_field_path("value").expect("test retained value path should parse");
    rows.iter()
        .filter_map(|row| row.field_value_at(&value_path))
        .cloned()
        .collect()
}

fn retained_string_field(row: &ForgeQueryRetainedMaterializedRow, field: &str) -> String {
    let field_path =
        retained_test_field_path(field).expect("test retained string path should parse");
    let value = row
        .field_value_at(&field_path)
        .expect("retained row should carry requested string field");
    let AspectValue::String(forge_foundational::facade::InternedString::Raw(value)) = value else {
        panic!("expected retained string field `{field}`, got {value:?}");
    };
    value.clone()
}

fn retained_u64_field(row: &ForgeQueryRetainedMaterializedRow, field: &str) -> u64 {
    let field_path =
        retained_test_field_path(field).expect("test retained integer path should parse");
    let value = row
        .field_value_at(&field_path)
        .expect("retained row should carry requested integer field");
    let AspectValue::UInt64(value) = value else {
        panic!("expected retained u64 field `{field}`, got {value:?}");
    };
    *value
}

fn delta_or_produced_touches(
    view: &ForgeQueryDerivedView,
    delta: &crate::memory_workspace::ForgeQueryMutationDelta,
) -> Vec<ForgeQueryAspectTouch> {
    if view.produced_aspect_touches().is_empty() {
        delta.admitted_touched_aspects().to_vec()
    } else {
        view.produced_aspect_touches().to_vec()
    }
}

fn dependency_or_produced_touches(view: &ForgeQueryDerivedView) -> Vec<ForgeQueryAspectTouch> {
    if view.produced_aspect_touches().is_empty() {
        view.dependency_aspect_touches().to_vec()
    } else {
        view.produced_aspect_touches().to_vec()
    }
}

#[test]
fn derived_view_receives_narrow_or_fallback_patch_notes() {
    let mut runtime = stateful_bridge_task_runtime();
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let task_titles = runtime
        .declare_derived_view(
            ForgeQueryDerivedView::new("task_titles", touches(["title"])).whole_refresh_fallback(),
        )
        .expect("derived view should declare");
    let task_titles_handle =
        ForgeQueryDerivedViewHandle::<ForgeQueryNativeRow>::new(task_titles.name());
    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("Derived task")),
            ],
        ))
        .expect("insert should route to derived view");
    let update = runtime
        .write(update_string_aspect(
            insert.deltas()[0].entity_identity.clone(),
            "title.value",
            "Derived task renamed",
        ))
        .expect("title update should route to derived view");

    let patches = runtime.drain_derived_patches(&task_titles_handle);

    assert_eq!(
        update.terminal_affected_derived_view_ids_projection(),
        &["task_titles".to_string()]
    );
    assert!(update.refresh_fallback());
    assert!(patches
        .derived_patch_notes
        .iter()
        .any(|note| note.starts_with("whole-refresh-fallback")));
}

#[test]
fn maintained_derived_view_materializes_incremental_patches() {
    let mut runtime = stateful_bridge_task_runtime();
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("task_titles", touches(["title"])),
            TitleListMaintainer,
        )
        .expect("maintained derived view should declare");

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("First title")),
            ],
        ))
        .expect("insert should route derived patch");
    let patches = runtime.drain_derived_patches(&titles);

    assert_eq!(
        insert.terminal_affected_derived_view_ids_projection(),
        &["task_titles".to_string()]
    );
    let expected_row = test_string_aspect_value(
        insert.deltas()[0]
            .entity_identity
            .terminal_projection_for_reporting()
            .to_string(),
    );
    assert_eq!(
        read_derived_value_aspects(&runtime, &titles),
        vec![expected_row.clone()]
    );
    assert_eq!(patches.derived_patches.len(), 1);
    assert_eq!(
        retained_value_aspects(patches.derived_patches[0].retained_payload_rows()),
        vec![expected_row]
    );

    runtime
        .write(update_string_aspect(
            insert.deltas()[0].entity_identity.clone(),
            "identity.id",
            "ignored",
        ))
        .expect("irrelevant update should not route derived patch");
    let irrelevant = runtime.drain_derived_patches(&titles);

    assert!(irrelevant.derived_patches.is_empty());
}

#[test]
fn nested_computed_views_route_in_deterministic_dependency_order() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.titles", touches(["title"]))
                .depends_on_live(&live)
                .produces(touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("source computed view should declare");
    let summary = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.summary", touches(["title.summary"]))
                .depends_on_derived(&titles)
                .produces(touches(["validation.state"])),
            SummaryMaintainer,
        )
        .expect("nested computed view should declare");

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("Nested title")),
            ],
        ))
        .expect("insert should update nested computeds");
    let title_patches = runtime.drain_derived_patches(&titles);
    let summary_patches = runtime.drain_derived_patches(&summary);

    assert_eq!(
        insert.terminal_affected_derived_view_ids_projection(),
        &[
            "computed.summary".to_string(),
            "computed.titles".to_string()
        ]
    );
    assert_eq!(insert.considered_computed_view_count(), 2);
    assert_eq!(title_patches.derived_patches.len(), 1);
    assert_eq!(
        title_patches.derived_patches[0].aspect_touches(),
        test_aspect_touches(["title.summary"]).as_slice()
    );
    assert_eq!(summary_patches.derived_patches.len(), 1);
    assert_eq!(
        summary_patches.derived_patches[0].aspect_touches(),
        test_aspect_touches(["validation.state"]).as_slice()
    );
    assert_eq!(
        read_derived_value_aspects(&runtime, &summary),
        vec![test_string_aspect_value(format!(
            "summary:{}",
            insert.deltas()[0]
                .entity_identity
                .terminal_projection_for_reporting()
        ))]
    );

    runtime
        .write(update_string_aspect(
            insert.deltas()[0].entity_identity.clone(),
            "identity.id",
            "ignored",
        ))
        .expect("irrelevant update should still write");
    assert!(runtime
        .drain_derived_patches(&titles)
        .derived_patches
        .is_empty());
    assert!(runtime
        .drain_derived_patches(&summary)
        .derived_patches
        .is_empty());
}

#[test]
fn computed_dependency_index_replaces_redeclared_view_membership() {
    let mut runtime = stateful_bridge_task_issue_runtime();
    let task_live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("task live should declare");
    let issue_live = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "issues.table",
            issue_live_request(),
            issue_schema(),
        )
        .expect("issue live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.shared", touches(["title"]))
                .depends_on_live(&task_live)
                .produces(touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("task-backed computed should declare");

    runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.shared", touches(["summary"]))
                .depends_on_live(&issue_live)
                .produces(touches(["issue.summary"])),
            SummaryMaintainer,
        )
        .expect("redeclared computed should replace old dependency index membership");

    let task_write = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Task should not wake redeclared computed"),
                ),
            ],
        ))
        .expect("task write should execute");
    assert!(task_write
        .terminal_affected_derived_view_ids_projection()
        .is_empty());
    assert_eq!(task_write.considered_computed_view_count(), 0);
    assert!(runtime
        .drain_derived_patches(&computed)
        .derived_patches
        .is_empty());

    let issue_write = runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "summary.value",
                    test_string_aspect_value("Issue wakes computed"),
                ),
            ],
        ))
        .expect("issue write should execute");
    let issue_patches = runtime.drain_derived_patches(&computed);

    assert_eq!(
        issue_write.terminal_affected_derived_view_ids_projection(),
        &["computed.shared".to_string()]
    );
    assert_eq!(issue_write.considered_computed_view_count(), 1);
    assert_eq!(issue_patches.derived_patches.len(), 1);
    assert_eq!(
        issue_patches.derived_patches[0].aspect_touches(),
        test_aspect_touches(["issue.summary"]).as_slice()
    );
}

#[test]
fn computed_handle_inspection_reports_dependencies_aspects_and_materialization() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.inspectable", touches(["title"]))
                .depends_on_live(&live)
                .produces(touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("Inspectable task")),
            ],
        ))
        .expect("write should materialize computed output");

    let evidence = runtime
        .inspect_derived_view(&computed)
        .expect("computed handle should inspect");

    assert_eq!(evidence.name(), "computed.inspectable");
    assert_eq!(
        evidence.authority_lane(),
        ForgeQueryAuthorityLane::DerivedRuntimeState
    );
    assert_eq!(evidence.upstream_live_views(), &["tasks.table".to_string()]);
    assert!(evidence.upstream_derived_views().is_empty());
    assert_eq!(
        evidence.dependency_aspect_touches(),
        test_aspect_touches(["title"]).as_slice()
    );
    assert_eq!(
        evidence.produced_aspect_touches(),
        test_aspect_touches(["title.summary"]).as_slice()
    );
    assert!(evidence.incremental_delivery());
    assert_eq!(evidence.materialized_row_count(), 1);
    assert_eq!(evidence.pending_patch_count(), 1);
    assert_eq!(evidence.pending_incremental_patch_count(), 1);
    assert_eq!(evidence.pending_refresh_fallback_count(), 0);
    assert!(!evidence.declaration_digest().is_empty());
    assert!(!evidence.dependency_digest().is_empty());
    assert!(!evidence.produced_aspect_digest().is_empty());
    assert!(!evidence.materialization_digest().is_empty());
    assert!(!evidence.pending_patch_digest().is_empty());
    assert!(!evidence.inspection_digest().is_empty());

    let foreign_runtime = stateful_bridge_task_runtime();
    let error = foreign_runtime
        .inspect_derived_view(&computed)
        .expect_err("foreign computed handle should not inspect in another runtime");
    assert!(matches!(
        error,
        ForgeQueryRuntimeError::MissingDerivedView(_)
    ));
}

#[test]
fn nested_computed_inspection_explains_dependency_and_patch_posture() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.inspect.titles", touches(["title"]))
                .depends_on_live(&live)
                .produces(touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("source computed should declare");
    let summary = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.inspect.summary", touches(["title.summary"]))
                .depends_on_derived(&titles)
                .produces(touches(["validation.state"])),
            SummaryMaintainer,
        )
        .expect("nested computed should declare");

    let before = runtime
        .inspect_derived_view(&summary)
        .expect("nested computed should inspect before materialization");
    assert_eq!(
        before.upstream_derived_views(),
        &["computed.inspect.titles".to_string()]
    );
    assert!(before.upstream_live_views().is_empty());
    assert_eq!(
        before.dependency_aspect_touches(),
        test_aspect_touches(["title.summary"]).as_slice()
    );
    assert_eq!(
        before.produced_aspect_touches(),
        test_aspect_touches(["validation.state"]).as_slice()
    );
    assert_eq!(before.materialized_row_count(), 0);
    assert_eq!(before.pending_patch_count(), 0);
    assert!(before.incremental_delivery());

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Nested inspectable"),
                ),
            ],
        ))
        .expect("insert should update nested computeds");
    let after = runtime
        .inspect_derived_view(&summary)
        .expect("nested computed should inspect after materialization");

    assert_eq!(insert.considered_computed_view_count(), 2);
    assert_eq!(after.materialized_row_count(), 1);
    assert_eq!(after.pending_patch_count(), 1);
    assert_eq!(after.pending_incremental_patch_count(), 1);
    assert_eq!(after.pending_refresh_fallback_count(), 0);
    assert_ne!(
        after.materialization_digest(),
        before.materialization_digest()
    );
    assert_ne!(after.pending_patch_digest(), before.pending_patch_digest());
    assert_ne!(after.inspection_digest(), before.inspection_digest());
    assert_eq!(after.dependency_digest(), before.dependency_digest());
    assert_eq!(
        after.produced_aspect_digest(),
        before.produced_aspect_digest()
    );
}

#[test]
fn refresh_fallback_computed_inspection_reports_fallback_posture() {
    let mut runtime = stateful_bridge_task_runtime();
    let _: ForgeQueryLiveView<ForgeQueryNativeRow> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let computed_definition = runtime
        .declare_derived_view(
            ForgeQueryDerivedView::new("computed.inspect.refresh", touches(["title"]))
                .whole_refresh_fallback(),
        )
        .expect("refresh fallback computed should declare");
    let computed =
        ForgeQueryDerivedViewHandle::<ForgeQueryNativeRow>::new(computed_definition.name());

    let before = runtime
        .inspect_derived_view(&computed)
        .expect("refresh fallback computed should inspect before write");
    assert!(!before.incremental_delivery());

    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Refresh inspectable"),
                ),
            ],
        ))
        .expect("insert should route fallback computed patch");
    let after = runtime
        .inspect_derived_view(&computed)
        .expect("refresh fallback computed should inspect after write");

    assert!(!after.incremental_delivery());
    assert_eq!(after.pending_patch_count(), 1);
    assert_eq!(after.pending_incremental_patch_count(), 0);
    assert_eq!(after.pending_refresh_fallback_count(), 1);
    assert_eq!(
        after.materialization_digest(),
        before.materialization_digest(),
        "fallback inspection must not pretend a full materialization happened"
    );
    assert_ne!(after.pending_patch_digest(), before.pending_patch_digest());
}

#[test]
fn refresh_fallback_maintainer_rebuilds_from_retained_live_rows() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let computed = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.refresh.count", touches(["title"]))
                .depends_on_live(&live)
                .produces(touches(["summary.count"]))
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("refresh maintainer computed should declare");

    let first = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("First title")),
            ],
        ))
        .expect("first insert should route retained refresh rebuild");
    let first_patches = runtime.drain_derived_patches(&computed);

    assert_eq!(
        first.terminal_affected_derived_view_ids_projection(),
        &["computed.refresh.count".to_string()]
    );
    assert!(first.refresh_fallback());
    assert_eq!(
        read_derived_value_aspects(&runtime, &computed),
        vec![test_string_aspect_value("count:1".to_string())]
    );
    assert_eq!(first_patches.derived_patches.len(), 1);
    assert!(first_patches.derived_patches[0].is_refresh_fallback());
    assert_eq!(
        first_patches.derived_patches[0].aspect_touches(),
        test_aspect_touches(["summary.count"]).as_slice()
    );
    assert_eq!(
        retained_value_aspects(first_patches.derived_patches[0].retained_payload_rows()),
        vec![test_string_aspect_value("count:1".to_string())]
    );

    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("Second title")),
            ],
        ))
        .expect("second insert should rebuild from retained live rows");

    assert_eq!(
        read_derived_value_aspects(&runtime, &computed),
        vec![test_string_aspect_value("count:2".to_string())]
    );
}

#[test]
fn refresh_fallback_maintainer_seeds_retained_live_rows_during_declaration() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");

    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Seeded before computed declaration"),
                ),
            ],
        ))
        .expect("task insert should retain upstream live row before computed declaration");

    let computed = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.refresh.seeded", touches(["title"]))
                .depends_on_live(&live)
                .produces(touches(["summary.count"]))
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("refresh maintainer should seed from retained live rows during declaration");

    assert_eq!(
        read_derived_value_aspects(&runtime, &computed),
        vec![test_string_aspect_value("count:1".to_string())]
    );
    assert_eq!(
        runtime
            .inspect_derived_view(&computed)
            .expect("seeded refresh computed should inspect")
            .materialized_row_count(),
        1
    );
    assert!(
        runtime
            .drain_derived_patches(&computed)
            .derived_patches
            .is_empty(),
        "declaration-time retained seeding should not enqueue derived patches",
    );
}

#[test]
fn refresh_fallback_maintainer_receives_all_declared_upstream_live_rows() {
    let mut runtime = stateful_bridge_task_issue_runtime();
    let tasks = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("task live view should declare");
    let issues = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "issues.table",
            issue_live_request(),
            issue_schema(),
        )
        .expect("issue live view should declare");
    let computed = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.refresh.multi", touches(["title"]))
                .depends_on_live(&tasks)
                .depends_on_live(&issues)
                .produces(touches(["summary.count"]))
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("multi-upstream refresh maintainer should declare");

    runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "summary.value",
                    test_string_aspect_value("Sibling upstream row"),
                ),
            ],
        ))
        .expect("issue insert should seed sibling upstream");
    runtime.drain_derived_patches(&computed);

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Task wakes refresh computed"),
                ),
            ],
        ))
        .expect("task insert should rebuild from both retained upstreams");
    let patches = runtime.drain_derived_patches(&computed);

    assert_eq!(
        insert.terminal_affected_derived_view_ids_projection(),
        &["computed.refresh.multi".to_string()]
    );
    assert!(insert.refresh_fallback());
    assert_eq!(
        read_derived_value_aspects(&runtime, &computed),
        vec![test_string_aspect_value("count:2".to_string())]
    );
    assert_eq!(patches.derived_patches.len(), 1);
    assert!(patches.derived_patches[0].is_refresh_fallback());
    assert_eq!(
        retained_value_aspects(patches.derived_patches[0].retained_payload_rows()),
        vec![test_string_aspect_value("count:2".to_string())]
    );
}

#[test]
fn downstream_refresh_fallback_seeds_retained_derived_and_live_rows_during_declaration() {
    struct MixedUpstreamSnapshotMaintainer;

    impl ForgeQueryDerivedViewMaintainer for MixedUpstreamSnapshotMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_string_test_row(
                "value",
                format!(
                    "incremental:{}",
                    delta.entity_identity.terminal_projection_for_reporting()
                ),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                external_commit("mixed-upstream-incremental"),
                delta.entity_identity.clone(),
                if view.produced_aspect_touches().is_empty() {
                    delta.admitted_touched_aspects().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _refresh: &ForgeQueryRetainedRefreshContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let derived_count = upstreams
                .declared_retained_computed_row_sets(view)
                .map(<[ForgeQueryRetainedMaterializedRow]>::len)
                .sum::<usize>();
            let live_count = upstreams
                .declared_live_row_sets(view)
                .map(<[crate::memory_workspace::ForgeQueryEntity]>::len)
                .sum::<usize>();
            let row = retained_string_test_row(
                "value",
                format!("derived:{derived_count}|live:{live_count}"),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("mixed-upstream-refresh"),
                if view.produced_aspect_touches().is_empty() {
                    view.dependency_aspect_touches().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
                "retained-mixed-upstream-snapshot",
            ))
        }
    }

    let mut runtime = stateful_bridge_task_issue_runtime();
    let tasks = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("task live view should declare");
    let issues = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "issues.table",
            issue_live_request(),
            issue_schema(),
        )
        .expect("issue live view should declare");

    runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "summary.value",
                    test_string_aspect_value("Retained issue sibling"),
                ),
            ],
        ))
        .expect("issue insert should retain live sibling before declaration");
    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Retained task upstream"),
                ),
            ],
        ))
        .expect("task insert should retain live upstream before declaration");

    let refresh = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.refresh.seed.count", touches(["title"]))
                .depends_on_live(&tasks)
                .produces(touches(["summary.count"]))
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("upstream refresh maintainer should seed during declaration");
    let downstream = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new(
                "computed.refresh.seed.mixed",
                touches(["summary.count", "summary"]),
            )
            .depends_on_derived(&refresh)
            .depends_on_live(&issues)
            .produces(touches(["validation.state"]))
            .whole_refresh_fallback(),
            MixedUpstreamSnapshotMaintainer,
        )
        .expect("downstream mixed refresh maintainer should seed during declaration");

    assert_eq!(
        read_derived_value_aspects(&runtime, &downstream),
        vec![test_string_aspect_value("derived:1|live:1".to_string())]
    );
    assert!(
        runtime
            .drain_derived_patches(&downstream)
            .derived_patches
            .is_empty(),
        "declaration-time retained seeding should not enqueue downstream derived patches",
    );
}

#[test]
fn refresh_rebuilt_computed_wakes_downstream_dependencies_through_produced_aspects() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let refresh = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.refresh.count", touches(["title"]))
                .depends_on_live(&live)
                .produces(touches(["summary.count"]))
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("refresh maintainer computed should declare");
    let downstream = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.refresh.summary", touches(["summary.count"]))
                .depends_on_derived(&refresh)
                .produces(touches(["validation.state"])),
            SummaryMaintainer,
        )
        .expect("downstream computed should declare");

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                ("title.value", test_string_aspect_value("Nested refresh")),
            ],
        ))
        .expect("insert should wake refresh and downstream computeds");
    let refresh_patches = runtime.drain_derived_patches(&refresh);
    let downstream_patches = runtime.drain_derived_patches(&downstream);

    assert_eq!(
        insert.terminal_affected_derived_view_ids_projection(),
        &[
            "computed.refresh.count".to_string(),
            "computed.refresh.summary".to_string(),
        ]
    );
    assert_eq!(insert.considered_computed_view_count(), 2);
    assert_eq!(refresh_patches.derived_patches.len(), 1);
    assert!(refresh_patches.derived_patches[0].is_refresh_fallback());
    assert_eq!(
        downstream_patches.derived_patches[0].aspect_touches(),
        test_aspect_touches(["validation.state"]).as_slice()
    );
    assert_eq!(
        read_derived_value_aspects(&runtime, &downstream),
        vec![test_string_aspect_value(
            "summary:computed.refresh.count".to_string()
        )]
    );
}

#[test]
fn downstream_refresh_fallback_receives_declared_live_siblings_through_computed_dependency_chain() {
    struct MixedUpstreamSnapshotMaintainer;

    impl ForgeQueryDerivedViewMaintainer for MixedUpstreamSnapshotMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_string_test_row(
                "value",
                format!(
                    "incremental:{}",
                    delta.entity_identity.terminal_projection_for_reporting()
                ),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                external_commit("mixed-upstream-incremental"),
                delta.entity_identity.clone(),
                if view.produced_aspect_touches().is_empty() {
                    delta.admitted_touched_aspects().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _refresh: &ForgeQueryRetainedRefreshContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let derived_count = upstreams
                .declared_retained_computed_row_sets(view)
                .map(<[ForgeQueryRetainedMaterializedRow]>::len)
                .sum::<usize>();
            let live_count = upstreams
                .declared_live_row_sets(view)
                .map(<[crate::memory_workspace::ForgeQueryEntity]>::len)
                .sum::<usize>();
            let row = retained_string_test_row(
                "value",
                format!("derived:{derived_count}|live:{live_count}"),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("mixed-upstream-refresh"),
                if view.produced_aspect_touches().is_empty() {
                    view.dependency_aspect_touches().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
                "retained-mixed-upstream-snapshot",
            ))
        }
    }

    let mut runtime = stateful_bridge_task_issue_runtime();
    let tasks = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("task live view should declare");
    let issues = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "issues.table",
            issue_live_request(),
            issue_schema(),
        )
        .expect("issue live view should declare");
    let refresh = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.refresh.count", touches(["title"]))
                .depends_on_live(&tasks)
                .produces(touches(["summary.count"]))
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("upstream refresh maintainer should declare");
    let downstream = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new(
                "computed.refresh.mixed",
                touches(["summary.count", "summary"]),
            )
            .depends_on_derived(&refresh)
            .depends_on_live(&issues)
            .produces(touches(["validation.state"]))
            .whole_refresh_fallback(),
            MixedUpstreamSnapshotMaintainer,
        )
        .expect("downstream mixed refresh maintainer should declare");

    runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "summary.value",
                    test_string_aspect_value("Issue sibling stays retained"),
                ),
            ],
        ))
        .expect("issue insert should seed sibling live state");
    runtime.drain_derived_patches(&refresh);
    runtime.drain_derived_patches(&downstream);

    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Task wakes upstream refresh"),
                ),
            ],
        ))
        .expect("task insert should rebuild downstream from derived and live siblings");
    let downstream_rows = read_derived_value_aspects(&runtime, &downstream);

    assert_eq!(
        downstream_rows,
        vec![test_string_aspect_value("derived:1|live:1".to_string())]
    );
}

#[cfg(any())]
#[test]
fn refresh_fallback_maintainer_receives_retained_mutation_metadata() {
    struct MetadataSnapshotMaintainer;

    impl ForgeQueryDerivedViewMaintainer for MetadataSnapshotMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_string_test_row(
                "value",
                format!(
                    "incremental:{}",
                    delta.entity_identity.terminal_projection_for_reporting()
                ),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                external_commit("mixed-upstream-incremental"),
                delta.entity_identity.clone(),
                if view.produced_aspect_touches().is_empty() {
                    delta.admitted_touched_aspects().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _refresh: &ForgeQueryRetainedRefreshContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let derived_count = upstreams
                .declared_retained_computed_row_sets(view)
                .map(<[ForgeQueryRetainedMaterializedRow]>::len)
                .sum::<usize>();
            let live_count = upstreams
                .declared_live_row_sets(view)
                .map(<[crate::memory_workspace::ForgeQueryEntity]>::len)
                .sum::<usize>();
            let row = retained_string_test_row(
                "value",
                format!("derived:{derived_count}|live:{live_count}"),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("mixed-upstream-refresh"),
                if view.produced_aspect_touches().is_empty() {
                    view.dependency_aspect_touches().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
                "retained-mixed-upstream-snapshot",
            ))
        }
    }

    let mut runtime = stateful_bridge_task_issue_runtime();
    let tasks = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("task live view should declare");
    let issues = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "issues.table",
            issue_live_request(),
            issue_schema(),
        )
        .expect("issue live view should declare");

    runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "summary.value",
                    test_string_aspect_value("Retained issue sibling"),
                ),
            ],
        ))
        .expect("issue insert should retain live sibling before declaration");
    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", test_string_aspect_value("")),
                (
                    "title.value",
                    test_string_aspect_value("Retained task upstream"),
                ),
            ],
        ))
        .expect("task insert should retain live upstream before declaration");

    let refresh = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new(
                "computed.refresh.seed.count",
                test_aspect_touches(["title"]),
            )
            .depends_on_live(&tasks)
            .produces(test_aspect_touches(["summary.count"]))
            .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("upstream refresh maintainer should seed during declaration");
    let downstream = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new(
                "computed.refresh.seed.mixed",
                ["summary.count".to_string(), "summary".to_string()],
            )
            .depends_on_derived(&refresh)
            .depends_on_live(&issues)
            .produces(test_aspect_touches(["validation.state"]))
            .whole_refresh_fallback(),
            MixedUpstreamSnapshotMaintainer,
        )
        .expect("downstream mixed refresh maintainer should seed during declaration");

    assert_eq!(
        read_derived(&runtime, &downstream),
        vec![test_string_aspect_value("derived:1|live:1".to_string())]
    );
    assert!(
        runtime
            .drain_derived_patches(&downstream)
            .derived_patches
            .is_empty(),
        "declaration-time retained seeding should not enqueue downstream derived patches",
    );
}

#[test]
fn refresh_fallback_maintainer_receives_retained_mutation_metadata() {
    struct MetadataSnapshotMaintainer;

    impl ForgeQueryDerivedViewMaintainer for MetadataSnapshotMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_string_test_row(
                "value",
                format!(
                    "incremental:{}",
                    delta.entity_identity.terminal_projection_for_reporting()
                ),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                external_commit("metadata-snapshot-incremental"),
                delta.entity_identity.clone(),
                if view.produced_aspect_touches().is_empty() {
                    delta.admitted_touched_aspects().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            refresh: &ForgeQueryRetainedRefreshContext,
            _upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let author = refresh
                .refresh_metadata()
                .get(&test_mutation_metadata_key("author"))
                .map(|value| value.native_digest_text())
                .unwrap_or("missing");
            let row = retained_string_test_row(
                "value",
                format!(
                    "{}:{}:{}",
                    refresh
                        .refresh_identity()
                        .terminal_projection_for_reporting(),
                    terminal_touched_aspect_paths_projection(refresh.admitted_touched_aspects())
                        .join("|"),
                    author
                ),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("metadata-snapshot-refresh"),
                if view.produced_aspect_touches().is_empty() {
                    view.dependency_aspect_touches().to_vec()
                } else {
                    view.produced_aspect_touches().to_vec()
                },
                payload,
                "retained-mutation-metadata",
            ))
        }
    }

    fn terminal_touched_aspect_paths_projection(touches: &[ForgeQueryAspectTouch]) -> Vec<String> {
        touches
            .iter()
            .map(|touch| touch.admitted_touch_digest_part().to_string())
            .collect()
    }

    let mut workspace = stateful_bridge_task_runtime()
        .workspace("computed.refresh.metadata")
        .expect("task runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-metadata-table")
        })
        .expect("task live view should declare");
    let metadata: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> = workspace
        .computed_view(
            ForgeQueryDerivedView::new("computed.refresh.metadata", touches(["title"]))
                .depends_on_live(&tasks)
                .produces(touches(["summary.metadata"]))
                .whole_refresh_fallback(),
            MetadataSnapshotMaintainer,
        )
        .expect("metadata refresh maintainer should declare");

    let receipt = workspace
        .insert("Task", |builder| {
            builder.metadata("author", "worth-topo").aspect(
                touch("title.value"),
                test_string_aspect_value("Metadata proof"),
            )
        })
        .expect("task insert should succeed");

    assert_eq!(
        retained_value_aspects(
            workspace
                .materialize_result(&metadata)
                .expect("metadata materialization should execute")
                .retained_rows(),
        ),
        vec![test_string_aspect_value(format!(
            "{}:title:value:worth-topo",
            receipt
                .commit_identity()
                .terminal_projection_for_reporting()
        ))]
    );
}

#[test]
fn retained_upstreams_decode_single_computed_rows_through_query_runtime_floor() {
    let upstreams = ForgeQueryRetainedUpstreamInputs::from_retained_computed_rows(
        Vec::<(String, Vec<ForgeQueryEntity>)>::new(),
        [(
            "computed.materialized".to_string(),
            vec![retained_test_row([("count", AspectValue::UInt64(4))])],
        )],
    );

    let materialized =
        ForgeQueryDerivedViewHandle::<ForgeQueryNativeRow>::new("computed.materialized");
    let row = upstreams
        .single_retained_computed_row_for(&materialized)
        .expect("single retained computed row should be available");
    assert_eq!(retained_u64_field(row, "count"), 4);

    let missing_handle =
        ForgeQueryDerivedViewHandle::<ForgeQueryNativeRow>::new("computed.missing");
    let missing = upstreams
        .single_retained_computed_row_for(&missing_handle)
        .expect_err("missing retained row should fail closed");
    match missing {
        ForgeQueryRuntimeError::RetainedRowDecode {
            view_name, stage, ..
        } => {
            assert_eq!(view_name, "computed.missing");
            assert_eq!(stage, "retained-upstream");
        }
        other => panic!("expected retained-row decode error, got {other:?}"),
    }

    let declaration = ForgeQueryDerivedView::new("computed.consumer", touches(["count"]))
        .depends_on_derived_name_from_workspace_declaration("computed.materialized");
    let declared_row = upstreams
        .single_declared_retained_computed_row_for(&declaration, &materialized)
        .expect("declared retained upstream row should be available");
    assert_eq!(retained_u64_field(declared_row, "count"), 4);

    let undeclared_handle =
        ForgeQueryDerivedViewHandle::<ForgeQueryNativeRow>::new("computed.other");
    let undeclared = upstreams
        .single_declared_retained_computed_row_for(&declaration, &undeclared_handle)
        .expect_err("undeclared retained upstream row should fail closed");
    match undeclared {
        ForgeQueryRuntimeError::RetainedRowDecode {
            view_name, stage, ..
        } => {
            assert_eq!(view_name, "computed.other");
            assert_eq!(stage, "retained-upstream");
        }
        other => panic!("expected retained-row decode error, got {other:?}"),
    }
}

#[test]
fn derived_materialization_bundle_decodes_multiple_retained_rows_through_query_runtime_floor() {
    struct TitleRowMaintainer {
        tasks: ForgeQueryLiveView<ForgeQueryNativeRow>,
    }
    struct CountRowMaintainer {
        title_row: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
    }

    impl ForgeQueryDerivedViewMaintainer for TitleRowMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_string_test_row(
                "value",
                delta
                    .entity_identity
                    .terminal_projection_for_reporting()
                    .to_string(),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                external_commit("title-row"),
                delta.entity_identity.clone(),
                delta_or_produced_touches(view, delta),
                payload,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _refresh: &ForgeQueryRetainedRefreshContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let row = retained_string_test_row(
                "value",
                upstreams
                    .declared_live_rows_for(view, &self.tasks)
                    .and_then(|rows| rows.first())
                    .map(|entity| entity.identity().terminal_projection_for_reporting())
                    .unwrap_or_else(|| "missing".to_string()),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("title-row-refresh"),
                dependency_or_produced_touches(view),
                payload,
                "title-row-refresh",
            ))
        }
    }

    impl ForgeQueryDerivedViewMaintainer for CountRowMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            _delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_test_row([("count", AspectValue::UInt64(1))]);
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("count-row"),
                dependency_or_produced_touches(view),
                payload,
                "count-refresh",
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _refresh: &ForgeQueryRetainedRefreshContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let row = retained_test_row([(
                "count",
                AspectValue::UInt64(
                    upstreams
                        .declared_retained_computed_rows_for(view, &self.title_row)
                        .len() as u64,
                ),
            )]);
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("count-row-refresh"),
                dependency_or_produced_touches(view),
                payload,
                "count-refresh",
            ))
        }
    }

    let mut workspace = stateful_bridge_task_runtime()
        .workspace("computed.materialization.bundle")
        .expect("task runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("tasks-bundle-table")
        })
        .expect("task live view should declare");
    let title_row: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> = workspace
        .computed_view(
            ForgeQueryDerivedView::new("computed.bundle.title_row", touches(["title"]))
                .depends_on_live(&tasks)
                .produces(touches(["title.row"]))
                .whole_refresh_fallback(),
            TitleRowMaintainer {
                tasks: tasks.clone(),
            },
        )
        .expect("title-row computed should declare");
    let count_row: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> = workspace
        .computed_view(
            ForgeQueryDerivedView::new("computed.bundle.count_row", touches(["title.row"]))
                .depends_on_derived(&title_row)
                .produces(touches(["count.row"]))
                .whole_refresh_fallback(),
            CountRowMaintainer {
                title_row: title_row.clone(),
            },
        )
        .expect("count-row computed should declare");

    let receipt = workspace
        .insert("Task", |builder| {
            builder.aspect(
                touch("title.value"),
                test_string_aspect_value("Bundle proof"),
            )
        })
        .expect("task insert should succeed");
    let bundle = workspace
        .materialize_derived_artifact_bundle([
            (&title_row).into(),
            (&count_row).into(),
            (&title_row).into(),
        ])
        .expect("bundle should materialize retained computed rows");

    assert_eq!(bundle.target_count(), 2);
    assert!(bundle.includes_target(
        &crate::runtime::surface::ForgeQueryDerivedMaterializationTarget::from(&title_row)
    ));
    assert!(bundle.includes_target(
        &crate::runtime::surface::ForgeQueryDerivedMaterializationTarget::from(&count_row)
    ));
    assert_eq!(
        bundle.snapshot_identity(),
        bundle
            .materialization(&title_row)
            .expect("title-row result should stay in bundle")
            .receipt()
            .snapshot_identity()
    );
    let materialized_title = bundle
        .materialization(&title_row)
        .expect("title row should stay in bundle")
        .single_retained_row()
        .expect("title materialization should retain one row");
    let materialized_count = bundle
        .materialization(&count_row)
        .expect("count row should stay in bundle")
        .single_retained_row()
        .expect("count materialization should retain one row");
    assert_eq!(
        retained_string_field(materialized_title, "value"),
        receipt.deltas()[0]
            .entity_identity
            .terminal_projection_for_reporting()
            .to_string()
    );
    assert_eq!(retained_u64_field(materialized_count, "count"), 1);

    let missing = bundle
        .materialization_by_name("computed.bundle.missing")
        .expect_err("missing retained bundle target should fail closed");
    match missing {
        ForgeQueryRuntimeError::RetainedRowDecode {
            view_name, stage, ..
        } => {
            assert_eq!(view_name, "computed.bundle.missing");
            assert_eq!(stage, "derived-materialization-bundle");
        }
        other => panic!("expected retained-row decode error, got {other:?}"),
    }
}

#[test]
fn computed_dependency_admission_rejects_missing_or_cyclic_upstream_views() {
    let mut runtime = stateful_bridge_task_runtime();
    let missing_live = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.missing-live", touches(["title"]))
                .depends_on_live_name_from_workspace_declaration("tasks.not-declared"),
            TitleListMaintainer,
        )
        .expect_err("missing live dependency should reject before registration");
    match missing_live {
        ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
            assert!(message.contains("tasks.not-declared"));
        }
        other => panic!("expected computed declaration error, got {other:?}"),
    }

    let missing = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.missing", touches(["title.summary"]))
                .depends_on_derived_name_from_workspace_declaration("computed.unknown"),
            SummaryMaintainer,
        )
        .expect_err("missing computed dependency should reject before registration");
    match missing {
        ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
            assert!(message.contains("computed.unknown"));
        }
        other => panic!("expected computed declaration error, got {other:?}"),
    }

    let first = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.first", touches(["title"]))
                .produces(touches(["title.summary"])),
            TitleListMaintainer,
        )
        .expect("first computed should declare");
    let second = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.second", touches(["title.summary"]))
                .depends_on_derived(&first)
                .produces(touches(["validation.state"])),
            SummaryMaintainer,
        )
        .expect("second computed should declare");

    let cycle = runtime
        .declare_maintained_derived_view::<ForgeQueryNativeRow>(
            ForgeQueryDerivedView::new("computed.first", touches(["validation.state"]))
                .depends_on_derived(&second),
            SummaryMaintainer,
        )
        .expect_err("redeclared computed dependency should not create a cycle");
    match cycle {
        ForgeQueryRuntimeError::ComputedDeclaration { message, .. } => {
            assert!(message.contains("cycle"));
        }
        other => panic!("expected computed cycle declaration error, got {other:?}"),
    }
}

#[test]
fn derived_materialization_bundle_binds_one_exact_retained_artifact() {
    struct TitleRowMaintainer {
        tasks: ForgeQueryLiveView<ForgeQueryNativeRow>,
    }
    struct CountRowMaintainer {
        title_row: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
    }

    impl ForgeQueryDerivedViewMaintainer for TitleRowMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_string_test_row(
                "value",
                delta
                    .entity_identity
                    .terminal_projection_for_reporting()
                    .to_string(),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                external_commit("title-row"),
                delta.entity_identity.clone(),
                delta_or_produced_touches(view, delta),
                payload,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _refresh: &ForgeQueryRetainedRefreshContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let row = retained_string_test_row(
                "value",
                upstreams
                    .declared_live_rows_for(view, &self.tasks)
                    .and_then(|rows| rows.first())
                    .map(|entity| entity.identity().terminal_projection_for_reporting())
                    .unwrap_or_else(|| "missing".to_string()),
            );
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("title-row-refresh"),
                dependency_or_produced_touches(view),
                payload,
                "title-row-refresh",
            ))
        }
    }

    impl ForgeQueryDerivedViewMaintainer for CountRowMaintainer {
        fn maintain(
            &mut self,
            view: &ForgeQueryDerivedView,
            _delta: &crate::memory_workspace::ForgeQueryMutationDelta,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> ForgeQueryDerivedPatch {
            let row = retained_test_row([("count", AspectValue::UInt64(1))]);
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("count-row"),
                dependency_or_produced_touches(view),
                payload,
                "count-refresh",
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _refresh: &ForgeQueryRetainedRefreshContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let row = retained_test_row([(
                "count",
                AspectValue::UInt64(
                    upstreams
                        .declared_retained_computed_rows_for(view, &self.title_row)
                        .len() as u64,
                ),
            )]);
            let payload = ForgeQueryDerivedPatchPayload::from_retained_row(row.clone());
            materialization.replace_retained_rows([row]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                external_commit("count-row-refresh"),
                dependency_or_produced_touches(view),
                payload,
                "count-refresh",
            ))
        }
    }

    let runtime = stateful_bridge_task_runtime();
    let mut workspace = runtime.workspace("computed-bundle-binding").unwrap();
    let tasks: ForgeQueryLiveView<ForgeQueryNativeRow> = workspace
        .live_view("computed.bundle.binding.tasks", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("computed-bundle-binding")
        })
        .expect("task live view should declare");
    let title_row: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> = workspace
        .computed_view(
            ForgeQueryDerivedView::new("computed.bundle.binding.title_row", touches(["title"]))
                .depends_on_live(&tasks)
                .produces(touches(["title.row"]))
                .whole_refresh_fallback(),
            TitleRowMaintainer {
                tasks: tasks.clone(),
            },
        )
        .expect("title-row computed should declare");
    let count_row: ForgeQueryDerivedViewHandle<ForgeQueryNativeRow> = workspace
        .computed_view(
            ForgeQueryDerivedView::new("computed.bundle.binding.count_row", touches(["title.row"]))
                .depends_on_derived(&title_row)
                .produces(touches(["count.row"]))
                .whole_refresh_fallback(),
            CountRowMaintainer {
                title_row: title_row.clone(),
            },
        )
        .expect("count-row computed should declare");

    workspace
        .insert("Task", |builder| {
            builder.aspect(
                touch("title.value"),
                test_string_aspect_value("Artifact proof"),
            )
        })
        .expect("task insert should succeed");

    let binding = workspace
        .materialize_derived_artifact_binding(
            "computed.bundle.binding.artifact",
            [(&title_row).into(), (&count_row).into()],
        )
        .expect("exact retained artifact binding should succeed");

    assert_eq!(binding.artifact_name(), "computed.bundle.binding.artifact");
    assert_eq!(binding.target_count(), 2);
    assert_eq!(
        binding
            .terminal_target_view_names_projection()
            .collect::<Vec<_>>(),
        vec![count_row.name(), title_row.name()]
    );
    assert_eq!(
        binding.snapshot_identity(),
        binding
            .materialization(&title_row)
            .expect("title-row result should stay in artifact")
            .receipt()
            .snapshot_identity()
    );

    let title = binding
        .materialization(&title_row)
        .expect("title row should stay in retained artifact binding")
        .single_retained_row()
        .expect("title materialization should retain one row");
    let count = binding
        .materialization(&count_row)
        .expect("count row should stay in retained artifact binding")
        .single_retained_row()
        .expect("count materialization should retain one row");
    assert!(!binding.binding_for_reporting().is_empty());
    assert!(!retained_string_field(title, "value").is_empty());
    assert_eq!(retained_u64_field(count, "count"), 1);

    let mismatch = workspace
        .materialize_derived_artifact_bundle([(&title_row).into()])
        .expect("bundle should materialize retained title row")
        .bind_retained_artifact(
            "computed.bundle.binding.mismatch",
            [(&title_row).into(), (&count_row).into()],
        )
        .expect_err("missing retained target should fail closed");
    match mismatch {
        ForgeQueryRuntimeError::RetainedRowDecode {
            view_name, stage, ..
        } => {
            assert_eq!(view_name, "computed.bundle.binding.mismatch");
            assert_eq!(stage, "derived-artifact-binding");
        }
        other => panic!("expected retained-row decode error, got {other:?}"),
    }
}
