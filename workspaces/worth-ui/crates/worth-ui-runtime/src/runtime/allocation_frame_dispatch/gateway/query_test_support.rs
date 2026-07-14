use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryReadBuilder, WorthQueryReadDenial,
    WorthQueryWorkspace,
};
use worth_query::facade::read::{
    current, declare, project_facts, WorthQueryProjectionOutcome, WorthQueryReadCompletion,
};
use worth_query::facade::foundation::{
    snapshot_resolution_report,
    AspectFieldSelector,
    AuthoredResultShapeField,
    EqualityPredicate,
    ProjectionAuthorityContract,
    ProjectionFactFieldPath,
    ScalarPredicateValue,
};
use worth_query::facade::certification::{
    consume_projection_contract_for_certification,
    ordinary_query_context_advisory_for_certification,
    resolve_runtime_current_snapshot_basis_for_certification,
};
use worth_query::facade::runtime::{
    WorthQueryAspectTouch,
    WorthQueryAuthoredAspectValue,
};
use worth_ui_query_binding::{WorthUiQueryBindingSubsystem, WorthUiQueryPrerequisiteEvidence};

pub(super) fn query_projection_consumption(
    label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, WorthQueryProjectionOutcome) {
    let (prerequisites, completion) = ordinary_completion(label);
    let outcome = completion.consume_projection(project_facts().display_field(query_size_path()));
    (prerequisites, outcome)
}

pub(super) fn partial_query_projection_consumption(
    label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, WorthQueryProjectionOutcome) {
    let (prerequisites, completion) = ordinary_completion(label);
    let outcome = completion.consume_projection(project_facts().display_field(query_size_path()));
    (
        prerequisites,
        ordinary_query_context_advisory_for_certification(outcome),
    )
}

pub(super) fn unsupported_query_projection_consumption(
    label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, WorthQueryProjectionOutcome) {
    let (prerequisites, completion) = ordinary_completion(label);
    let outcome = consume_projection_contract_for_certification(
        &completion,
        authority_contract().require_target_identity(),
    );
    (prerequisites, outcome)
}

fn ordinary_completion(
    label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, WorthQueryReadCompletion) {
    let (mut workspace, schema_basis_authority) = workspace_and_schema(label);
    let completion = declare(query_size_graph)
        .expect("ordinary Query declaration")
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect("ordinary Query read");
    (
        prerequisites_from_schema(&workspace, schema_basis_authority),
        completion,
    )
}

fn workspace_and_schema(
    label: &str,
) -> (
    WorthQueryWorkspace,
    worth_query::facade::foundation::QuerySchemaBasisAuthority,
) {
    let schema = WorthQueryTestBackendSchema::single_collection("task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("size.value", "size.value")
        .expect("size aspect");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(&format!("worth-ui.phase4.{label}"))
        .expect("Query workspace");
    workspace
        .insert("task", |task| {
            task.set_aspect(
                query_touch("identity.id"),
                WorthQueryAuthoredAspectValue::string("task"),
            )
            .set_aspect(
                query_touch("size.value"),
                WorthQueryAuthoredAspectValue::string("240"),
            )
        })
        .expect("Query insert");
    (workspace, query_schema().basis_authority())
}

fn prerequisites_from_basis(
    basis: worth_query::facade::foundation::ResolvedSnapshotBasis,
) -> WorthUiQueryPrerequisiteEvidence {
    WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("Query prerequisites")
}

fn prerequisites_from_schema(
    workspace: &WorthQueryWorkspace,
    schema_basis_authority: worth_query::facade::foundation::QuerySchemaBasisAuthority,
) -> WorthUiQueryPrerequisiteEvidence {
    let basis = resolve_runtime_current_snapshot_basis_for_certification(
        &workspace.snapshot_identity().evidence_identity(),
        schema_basis_authority,
    )
    .expect("Query basis");
    prerequisites_from_basis(basis)
}

fn authority_contract() -> ProjectionAuthorityContract {
    ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
}

fn query_size_graph<Output>(
    read: WorthQueryReadBuilder<Output>,
) -> Result<Output, WorthQueryReadDenial> {
    read.local_detail(
        "task",
        query_schema(),
        |query| {
            query
                .where_equal(
                    EqualityPredicate::new(
                        "identity",
                        "id",
                        ScalarPredicateValue::String("task".into()),
                    )
                    .expect("predicate"),
                )
                .project(AspectFieldSelector::new("size", "value").expect("selector"))
        },
        |shape| {
            shape.field(
                AuthoredResultShapeField::new("size", "value", "size.value").expect("result field"),
            )
        },
    )
}

fn query_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "task",
        [
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("identity").expect("aspect"),
                worth_query::facade::foundation::FieldName::new("id").expect("field"),
                SchemaFieldKind::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("size").expect("aspect"),
                worth_query::facade::foundation::FieldName::new("value").expect("field"),
                SchemaFieldKind::String,
            ),
        ],
        [],
    )
}

fn query_size_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("size").expect("field"),
            FieldKey::new("value").expect("field"),
        ])
        .expect("path"),
    )
}

fn query_touch(text: &str) -> WorthQueryAspectTouch {
    let mut parts = text.split('.');
    let aspect = worth_foundational::facade::AspectKey::new(parts.next().expect("aspect"))
        .expect("aspect key");
    let fields = parts
        .map(|part| FieldKey::new(part).expect("field"))
        .collect::<Vec<_>>();
    WorthQueryAspectTouch::aspect_field_path(aspect, CanonicalFieldPath::new(fields).expect("path"))
}
