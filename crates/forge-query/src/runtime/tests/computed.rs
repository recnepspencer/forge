use super::support::*;

#[test]
fn derived_view_receives_narrow_or_fallback_patch_notes() {
    let mut runtime = task_runtime();
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Derived task" },
            }),
        })
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
    let mut runtime = task_runtime();
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "First title" },
            }),
        })
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
    let mut runtime = task_runtime();
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Nested title" },
            }),
        })
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
    let mut runtime = task_issue_memory_runtime();
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Task should not wake redeclared computed" },
            }),
        })
        .expect("task write should execute");
    assert!(task_write.affected_derived_view_ids().is_empty());
    assert_eq!(task_write.considered_computed_view_count(), 0);
    assert!(runtime
        .drain_derived_patches(computed.name())
        .derived_patches
        .is_empty());

    let issue_write = runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Issue".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "summary": { "value": "Issue wakes computed" },
            }),
        })
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
    let mut runtime = task_runtime();
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Inspectable task" },
            }),
        })
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

    let foreign_runtime = task_runtime();
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
    let mut runtime = task_runtime();
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
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Nested inspectable" },
            }),
        })
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
    let mut runtime = task_runtime();
    let _: ForgeQueryLiveView<Value> = runtime
        .declare_live_view("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");
    let computed_declaration = runtime
        .declare_derived_view(
            ForgeQueryDerivedView::new("computed.inspect.refresh", ["title".to_string()])
                .whole_refresh_fallback(),
        )
        .expect("refresh fallback computed should declare");
    let computed = ForgeQueryDerivedViewHandle::<Value>::new(computed_declaration.name());

    let before = runtime
        .inspect_derived_view(&computed)
        .expect("refresh fallback computed should inspect before write");
    assert!(!before.incremental_delivery());

    runtime
        .write(ForgeQueryWriteCommand::Insert {
            collection: "Task".to_string(),
            payload: json!({
                "identity": { "id": "" },
                "title": { "value": "Refresh inspectable" },
            }),
        })
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
fn computed_dependency_admission_rejects_missing_or_cyclic_upstream_views() {
    let mut runtime = task_runtime();
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
