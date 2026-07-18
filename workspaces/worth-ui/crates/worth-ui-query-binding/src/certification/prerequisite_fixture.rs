use std::sync::Arc;
use worth_foundational::{CanonicalFieldPath, FieldKey};

use worth_query::facade::certification::{
    admit_runtime_current_snapshot_basis_for_certification,
    resolve_runtime_current_snapshot_basis_for_certification,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::foundation::{
    snapshot_resolution_report, AspectFieldSelector, AuthoredResultShapeField, EqualityPredicate,
    ProjectionFactFieldPath, QueryExternalIdentityToken, QueryExternalSchemaBasisToken,
    WorthQueryPredicateOperand, WorthQuerySnapshotIdentity,
};
use worth_query::facade::read::{current, declare, project_facts};
use worth_query::facade::runtime::{
    QuerySchemaView, SchemaFieldView, WorthQueryAspectTouch, WorthQueryAuthoredAspectValue,
    WorthQueryReadBuilder, WorthQueryReadDenial, WorthQueryWorkspace,
};

use crate::{
    WorthUiQueryAuthorityHandle, WorthUiQueryPrerequisiteBoundary, WorthUiQueryPrerequisiteEvidence,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQueryCertificationProjection {
    DisplayField,
    EntityIdentities,
    DisplayFieldAndEntityIdentities,
}

pub fn worth_ui_query_snapshot_prerequisites(
    snapshot_label: &str,
    schema_basis_parts: [&str; 3],
) -> WorthUiQueryPrerequisiteEvidence {
    let snapshot_identity = WorthQuerySnapshotIdentity::admit_external_token(
        QueryExternalIdentityToken::new(Arc::<str>::from(snapshot_label)),
    );
    let basis = admit_runtime_current_snapshot_basis_for_certification(
        snapshot_identity.evidence_identity(),
        QueryExternalSchemaBasisToken::from_domain_parts(
            schema_basis_parts
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>(),
        ),
    )
    .expect("runtime current snapshot basis should resolve");

    WorthUiQueryPrerequisiteBoundary::new()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query prerequisites should admit")
}

pub fn worth_ui_query_prerequisite_fixture(
    label: &str,
    projection: WorthUiQueryCertificationProjection,
) -> (
    WorthUiQueryPrerequisiteEvidence,
    WorthUiQueryAuthorityHandle,
) {
    let (mut workspace, schema_basis_authority) = measurement_projection_workspace(label);
    let completion = declare(size_family_graph)
        .expect("ordinary query declaration should admit")
        .using(current())
        .run(&mut workspace)
        .into_result()
        .expect("ordinary query read should execute");
    let declaration = match projection {
        WorthUiQueryCertificationProjection::DisplayField => {
            project_facts().display_field(size_value_field_path())
        }
        WorthUiQueryCertificationProjection::EntityIdentities => {
            project_facts().entity_identities()
        }
        WorthUiQueryCertificationProjection::DisplayFieldAndEntityIdentities => project_facts()
            .entity_identities()
            .display_field(size_value_field_path()),
    };
    let outcome = completion.consume_projection(declaration);
    let basis = resolve_runtime_current_snapshot_basis_for_certification(
        &workspace.snapshot_identity().evidence_identity(),
        schema_basis_authority,
    )
    .expect("runtime current snapshot basis should resolve");
    let prerequisites = WorthUiQueryPrerequisiteBoundary::new()
        .graph_aligned(basis.clone(), snapshot_resolution_report(&basis))
        .expect("query prerequisites should admit");
    let (authority, _) = WorthUiQueryAuthorityHandle::from_outcome(outcome)
        .expect("real Query consumption should mint authority");
    (prerequisites, authority)
}

fn measurement_projection_workspace(
    label: &str,
) -> (
    WorthQueryWorkspace,
    worth_query::facade::foundation::QuerySchemaBasisAuthority,
) {
    let schema = WorthQueryTestBackendSchema::single_collection("task")
        .aspect_contracts(crate::worth_ui_native_aspect_contracts())
        .expect("Worth UI native aspect contracts should admit")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("size.value", "size.value")
        .expect("size aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(format!("worth-ui.certification.measurement-basis.{label}"))
        .expect("in-memory query backend should build a workspace");
    workspace
        .insert("task", |task| {
            task.set_aspect(
                aspect_touch("identity.id"),
                WorthQueryAuthoredAspectValue::string("task"),
            )
            .set_aspect(
                aspect_touch("size.value"),
                WorthQueryAuthoredAspectValue::native(worth_foundational::AspectValue::Float32(
                    worth_foundational::CanonicalF32::from_f32(240.0),
                )),
            )
        })
        .expect("fixture insert should admit");
    (workspace, task_query_schema().basis_authority())
}

fn size_family_graph<Output>(
    read: WorthQueryReadBuilder<Output>,
) -> Result<Output, WorthQueryReadDenial> {
    read.local_detail(
        "task",
        task_query_schema(),
        |query| {
            query
                .where_equal(
                    EqualityPredicate::new(
                        "identity",
                        "id",
                        WorthQueryPredicateOperand::string("task".to_string()),
                    )
                    .expect("identity anchor predicate should build"),
                )
                .project(field("size", "value"))
        },
        |shape| shape.field(result_field("size", "value", "size.value")),
    )
}

fn task_query_schema() -> QuerySchemaView {
    QuerySchemaView::new(
        "task",
        [
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("identity")
                    .expect("schema aspect should admit"),
                worth_query::facade::foundation::FieldName::new("id")
                    .expect("schema field should admit"),
                worth_foundational::ScalarAspectType::String,
            ),
            SchemaFieldView::new(
                worth_query::facade::foundation::AspectName::new("size")
                    .expect("schema aspect should admit"),
                worth_query::facade::foundation::FieldName::new("value")
                    .expect("schema field should admit"),
                worth_foundational::ScalarAspectType::Float32,
            ),
        ],
        [],
    )
}

fn size_value_field_path() -> ProjectionFactFieldPath {
    ProjectionFactFieldPath::from_canonical_field_path(
        CanonicalFieldPath::new(vec![
            FieldKey::new("size").expect("field key should admit"),
            FieldKey::new("value").expect("field key should admit"),
        ])
        .expect("canonical size.value path should admit"),
    )
}

fn field(aspect: &str, field: &str) -> AspectFieldSelector {
    AspectFieldSelector::new(aspect, field).expect("field selector should build")
}

fn result_field(aspect: &str, field: &str, delivered: &str) -> AuthoredResultShapeField {
    AuthoredResultShapeField::new(aspect, field, delivered)
        .expect("result-shape field should build")
}

fn aspect_touch(authored_touch_text: &str) -> WorthQueryAspectTouch {
    WorthQueryAspectTouch::from_authoring_ingress_text(authored_touch_text)
        .expect("touch should admit")
}
