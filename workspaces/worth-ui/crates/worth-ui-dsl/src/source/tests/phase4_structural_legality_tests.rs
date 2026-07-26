use crate::{
    WorthUiArtifactInputBodyAtom, WorthUiDslCompileDiagnosticCode, WorthUiDslCompileStopClass,
    WorthUiDslCompiler, WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
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
