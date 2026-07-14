use std::path::Path;

use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{foundation::basis_lifecycle, inspection, read};

const FEATURE_DOC: &str = include_str!("../docs/capabilities/declarative-query-experience.md");
const AI_README: &str = include_str!("../docs/AI_README.md");
const DOCS_README: &str = include_str!("../docs/README.md");

#[test]
fn documented_read_projection_and_inspection_examples_compile_and_run() {
    let mut workspace = task_workspace("declarative-doc-example");
    let declaration = task_declaration();
    let completion = declaration
        .using(read::current())
        .run(&mut workspace)
        .into_result()
        .expect("documented task read should complete");
    assert_eq!(completion.result().rows().len(), 1);

    let projection = completion.consume_projection(read::project_facts().entity_identities());
    let (_authority, warnings) = projection
        .into_admitted()
        .expect("documented identity projection should admit");
    assert!(warnings.is_none());

    let basis = basis_lifecycle()
        .historical_snapshot("declarative-doc-inspection", true)
        .inspect()
        .expect("documented inspection basis should admit");
    let inspection_outcome = inspection::declare(&completion)
        .with_rich_inspection()
        .using(inspection::inspection_basis(basis))
        .run(&workspace);
    assert!(
        inspection_outcome.settled().is_some()
            || inspection_outcome.stop().is_some()
            || inspection_outcome.unavailable().is_some(),
        "inspection must preserve a typed terminal posture"
    );
}

#[test]
fn discovery_links_resolve_to_each_current_ordinary_capability() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let rows = [
        ("read", "src/ordinary/read/declaration.rs", "pub fn declare"),
        ("live", "src/ordinary/live/declaration.rs", "pub fn declare"),
        ("history", "src/ordinary/history/declaration.rs", "pub fn declare"),
        (
            "comparison",
            "src/ordinary/comparison/declaration.rs",
            "pub fn declare",
        ),
        ("preview", "src/ordinary/preview/declaration.rs", "pub fn declare"),
        (
            "mutation",
            "src/ordinary/mutation/declaration.rs",
            "pub fn declare",
        ),
        (
            "workflow",
            "src/ordinary/workflow/declaration.rs",
            "pub fn declare",
        ),
        (
            "inspection",
            "src/ordinary/inspection/declaration.rs",
            "pub fn declare",
        ),
        ("domain", "src/ordinary/domain/mod.rs", "pub fn declare"),
    ];

    for (namespace, relative_path, probe) in rows {
        assert!(FEATURE_DOC.contains(&format!("facade::{namespace}")));
        let source = std::fs::read_to_string(root.join(relative_path))
            .unwrap_or_else(|error| panic!("failed to read {relative_path}: {error}"));
        assert!(source.contains(probe), "missing {namespace} declaration");
    }

    let link = "./capabilities/declarative-query-experience.md";
    assert!(AI_README.contains(link));
    assert!(DOCS_README.contains(link));
    assert!(root.join("docs/capabilities/declarative-query-experience.md").is_file());
}

fn task_declaration() -> read::WorthQueryReadDeclaration {
    read::declare(|read| {
        read.local_collection(
            "Task",
            task_schema(),
            |query| {
                query
                    .project(
                        read::AspectFieldSelector::new("identity", "id")
                            .expect("static identity selector"),
                    )
                    .project(
                        read::AspectFieldSelector::new("title", "value")
                            .expect("static title selector"),
                    )
            },
            |shape| {
                shape
                    .field(
                        read::AuthoredResultShapeField::new(
                            "identity",
                            "id",
                            "identity.id",
                        )
                        .expect("static identity result field"),
                    )
                    .field(
                        read::AuthoredResultShapeField::new(
                            "title",
                            "value",
                            "title.value",
                        )
                        .expect("static title result field"),
                    )
            },
        )
    })
    .expect("static task declaration should admit")
}

fn task_workspace(name: &str) -> worth_query::facade::runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("Task")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should admit")
        .aspect("title.value", "title.value")
        .expect("title aspect should admit");
    let mut workspace = in_memory_test_runtime()
        .with_schema(schema)
        .workspace(name)
        .expect("documented workspace should build");
    workspace
        .insert("Task", |task| {
            task.set_aspect(
                worth_query::facade::runtime::WorthQueryAspectTouch::from_authoring_ingress_text(
                    "identity.id",
                )
                .expect("identity touch should admit"),
                worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string("task-1"),
            )
            .set_aspect(
                worth_query::facade::runtime::WorthQueryAspectTouch::from_authoring_ingress_text(
                    "title.value",
                )
                .expect("title touch should admit"),
                worth_query::facade::runtime::WorthQueryAuthoredAspectValue::string("Draft"),
            )
        })
        .expect("documented task should insert");
    workspace
}

fn task_schema() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "task-query",
        [
            read::SchemaFieldView::new(
                read::AspectName::new("identity").expect("static aspect"),
                read::FieldName::new("id").expect("static field"),
                read::SchemaFieldKind::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("title").expect("static aspect"),
                read::FieldName::new("value").expect("static field"),
                read::SchemaFieldKind::String,
            ),
        ],
        [],
    )
}
