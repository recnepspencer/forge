use crate::source::{
    WorthUiParseDiagnosticCode, WorthUiParsedSourceDeclaration, WorthUiSourcePackageLoader,
    WorthUiSourceParser, WorthUiSourceTokenKind,
};

#[test]
fn equivalent_source_text_produces_equivalent_parsed_structure() {
    let package_a = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(
            "app/main.wui",
            r#"
            import "app/panels/inspector.wui";
            component Dashboard {}
            token accent = "blue";
            "#,
        )
        .compile()
        .expect("package a should compile");

    let package_b = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(
            "app/main.wui",
            r#"
            // same meaning, different trivia
            import   "app/panels/inspector.wui" ;
            component Dashboard
            {
            }
            token accent = "blue";
            "#,
        )
        .compile()
        .expect("package b should compile");

    let parsed_a = WorthUiSourceParser::parse_package(&package_a).expect("package a should parse");
    let parsed_b = WorthUiSourceParser::parse_package(&package_b).expect("package b should parse");

    assert_ne!(parsed_a, parsed_b);
    assert!(parsed_a.equivalent_shape(&parsed_b));
}

#[test]
fn malformed_source_localizes_parse_diagnostics_to_source_spans() {
    let source_text = "component Dashboard {";
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", source_text)
        .compile()
        .expect("package should compile");

    let report = WorthUiSourceParser::parse_package(&package)
        .expect_err("unterminated block should fail parsing");
    let diagnostic = report
        .diagnostics()
        .first()
        .expect("parse diagnostic should be present");

    assert_eq!(
        diagnostic.code(),
        WorthUiParseDiagnosticCode::UnterminatedBlock
    );
    assert_eq!(diagnostic.span().module_id().as_str(), "app/main.wui");
    assert_eq!(
        diagnostic.span().start_byte(),
        source_text.find('{').expect("brace should exist")
    );
    assert_eq!(
        diagnostic.span().end_byte(),
        source_text.find('{').expect("brace should exist") + 1
    );
}

#[test]
fn parse_replay_is_deterministic() {
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(
            "app/main.wui",
            r#"
            import "app/panels/inspector.wui";
            component Dashboard {}
            token accent = "blue";
            "#,
        )
        .register_module_with_source(
            "app/panels/inspector.wui",
            r#"
            surface Inspector {
                token nested = "present";
            }
            "#,
        )
        .compile()
        .expect("package should compile");

    let first_parse =
        WorthUiSourceParser::parse_package(&package).expect("first parse should pass");
    let replay_parse =
        WorthUiSourceParser::parse_package(&package).expect("replayed parse should pass");

    assert_eq!(first_parse, replay_parse);
}

#[test]
fn multiple_parse_failures_preserve_canonical_diagnostic_order() {
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/zeta.wui", "component Zeta {")
        .register_module_with_source("app/alpha.wui", "@")
        .compile()
        .expect("package should compile");

    let report = WorthUiSourceParser::parse_package(&package)
        .expect_err("multiple malformed modules should fail parsing");

    assert_eq!(report.diagnostics().len(), 2);
    assert_eq!(
        report.diagnostics()[0].span().module_id().as_str(),
        "app/alpha.wui"
    );
    assert_eq!(
        report.diagnostics()[1].span().module_id().as_str(),
        "app/zeta.wui"
    );
}

#[test]
fn parsed_module_order_matches_phase_1_canonical_order() {
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/zeta.wui", "component Zeta {}")
        .register_module_with_source("app/alpha.wui", "component Alpha {}")
        .compile()
        .expect("package should compile");

    let parsed_package =
        WorthUiSourceParser::parse_package(&package).expect("package should parse");

    assert_eq!(parsed_package.module_ids(), package.module_ids());
}

#[test]
fn eof_parse_failure_localizes_to_end_of_module() {
    let source_text = "token accent = \"blue\"";
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source("app/main.wui", source_text)
        .compile()
        .expect("package should compile");

    let report =
        WorthUiSourceParser::parse_package(&package).expect_err("missing semicolon should fail");
    let diagnostic = report
        .diagnostics()
        .first()
        .expect("parse diagnostic should be present");

    assert_eq!(
        diagnostic.code(),
        WorthUiParseDiagnosticCode::MissingSemicolon
    );
    assert_eq!(diagnostic.span().module_id().as_str(), "app/main.wui");
    assert_eq!(diagnostic.span().start_byte(), source_text.len());
    assert_eq!(diagnostic.span().end_byte(), source_text.len());
}

#[test]
fn nested_block_tokens_preserve_structure_without_trivia_noise() {
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(
            "app/main.wui",
            r#"
            surface Inspector {
                component Body {
                    token nested = "value";
                }
            }
            "#,
        )
        .compile()
        .expect("package should compile");

    let parsed_package =
        WorthUiSourceParser::parse_package(&package).expect("package should parse");
    let module = parsed_package
        .module(&package.module_ids()[0])
        .expect("parsed module should exist");
    let declaration = &module.declarations()[0];

    match declaration {
        WorthUiParsedSourceDeclaration::Surface(surface) => {
            assert_eq!(surface.name_text(), "Inspector");
            assert!(surface
                .body()
                .tokens()
                .iter()
                .any(|token| matches!(token.kind(), WorthUiSourceTokenKind::KeywordComponent)));
        }
        _ => panic!("expected surface declaration"),
    }
}

#[test]
fn page_template_parameters_parse_as_typed_structure() {
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(
            "app/main.wui",
            r#"
            page ProductDetailPage(product_id: ProductId, shop_id: ShopId) {
                runtime ProductDetailRuntime
                layout ProductDetailLayout
                content ProductDetailContent
            }
            "#,
        )
        .compile()
        .expect("package should compile");

    let parsed_package =
        WorthUiSourceParser::parse_package(&package).expect("package should parse");
    let module = parsed_package
        .module(&package.module_ids()[0])
        .expect("parsed module should exist");

    match &module.declarations()[0] {
        WorthUiParsedSourceDeclaration::Page(page) => {
            let parameters = page.template_parameters();
            assert_eq!(parameters.len(), 2);
            assert_eq!(parameters[0].name_text(), "product_id");
            assert_eq!(parameters[0].type_text(), "ProductId");
            assert_eq!(parameters[1].name_text(), "shop_id");
            assert_eq!(parameters[1].type_text(), "ShopId");
        }
        _ => panic!("expected page declaration"),
    }
}

#[test]
fn parse_recovery_accumulates_multiple_same_module_diagnostics() {
    let package = WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
        .register_module_with_source(
            "app/main.wui",
            r#"
            token accent = "blue"
            token = "green";
            component Stable {}
            "#,
        )
        .compile()
        .expect("package should compile");

    let report = WorthUiSourceParser::parse_package(&package)
        .expect_err("multiple malformed declarations should fail parsing");

    assert_eq!(report.diagnostics().len(), 2);
    assert_eq!(
        report.diagnostics()[0].code(),
        WorthUiParseDiagnosticCode::MissingSemicolon
    );
    assert_eq!(
        report.diagnostics()[1].code(),
        WorthUiParseDiagnosticCode::MissingIdentifier
    );
}
