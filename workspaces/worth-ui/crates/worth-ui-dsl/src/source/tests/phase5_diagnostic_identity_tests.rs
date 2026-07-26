use crate::{
    WorthUiAuthoredSourceInput, WorthUiDslCompileDiagnosticCode, WorthUiDslCompileStopClass,
    WorthUiDslCompiler,
};

#[test]
fn equivalent_transport_roots_preserve_diagnostic_identity_and_stop_class() {
    let file_report = compile_malformed_source(r"C:\workspace\from-filesystem");
    let memory_report = compile_malformed_source(".");

    assert_eq!(
        file_report.diagnostics()[0].identity(),
        memory_report.diagnostics()[0].identity()
    );
    assert_eq!(
        file_report.diagnostics()[0].stop_class(),
        memory_report.diagnostics()[0].stop_class()
    );
    assert_eq!(
        file_report.diagnostics()[0].stop_class(),
        WorthUiDslCompileStopClass::LanguageSyntax
    );
}

#[test]
fn malformed_source_identity_carries_canonical_module_and_exact_span() {
    let report = compile_malformed_source(r"C:\workspace");
    let identity = report.diagnostics()[0].identity();
    let span = identity
        .span()
        .expect("syntax failure should retain a span");

    assert_eq!(
        identity.code(),
        WorthUiDslCompileDiagnosticCode::MissingSemicolon
    );
    assert_eq!(identity.module_id(), Some("app/main.wui"));
    assert_eq!(span.module_id(), "app/main.wui");
    assert_eq!(span.start_byte(), 20);
    assert_eq!(span.end_byte(), 20);
}

fn compile_malformed_source(workspace_root: &str) -> crate::WorthUiDslCompileReport {
    WorthUiDslCompiler::compile_source(
        WorthUiAuthoredSourceInput::rooted_at(workspace_root)
            .with_module("app/main.wui", "token theme = \"dark\""),
    )
    .expect_err("missing semicolon should stop DSL compilation")
}
