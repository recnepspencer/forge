use hadwiger_research::facade::{
    ConflictGraphExtractionDeclaration, CoreExtractionDeclaration,
    GeneratedPatternClosureDeclaration, MotifSeedDeclaration, PeriodicQuotientCellDeclaration,
    TerminalForcingStudyDeclaration, TileContactWitnessDeclaration,
};

fn main() -> Result<(), hadwiger_research::facade::HadwigerResearchDeclarationShapeError> {
    let _motif = MotifSeedDeclaration::new("motif-a")
        .with_source_family("parts-core")
        .with_novelty_signature("wl:a");
    let _terminal = TerminalForcingStudyDeclaration::new("terminal-a", "motif-a")
        .with_terminal("left")?
        .with_terminal("right")?
        .with_relation_goal("must-differ");
    let _cell = PeriodicQuotientCellDeclaration::new("cell-a")
        .with_lattice_basis_ref("lattice-a")
        .with_boundary_ownership_ref("boundary-a");
    let _closure = GeneratedPatternClosureDeclaration::new("closure-a", "cell-a")
        .with_generator("translate:e1")?
        .with_generator("translate:e2")?;
    let _contact = TileContactWitnessDeclaration::new("contact-a")
        .with_left_tile_ref("tile-a")
        .with_right_tile_ref("tile-b")
        .with_contact_signature("center-north");
    let _conflict = ConflictGraphExtractionDeclaration::new("extract-a", "cell-a")
        .with_distance_certificate_family("unit-distance-exact");
    let _core = CoreExtractionDeclaration::new("core-a", "conflict-graph-a");

    Ok(())
}
