use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::runtime::WorthQueryWorkspace;

mod live_journey {
    use worth_query::facade::live::{
        current, declare, AspectFieldSelector, AspectName, AuthoredResultShapeField, FieldName,
        QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryManagedLiveCloseOutcome,
        WorthQueryManagedLiveDeliveryCauseKind, WorthQueryManagedLiveLifecyclePosture,
        WorthQueryOrdinaryRuntimePostureKind,
    };

    #[test]
    fn facade_live_completes_declare_context_open_observe_delivery_and_close() {
        let mut workspace = super::live_workspace();
        let opened = declare("tasks.public", |read| {
            read.local_collection(
                "Task",
                QuerySchemaView::new(
                    "public-live-task",
                    [SchemaFieldView::new(
                        AspectName::new("identity").expect("aspect should build"),
                        FieldName::new("id").expect("field should build"),
                        SchemaFieldKind::String,
                    )],
                    [],
                ),
                |query| {
                    query.project(
                        AspectFieldSelector::new("identity", "id").expect("selector should build"),
                    )
                },
                |shape| {
                    shape.field(
                        AuthoredResultShapeField::new("identity", "id", "identity.id")
                            .expect("result field should build"),
                    )
                },
            )
        })
        .expect("live declaration should author")
        .using(current())
        .open(&mut workspace);
        let handle = match opened {
            worth_query::facade::live::WorthQueryLiveOpenOutcome::Opened(completion) => {
                completion.into_handle()
            }
            worth_query::facade::live::WorthQueryLiveOpenOutcome::Stopped(stop) => {
                panic!("public live open unexpectedly stopped: {:?}", stop.source())
            }
        };

        let observation = handle
            .observe(&mut workspace)
            .expect("public live lifecycle should be observable");
        assert_eq!(
            observation.posture(),
            WorthQueryManagedLiveLifecyclePosture::Active
        );
        assert_eq!(
            observation.runtime_posture().kind(),
            WorthQueryOrdinaryRuntimePostureKind::Current
        );
        super::insert_task(&mut workspace);
        let delivery = handle
            .drain(&mut workspace)
            .expect("public typed delivery should be observable");
        assert_eq!(delivery.batches().len(), 1);
        assert_eq!(
            delivery.batches()[0].cause_kind(),
            WorthQueryManagedLiveDeliveryCauseKind::RelationalChange
        );

        match handle.close(&mut workspace) {
            WorthQueryManagedLiveCloseOutcome::Closed(receipt) => {
                assert_eq!(receipt.resource_name(), "tasks.public");
                assert!(receipt.lane_terminal());
            }
            WorthQueryManagedLiveCloseOutcome::Stopped(stop) => {
                panic!("public live close unexpectedly stopped: {:?}", stop.error())
            }
        }
    }
}

fn insert_task(workspace: &mut WorthQueryWorkspace) {
    workspace
        .insert("Task", |task| {
            task.set_aspect(
                worth_query::facade::runtime::WorthQueryAspectTouch::aspect_field_path(
                    AspectKey::new("identity").expect("identity aspect should build"),
                    CanonicalFieldPath::new([
                        FieldKey::new("id").expect("identity field should build")
                    ])
                    .expect("identity path should build"),
                ),
                worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string("task-1"),
            )
        })
        .expect("public live delivery fixture should write through the real runtime");
}

fn live_workspace() -> WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should build");
    in_memory_test_runtime()
        .with_schema(schema)
        .workspace("declarative-live-public-dx")
        .expect("public live test workspace should build")
}
