use worth_foundational::facade::{CanonicalFieldPath, FieldKey};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldKind, SchemaFieldView, WorthQueryReadBuilder, WorthQueryReadDenial,
    WorthQueryReadFamily, WorthQueryReadGraph, WorthQueryWorkspace,
};
use worth_query::facade::policy::{
    admit_query_basis_context,
    execute_query_basis_context,
    QueryContextBindingSource,
};
use worth_query::facade::{
    bind_query_basis_context,
    QueryBasisContextRequest,
};
use worth_query::facade::foundation::{
    preflight_execution_basis,
    resolve_runtime_current_snapshot_basis,
    snapshot_resolution_report,
    AspectFieldSelector,
    AuthoredResultShapeField,
    EqualityPredicate,
    ProjectionAuthorityContract,
    ProjectionAuthorityOutcome,
    ProjectionFactFieldPath,
    ScalarPredicateValue,
};
use worth_query::facade::certification::public_bridge_projection_artifacts_for_read_graph;
use worth_query::facade::runtime::{
    WorthQueryAspectTouch,
    WorthQueryAuthoredAspectValue,
};
use worth_ui_query_binding::{WorthUiQueryBindingSubsystem, WorthUiQueryPrerequisiteEvidence};

pub(super) fn query_projection_consumption(
    label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, ProjectionAuthorityOutcome) {
    let (mut workspace, family) = workspace_and_family(label);
    let read = workspace.execute_read_family(&family).expect("Query read");
    let (shape, projection) =
        public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let outcome = read
        .consume_projection_authority(&shape, &projection, requested_authority())
        .expect("projection authority consumption");
    (prerequisites(&workspace, &family), outcome)
}

pub(super) fn partial_query_projection_consumption(
    label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, ProjectionAuthorityOutcome) {
    let (workspace, family) = workspace_and_family(label);
    let basis = query_basis(&workspace, &family);
    let preflight =
        preflight_execution_basis(family.read_graph().execution_plan().clone(), basis.clone())
            .expect("Query preflight");
    let binding = bind_query_basis_context(
        QueryBasisContextRequest::current_branch_head(),
        QueryContextBindingSource::RuntimeCurrent(&preflight),
    )
    .expect("Query context binding");
    let context = admit_query_basis_context(binding).expect("Query context admission");
    let execution = execute_query_basis_context(&context).expect("Query context execution");
    let (_, projection) = public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let outcome = execution
        .consume_projection_authority(&projection, requested_authority())
        .expect("warning-bearing projection authority consumption");
    (prerequisites_from_basis(basis), outcome)
}

pub(super) fn unsupported_query_projection_consumption(
    label: &str,
) -> (WorthUiQueryPrerequisiteEvidence, ProjectionAuthorityOutcome) {
    let (mut workspace, family) = workspace_and_family(label);
    let read = workspace.execute_read_family(&family).expect("Query read");
    let (shape, projection) =
        public_bridge_projection_artifacts_for_read_graph(family.read_graph());
    let outcome = read
        .consume_projection_authority(
            &shape,
            &projection,
            authority_contract().require_target_identity(),
        )
        .expect("unsupported projection authority remains typed");
    (prerequisites(&workspace, &family), outcome)
}

fn workspace_and_family(label: &str) -> (WorthQueryWorkspace, WorthQueryReadFamily) {
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
    let family = workspace
        .define_read_family(label, query_size_graph)
        .expect("Query family");
    (workspace, family)
}

fn prerequisites(
    workspace: &WorthQueryWorkspace,
    family: &WorthQueryReadFamily,
) -> WorthUiQueryPrerequisiteEvidence {
    prerequisites_from_basis(query_basis(workspace, family))
}

fn query_basis(
    workspace: &WorthQueryWorkspace,
    family: &WorthQueryReadFamily,
) -> worth_query::facade::foundation::ResolvedSnapshotBasis {
    resolve_runtime_current_snapshot_basis(
        workspace.snapshot_identity().evidence_identity(),
        family.read_graph().schema_basis_authority(),
    )
    .expect("Query basis")
}

fn prerequisites_from_basis(
    basis: worth_query::facade::foundation::ResolvedSnapshotBasis,
) -> WorthUiQueryPrerequisiteEvidence {
    WorthUiQueryBindingSubsystem::bootstrap()
        .prerequisites()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("Query prerequisites")
}

fn requested_authority() -> ProjectionAuthorityContract {
    authority_contract().require_display_field(query_size_path())
}

fn authority_contract() -> ProjectionAuthorityContract {
    ProjectionAuthorityContract::declare()
        .require_settled_consumption()
        .require_source_authority()
}

fn query_size_graph(
    read: WorthQueryReadBuilder,
) -> Result<WorthQueryReadGraph, WorthQueryReadDenial> {
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
