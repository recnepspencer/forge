use worth_ui_host_contract::{
    UiGlyphRasterLane, UiMountedRgba8, UiMountedTextForegroundSpan, UiMountedTextPaintSpanIdentity,
    UiTextOriginalRange,
};

use super::demand_alpha_tests::{
    demand_for, demand_for_spans, full_damage, layout_for, DemandScenario,
};

#[test]
pub(crate) fn dpi_is_a_raster_identity_boundary_without_relayout() {
    let (layout, one_x, one_and_a_half_x) = dpi_demands();
    assert_eq!(one_x.layout_identity(), one_and_a_half_x.layout_identity());
    assert_eq!(one_x.layout_identity(), layout.identity());
    assert_ne!(one_x.identity(), one_and_a_half_x.identity());
    assert_eq!(one_x.records().len(), one_and_a_half_x.records().len());
    assert!(one_x
        .records()
        .iter()
        .zip(one_and_a_half_x.records())
        .any(|(left, right)| left.key().dpi_milli() != right.key().dpi_milli()));
    assert!(one_x
        .records()
        .iter()
        .zip(one_and_a_half_x.records())
        .all(|(left, right)| left.attribution() == right.attribution()));
}

#[test]
pub(crate) fn stale_dpi_raster_key_cannot_satisfy_the_successor_demand() {
    let (_, one_x, one_and_a_half_x) = dpi_demands();
    assert!(one_x
        .records()
        .iter()
        .zip(one_and_a_half_x.records())
        .all(|(stale, current)| stale.key() != current.key()));
}

#[test]
fn foreground_value_changes_reuse_layout_and_raster_identity() {
    let source = "WORTH";
    let layout = layout_for(source);
    let damage = [full_damage()];
    let scenario = || DemandScenario {
        source,
        damage: &damage,
        dpi_milli: 1_000,
        lane: UiGlyphRasterLane::Ordinary,
    };
    let span = |color| {
        UiMountedTextForegroundSpan::from_runtime_mounting(
            UiTextOriginalRange::new(0, source.len() as u32).unwrap(),
            color,
            UiMountedTextPaintSpanIdentity::from_runtime_mounting([31; 32]),
        )
    };
    let red = demand_for_spans(
        &layout,
        scenario(),
        &[span(UiMountedRgba8::new(255, 0, 0, 255))],
    )
    .unwrap();
    let blue = demand_for_spans(
        &layout,
        scenario(),
        &[span(UiMountedRgba8::new(0, 0, 255, 255))],
    )
    .unwrap();

    assert_eq!(red.layout_identity(), blue.layout_identity());
    assert_eq!(red.identity(), blue.identity());
    assert!(red
        .records()
        .iter()
        .zip(blue.records())
        .all(|(left, right)| left.key() == right.key()));
}

#[test]
fn reconstruction_demand_keeps_cost_in_reconstruction_lane() {
    let layout = layout_for("WORTH");
    let demand = demand_for(
        &layout,
        DemandScenario {
            source: "WORTH",
            damage: &[full_damage()],
            dpi_milli: 1_000,
            lane: UiGlyphRasterLane::Reconstruction,
        },
    );
    assert_eq!(demand.cost().ordinary().demanded_glyphs(), 0);
    assert!(demand.cost().reconstructive().demanded_glyphs() > 0);
}

fn dpi_demands() -> (
    crate::UiQualifiedTextLayout,
    super::UiGlyphRasterDemandBatch,
    super::UiGlyphRasterDemandBatch,
) {
    let layout = layout_for("WORTH");
    let make = |dpi_milli| {
        demand_for(
            &layout,
            DemandScenario {
                source: "WORTH",
                damage: &[full_damage()],
                dpi_milli,
                lane: UiGlyphRasterLane::Ordinary,
            },
        )
    };
    let one_x = make(1_000);
    let one_and_a_half_x = make(1_500);
    (layout, one_x, one_and_a_half_x)
}
