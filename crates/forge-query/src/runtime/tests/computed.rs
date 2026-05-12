use super::support::*;

#[test]
fn derived_view_receives_narrow_or_fallback_patch_notes() {
    let mut runtime = stateful_bridge_task_runtime();
    let _: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    runtime
        .declare_derived_view(
            ForgeQueryDerivedView::new("task_titles", ["title".to_string()])
                .whole_refresh_fallback(),
        )
        .expect("derived view should declare");
    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Derived task")),
            ],
        ))
        .expect("insert should route to derived view");
    let update = runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: insert.deltas()[0].entity_identity.clone(),
            aspect_path: "title.value".to_string(),
            value: Value::String("Derived task renamed".to_string()),
        })
        .expect("title update should route to derived view");

    let patches = runtime.drain_derived_patches("task_titles");

    assert_eq!(
        update.affected_derived_view_ids(),
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
    let _: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("task_titles", ["title".to_string()]),
            TitleListMaintainer,
        )
        .expect("maintained derived view should declare");

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("First title")),
            ],
        ))
        .expect("insert should route derived patch");
    let patches = runtime.drain_derived_patches(titles.name());

    assert_eq!(
        insert.affected_derived_view_ids(),
        &["task_titles".to_string()]
    );
    let expected_row = Value::String(insert.deltas()[0].entity_identity.clone());
    assert_eq!(runtime.read_derived(&titles), vec![expected_row.clone()]);
    assert_eq!(patches.derived_patches.len(), 1);
    assert_eq!(patches.derived_patches[0].payload(), &expected_row);

    runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: insert.deltas()[0].entity_identity.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("ignored".to_string()),
        })
        .expect("irrelevant update should not route derived patch");
    let irrelevant = runtime.drain_derived_patches(titles.name());

    assert!(irrelevant.derived_patches.is_empty());
}

#[test]
fn nested_computed_views_route_in_deterministic_dependency_order() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.titles", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("source computed view should declare");
    let summary = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.summary", ["title.summary".to_string()])
                .depends_on_derived(&titles)
                .produces(["validation.state".to_string()]),
            SummaryMaintainer,
        )
        .expect("nested computed view should declare");

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Nested title")),
            ],
        ))
        .expect("insert should update nested computeds");
    let title_patches = runtime.drain_derived_patches(titles.name());
    let summary_patches = runtime.drain_derived_patches(summary.name());

    assert_eq!(
        insert.affected_derived_view_ids(),
        &[
            "computed.summary".to_string(),
            "computed.titles".to_string()
        ]
    );
    assert_eq!(insert.considered_computed_view_count(), 2);
    assert_eq!(title_patches.derived_patches.len(), 1);
    assert_eq!(
        title_patches.derived_patches[0].aspect_paths(),
        &["title.summary".to_string()]
    );
    assert_eq!(summary_patches.derived_patches.len(), 1);
    assert_eq!(
        summary_patches.derived_patches[0].aspect_paths(),
        &["validation.state".to_string()]
    );
    assert_eq!(
        runtime.read_derived(&summary),
        vec![Value::String(format!(
            "summary:{}",
            insert.deltas()[0].entity_identity
        ))]
    );

    runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: insert.deltas()[0].entity_identity.clone(),
            aspect_path: "identity.id".to_string(),
            value: Value::String("ignored".to_string()),
        })
        .expect("irrelevant update should still write");
    assert!(runtime
        .drain_derived_patches(titles.name())
        .derived_patches
        .is_empty());
    assert!(runtime
        .drain_derived_patches(summary.name())
        .derived_patches
        .is_empty());
}

