use super::demand_alpha_tests::{demand_for, full_damage, layout_for, DemandScenario};
use super::UiGlyphRasterLane;

#[test]
fn demand_joins_contiguous_coverage_for_a_multi_grapheme_ligature() {
    let layout = layout_for("qualified");
    let demand = demand_for(
        &layout,
        DemandScenario {
            source: "qualified",
            damage: &[full_damage()],
            dpi_milli: 1_000,
            lane: UiGlyphRasterLane::Ordinary,
        },
    );
    assert!(demand.records().iter().any(|record| {
        let range = record.attribution().original_range();
        range.end() - range.start() > 1
    }));
}
