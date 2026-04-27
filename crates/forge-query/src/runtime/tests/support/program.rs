use super::*;

pub(in crate::runtime::tests) struct FakeDsl;

pub(in crate::runtime::tests) struct FakeSchemaAdapter;
pub(in crate::runtime::tests) fn preview_safe_program() -> ForgeQueryProgram {
    ForgeQueryProgram::new(
        "fake.preview.safe",
        [ForgeQueryOperation::new("create_task")
            .with_input(ForgeQueryTypedPort::new(
                "title",
                ForgeQueryPortType::String,
            ))
            .requires(ForgeQueryAuthorityRequirement::Live)
            .requires(ForgeQueryAuthorityRequirement::Writeback)
            .with_effect(ForgeQueryProgramEffect::WriteTemplate(
                ForgeQueryWriteCommandTemplate::Insert {
                    collection: "Task".to_string(),
                    payload: ForgeQueryValueExpr::object([
                        (
                            "identity".to_string(),
                            ForgeQueryValueExpr::object([(
                                "id".to_string(),
                                ForgeQueryValueExpr::literal(Value::String(String::new())),
                            )]),
                        ),
                        (
                            "title".to_string(),
                            ForgeQueryValueExpr::object([(
                                "value".to_string(),
                                ForgeQueryValueExpr::input("title"),
                            )]),
                        ),
                    ]),
                },
            ))
            .with_effect(ForgeQueryProgramEffect::ReadLive {
                view_name: "tasks.table".to_string(),
            })
            .with_effect(ForgeQueryProgramEffect::DrainPatches {
                view_name: "tasks.table".to_string(),
            })],
    )
    .expect("preview-safe test program should build")
}

pub(in crate::runtime::tests) struct TitleListMaintainer;
pub(in crate::runtime::tests) struct SummaryMaintainer;

impl ForgeQueryDerivedViewMaintainer for TitleListMaintainer {
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
            "derived-test-commit",
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

impl ForgeQueryDerivedViewMaintainer for SummaryMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &crate::memory_workspace::ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let row = Value::String(format!("summary:{}", delta.entity_identity));
        materialization.replace_rows([row.clone()]);
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            "derived-summary-commit",
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

impl ForgeQuerySchemaAdapter for FakeSchemaAdapter {
    fn schema_view(&self, operation_id: &str) -> Option<QuerySchemaView> {
        (operation_id == "create_task").then(task_schema)
    }
}

impl ForgeQueryProgramSource for FakeDsl {
    fn compile_program<A>(
        self,
        schema_adapter: &A,
    ) -> Result<ForgeQueryProgram, ForgeQueryProgramError>
    where
        A: ForgeQuerySchemaAdapter + ?Sized,
    {
        let schema_view = schema_adapter
            .schema_view("create_task")
            .ok_or_else(|| ForgeQueryProgramError::new("missing schema for create_task"))?;
        ForgeQueryProgram::new(
            "fake.strict.dsl",
            [ForgeQueryOperation::new("create_task")
                .with_input(ForgeQueryTypedPort::new(
                    "title",
                    ForgeQueryPortType::String,
                ))
                .requires(ForgeQueryAuthorityRequirement::Live)
                .requires(ForgeQueryAuthorityRequirement::Writeback)
                .with_effect(ForgeQueryProgramEffect::DeclareLiveView {
                    name: "tasks.table".to_string(),
                    request: task_live_request(),
                    schema_view,
                })
                .with_effect(ForgeQueryProgramEffect::WriteTemplate(
                    ForgeQueryWriteCommandTemplate::Insert {
                        collection: "Task".to_string(),
                        payload: ForgeQueryValueExpr::object([
                            (
                                "identity".to_string(),
                                ForgeQueryValueExpr::object([(
                                    "id".to_string(),
                                    ForgeQueryValueExpr::literal(Value::String(String::new())),
                                )]),
                            ),
                            (
                                "title".to_string(),
                                ForgeQueryValueExpr::object([(
                                    "value".to_string(),
                                    ForgeQueryValueExpr::input("title"),
                                )]),
                            ),
                        ]),
                    },
                ))
                .with_effect(ForgeQueryProgramEffect::ReadLive {
                    view_name: "tasks.table".to_string(),
                })
                .with_effect(ForgeQueryProgramEffect::DrainPatches {
                    view_name: "tasks.table".to_string(),
                })],
        )
    }
}
