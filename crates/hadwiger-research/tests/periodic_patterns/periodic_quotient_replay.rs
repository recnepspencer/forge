use hadwiger_research::facade::*;

fn handle() -> HadwigerResearchHandle {
    crate::installed_support::installed_hadwiger_research_handle().unwrap()
}

fn rational(value: i128) -> ExactRational {
    ExactRational::integer(value)
}

fn tile(tile_id: &str, color: &str, x_min: i128, x_max: i128) -> RectangularTileRegion {
    RectangularTileRegion::new(
        tile_id,
        TilingColorId::new(color).unwrap(),
        rational(x_min),
        rational(x_max),
        rational(0),
        rational(1),
    )
    .unwrap()
    .with_boundary_ownership(BoundaryOwnershipPolicy::owned_half_open("left,bottom").unwrap())
}

fn source_cell() -> TilingCell {
    TilingCell::builder("periodic-cell")
        .with_rectangular_tile(tile("tile-a", "red", 0, 1))
        .unwrap()
        .with_rectangular_tile(tile("tile-b", "blue", 1, 2))
        .unwrap()
        .finish()
        .unwrap()
}

fn quotient(cell: &TilingCell, dx: i128) -> PeriodicQuotientCell {
    PeriodicQuotientCell::builder("quotient-a", cell.reference())
        .with_source_cell(cell.clone())
        .with_lattice_basis_vector("u", rational(dx), rational(0))
        .unwrap()
        .with_lattice_basis_vector("v", rational(0), rational(2))
        .unwrap()
        .with_translation_rule(
            PeriodicTranslationRule::new("wrap-east", "tile-a", "tile-a")
                .with_translation("u")
                .unwrap()
                .with_color_preserved()
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap()
}

#[test]
fn periodic_quotient_replay_lowers_through_query_and_keeps_authority_blocked() {
    let handle = handle();
    let cell = source_cell();
    let quotient = quotient(&cell, 2);
    let suite = GeneratedPatternReplaySuite::builder("suite-a", quotient.reference())
        .with_periodic_quotient_cell(quotient.clone())
        .unwrap()
        .finish()
        .unwrap();

    let checked = certify_periodic_quotient_replay_checked(&handle, suite).unwrap();

    assert_eq!(
        checked.periodic_quotient_cell().reference(),
        quotient.reference()
    );
    assert_eq!(checked.query_declarations_performed(), 1);
    assert_eq!(
        checked
            .periodic_quotient_report()
            .counters()
            .translation_rules_checked(),
        1
    );
    assert_eq!(
        checked
            .periodic_quotient_report()
            .counters()
            .wraparound_checks_performed(),
        1
    );
    assert!(!checked
        .periodic_quotient_report()
        .query_declaration_digest()
        .is_empty());
    assert!(!checked.admits_theorem_authority());
    assert!(!checked.registers_query_invariant_authority());
}

#[test]
fn periodic_quotient_digest_changes_with_lattice_basis_and_rejects_bad_rules() {
    let cell = source_cell();
    let base = quotient(&cell, 2);
    let changed = quotient(&cell, 3);

    assert_ne!(base.artifact_digest(), changed.artifact_digest());
    assert!(matches!(
        PeriodicQuotientCell::builder("bad-quotient", cell.reference())
            .with_source_cell(cell)
            .with_lattice_basis_vector("u", rational(2), rational(0))
            .unwrap()
            .with_translation_rule(
                PeriodicTranslationRule::new("bad-wrap", "tile-a", "missing")
                    .with_translation("u")
                    .unwrap()
                    .with_color_preserved()
                    .unwrap(),
            )
            .unwrap()
            .finish(),
        Err(GeneratedPatternReplayError::TilingGeometry(TilingGeometryError::MissingTile {
            tile_id
        })) if tile_id == "missing"
    ));
}

#[test]
fn periodic_quotient_requires_source_cell_for_rule_replay() {
    let cell = source_cell();

    let result = PeriodicQuotientCell::builder("missing-source-cell", cell.reference())
        .with_lattice_basis_vector("u", rational(2), rational(0))
        .unwrap()
        .with_translation_rule(
            PeriodicTranslationRule::new("wrap-east", "tile-a", "tile-a")
                .with_translation("u")
                .unwrap()
                .with_color_preserved()
                .unwrap(),
        )
        .unwrap()
        .finish();

    assert!(matches!(
        result,
        Err(GeneratedPatternReplayError::Shape(
            GeneratedPatternReplayShapeError::MissingSourceCell
        ))
    ));
}

#[test]
fn generated_suite_rejects_mismatched_periodic_quotient_reference() {
    let cell = source_cell();
    let quotient = quotient(&cell, 2);
    let other = PeriodicQuotientCell::builder("other-quotient", cell.reference())
        .with_source_cell(cell)
        .with_lattice_basis_vector("u", rational(2), rational(0))
        .unwrap()
        .with_translation_rule(
            PeriodicTranslationRule::new("wrap-east", "tile-a", "tile-a")
                .with_translation("u")
                .unwrap()
                .with_color_preserved()
                .unwrap(),
        )
        .unwrap()
        .finish()
        .unwrap();

    let result = GeneratedPatternReplaySuite::builder("suite-mismatch", quotient.reference())
        .with_periodic_quotient_cell(other);

    assert!(matches!(
        result,
        Err(GeneratedPatternReplayError::Shape(
            GeneratedPatternReplayShapeError::QuotientReferenceMismatch
        ))
    ));
}
