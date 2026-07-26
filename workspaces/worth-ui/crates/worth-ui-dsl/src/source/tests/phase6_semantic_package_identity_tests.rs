use std::path::PathBuf;

use crate::source::{
    WorthUiArtifactInputProvenance, WorthUiAuthoredMode, WorthUiAuthoredSourceInput,
    WorthUiDslCompiler, WorthUiDslProtocolIdentity, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiSealedSemanticPackage,
    WorthUiSemanticArtifactDeclaration, WorthUiSemanticDeclaration, WorthUiSemanticProvenanceRef,
};
use crate::{
    UiDslAspectName, UiDslSemanticFamily, UiDslSemanticKey, UiDslStructuralToken, UiDslSupportToken,
};

#[test]
fn file_and_rust_authorship_share_exact_meaning_but_keep_distinct_evidence() {
    let file = compile_file(
        r#"
        // Authorship does not change semantic meaning.
        component Dashboard {}
        token accent = "blue";
        "#,
    );
    let rust = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component("Dashboard")
            .with_token("accent", "blue"),
    );

    assert_eq!(file.identity(), rust.identity());
    assert_eq!(file.authored_mode(), WorthUiAuthoredMode::File);
    assert_eq!(rust.authored_mode(), WorthUiAuthoredMode::Rust);
    assert_eq!(file.protocol(), WorthUiDslProtocolIdentity::current());
    assert!(file.protocol().is_current());
}

#[test]
fn formatting_comments_and_declaration_order_do_not_change_semantic_identity() {
    let first = compile_file(
        r#"
        component Dashboard {}
        token accent = "blue";
        "#,
    );
    let second = compile_file(
        r#"
        // declaration order and layout are non-semantic
        token accent="blue";

        component    Dashboard {
        }
        "#,
    );

    assert_eq!(first.identity(), second.identity());
}

#[test]
fn authored_identity_import_structure_and_token_value_are_semantic() {
    let baseline = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/dependency.wui")
            .with_component_authored_identity("Dashboard", "dashboard.primary")
            .with_token("accent", "blue"),
    );
    let changed_authored_identity = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/dependency.wui")
            .with_component_authored_identity("Dashboard", "dashboard.secondary")
            .with_token("accent", "blue"),
    );
    let changed_import = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/other.wui")
            .with_component_authored_identity("Dashboard", "dashboard.primary")
            .with_token("accent", "blue"),
    );
    let changed_token = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_import("app/dependency.wui")
            .with_component_authored_identity("Dashboard", "dashboard.primary")
            .with_token("accent", "green"),
    );

    assert_ne!(baseline.identity(), changed_authored_identity.identity());
    assert_ne!(baseline.identity(), changed_import.identity());
    assert_ne!(baseline.identity(), changed_token.identity());
}

#[test]
fn equal_narrowing_fingerprint_cannot_alias_different_exact_meaning() {
    let first = compile_file("component Dashboard {}");
    let second = compile_file("component Inspector {}");
    let forced_fingerprint = 0x5eed;
    let first_identity = first
        .identity()
        .clone()
        .with_narrowing_fingerprint_for_test(forced_fingerprint);
    let second_identity = second
        .identity()
        .clone()
        .with_narrowing_fingerprint_for_test(forced_fingerprint);

    assert_eq!(
        first_identity.narrowing_fingerprint(),
        second_identity.narrowing_fingerprint()
    );
    assert_ne!(first_identity, second_identity);
}

#[test]
fn unsupported_protocol_is_exactly_distinct_from_current_protocol() {
    let unsupported = WorthUiDslProtocolIdentity::unsupported_for_test();

    assert_ne!(unsupported, WorthUiDslProtocolIdentity::current());
    assert!(!unsupported.is_current());
}

#[test]
fn sealed_declarations_carry_compact_package_resolved_provenance_references() {
    let file = compile_file("component Dashboard {} token accent = \"blue\";");
    let module_id = file
        .module_ids()
        .iter()
        .find(|module_id| module_id.as_str() == "app/main.wui")
        .expect("main module should retain canonical identity");
    let views = file
        .declaration_views(module_id)
        .expect("canonical module should expose sealed declaration views")
        .collect::<Vec<_>>();

    assert_eq!(views.len(), 2);
    assert_ne!(views[0].provenance_ref(), views[1].provenance_ref());
    assert!(matches!(
        views[0].provenance(),
        WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. }
    ));
    assert!(std::mem::size_of::<WorthUiSemanticProvenanceRef>() <= std::mem::size_of::<usize>());
}

