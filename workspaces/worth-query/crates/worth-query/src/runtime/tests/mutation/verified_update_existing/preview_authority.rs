use super::*;

#[test]
fn preview_update_existing_verified_requires_authoritative_lane() {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("tasks.preview-update-existing-verified")
        .expect("task runtime should open a named workspace");
    let _: WorthQueryLiveView<WorthQueryUnrefinedLiveShape> = workspace
        .live_view("tasks.preview-update-existing-verified-table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                    crate::authoring::AspectFieldKey::from_authoring_parts("status", "value")
                        .unwrap(),
                ])
                .order_by(
                    crate::authoring::AspectFieldKey::from_authoring_parts("identity", "id")
                        .unwrap(),
                )
                .schema_basis("tasks-preview-update-existing-verified-table")
        })
        .expect("live view should declare");

    let seed = workspace
        .insert("Task", |task| {
            task.set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-1"),
            )
            .set_aspect(
                test_aspect_touch("status.value"),
                test_authored_string_aspect_value("open"),
            )
        })
        .expect("seed insert should execute");
    let mut preview = workspace
        .preview(test_session_label("update-existing-verified-preview"))
        .expect("preview should open");
    let binding = preview
        .bind_existing_entity(
            WorthQueryExistingEntityTarget::new(
                crate::runtime::WorthQueryMutationAuthorityIdentity::existing_truth_binding_authority(crate::runtime::WorthQueryExistingTruthBindingAuthorityLabel::new("authority:task-1").expect("existing-truth authority label")).expect("existing-truth authority identity"),
                seed.deltas()[0].entity_identity.clone(),
            )
            .expect("existing entity target should build")
            .in_target_collection("Task")
            .expect("existing entity target collection should build"),
        )
        .expect("binding should build");

    let error = preview
        .update_existing_verified(
            binding,
            |task| {
                task.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("open"),
                )
            },
            |task| {
                task.set_aspect(
                    test_aspect_touch("status.value"),
                    test_authored_string_aspect_value("closed"),
                )
            },
        )
        .expect_err("preview verified update should require authoritative lane");

    match error.stop_class() {
        WorthQueryStopClass::ExistingTruthAssertionRequiresAuthorityLane { required_lane } => {
            assert_eq!(required_lane, WorthQueryAuthorityLane::AuthoritativeTruth);
        }
        other => panic!("expected typed authoritative lane stop class, got {other:?}"),
    }
}
