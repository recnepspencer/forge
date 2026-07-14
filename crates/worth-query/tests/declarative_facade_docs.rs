use std::path::{Path, PathBuf};

use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::{aggregate, foundation::basis_lifecycle, inspection, read};

const FEATURE_DOC: &str = include_str!("../docs/capabilities/declarative-query-experience.md");
const AI_README: &str = include_str!("../docs/AI_README.md");
const DOCS_README: &str = include_str!("../docs/README.md");
const COLLECTIONS_DOC: &str =
    include_str!("../docs/authoring/collections-cursors-ordering-and-aggregations.md");
const PROJECTION_RECIPE: &str = include_str!(
    "../docs/domain-capabilities/recipes/carry-query-facts-into-a-downstream-runtime.md"
);

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

    let count = task_count_declaration()
        .using(aggregate::current())
        .run(&mut workspace)
        .into_result()
        .expect("documented task count should complete")
        .into_result();
    assert_eq!(count.count(), 1);

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
        (
            "aggregate",
            "src/facade/exports_aggregate.rs",
            "declare_count as declare",
        ),
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
    assert_eq!(rows.len(), 10, "ordinary grammar must cover ten namespaces");

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
    assert!(AI_README.contains("facade::aggregate"));

    let read_exports = std::fs::read_to_string(root.join("src/facade/exports_read.rs"))
        .expect("read facade exports should be readable");
    assert!(
        !read_exports.contains("declare_count"),
        "count declarations must not reappear as a second read-facade path"
    );
}

#[test]
fn ai_discovery_excludes_displaced_phase_assembly_guidance() {
    for displaced in [
        "## Platform Entry For Serious Downstream Work",
        "## Domain Entry And Configured Handles",
        "## Readiness, Orchestration, Route, Receipt, And Envelope",
        "## Family Helpers And Declaration Progression",
        "Declaration Entry Orchestration",
        "Binding Vs Orchestration Vs Helpers",
        "consume_projection_authority",
        "ProjectionAuthorityContract::declare",
    ] {
        assert!(
            !AI_README.contains(displaced),
            "AI discovery still teaches displaced path: {displaced}"
        );
    }
}

#[test]
fn ordinary_docs_do_not_teach_internal_planner_or_manual_projection_assembly() {
    for displaced in [
        "plan_validated_bundle",
        "ProjectionAuthorityContract::declare",
        "consume_projection_authority(",
    ] {
        assert!(
            !COLLECTIONS_DOC.contains(displaced),
            "collection discovery still teaches displaced path: {displaced}"
        );
        assert!(
            !PROJECTION_RECIPE.contains(displaced),
            "ordinary projection recipe still teaches displaced path: {displaced}"
        );
    }
}

#[test]
fn relative_markdown_links_in_product_docs_resolve() {
    let docs = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs");
    let mut markdown_files = Vec::new();
    collect_markdown_files(&docs, &mut markdown_files);

    for path in markdown_files {
        let contents = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
        for target in markdown_targets(&contents) {
            if target.starts_with('#')
                || target.starts_with('/')
                || target.contains("://")
                || target.starts_with("mailto:")
            {
                continue;
            }
            let relative = target.split('#').next().unwrap_or_default();
            if relative.is_empty() {
                continue;
            }
            let resolved = path
                .parent()
                .expect("markdown file should have a parent")
                .join(relative);
            assert!(
                resolved.exists(),
                "broken relative link `{target}` in {} (resolved to {})",
                path.display(),
                resolved.display()
            );
        }
    }
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

fn task_count_declaration() -> aggregate::WorthQueryCountDeclaration {
    aggregate::declare(|query| {
        query.local_collection(
            "Task",
            task_schema(),
            |tasks| {
                tasks.project(
                    aggregate::AspectFieldSelector::new("identity", "id")
                        .expect("static identity selector"),
                )
            },
            |shape| {
                shape.field(
                    aggregate::AuthoredResultShapeField::new(
                        "identity",
                        "id",
                        "identity.id",
                    )
                    .expect("static identity result field"),
                )
            },
        )
    })
    .expect("static task count should admit")
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

fn collect_markdown_files(directory: &Path, output: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", directory.display()))
    {
        let path = entry.expect("docs directory entry should resolve").path();
        if path.is_dir() {
            collect_markdown_files(&path, output);
        } else if path.extension().and_then(|value| value.to_str()) == Some("md") {
            output.push(path);
        }
    }
}

fn markdown_targets(contents: &str) -> impl Iterator<Item = &str> {
    contents.lines().flat_map(|line| {
        let mut targets = Vec::new();
        let mut rest = line;
        while let Some(start) = rest.find("](") {
            rest = &rest[start + 2..];
            let Some(end) = rest.find(')') else {
                break;
            };
            targets.push(&rest[..end]);
            rest = &rest[end + 1..];
        }
        targets
    })
}