#[test]
fn computed_dependency_index_replaces_redeclared_view_membership() {
    let mut runtime = stateful_bridge_task_issue_runtime();
    let task_live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("task live should declare");
    let issue_live = runtime
        .declare_live_view::<Value>("issues.table", issue_live_request(), issue_schema())
        .expect("issue live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.shared", ["title".to_string()])
                .depends_on_live(&task_live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("task-backed computed should declare");

    runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.shared", ["summary".to_string()])
                .depends_on_live(&issue_live)
                .produces(["issue.summary".to_string()]),
            SummaryMaintainer,
        )
        .expect("redeclared computed should replace old dependency index membership");

    let task_write = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                (
                    "title.value",
                    json!("Task should not wake redeclared computed"),
                ),
            ],
        ))
        .expect("task write should execute");
    assert!(task_write.affected_derived_view_ids().is_empty());
    assert_eq!(task_write.considered_computed_view_count(), 0);
    assert!(runtime
        .drain_derived_patches(computed.name())
        .derived_patches
        .is_empty());

    let issue_write = runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", json!("")),
                ("summary.value", json!("Issue wakes computed")),
            ],
        ))
        .expect("issue write should execute");
    let issue_patches = runtime.drain_derived_patches(computed.name());

    assert_eq!(
        issue_write.affected_derived_view_ids(),
        &["computed.shared".to_string()]
    );
    assert_eq!(issue_write.considered_computed_view_count(), 1);
    assert_eq!(issue_patches.derived_patches.len(), 1);
    assert_eq!(
        issue_patches.derived_patches[0].aspect_paths(),
        &["issue.summary".to_string()]
    );
}

#[test]
fn computed_handle_inspection_reports_dependencies_aspects_and_materialization() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.inspectable", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("computed should declare");
    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Inspectable task")),
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
    assert_eq!(evidence.dependency_aspects(), &["title".to_string()]);
    assert_eq!(evidence.produced_aspects(), &["title.summary".to_string()]);
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
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let titles = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.inspect.titles", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("source computed should declare");
    let summary = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.inspect.summary", ["title.summary".to_string()])
                .depends_on_derived(&titles)
                .produces(["validation.state".to_string()]),
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
    assert_eq!(before.dependency_aspects(), &["title.summary".to_string()]);
    assert_eq!(before.produced_aspects(), &["validation.state".to_string()]);
    assert_eq!(before.materialized_row_count(), 0);
    assert_eq!(before.pending_patch_count(), 0);
    assert!(before.incremental_delivery());

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Nested inspectable")),
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
    let _: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let computed_definition = runtime
        .declare_derived_view(
            ForgeQueryDerivedView::new("computed.inspect.refresh", ["title".to_string()])
                .whole_refresh_fallback(),
        )
        .expect("refresh fallback computed should declare");
    let computed = ForgeQueryDerivedViewHandle::<Value>::new(computed_definition.name());

    let before = runtime
        .inspect_derived_view(&computed)
        .expect("refresh fallback computed should inspect before write");
    assert!(!before.incremental_delivery());

    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Refresh inspectable")),
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
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.refresh.count", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["summary.count".to_string()])
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("refresh maintainer computed should declare");

    let first = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("First title")),
            ],
        ))
        .expect("first insert should route retained refresh rebuild");
    let first_patches = runtime.drain_derived_patches(computed.name());

    assert_eq!(
        first.affected_derived_view_ids(),
        &["computed.refresh.count".to_string()]
    );
    assert!(first.refresh_fallback());
    assert_eq!(
        runtime.read_derived(&computed),
        vec![Value::String("count:1".to_string())]
    );
    assert_eq!(first_patches.derived_patches.len(), 1);
    assert!(first_patches.derived_patches[0].is_refresh_fallback());
    assert_eq!(
        first_patches.derived_patches[0].aspect_paths(),
        &["summary.count".to_string()]
    );
    assert_eq!(
        first_patches.derived_patches[0].payload(),
        &Value::String("count:1".to_string())
    );

    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Second title")),
            ],
        ))
        .expect("second insert should rebuild from retained live rows");

    assert_eq!(
        runtime.read_derived(&computed),
        vec![Value::String("count:2".to_string())]
    );
}

#[test]
fn refresh_fallback_maintainer_receives_all_declared_upstream_live_rows() {
    let mut runtime = stateful_bridge_task_issue_runtime();
    let tasks = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("task live view should declare");
    let issues = runtime
        .declare_live_view::<Value>("issues.table", issue_live_request(), issue_schema())
        .expect("issue live view should declare");
    let computed = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.refresh.multi", ["title".to_string()])
                .depends_on_live(&tasks)
                .depends_on_live(&issues)
                .produces(["summary.count".to_string()])
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("multi-upstream refresh maintainer should declare");

    runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", json!("")),
                ("summary.value", json!("Sibling upstream row")),
            ],
        ))
        .expect("issue insert should seed sibling upstream");
    runtime.drain_derived_patches(computed.name());

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Task wakes refresh computed")),
            ],
        ))
        .expect("task insert should rebuild from both retained upstreams");
    let patches = runtime.drain_derived_patches(computed.name());

    assert_eq!(
        insert.affected_derived_view_ids(),
        &["computed.refresh.multi".to_string()]
    );
    assert!(insert.refresh_fallback());
    assert_eq!(
        runtime.read_derived(&computed),
        vec![Value::String("count:2".to_string())]
    );
    assert_eq!(patches.derived_patches.len(), 1);
    assert!(patches.derived_patches[0].is_refresh_fallback());
    assert_eq!(
        patches.derived_patches[0].payload(),
        &Value::String("count:2".to_string())
    );
}

