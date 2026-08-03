use std::path::PathBuf;

use crate::{
    WorthUiAuthoredSourceInput, WorthUiDslCompileDiagnosticCode, WorthUiDslCompiler,
    WorthUiProjectionCollectionPolicy, WorthUiProjectionCollectionSelection,
    WorthUiProjectionLifecycle, WorthUiProjectionRequirement,
};

use super::phase7_projection_expectation::{CollectionExpectation, ProjectionExpectation};

#[test]
fn whitespace_import_and_declaration_order_do_not_change_canonical_identity() {
    let left = compile_modules(
        r#"
        import "shared.wui";
        query_scalar z.status { view z.status field status require text }
        query_collection a.rows {
            view a.rows row identity field value field label require text
            completeness complete continuation forbidden lifecycle snapshot
        }
        "#,
        "token shared = \"value\";",
    );
    let right = compile_modules(
        r#"
        query_collection a.rows {
            continuation forbidden;
            completeness complete;
            require text;
            field label;
            field value;
            row identity;
            lifecycle snapshot;
            view a.rows;
        }
        query_scalar z.status {
            require text;
            field status;
            view z.status;
        }
        import "shared.wui";
        "#,
        "token shared=\"value\";",
    );

    assert_eq!(left.identity(), right.identity());
    assert_eq!(projection_identities(&left), projection_identities(&right));
    let expected = ProjectionExpectation::collection(
        "a.rows",
        "a.rows",
        "identity",
        CollectionExpectation::new(
            &["label", "value"],
            WorthUiProjectionLifecycle::Snapshot,
            (true, false),
        ),
    );
    assert_eq!(
        ProjectionExpectation::capture(projection(&left, "a.rows")),
        expected
    );
    assert_eq!(
        ProjectionExpectation::capture(projection(&right, "a.rows")),
        expected
    );
}

#[test]
fn each_scalar_projection_axis_changes_identity_or_fails_closed() {
    let baseline = WorthUiProjectionRequirement::scalar_text(
        "pulse.status",
        "pulse.status",
        "status",
        WorthUiProjectionLifecycle::Live,
    )
    .unwrap();
    let different_declaration = WorthUiProjectionRequirement::scalar_text(
        "pulse.other-declaration",
        "pulse.status",
        "status",
        WorthUiProjectionLifecycle::Live,
    )
    .unwrap();
    let different_view = WorthUiProjectionRequirement::scalar_text(
        "pulse.status",
        "pulse.other",
        "status",
        WorthUiProjectionLifecycle::Live,
    )
    .unwrap();
    let different_field = WorthUiProjectionRequirement::scalar_text(
        "pulse.status",
        "pulse.status",
        "other",
        WorthUiProjectionLifecycle::Live,
    )
    .unwrap();
    let different_lifecycle = WorthUiProjectionRequirement::scalar_text(
        "pulse.status",
        "pulse.status",
        "status",
        WorthUiProjectionLifecycle::Snapshot,
    )
    .unwrap();
    let collection = WorthUiProjectionRequirement::collection_text(
        "pulse.status",
        "pulse.status",
        "identity",
        WorthUiProjectionCollectionSelection::new(
            ["status"],
            WorthUiProjectionLifecycle::Live,
            WorthUiProjectionCollectionPolicy::new(true, false),
        ),
    )
    .unwrap();

    assert_eq!(
        ProjectionExpectation::capture(&baseline),
        ProjectionExpectation::scalar(
            "pulse.status",
            "pulse.status",
            "status",
            WorthUiProjectionLifecycle::Live,
        )
    );
    for changed in [
        different_declaration,
        different_view,
        different_field,
        different_lifecycle,
        collection,
    ] {
        assert_ne!(baseline.identity(), changed.identity());
    }
    assert_projection_error(
        "query_scalar pulse.status { view pulse.status field status require float }",
    );
}

