use crate::{
    WorthUiArtifactInputBodyAtom, WorthUiDslCompileDiagnosticCode, WorthUiDslCompileStopClass,
    WorthUiDslCompiler, WorthUiProjectionLifecycle, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiSemanticDeclaration,
};

#[test]
fn duplicate_structural_declaration_is_a_dsl_legality_stop() {
    let report = compile_component_body(vec![
        ident("region"),
        ident("workspace.region.primary"),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        ident("sizing"),
        ident("workspace.sizing.fill"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        ident("sizing"),
        ident("workspace.sizing.overlay"),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ]);

    assert_eq!(
        report.diagnostics()[0].identity().code(),
        WorthUiDslCompileDiagnosticCode::DuplicateRegionSizingDeclaration
    );
    assert_eq!(
        report.diagnostics()[0].stop_class(),
        WorthUiDslCompileStopClass::LanguageLegality
    );
}

#[test]
fn root_mount_is_rejected_before_runtime_admission() {
    let report = compile_component_body(vec![
        ident("mount"),
        ident("workspace.surface.main"),
        WorthUiArtifactInputBodyAtom::Semicolon,
    ]);

    assert_eq!(
        report.diagnostics()[0].identity().code(),
        WorthUiDslCompileDiagnosticCode::IllegalRootStructuralStatement
    );
}

#[test]
fn component_projection_content_is_sealed_as_first_class_meaning() {
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms(
            "workspace.component.status",
            vec![
                ident("content"),
                ident("projection"),
                ident("platform.pulse.status"),
            ],
        )
        .try_with_query_scalar_text(
            "platform.pulse.status",
            "platform.pulse.status",
            "status",
            WorthUiProjectionLifecycle::Live,
        )
        .unwrap();
    let package = WorthUiDslCompiler::compile_rust_authored(
        &WorthUiRustAuthoredArtifactInput::from_modules([module]),
    )
    .expect("declared projection content should seal");
    let module = package.module(&package.module_ids()[0]).unwrap();
    let component = module
        .declarations()
        .iter()
        .find_map(|declaration| match declaration {
            WorthUiSemanticDeclaration::Component(component) => Some(component),
            _ => None,
        })
        .expect("component declaration");

    assert_eq!(
        component.structure().projection_contents()[0].projection_identity_text(),
        "platform.pulse.status"
    );
}

#[test]
fn unknown_projection_content_stops_during_dsl_legality() {
    let input = WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component_body_atoms(
            "workspace.component.status",
            vec![
                ident("content"),
                ident("projection"),
                ident("platform.pulse.missing"),
            ],
        ),
    ]);
    let report = WorthUiDslCompiler::compile_rust_authored(&input)
        .expect_err("unknown projection content must not reach runtime");

    assert_eq!(
        report.diagnostics()[0].identity().code(),
        WorthUiDslCompileDiagnosticCode::UnknownProjectionContent
    );
}

fn compile_component_body(
    body_atoms: Vec<WorthUiArtifactInputBodyAtom>,
) -> crate::WorthUiDslCompileReport {
    let input = WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
            .with_component_body_atoms("workspace.component.dashboard", body_atoms),
    ]);
    WorthUiDslCompiler::compile_rust_authored(&input)
        .expect_err("invalid structural language should fail DSL compilation")
}

fn ident(text: &str) -> WorthUiArtifactInputBodyAtom {
    WorthUiArtifactInputBodyAtom::Identifier(text.to_owned())
}
