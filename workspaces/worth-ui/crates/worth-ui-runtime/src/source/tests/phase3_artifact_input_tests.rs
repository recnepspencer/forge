use crate::source::{
    WorthUiArtifactInputBodyAtom, WorthUiArtifactInputNode, WorthUiArtifactInputNormalizer,
    WorthUiArtifactInputProvenance, WorthUiParsedSourceToArtifactInputLowerer,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiRustAuthoredToArtifactInputLowerer, WorthUiSourcePackageLoader, WorthUiSourceParser,
};

#[test]
fn equivalent_file_and_rust_authoring_produce_equivalent_artifact_input() {
    let file_authored = lower_file_authored(
        WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
            .register_module_with_source(
                "app/main.wui",
                r#"
                import "app/panels/inspector.wui";
                component Dashboard {}
                surface Inspector {}
                binding Selection {}
                token accent = "blue";
                "#,
            )
            .register_module_with_source("app/panels/inspector.wui", "component InspectorPanel {}")
            .compile()
            .expect("file-authored package should compile"),
    );

    let rust_authored = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_import("app/panels/inspector.wui")
                .with_component("Dashboard")
                .with_surface("Inspector")
                .with_binding("Selection")
                .with_token("accent", "blue"),
            WorthUiRustAuthoredArtifactInputModule::new("app/panels/inspector.wui")
                .with_component("InspectorPanel"),
        ]),
    );

    assert!(file_authored.equivalent_shape(&rust_authored));
}

#[test]
fn artifact_input_normalization_is_canonical() {
    let normalized_a = WorthUiArtifactInputNormalizer::normalize(
        WorthUiRustAuthoredToArtifactInputLowerer::lower(
            &WorthUiRustAuthoredArtifactInput::from_modules([
                WorthUiRustAuthoredArtifactInputModule::new("app/zeta.wui")
                    .with_token("z", "1")
                    .with_component("Zeta"),
                WorthUiRustAuthoredArtifactInputModule::new("app/alpha.wui")
                    .with_surface("Alpha")
                    .with_binding("Focus"),
            ]),
        ),
    );

    let normalized_b = WorthUiArtifactInputNormalizer::normalize(
        WorthUiRustAuthoredToArtifactInputLowerer::lower(
            &WorthUiRustAuthoredArtifactInput::from_modules([
                WorthUiRustAuthoredArtifactInputModule::new("app/alpha.wui")
                    .with_binding("Focus")
                    .with_surface("Alpha"),
                WorthUiRustAuthoredArtifactInputModule::new("app/zeta.wui")
                    .with_component("Zeta")
                    .with_token("z", "1"),
            ]),
        ),
    );

    assert_eq!(normalized_a.module_ids(), normalized_b.module_ids());
    assert!(normalized_a.equivalent_shape(&normalized_b));

    let alpha_module = normalized_a
        .module(&normalized_a.module_ids()[0])
        .expect("alpha module should exist");
    assert_eq!(alpha_module.module_id(), &normalized_a.module_ids()[0]);
}

#[test]
fn authoring_specific_escape_hatch_does_not_bypass_ir_boundary() {
    let rust_authored_input = WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component("Dashboard"),
    ]);

    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(&rust_authored_input);
    let module = artifact_input
        .module(&artifact_input.module_ids()[0])
        .expect("artifact-input module should exist");
    let node = &module.nodes()[0];

    match node {
        WorthUiArtifactInputNode::Component(component_node) => {
            assert!(matches!(
                component_node.provenance(),
                WorthUiArtifactInputProvenance::RustAuthoredDeclaration { .. }
            ));
        }
        _ => panic!("expected component node"),
    }
}

#[test]
fn file_authored_lowering_preserves_source_span_provenance() {
    let artifact_input = lower_file_authored(
        WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
            .register_module_with_source("app/main.wui", "token accent = \"blue\";")
            .compile()
            .expect("file-authored package should compile"),
    );

    let module = artifact_input
        .module(&artifact_input.module_ids()[0])
        .expect("artifact-input module should exist");

    match &module.nodes()[0] {
        WorthUiArtifactInputNode::Token(token_node) => match token_node.provenance() {
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
                declaration_span,
                detail_span,
                ..
            } => {
                assert_eq!(declaration_span.module_id().as_str(), "app/main.wui");
                assert!(detail_span.is_some());
            }
            _ => panic!("expected parsed-source provenance"),
        },
        _ => panic!("expected token node"),
    }
}

