use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
        .expect("real Hadwiger handle admits")
}

fn tile(
    tile_id: &str,
    color_id: &str,
    x_min: ExactRational,
    x_max: ExactRational,
) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        TilingColorId::new(color_id).unwrap(),
        x_min,
        x_max,
        ExactRational::integer(0),
        ExactRational::integer(1),
    )
    .unwrap()
    .with_boundary_ownership(BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap())
}

fn cell_for_unit_crossing() -> TilingCell {
    TilingCell::builder("conflict-cell")
        .with_rectangular_tile(tile(
            "tile-a",
            "red",
            ExactRational::integer(0),
            ExactRational::integer(1),
        ))
        .unwrap()
        .with_rectangular_tile(tile(
            "tile-b",
            "red",
            ExactRational::integer(1),
            ExactRational::integer(2),
        ))
        .unwrap()
        .finish()
        .unwrap()
}

fn cell_without_unit_crossing() -> TilingCell {
    TilingCell::builder("safe-cell")
        .with_rectangular_tile(tile(
            "tile-a",
            "red",
            ExactRational::integer(0),
            ExactRational::fraction(1, 4).unwrap(),
        ))
        .unwrap()
        .with_rectangular_tile(tile(
            "tile-b",
            "red",
            ExactRational::integer(3),
            ExactRational::fraction(13, 4).unwrap(),
        ))
        .unwrap()
        .finish()
        .unwrap()
}

fn screening_unit_square(region_id: &str, x_offset: i128) -> ScreeningRectangularRegion {
    ScreeningRectangularRegion::new(
        region_id,
        ScreeningRational::integer(x_offset),
        ScreeningRational::integer(x_offset + 1),
        ScreeningRational::integer(0),
        ScreeningRational::integer(1),
    )
    .unwrap()
}

fn periodic_conflict_model() -> PeriodicQuotientRectangleModel {
    PeriodicQuotientRectangleModel::new(
        "generated-conflict-rectangles",
        vec![
            PeriodicQuotientTile::new("left", "red", screening_unit_square("left-region", 0))
                .unwrap(),
            PeriodicQuotientTile::new("right", "red", screening_unit_square("right-region", 2))
                .unwrap(),
        ],
    )
    .unwrap()
}

fn generated_quotient() -> PeriodicQuotientCell {
    let cell = TilingCell::builder("generated-source-cell")
        .with_rectangular_tile(tile(
            "tile-a",
            "red",
            ExactRational::integer(0),
            ExactRational::integer(1),
        ))
        .unwrap()
        .finish()
        .unwrap();
    PeriodicQuotientCell::builder("generated-quotient", cell.reference())
        .with_source_cell(cell)
        .with_lattice_basis_vector("u", ExactRational::integer(2), ExactRational::integer(0))
        .unwrap()
        .with_translation_rule(
            PeriodicTranslationRule::new("wrap", "tile-a", "tile-a")
                .with_translation("u")
                .unwrap()
                .with_color_preserved()
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap()
}

fn screening_transcript(status: &str) -> ScreeningSolverTranscript {
    ScreeningSolverTranscript::new(
        "conflict-graph-extraction-test",
        "phase5",
        format!("transcript-{status}"),
        status,
    )
    .unwrap()
}

fn color_holonomy_loop(to_color: &str) -> ColorHolonomyLoopCertificate {
    ColorHolonomyLoopCertificate::builder("loop-a", "tile-a", "red")
        .with_color_permutation("permute", [("red", to_color), ("blue", "blue")])
        .unwrap()
        .finish()
        .unwrap()
}

#[test]
fn conflict_graph_extraction_lowers_through_query_and_retains_exact_evidence() {
    let handle = handle();
    let contact = evaluate_tiling_same_color_contact_checked(
        &handle,
        &cell_for_unit_crossing(),
        "tile-b",
        "tile-a",
    )
    .unwrap();

    let graph = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report("extract-a", contact),
    )
    .unwrap();

    assert_eq!(graph.graph_version().vertices().len(), 2);
    assert_eq!(graph.graph_version().edges().len(), 1);
    assert_eq!(graph.conflict_edges().len(), 1);
    assert_eq!(graph.conflict_edges()[0].left_vertex_label(), "tile-a");
    assert_eq!(graph.conflict_edges()[0].right_vertex_label(), "tile-b");
    assert_eq!(
        graph.conflict_edges()[0].basis(),
        TilingConflictEdgeBasis::ExactContactReplay
    );
    assert!(graph.conflict_edges()[0].has_exact_conflict_evidence());
    assert_eq!(graph.counters().query_declarations_performed(), 1);
    assert_eq!(
        graph.query_declaration_reference().declaration_family_key(),
        "hadwiger.tiling.conflict_graph_extraction"
    );
    assert!(!graph.admits_theorem_authority());
}

