use super::demand_alpha_tests::{
    demand_for, demand_for_spans, full_damage, layout_for, DemandScenario,
};
use super::{UiGlyphRasterDemandDenial, UiGlyphRasterLane};

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

#[test]
fn paint_boundary_cannot_split_a_shaping_cluster() {
    let source = "qualified";
    let layout = layout_for(source);
    let demand = demand_for(
        &layout,
        DemandScenario {
            source,
            damage: &[full_damage()],
            dpi_milli: 1_000,
            lane: UiGlyphRasterLane::Ordinary,
        },
    );
    let ligature = demand
        .records()
        .iter()
        .map(|record| record.attribution().original_range())
        .find(|range| range.end() - range.start() > 1)
        .expect("fixture must retain a multi-byte shaping cluster");
    let boundary = ligature.start() + 1;
    let spans = [
        worth_ui_host_contract::UiMountedTextForegroundSpan::from_runtime_mounting(
            worth_ui_host_contract::UiTextOriginalRange::new(0, boundary).unwrap(),
            worth_ui_host_contract::UiMountedRgba8::new(255, 0, 0, 255),
            worth_ui_host_contract::UiMountedTextPaintSpanIdentity::from_runtime_mounting([1; 32]),
        ),
        worth_ui_host_contract::UiMountedTextForegroundSpan::from_runtime_mounting(
            worth_ui_host_contract::UiTextOriginalRange::new(boundary, source.len() as u32)
                .unwrap(),
            worth_ui_host_contract::UiMountedRgba8::new(0, 0, 255, 255),
            worth_ui_host_contract::UiMountedTextPaintSpanIdentity::from_runtime_mounting([2; 32]),
        ),
    ];

    assert_eq!(
        demand_for_spans(
            &layout,
            DemandScenario {
                source,
                damage: &[full_damage()],
                dpi_milli: 1_000,
                lane: UiGlyphRasterLane::Ordinary,
            },
            &spans,
        )
        .unwrap_err(),
        UiGlyphRasterDemandDenial::PaintSpanMismatch
    );
}