#[test]
fn each_collection_projection_axis_changes_canonical_meaning() {
    let baseline = collection_requirement(
        "identity",
        &["label", "status"],
        WorthUiProjectionLifecycle::Live,
        WorthUiProjectionCollectionPolicy::new(true, false),
    );
    let changed = [
        collection_requirement(
            "other_identity",
            &["label", "status"],
            WorthUiProjectionLifecycle::Live,
            WorthUiProjectionCollectionPolicy::new(true, false),
        ),
        collection_requirement(
            "identity",
            &["label", "other"],
            WorthUiProjectionLifecycle::Live,
            WorthUiProjectionCollectionPolicy::new(true, false),
        ),
        collection_requirement(
            "identity",
            &["label", "status"],
            WorthUiProjectionLifecycle::Snapshot,
            WorthUiProjectionCollectionPolicy::new(true, false),
        ),
        collection_requirement(
            "identity",
            &["label", "status"],
            WorthUiProjectionLifecycle::Live,
            WorthUiProjectionCollectionPolicy::new(false, false),
        ),
        collection_requirement(
            "identity",
            &["label", "status"],
            WorthUiProjectionLifecycle::Live,
            WorthUiProjectionCollectionPolicy::new(true, true),
        ),
    ];

    assert_eq!(
        ProjectionExpectation::capture(&baseline),
        ProjectionExpectation::collection(
            "pulse.rows",
            "pulse.rows",
            "identity",
            CollectionExpectation::new(
                &["label", "status"],
                WorthUiProjectionLifecycle::Live,
                (true, false),
            ),
        )
    );
    for requirement in changed {
        assert_ne!(baseline.identity(), requirement.identity());
        assert_ne!(
            ProjectionExpectation::capture(&baseline),
            ProjectionExpectation::capture(&requirement)
        );
    }
}

#[test]
fn malformed_or_shape_crossing_projection_declarations_fail_at_language_legality() {
    for source in [
        "query_scalar pulse.status { view pulse.status require text }",
        "query_scalar pulse.status { view a view b field status require text }",
        "query_scalar pulse.status { view pulse.status row identity field status require text }",
        "query_collection pulse.rows { view pulse.rows row identity field status require text }",
        "query_collection pulse.rows { view pulse.rows row identity require text completeness complete continuation forbidden }",
        "query_collection pulse.rows { view pulse.rows row identity field status field status require text completeness complete continuation forbidden }",
        "query_scalar pulse.status { view pulse.status field status require text } query_scalar pulse.status { view pulse.other field status require text }",
    ] {
        assert_projection_error(source);
    }
}

fn assert_projection_error(source: &str) {
    let report = WorthUiDslCompiler::compile_source(
        WorthUiAuthoredSourceInput::rooted_at(PathBuf::from("workspace"))
            .with_module("main.wui", source),
    )
    .expect_err("invalid projection declaration should fail closed");
    assert!(report.diagnostics().iter().any(|diagnostic| {
        diagnostic.identity().code()
            == WorthUiDslCompileDiagnosticCode::InvalidProjectionDeclaration
    }));
}

fn compile_modules(main: &str, shared: &str) -> crate::WorthUiSealedSemanticPackage {
    WorthUiDslCompiler::compile_source(
        WorthUiAuthoredSourceInput::rooted_at(PathBuf::from("workspace"))
            .with_module("main.wui", main)
            .with_module("shared.wui", shared),
    )
    .expect("projection source should compile")
}

fn projection<'package>(
    package: &'package crate::WorthUiSealedSemanticPackage,
    declaration: &str,
) -> &'package WorthUiProjectionRequirement {
    package
        .projection_requirements()
        .find(|requirement| requirement.declaration_identity() == declaration)
        .expect("expected canonical projection declaration")
}

fn collection_requirement(
    row_identity: &str,
    selected_fields: &[&str],
    lifecycle: WorthUiProjectionLifecycle,
    policy: WorthUiProjectionCollectionPolicy,
) -> WorthUiProjectionRequirement {
    WorthUiProjectionRequirement::collection_text(
        "pulse.rows",
        "pulse.rows",
        row_identity,
        WorthUiProjectionCollectionSelection::new(
            selected_fields.iter().copied(),
            lifecycle,
            policy,
        ),
    )
    .unwrap()
}

fn projection_identities(
    package: &crate::WorthUiSealedSemanticPackage,
) -> Vec<crate::WorthUiProjectionRequirementIdentity> {
    package
        .projection_requirements()
        .map(WorthUiProjectionRequirement::identity)
        .collect()
}
