use std::path::PathBuf;

use crate::{
    WorthUiAuthoredMode, WorthUiAuthoredSourceInput, WorthUiDslCompiler,
    WorthUiProjectionCollectionPolicy, WorthUiProjectionCollectionSelection,
    WorthUiProjectionDeclarationErrorKind, WorthUiProjectionLifecycle,
    WorthUiProjectionRequirement, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule,
};

use super::phase7_projection_expectation::{CollectionExpectation, ProjectionExpectation};

#[test]
fn file_and_rust_scalar_declarations_converge_on_one_canonical_requirement() {
    let file = compile_file(
        r#"
        query_scalar platform.pulse.status {
            view platform.pulse.status
            field status
            require text
        }
        "#,
    );
    let rust = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("main.wui")
            .try_with_query_scalar_text(
                "platform.pulse.status",
                "platform.pulse.status",
                "status",
                WorthUiProjectionLifecycle::Live,
            )
            .unwrap(),
    );
    let file_requirement = only_projection(&file);
    let rust_requirement = only_projection(&rust);

    assert_eq!(file_requirement, rust_requirement);
    assert_eq!(file.authored_mode(), WorthUiAuthoredMode::File);
    assert_eq!(rust.authored_mode(), WorthUiAuthoredMode::Rust);
    assert_eq!(
        ProjectionExpectation::capture(file_requirement),
        ProjectionExpectation::scalar(
            "platform.pulse.status",
            "platform.pulse.status",
            "status",
            WorthUiProjectionLifecycle::Live,
        )
    );
}

#[test]
fn file_and_rust_collection_declarations_converge_without_positional_identity() {
    let file = compile_file(
        r#"
        query_collection platform.pulse.rows {
            view platform.pulse.rows
            row identity
            field status
            require text
            completeness partial
            continuation allowed
            lifecycle live
        }
        "#,
    );
    let rust = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("main.wui")
            .try_with_query_collection_text(
                "platform.pulse.rows",
                "platform.pulse.rows",
                "identity",
                WorthUiProjectionCollectionSelection::new(
                    ["status"],
                    WorthUiProjectionLifecycle::Live,
                    WorthUiProjectionCollectionPolicy::new(false, true),
                ),
            )
            .unwrap(),
    );
    let file_requirement = only_projection(&file);
    let rust_requirement = only_projection(&rust);

    assert_eq!(file_requirement, rust_requirement);
    assert_eq!(file.authored_mode(), WorthUiAuthoredMode::File);
    assert_eq!(rust.authored_mode(), WorthUiAuthoredMode::Rust);
    assert_eq!(
        ProjectionExpectation::capture(file_requirement),
        ProjectionExpectation::collection(
            "platform.pulse.rows",
            "platform.pulse.rows",
            "identity",
            CollectionExpectation::new(
                &["status"],
                WorthUiProjectionLifecycle::Live,
                (false, true),
            ),
        )
    );
}

#[test]
fn rust_authored_projection_order_is_canonical_and_invalid_input_fails_early() {
    let left = WorthUiRustAuthoredArtifactInputModule::new("main.wui")
        .try_with_query_scalar_text(
            "z.status",
            "z.status",
            "status",
            WorthUiProjectionLifecycle::Live,
        )
        .unwrap()
        .try_with_query_collection_text(
            "a.rows",
            "a.rows",
            "identity",
            collection_selection(["value", "label"]),
        )
        .unwrap();
    let right = WorthUiRustAuthoredArtifactInputModule::new("main.wui")
        .try_with_query_collection_text(
            "a.rows",
            "a.rows",
            "identity",
            collection_selection(["label", "value"]),
        )
        .unwrap()
        .try_with_query_scalar_text(
            "z.status",
            "z.status",
            "status",
            WorthUiProjectionLifecycle::Live,
        )
        .unwrap();
    assert_eq!(
        compile_rust(left).identity(),
        compile_rust(right).identity()
    );

    let denial = WorthUiRustAuthoredArtifactInputModule::new("main.wui")
        .try_with_query_collection_text(
            "a.rows",
            "a.rows",
            "identity",
            collection_selection(["value", "value"]),
        )
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthUiProjectionDeclarationErrorKind::DuplicateSelectedField
    );
}

fn collection_selection<const FIELD_COUNT: usize>(
    fields: [&str; FIELD_COUNT],
) -> WorthUiProjectionCollectionSelection {
    WorthUiProjectionCollectionSelection::new(
        fields,
        WorthUiProjectionLifecycle::Snapshot,
        WorthUiProjectionCollectionPolicy::new(true, false),
    )
}

fn compile_file(source: &str) -> crate::WorthUiSealedSemanticPackage {
    WorthUiDslCompiler::compile_source(
        WorthUiAuthoredSourceInput::rooted_at(PathBuf::from("workspace"))
            .with_module("main.wui", source),
    )
    .expect("file-authored projection should compile")
}

fn compile_rust(
    module: WorthUiRustAuthoredArtifactInputModule,
) -> crate::WorthUiSealedSemanticPackage {
    let input = WorthUiRustAuthoredArtifactInput::from_modules([module]);
    WorthUiDslCompiler::compile_rust_authored(&input)
        .expect("Rust-authored projection should compile through the canonical pipeline")
}

fn only_projection(package: &crate::WorthUiSealedSemanticPackage) -> &WorthUiProjectionRequirement {
    let requirements = package.projection_requirements().collect::<Vec<_>>();
    assert_eq!(requirements.len(), 1);
    requirements[0]
}
