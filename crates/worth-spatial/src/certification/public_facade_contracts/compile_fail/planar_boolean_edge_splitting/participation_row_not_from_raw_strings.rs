use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEventParticipationRow;

fn main() {
    let _ = PlanarBooleanSplitEventParticipationRow::new(
        "synthetic event ledger",
        "synthetic carrier",
        "synthetic source edge",
        "synthetic start source endpoint",
        "synthetic start projected endpoint",
        "synthetic end source endpoint",
        "synthetic end projected endpoint",
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
}