#[test]
fn generated_periodic_conflict_uses_source_tile_and_translation_labels() {
    let handle = handle();
    let quotient = generated_quotient();
    let conflict = PeriodicQuotientConflictCertificate::new(
        "left",
        "right",
        ScreeningRational::integer(-1),
        ScreeningRational::integer(0),
        screening_transcript("periodic-conflict"),
    )
    .unwrap();
    let suite =
        GeneratedPatternReplaySuite::builder("generated-conflict-suite", quotient.reference())
            .with_periodic_quotient_cell(quotient)
            .unwrap()
            .with_periodic_quotient_conflict_certificate(periodic_conflict_model(), conflict)
            .unwrap()
            .finish()
            .unwrap();
    let checked = certify_generated_pattern_replay_checked(&handle, suite).unwrap();

    let graph = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_generated_pattern_replay(
            "generated-conflict-graph",
            &checked,
        ),
    )
    .unwrap();

    assert_eq!(graph.conflict_edges().len(), 1);
    assert_eq!(graph.conflict_edges()[0].left_vertex_label(), "left");
    assert_eq!(graph.conflict_edges()[0].right_vertex_label(), "right");
    assert_eq!(
        graph.conflict_edges()[0].basis(),
        TilingConflictEdgeBasis::PeriodicGeneratedReplay
    );
    assert!(graph.conflict_edges()[0]
        .translated_boundary()
        .unwrap()
        .starts_with("periodic_translation:"));
}

#[test]
fn generated_non_graph_failure_does_not_mint_synthetic_conflict_edges() {
    let handle = handle();
    let quotient = generated_quotient();
    let suite =
        GeneratedPatternReplaySuite::builder("generated-holonomy-suite", quotient.reference())
            .with_periodic_quotient_cell(quotient)
            .unwrap()
            .with_color_holonomy_loop(color_holonomy_loop("blue"))
            .unwrap()
            .finish()
            .unwrap();
    let checked = certify_generated_pattern_replay_checked(&handle, suite).unwrap();

    assert_eq!(
        extract_conflict_graph_checked(
            &handle,
            TilingConflictGraphExtractionRequest::from_generated_pattern_replay(
                "generated-holonomy-graph",
                &checked
            )
        ),
        Err(ConflictGraphError::GeneratedReplayHasNoExtractableConflictEdges)
    );
}

#[test]
fn equivalent_contact_order_converges_to_same_conflict_graph_digest() {
    let handle = handle();
    let forward = evaluate_tiling_same_color_contact_checked(
        &handle,
        &cell_for_unit_crossing(),
        "tile-a",
        "tile-b",
    )
    .unwrap();
    let reversed = evaluate_tiling_same_color_contact_checked(
        &handle,
        &cell_for_unit_crossing(),
        "tile-b",
        "tile-a",
    )
    .unwrap();

    let left = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report("extract-a", forward),
    )
    .unwrap();
    let right = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report("extract-a", reversed),
    )
    .unwrap();

    assert_eq!(
        (
            left.conflict_edges()[0].left_vertex_label(),
            left.conflict_edges()[0].right_vertex_label()
        ),
        (
            right.conflict_edges()[0].left_vertex_label(),
            right.conflict_edges()[0].right_vertex_label()
        )
    );
    assert_eq!(left.artifact_digest(), right.artifact_digest());
}

#[test]
fn changed_distance_certificate_basis_changes_conflict_graph_digest() {
    let handle = handle();
    let cell = cell_for_unit_crossing();
    let same_color =
        evaluate_tiling_same_color_contact_checked(&handle, &cell, "tile-a", "tile-b").unwrap();
    let minkowski =
        evaluate_tiling_minkowski_contact_checked(&handle, &cell, "tile-a", "tile-b").unwrap();

    let left = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report(
            "extract-basis",
            same_color,
        ),
    )
    .unwrap();
    let right = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report(
            "extract-basis",
            minkowski,
        ),
    )
    .unwrap();

    assert_eq!(
        (
            left.conflict_edges()[0].left_vertex_label(),
            left.conflict_edges()[0].right_vertex_label()
        ),
        (
            right.conflict_edges()[0].left_vertex_label(),
            right.conflict_edges()[0].right_vertex_label()
        )
    );
    assert_ne!(
        left.conflict_edges()[0].translated_boundary(),
        right.conflict_edges()[0].translated_boundary()
    );
    assert_ne!(left.artifact_digest(), right.artifact_digest());
}

#[test]
fn required_color_count_is_retained_in_conflict_graph_identity() {
    let handle = handle();
    let contact = evaluate_tiling_same_color_contact_checked(
        &handle,
        &cell_for_unit_crossing(),
        "tile-a",
        "tile-b",
    )
    .unwrap();
    let baseline = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report(
            "extract-color",
            contact.clone(),
        ),
    )
    .unwrap();
    let constrained = extract_conflict_graph_checked(
        &handle,
        TilingConflictGraphExtractionRequest::from_tiling_contact_report("extract-color", contact)
            .with_required_color_count(6)
            .unwrap(),
    )
    .unwrap();

    assert_eq!(baseline.required_color_count(), None);
    assert_eq!(constrained.required_color_count(), Some(6));
    assert_ne!(
        baseline.query_declaration_digest(),
        constrained.query_declaration_digest()
    );
    assert_ne!(baseline.artifact_digest(), constrained.artifact_digest());
}

#[test]
fn non_rejecting_contact_report_cannot_mint_conflict_edges() {
    let handle = handle();
    assert!(matches!(
        evaluate_tiling_same_color_contact_checked(
            &handle,
            &cell_without_unit_crossing(),
            "tile-a",
            "tile-b"
        ),
        Err(TilingGeometryError::Screening(
            CandidateScreeningError::CertificateReplayRejected { .. }
        ))
    ));
}
