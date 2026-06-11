use hadwiger_research::facade::*;

use super::query_entry_world::complete_graph;

pub fn terminal_relation(
    handle: &HadwigerResearchHandle,
    motif_id: &str,
    relation_id: &str,
) -> (MotifArtifact, TerminalForcingRelation) {
    let motif = terminal_motif(handle, motif_id);
    let graph = complete_graph(handle, &format!("{motif_id}-terminal-k2"), &["a", "b"]);
    let color_checked = verify_k_colorability_checked(handle, &graph, 1).unwrap();
    let certificate = TerminalForcingRelationCertificate::from_checked_colorability(
        relation_id,
        motif.reference(),
        TerminalForcingRelationKind::MustDiffer,
        ["b", "a"],
        color_checked.colorability_verification().clone(),
        color_checked.not_k_colorable_aspect().clone(),
    )
    .unwrap();
    let relation = certify_terminal_forcing_relation_checked(
        handle,
        TerminalForcingStudyDeclaration::new(
            format!("{relation_id}-study"),
            motif.reference().stable_token(),
        )
        .with_terminal("a")
        .unwrap()
        .with_terminal("b")
        .unwrap()
        .with_relation_goal("must_differ"),
        &motif,
        certificate,
    )
    .unwrap();
    (motif, relation)
}

fn terminal_motif(handle: &HadwigerResearchHandle, motif_id: &str) -> MotifArtifact {
    let declaration = declare_research_request_checked(
        handle,
        MotifSeedDeclaration::new(motif_id).with_source_family("tiling-equivalence-terminal"),
    )
    .admitted()
    .expect("motif declaration admits");
    MotifArtifact::builder(motif_id, declaration.into())
        .with_terminal(MotifTerminal::new("a").unwrap())
        .unwrap()
        .with_terminal(MotifTerminal::new("b").unwrap())
        .unwrap()
        .with_forbidden_same_color_pair(MotifForbiddenSameColorPair::new("a", "b").unwrap())
        .unwrap()
        .finish()
        .unwrap()
}
