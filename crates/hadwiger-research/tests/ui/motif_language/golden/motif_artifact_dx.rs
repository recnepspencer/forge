use hadwiger_research::facade::{
    admit_hadwiger_research_handle, build_motif_from_seed_declaration_checked,
    declare_research_request_checked, HadwigerCanonicalArtifact, HadwigerDeclaredFamilyCheckedExt,
    HadwigerResearchOperatingContext, MotifArtifact, MotifForbiddenSameColorPair,
    MotifSeedDeclaration, MotifTerminal, MotifUnitEdge, MotifVertex,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .expect("handle admits");
    let declaration = declare_research_request_checked(
        &handle,
        MotifSeedDeclaration::new("moser-basin-motif")
            .with_source_family("moser-basin")
            .with_novelty_signature("terminal-pressure:v1"),
    )
    .admitted()
    .expect("motif declaration should admit");
    let source_reference = declaration.clone().into();

    let motif = build_motif_from_seed_declaration_checked(
        &handle,
        declaration,
        MotifArtifact::builder("moser-basin-motif", source_reference)
            .with_source_family("moser-basin")
            .expect("source family admits")
            .with_vertex(MotifVertex::new("a").expect("vertex admits"))
            .expect("vertex admits")
            .with_vertex(MotifVertex::new("b").expect("vertex admits"))
            .expect("vertex admits")
            .with_terminal(MotifTerminal::new("north").expect("terminal admits"))
            .expect("terminal admits")
            .with_terminal(MotifTerminal::new("south").expect("terminal admits"))
            .expect("terminal admits")
            .with_unit_edge(MotifUnitEdge::new("a", "b").expect("unit edge admits"))
            .expect("unit edge admits")
            .with_forbidden_same_color_pair(
                MotifForbiddenSameColorPair::new("north", "south").expect("pair admits"),
            )
            .expect("forbidden pair admits"),
    )
    .expect("motif builds");

    assert_eq!(motif.motif_id(), "moser-basin-motif");
    assert!(!motif.admits_theorem_authority());
    assert!(!motif.reference().stable_token().is_empty());
}