#[test]
fn refresh_rebuilt_computed_wakes_downstream_dependencies_through_produced_aspects() {
    let mut runtime = stateful_bridge_task_runtime();
    let live = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let refresh = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.refresh.count", ["title".to_string()])
                .depends_on_live(&live)
                .produces(["summary.count".to_string()])
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("refresh maintainer computed should declare");
    let downstream = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.refresh.summary", ["summary.count".to_string()])
                .depends_on_derived(&refresh)
                .produces(["validation.state".to_string()]),
            SummaryMaintainer,
        )
        .expect("downstream computed should declare");

    let insert = runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Nested refresh")),
            ],
        ))
        .expect("insert should wake refresh and downstream computeds");
    let refresh_patches = runtime.drain_derived_patches(refresh.name());
    let downstream_patches = runtime.drain_derived_patches(downstream.name());

    assert_eq!(
        insert.affected_derived_view_ids(),
        &[
            "computed.refresh.count".to_string(),
            "computed.refresh.summary".to_string(),
        ]
    );
    assert_eq!(insert.considered_computed_view_count(), 2);
    assert_eq!(refresh_patches.derived_patches.len(), 1);
    assert!(refresh_patches.derived_patches[0].is_refresh_fallback());
    assert_eq!(
        downstream_patches.derived_patches[0].aspect_paths(),
        &["validation.state".to_string()]
    );
    assert_eq!(
        runtime.read_derived(&downstream),
        vec![Value::String("summary:computed.refresh.count".to_string())]
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
            let row = Value::String(format!("incremental:{}", delta.entity_identity));
            materialization.replace_rows([row.clone()]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                "mixed-upstream-incremental",
                delta.entity_identity.clone(),
                if view.produced_aspects().is_empty() {
                    delta.aspect_paths.clone()
                } else {
                    view.produced_aspects().to_vec()
                },
                row,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            _mutation: &ForgeQueryRetainedMutationContext,
            upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let derived_count = upstreams
                .computed_view_names()
                .flat_map(|view_name| upstreams.computed_rows(view_name).into_iter().flatten())
                .count();
            let live_count = upstreams
                .live_view_names()
                .flat_map(|view_name| upstreams.live_rows(view_name).into_iter().flatten())
                .count();
            let row = Value::String(format!("derived:{derived_count}|live:{live_count}"));
            materialization.replace_rows([row.clone()]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                "mixed-upstream-refresh",
                if view.produced_aspects().is_empty() {
                    view.dependency_aspects().to_vec()
                } else {
                    view.produced_aspects().to_vec()
                },
                row,
                "retained-mixed-upstream-snapshot",
            ))
        }
    }

    let mut runtime = stateful_bridge_task_issue_runtime();
    let tasks = runtime
        .declare_live_view::<Value>("tasks.table", task_live_request(), task_schema())
        .expect("task live view should declare");
    let issues = runtime
        .declare_live_view::<Value>("issues.table", issue_live_request(), issue_schema())
        .expect("issue live view should declare");
    let refresh = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.refresh.count", ["title".to_string()])
                .depends_on_live(&tasks)
                .produces(["summary.count".to_string()])
                .whole_refresh_fallback(),
            RefreshCountMaintainer,
        )
        .expect("upstream refresh maintainer should declare");
    let downstream = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new(
                "computed.refresh.mixed",
                ["summary.count".to_string(), "summary".to_string()],
            )
            .depends_on_derived(&refresh)
            .depends_on_live(&issues)
            .produces(["validation.state".to_string()])
            .whole_refresh_fallback(),
            MixedUpstreamSnapshotMaintainer,
        )
        .expect("downstream mixed refresh maintainer should declare");

    runtime
        .write(insert_command(
            "Issue",
            [
                ("identity.id", json!("")),
                ("summary.value", json!("Issue sibling stays retained")),
            ],
        ))
        .expect("issue insert should seed sibling live state");
    runtime.drain_derived_patches(refresh.name());
    runtime.drain_derived_patches(downstream.name());

    runtime
        .write(insert_command(
            "Task",
            [
                ("identity.id", json!("")),
                ("title.value", json!("Task wakes upstream refresh")),
            ],
        ))
        .expect("task insert should rebuild downstream from derived and live siblings");
    let downstream_rows = runtime.read_derived(&downstream);

    assert_eq!(
        downstream_rows,
        vec![Value::String("derived:1|live:1".to_string())]
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
            let row = Value::String(format!("incremental:{}", delta.entity_identity));
            materialization.replace_rows([row.clone()]);
            ForgeQueryDerivedPatch::incremental(
                view.name(),
                "metadata-snapshot-incremental",
                delta.entity_identity.clone(),
                if view.produced_aspects().is_empty() {
                    delta.aspect_paths.clone()
                } else {
                    view.produced_aspects().to_vec()
                },
                row,
            )
        }

        fn refresh_from_upstreams(
            &mut self,
            view: &ForgeQueryDerivedView,
            mutation: &ForgeQueryRetainedMutationContext,
            _upstreams: &ForgeQueryRetainedUpstreamInputs,
            materialization: &mut ForgeQueryDerivedViewMaterialization,
        ) -> Option<ForgeQueryDerivedPatch> {
            let author = mutation
                .mutation_metadata()
                .get("author")
                .and_then(Value::as_str)
                .unwrap_or("missing");
            let row = Value::String(format!(
                "{}:{}:{}",
                mutation.commit_identity(),
                mutation.touched_aspect_paths().join("|"),
                author
            ));
            materialization.replace_rows([row.clone()]);
            Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
                view.name(),
                "metadata-snapshot-refresh",
                if view.produced_aspects().is_empty() {
                    view.dependency_aspects().to_vec()
                } else {
                    view.produced_aspects().to_vec()
                },
                row,
                "retained-mutation-metadata",
            ))
        }
    }

    let mut workspace = stateful_bridge_task_runtime()
        .workspace("computed.refresh.metadata")
        .expect("task runtime should open a named workspace");
    let tasks: ForgeQueryLiveView<Value> = workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select(["identity.id", "title.value"])
                .order_by("title.value")
                .schema_basis("tasks-metadata-table")
        })
        .expect("task live view should declare");
    let metadata: ForgeQueryDerivedViewHandle<Value> = workspace
        .computed_view(
            ForgeQueryDerivedView::new("computed.refresh.metadata", ["title".to_string()])
                .depends_on_live(&tasks)
                .produces(["summary.metadata".to_string()])
                .whole_refresh_fallback(),
            MetadataSnapshotMaintainer,
        )
        .expect("metadata refresh maintainer should declare");

    let receipt = workspace
        .insert("Task", |builder| {
            builder
                .metadata("author", "worth-topo")
                .aspect("title.value", "Metadata proof")
        })
        .expect("task insert should succeed");

    assert_eq!(
        workspace.materialize(&metadata),
        vec![Value::String(format!(
            "{}:title.value:worth-topo",
            receipt.commit_identity()
        ))]
    );
}

#[test]
fn computed_dependency_admission_rejects_missing_or_cyclic_upstream_views() {
    let mut runtime = stateful_bridge_task_runtime();
    let missing_live = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.missing-live", ["title".to_string()])
                .depends_on_live_name("tasks.not-declared"),
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
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.missing", ["title.summary".to_string()])
                .depends_on_derived_name("computed.unknown"),
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
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.first", ["title".to_string()])
                .produces(["title.summary".to_string()]),
            TitleListMaintainer,
        )
        .expect("first computed should declare");
    let second = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.second", ["title.summary".to_string()])
                .depends_on_derived(&first)
                .produces(["validation.state".to_string()]),
            SummaryMaintainer,
        )
        .expect("second computed should declare");

    let cycle = runtime
        .declare_maintained_derived_view::<Value>(
            ForgeQueryDerivedView::new("computed.first", ["validation.state".to_string()])
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
