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
                ForgeQueryWriteCommandTemplate::InsertAspects {
                    collection: "Task".to_string(),
                    aspects: vec![
                        ForgeQueryAspectValueTemplate::new(
                            "identity.id",
                            ForgeQueryValueExpr::literal(Value::String(String::new())),
                        ),
                        ForgeQueryAspectValueTemplate::new(
                            "title.value",
                            ForgeQueryValueExpr::input("title"),
                        ),
                    ],
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
pub(in crate::runtime::tests) struct RefreshCountMaintainer;

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

impl ForgeQueryDerivedViewMaintainer for RefreshCountMaintainer {
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
            "refresh-count-incremental",
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
        let count = upstreams
            .live_view_names()
            .flat_map(|view_name| upstreams.live_rows(view_name).into_iter().flatten())
            .count();
        let row = Value::String(format!("count:{count}"));
        materialization.replace_rows([row.clone()]);
        Some(ForgeQueryDerivedPatch::whole_refresh_materialized(
            view.name(),
            "refresh-count-rebuild",
            if view.produced_aspects().is_empty() {
                view.dependency_aspects().to_vec()
            } else {
                view.produced_aspects().to_vec()
            },
            row,
            "retained-live-snapshot-rebuild",
        ))
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
                    ForgeQueryWriteCommandTemplate::InsertAspects {
                        collection: "Task".to_string(),
                        aspects: vec![
                            ForgeQueryAspectValueTemplate::new(
                                "identity.id",
                                ForgeQueryValueExpr::literal(Value::String(String::new())),
                            ),
                            ForgeQueryAspectValueTemplate::new(
                                "title.value",
                                ForgeQueryValueExpr::input("title"),
                            ),
                        ],
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
