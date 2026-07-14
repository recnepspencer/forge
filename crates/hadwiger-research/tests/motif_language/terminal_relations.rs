use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle()
        .expect("Hadwiger handle should admit")
}

fn complete_graph(
    handle: &HadwigerResearchHandle,
    graph_id: &str,
    labels: &[&str],
) -> GraphVersion {
    let declaration = declare_research_request_checked(
        handle,
        CandidateGraphDeclaration::new(graph_id).with_graph_version("v1"),
    )
    .admitted()
    .expect("candidate graph declaration should admit");
    let graph = GraphIdentity::from_query_declaration(graph_id, declaration.into()).unwrap();
    let mut builder = GraphVersion::builder(graph.reference(), "v1");
    for label in labels {
        builder = builder.with_vertex(*label).unwrap();
    }
    for left in 0..labels.len() {
        for right in (left + 1)..labels.len() {
            builder = builder
                .with_undirected_edge(labels[left], labels[right])
                .unwrap();
        }
    }
    builder.finish().unwrap()
}

fn motif(handle: &HadwigerResearchHandle, motif_id: &str) -> MotifArtifact {
    let declaration = declare_research_request_checked(
        handle,
        MotifSeedDeclaration::new(motif_id).with_source_family("terminal-fixture"),
    )
    .admitted()
    .expect("motif declaration should admit");
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

fn terminal_study(study_id: &str, motif: &MotifArtifact) -> TerminalForcingStudyDeclaration {
    TerminalForcingStudyDeclaration::new(study_id, motif.reference().stable_token())
        .with_terminal("a")
        .unwrap()
        .with_terminal("b")
        .unwrap()
        .with_relation_goal("must_differ")
}

#[test]
fn checked_terminal_relation_requires_admitted_colorability_evidence() {
    let handle = handle();
    let motif = motif(&handle, "motif-terminal");
    let graph = complete_graph(&handle, "terminal-k2", &["a", "b"]);
    let color_checked = verify_k_colorability_checked(&handle, &graph, 1).unwrap();
    let certificate = TerminalForcingRelationCertificate::from_checked_colorability(
        "a-b-must-differ",
        motif.reference(),
        TerminalForcingRelationKind::MustDiffer,
        ["a", "b"],
        color_checked.colorability_verification().clone(),
        color_checked.not_k_colorable_aspect().clone(),
    )
    .unwrap();
    let relation = certify_terminal_forcing_relation_checked(
        &handle,
        terminal_study("terminal-study-a", &motif),
        &motif,
        certificate,
    )
    .unwrap();

    assert!(relation.is_checked());
    assert!(relation.satisfies_terminal_relation_dependency());
    assert_eq!(
        relation.relation_kind(),
        TerminalForcingRelationKind::MustDiffer
    );
    assert_eq!(
        relation.terminal_labels(),
        &["a".to_string(), "b".to_string()]
    );
    assert!(!relation.admits_theorem_authority());
    let evidence = TerminalForcingRelationEvidence::from_checked_relation(&relation);
    assert_eq!(evidence.relation_reference(), &relation.reference());
}

#[test]
fn equivalent_terminal_relations_converge_despite_terminal_insertion_order() {
    let handle = handle();
    let motif = motif(&handle, "motif-terminal-converges");
    let graph = complete_graph(&handle, "terminal-k2-converges", &["a", "b"]);
    let color_checked = verify_k_colorability_checked(&handle, &graph, 1).unwrap();
    let left_certificate = TerminalForcingRelationCertificate::from_checked_colorability(
        "a-b-must-differ",
        motif.reference(),
        TerminalForcingRelationKind::MustDiffer,
        ["a", "b"],
        color_checked.colorability_verification().clone(),
        color_checked.not_k_colorable_aspect().clone(),
    )
    .unwrap();
    let right_certificate = TerminalForcingRelationCertificate::from_checked_colorability(
        "a-b-must-differ",
        motif.reference(),
        TerminalForcingRelationKind::MustDiffer,
        ["b", "a"],
        color_checked.colorability_verification().clone(),
        color_checked.not_k_colorable_aspect().clone(),
    )
    .unwrap();
    let left_relation = certify_terminal_forcing_relation_checked(
        &handle,
        terminal_study("terminal-study-converges", &motif),
        &motif,
        left_certificate,
    )
    .unwrap();
    let right_relation = certify_terminal_forcing_relation_checked(
        &handle,
        TerminalForcingStudyDeclaration::new(
            "terminal-study-converges",
            motif.reference().stable_token(),
        )
        .with_terminal("b")
        .unwrap()
        .with_terminal("a")
        .unwrap()
        .with_relation_goal("must_differ"),
        &motif,
        right_certificate,
    )
    .unwrap();

    assert_eq!(
        left_relation.artifact_digest(),
        right_relation.artifact_digest()
    );
    assert_eq!(
        left_relation.terminal_labels(),
        right_relation.terminal_labels()
    );
}

#[test]
fn satisfiable_or_crossed_evidence_cannot_certify_terminal_relation() {
    let handle = handle();
    let motif = motif(&handle, "motif-blocked");
    let sat_graph = complete_graph(&handle, "terminal-sat", &["a", "b"]);
    let sat_checked = verify_k_colorability_checked(&handle, &sat_graph, 2).unwrap();

    assert!(matches!(
        TerminalForcingRelationCertificate::from_checked_colorability(
            "sat-cannot-force",
            motif.reference(),
            TerminalForcingRelationKind::MustDiffer,
            ["a", "b"],
            sat_checked.colorability_verification().clone(),
            sat_checked.not_k_colorable_aspect().clone(),
        ),
        Err(MotifLanguageError::TerminalRelationEvidenceNotAdmitted)
    ));

    let left = complete_graph(&handle, "terminal-left-k2", &["a", "b"]);
    let right = complete_graph(&handle, "terminal-right-k2", &["a", "b"]);
    let left_checked = verify_k_colorability_checked(&handle, &left, 1).unwrap();
    let right_checked = verify_k_colorability_checked(&handle, &right, 1).unwrap();
    assert!(matches!(
        TerminalForcingRelationCertificate::from_checked_colorability(
            "crossed-cannot-force",
            motif.reference(),
            TerminalForcingRelationKind::MustDiffer,
            ["a", "b"],
            left_checked.colorability_verification().clone(),
            right_checked.not_k_colorable_aspect().clone(),
        ),
        Err(MotifLanguageError::TerminalRelationEvidenceNotAdmitted)
    ));
}

#[test]
fn terminal_relation_rejects_wrong_motif_or_missing_terminal() {
    let handle = handle();
    let owner_motif = motif(&handle, "motif-owner");
    let other_motif = motif(&handle, "motif-other");
    let graph = complete_graph(&handle, "terminal-k2-owner", &["a", "b"]);
    let color_checked = verify_k_colorability_checked(&handle, &graph, 1).unwrap();
    let wrong_motif_certificate = TerminalForcingRelationCertificate::from_checked_colorability(
        "wrong-motif",
        other_motif.reference(),
        TerminalForcingRelationKind::MustDiffer,
        ["a", "b"],
        color_checked.colorability_verification().clone(),
        color_checked.not_k_colorable_aspect().clone(),
    )
    .unwrap();
    assert_eq!(
        certify_terminal_forcing_relation_checked(
            &handle,
            terminal_study("terminal-study-wrong", &owner_motif),
            &owner_motif,
            wrong_motif_certificate,
        ),
        Err(MotifLanguageError::TerminalRelationMotifMismatch)
    );

    let missing_terminal_certificate =
        TerminalForcingRelationCertificate::from_checked_colorability(
            "missing-terminal",
            owner_motif.reference(),
            TerminalForcingRelationKind::MustDiffer,
            ["a", "z"],
            color_checked.colorability_verification().clone(),
            color_checked.not_k_colorable_aspect().clone(),
        )
        .unwrap();
    assert_eq!(
        certify_terminal_forcing_relation_checked(
            &handle,
            TerminalForcingStudyDeclaration::new(
                "terminal-study-missing",
                owner_motif.reference().stable_token(),
            )
            .with_terminal("a")
            .unwrap()
            .with_terminal("z")
            .unwrap()
            .with_relation_goal("must_differ"),
            &owner_motif,
            missing_terminal_certificate,
        ),
        Err(MotifLanguageError::MissingMotifTerminal {
            terminal_label: "z".to_string()
        })
    );
}

#[test]
fn terminal_relation_rejects_mismatched_query_study_intent() {
    let handle = handle();
    let owner_motif = motif(&handle, "motif-study-owner");
    let other_motif = motif(&handle, "motif-study-other");
    let graph = complete_graph(&handle, "terminal-k2-study", &["a", "b"]);
    let color_checked = verify_k_colorability_checked(&handle, &graph, 1).unwrap();
    let certificate = TerminalForcingRelationCertificate::from_checked_colorability(
        "study-mismatch",
        owner_motif.reference(),
        TerminalForcingRelationKind::MustDiffer,
        ["a", "b"],
        color_checked.colorability_verification().clone(),
        color_checked.not_k_colorable_aspect().clone(),
    )
    .unwrap();

    assert_eq!(
        certify_terminal_forcing_relation_checked(
            &handle,
            terminal_study("wrong-study-motif", &other_motif),
            &owner_motif,
            certificate.clone(),
        ),
        Err(MotifLanguageError::TerminalStudyMotifMismatch)
    );
    assert_eq!(
        certify_terminal_forcing_relation_checked(
            &handle,
            TerminalForcingStudyDeclaration::new(
                "wrong-study-terminals",
                owner_motif.reference().stable_token(),
            )
            .with_terminal("a")
            .unwrap()
            .with_relation_goal("must_differ"),
            &owner_motif,
            certificate.clone(),
        ),
        Err(MotifLanguageError::TerminalStudyTerminalMismatch)
    );
    assert_eq!(
        certify_terminal_forcing_relation_checked(
            &handle,
            TerminalForcingStudyDeclaration::new(
                "wrong-study-goal",
                owner_motif.reference().stable_token(),
            )
            .with_terminal("a")
            .unwrap()
            .with_terminal("b")
            .unwrap()
            .with_relation_goal("requires_distinct_color_count"),
            &owner_motif,
            certificate,
        ),
        Err(MotifLanguageError::TerminalStudyRelationGoalMismatch {
            expected: "requires_distinct_color_count".to_string(),
            actual: "must_differ".to_string()
        })
    );
}

#[test]
fn terminal_certificate_rejects_duplicate_terminal_identity() {
    let handle = handle();
    let owner_motif = motif(&handle, "motif-duplicate-terminal");
    let graph = complete_graph(&handle, "terminal-k2-duplicate", &["a", "b"]);
    let color_checked = verify_k_colorability_checked(&handle, &graph, 1).unwrap();

    assert!(matches!(
        TerminalForcingRelationCertificate::from_checked_colorability(
            "duplicate-terminal",
            owner_motif.reference(),
            TerminalForcingRelationKind::MustDiffer,
            ["a", "a"],
            color_checked.colorability_verification().clone(),
            color_checked.not_k_colorable_aspect().clone(),
        ),
        Err(MotifLanguageError::DuplicateIdentityField {
            field: "terminal_label",
            value
        }) if value == "a"
    ));
}
