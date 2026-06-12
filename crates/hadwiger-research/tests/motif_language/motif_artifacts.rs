use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("Hadwiger handle should admit")
}

fn motif_source(
    handle: &HadwigerResearchHandle,
    motif_id: &str,
) -> HadwigerQueryDeclarationReference {
    declare_research_request_checked(
        handle,
        MotifSeedDeclaration::new(motif_id)
            .with_source_family("moser-basin")
            .with_novelty_signature("terminal-pressure:v1"),
    )
    .admitted()
    .expect("motif declaration should admit")
    .into()
}

#[test]
fn equivalent_motifs_converge_despite_insertion_order() {
    let handle = handle();
    let source = motif_source(&handle, "motif-a");
    let left = MotifArtifact::builder("motif-a", source.clone())
        .with_source_family("moser-basin")
        .unwrap()
        .with_novelty_signature("terminal-pressure:v1")
        .unwrap()
        .with_parameter(MotifParameterBinding::new("scale", "1").unwrap())
        .unwrap()
        .with_vertex(MotifVertex::new("b").unwrap())
        .unwrap()
        .with_vertex(MotifVertex::new("a").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("south").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("north").unwrap())
        .unwrap()
        .with_unit_edge(MotifUnitEdge::new("b", "a").unwrap())
        .unwrap()
        .with_forbidden_same_color_pair(MotifForbiddenSameColorPair::new("south", "north").unwrap())
        .unwrap()
        .finish()
        .unwrap();
    let right = MotifArtifact::builder("motif-a", source)
        .with_novelty_signature("terminal-pressure:v1")
        .unwrap()
        .with_source_family("moser-basin")
        .unwrap()
        .with_vertex(MotifVertex::new("a").unwrap())
        .unwrap()
        .with_vertex(MotifVertex::new("b").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("north").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("south").unwrap())
        .unwrap()
        .with_forbidden_same_color_pair(MotifForbiddenSameColorPair::new("north", "south").unwrap())
        .unwrap()
        .with_unit_edge(MotifUnitEdge::new("a", "b").unwrap())
        .unwrap()
        .with_parameter(MotifParameterBinding::new("scale", "1").unwrap())
        .unwrap()
        .finish()
        .unwrap();

    assert_eq!(left.artifact_digest(), right.artifact_digest());
    assert_eq!(left.vertices().len(), 2);
    assert_eq!(left.terminals().len(), 2);
    assert!(!left.admits_theorem_authority());
    assert!(!left.claims_terminal_forcing_authority());
}

#[test]
fn changed_motif_identity_fields_change_digest() {
    let handle = handle();
    let source = motif_source(&handle, "motif-drift");
    let left = MotifArtifact::builder("motif-drift", source.clone())
        .with_terminal(MotifTerminal::new("north").unwrap())
        .unwrap()
        .finish()
        .unwrap();
    let changed = MotifArtifact::builder("motif-drift", source)
        .with_terminal(MotifTerminal::new("south").unwrap())
        .unwrap()
        .finish()
        .unwrap();

    assert_ne!(left.artifact_digest(), changed.artifact_digest());
}

#[test]
fn motif_build_operation_rejects_crossed_query_declaration_source() {
    let handle = handle();
    let left_declaration = declare_research_request_checked(
        &handle,
        MotifSeedDeclaration::new("motif-left").with_source_family("left"),
    )
    .admitted()
    .expect("left declaration should admit");
    let right_source = motif_source(&handle, "motif-right");

    assert_eq!(
        build_motif_from_seed_declaration_checked(
            &handle,
            left_declaration,
            MotifArtifact::builder("motif-left", right_source),
        ),
        Err(MotifLanguageError::MotifSourceDeclarationMismatch)
    );
}

#[test]
fn motif_builder_rejects_shape_and_terminal_scope_errors() {
    let handle = handle();
    let source = motif_source(&handle, "motif-shape");
    let duplicate_terminal = MotifArtifact::builder("motif-shape", source.clone())
        .with_terminal(MotifTerminal::new("north").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("north").unwrap());
    assert!(matches!(
        duplicate_terminal,
        Err(MotifLanguageError::DuplicateIdentityField {
            field: "terminal",
            value
        }) if value == "north"
    ));
    assert_eq!(
        MotifArtifact::builder("motif-shape", source.clone())
            .with_terminal(MotifTerminal::new("north").unwrap())
            .unwrap()
            .with_forbidden_same_color_pair(
                MotifForbiddenSameColorPair::new("north", "missing").unwrap(),
            )
            .unwrap()
            .finish(),
        Err(MotifLanguageError::MissingMotifTerminal {
            terminal_label: "missing".to_string()
        })
    );
    assert_eq!(
        MotifArtifact::builder("motif-shape", source)
            .with_vertex(MotifVertex::new("a").unwrap())
            .unwrap()
            .with_unit_edge(MotifUnitEdge::new("a", "missing").unwrap())
            .unwrap()
            .finish(),
        Err(MotifLanguageError::MissingMotifVertex {
            vertex_label: "missing".to_string()
        })
    );
}