#[test]
fn file_and_rust_authoring_preserve_distinct_provenance_while_converging_on_shape() {
    let file_authored = lower_file_authored(
        WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
            .register_module_with_source("app/main.wui", "component Dashboard {}")
            .compile()
            .expect("file-authored package should compile"),
    );
    let rust_authored = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component("Dashboard"),
        ]),
    );

    let file_node = &file_authored
        .module(&file_authored.module_ids()[0])
        .expect("file-authored module should exist")
        .nodes()[0];
    let rust_node = &rust_authored
        .module(&rust_authored.module_ids()[0])
        .expect("rust-authored module should exist")
        .nodes()[0];

    assert!(file_authored.equivalent_shape(&rust_authored));
    assert_ne!(file_node.provenance(), rust_node.provenance());
}

#[test]
fn non_empty_block_body_converges_between_file_and_rust_authoring() {
    let file_authored = lower_file_authored(
        WorthUiSourcePackageLoader::from_workspace_root(r"C:\workspace")
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
            .expect("file-authored package should compile"),
    );
    let rust_authored = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_surface_body_atoms(
                "Inspector",
                [
                    WorthUiArtifactInputBodyAtom::KeywordComponent,
                    WorthUiArtifactInputBodyAtom::Identifier("Body".to_owned()),
                    WorthUiArtifactInputBodyAtom::LeftBrace,
                    WorthUiArtifactInputBodyAtom::KeywordToken,
                    WorthUiArtifactInputBodyAtom::Identifier("nested".to_owned()),
                    WorthUiArtifactInputBodyAtom::Equals,
                    WorthUiArtifactInputBodyAtom::StringLiteral("value".to_owned()),
                    WorthUiArtifactInputBodyAtom::Semicolon,
                    WorthUiArtifactInputBodyAtom::RightBrace,
                ],
            ),
        ]),
    );

    assert!(file_authored.equivalent_shape(&rust_authored));
}

#[test]
fn rust_authored_block_body_builders_lower_into_artifact_input() {
    let artifact_input = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                .with_component_body_atoms(
                    "Dashboard",
                    [WorthUiArtifactInputBodyAtom::Identifier("body".to_owned())],
                )
                .with_binding_body_atoms(
                    "Selection",
                    [WorthUiArtifactInputBodyAtom::StringLiteral(
                        "detail".to_owned(),
                    )],
                ),
        ]),
    );

    let module = artifact_input
        .module(&artifact_input.module_ids()[0])
        .expect("artifact-input module should exist");

    let binding_node = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiArtifactInputNode::Binding(binding_node) => Some(binding_node),
            _ => None,
        })
        .expect("binding node should exist");
    assert_eq!(
        binding_node.body_atoms(),
        &[WorthUiArtifactInputBodyAtom::StringLiteral(
            "detail".to_owned()
        )]
    );

    let component_node = module
        .nodes()
        .iter()
        .find_map(|node| match node {
            WorthUiArtifactInputNode::Component(component_node) => Some(component_node),
            _ => None,
        })
        .expect("component node should exist");
    assert_eq!(
        component_node.body_atoms(),
        &[WorthUiArtifactInputBodyAtom::Identifier("body".to_owned())]
    );
}

#[test]
fn rust_authored_duplicate_canonical_module_identity_is_rejected() {
    let duplicate_modules = WorthUiRustAuthoredArtifactInput::from_modules([
        WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component("Dashboard"),
        WorthUiRustAuthoredArtifactInputModule::new("app/./main.wui").with_surface("Inspector"),
    ]);

    let panic_result = std::panic::catch_unwind(|| {
        WorthUiRustAuthoredToArtifactInputLowerer::lower(&duplicate_modules)
    });

    assert!(panic_result.is_err());
}

#[test]
fn different_body_atoms_do_not_compare_as_equivalent_shape() {
    let first = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component_body_atoms(
                "Dashboard",
                [WorthUiArtifactInputBodyAtom::Identifier("alpha".to_owned())],
            ),
        ]),
    );
    let second = WorthUiRustAuthoredToArtifactInputLowerer::lower(
        &WorthUiRustAuthoredArtifactInput::from_modules([
            WorthUiRustAuthoredArtifactInputModule::new("app/main.wui").with_component_body_atoms(
                "Dashboard",
                [WorthUiArtifactInputBodyAtom::Identifier("beta".to_owned())],
            ),
        ]),
    );

    assert!(!first.equivalent_shape(&second));
}

fn lower_file_authored(
    source_package: crate::source::WorthUiSourcePackage,
) -> crate::source::WorthUiArtifactInput {
    let parsed_source_package =
        WorthUiSourceParser::parse_package(&source_package).expect("source package should parse");
    WorthUiParsedSourceToArtifactInputLowerer::lower(&parsed_source_package)
}