#[test]
fn rich_rust_declarations_are_canonicalized_and_provenance_is_compiler_minted() {
    let package = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_semantic_declaration(
            WorthUiSemanticArtifactDeclaration::new(
                UiDslSemanticKey::new("dashboard.save"),
                UiDslSemanticFamily::Control,
            )
            .with_published_aspect(UiDslAspectName::new("content.text"))
            .with_published_aspect(UiDslAspectName::new("action.invoke"))
            .with_published_aspect(UiDslAspectName::new("content.text"))
            .with_support_token(UiDslSupportToken::new("support:command")),
        ),
    );
    let main_module = package
        .module_ids()
        .iter()
        .find(|module_id| module_id.as_str() == "app/main.wui")
        .expect("main module should exist");
    let view = package
        .declaration_views(main_module)
        .expect("main module should expose declarations")
        .next()
        .expect("semantic declaration should exist");
    let artifact = match view.declaration() {
        WorthUiSemanticDeclaration::SemanticArtifact(artifact) => artifact,
        other => panic!("expected sealed semantic artifact, observed {other:?}"),
    };

    assert_eq!(
        artifact
            .declaration()
            .published_aspects()
            .iter()
            .map(|aspect| aspect.as_str())
            .collect::<Vec<_>>(),
        vec!["action.invoke", "content.text"]
    );
    assert!(matches!(
        view.provenance(),
        WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
            authored_module_path,
            declaration_index: 0,
        } if authored_module_path == "app/main.wui"
    ));
}

#[test]
fn rich_semantic_set_order_and_duplicates_do_not_change_exact_identity() {
    let first = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_semantic_declaration(
            WorthUiSemanticArtifactDeclaration::new(
                UiDslSemanticKey::new("dashboard.save"),
                UiDslSemanticFamily::Control,
            )
            .with_published_aspect(UiDslAspectName::new("action.invoke"))
            .with_published_aspect(UiDslAspectName::new("content.text")),
        ),
    );
    let second = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_semantic_declaration(
            WorthUiSemanticArtifactDeclaration::new(
                UiDslSemanticKey::new("dashboard.save"),
                UiDslSemanticFamily::Control,
            )
            .with_published_aspect(UiDslAspectName::new("content.text"))
            .with_published_aspect(UiDslAspectName::new("action.invoke"))
            .with_published_aspect(UiDslAspectName::new("content.text")),
        ),
    );

    assert_eq!(first.identity(), second.identity());
}

#[test]
fn sealed_package_receipts_cover_declarations_without_runtime_bootstrap() {
    let package = compile_rust(
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component_authored_identity("Dashboard", "dashboard.primary")
            .with_token("accent", "blue")
            .with_semantic_declaration(
                WorthUiSemanticArtifactDeclaration::new(
                    UiDslSemanticKey::new("dashboard.save"),
                    UiDslSemanticFamily::Control,
                )
                .with_structural_token(UiDslStructuralToken::new("control:save")),
            ),
    );
    let receipts = package.declaration_lowering_receipts();

    assert_eq!(receipts.len(), 2);
    assert_eq!(
        receipts
            .iter()
            .map(|receipt| receipt.semantic_artifact().key().as_str())
            .collect::<Vec<_>>(),
        vec!["component:Dashboard", "dashboard.save"]
    );
    assert!(receipts
        .iter()
        .all(|receipt| { receipt.source_provenance().module_path() == "app/main.wui" }));
    assert!(receipts.iter().all(|receipt| {
        receipt.semantic_artifact().key().as_str() != "worth_ui.runtime.bootstrap.product_root"
    }));
    assert_eq!(
        receipts[0].semantic_artifact().structural_tokens()[0].as_str(),
        "mosaic:app/main.wui|component:authored:dashboard.primary"
    );
}

fn compile_file(source: &str) -> WorthUiSealedSemanticPackage {
    WorthUiDslCompiler::compile_source(
        WorthUiAuthoredSourceInput::rooted_at(PathBuf::from("workspace"))
            .with_module("app/main.wui", source)
            .with_module("app/dependency.wui", "")
            .with_module("app/other.wui", ""),
    )
    .expect("file-authored fixture should seal")
}

fn compile_rust(
    main_module: WorthUiRustAuthoredArtifactInputModule,
) -> WorthUiSealedSemanticPackage {
    WorthUiDslCompiler::compile_rust_authored(&WorthUiRustAuthoredArtifactInput::from_modules([
        main_module,
        WorthUiRustAuthoredArtifactInputModule::new("app/dependency.wui"),
        WorthUiRustAuthoredArtifactInputModule::new("app/other.wui"),
    ]))
    .expect("Rust-authored fixture should seal")
}
