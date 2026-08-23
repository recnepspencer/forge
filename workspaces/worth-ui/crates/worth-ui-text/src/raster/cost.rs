//! Actual ordinary and reconstructive raster work.
//!
//! Demand derivation records visits and selected records. Raster production
//! adds only work that really happened; no result posture manufactures a
//! counter value.

use super::UiGlyphRasterLane;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiGlyphRasterLaneCost {
    layout_visits: u32,
    outer_traversals: u32,
    validation_checks: u32,
    provenance_checks: u32,
    demanded_glyphs: u32,
    face_resource_lookups: u32,
    outline_evaluations: u32,
    bitmap_source_evaluations: u32,
    retained_scans: u32,
    cache_hits: u32,
    cache_misses: u32,
    rasterized_glyphs: u32,
    rasterized_texels: u64,
    produced_bytes: u64,
}

pub(crate) struct UiGlyphRasterLaneCostInput {
    pub layout_visits: u32,
    pub outer_traversals: u32,
    pub validation_checks: u32,
    pub provenance_checks: u32,
    pub demanded_glyphs: u32,
    pub face_resource_lookups: u32,
    pub outline_evaluations: u32,
    pub bitmap_source_evaluations: u32,
    pub retained_scans: u32,
    pub cache_hits: u32,
    pub cache_misses: u32,
    pub rasterized_glyphs: u32,
    pub rasterized_texels: u64,
    pub produced_bytes: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiGlyphRasterCost {
    ordinary: UiGlyphRasterLaneCost,
    reconstructive: UiGlyphRasterLaneCost,
}

impl UiGlyphRasterCost {
    pub(crate) const fn from_text_mechanics(
        ordinary: UiGlyphRasterLaneCost,
        reconstructive: UiGlyphRasterLaneCost,
    ) -> Self {
        Self {
            ordinary,
            reconstructive,
        }
    }

    pub(crate) fn add_lane_work(
        &mut self,
        lane: UiGlyphRasterLane,
        input: UiGlyphRasterLaneCostInput,
    ) {
        lane_cost_mut(self, lane).add(input);
    }

    pub const fn ordinary(self) -> UiGlyphRasterLaneCost {
        self.ordinary
    }

    pub const fn reconstructive(self) -> UiGlyphRasterLaneCost {
        self.reconstructive
    }
}

impl UiGlyphRasterLaneCost {
    pub(crate) const fn from_text_mechanics(input: UiGlyphRasterLaneCostInput) -> Self {
        Self {
            layout_visits: input.layout_visits,
            outer_traversals: input.outer_traversals,
            validation_checks: input.validation_checks,
            provenance_checks: input.provenance_checks,
            demanded_glyphs: input.demanded_glyphs,
            face_resource_lookups: input.face_resource_lookups,
            outline_evaluations: input.outline_evaluations,
            bitmap_source_evaluations: input.bitmap_source_evaluations,
            retained_scans: input.retained_scans,
            cache_hits: input.cache_hits,
            cache_misses: input.cache_misses,
            rasterized_glyphs: input.rasterized_glyphs,
            rasterized_texels: input.rasterized_texels,
            produced_bytes: input.produced_bytes,
        }
    }

    fn add(&mut self, input: UiGlyphRasterLaneCostInput) {
        self.layout_visits = self.layout_visits.saturating_add(input.layout_visits);
        self.outer_traversals = self.outer_traversals.saturating_add(input.outer_traversals);
        self.validation_checks = self
            .validation_checks
            .saturating_add(input.validation_checks);
        self.provenance_checks = self
            .provenance_checks
            .saturating_add(input.provenance_checks);
        self.demanded_glyphs = self.demanded_glyphs.saturating_add(input.demanded_glyphs);
        self.face_resource_lookups = self
            .face_resource_lookups
            .saturating_add(input.face_resource_lookups);
        self.outline_evaluations = self
            .outline_evaluations
            .saturating_add(input.outline_evaluations);
        self.bitmap_source_evaluations = self
            .bitmap_source_evaluations
            .saturating_add(input.bitmap_source_evaluations);
        self.retained_scans = self.retained_scans.saturating_add(input.retained_scans);
        self.cache_hits = self.cache_hits.saturating_add(input.cache_hits);
        self.cache_misses = self.cache_misses.saturating_add(input.cache_misses);
        self.rasterized_glyphs = self
            .rasterized_glyphs
            .saturating_add(input.rasterized_glyphs);
        self.rasterized_texels = self
            .rasterized_texels
            .saturating_add(input.rasterized_texels);
        self.produced_bytes = self.produced_bytes.saturating_add(input.produced_bytes);
    }

    pub const fn layout_visits(self) -> u32 {
        self.layout_visits
    }

    pub const fn outer_traversals(self) -> u32 {
        self.outer_traversals
    }

    pub const fn validation_checks(self) -> u32 {
        self.validation_checks
    }

    pub const fn provenance_checks(self) -> u32 {
        self.provenance_checks
    }

    pub const fn demanded_glyphs(self) -> u32 {
        self.demanded_glyphs
    }

    pub const fn requested_glyphs(self) -> u32 {
        self.demanded_glyphs
    }

    pub const fn face_resource_lookups(self) -> u32 {
        self.face_resource_lookups
    }

    pub const fn outline_evaluations(self) -> u32 {
        self.outline_evaluations
    }

    pub const fn bitmap_source_evaluations(self) -> u32 {
        self.bitmap_source_evaluations
    }

    pub const fn retained_scans(self) -> u32 {
        self.retained_scans
    }

    pub const fn cache_hits(self) -> u32 {
        self.cache_hits
    }

    pub const fn cache_misses(self) -> u32 {
        self.cache_misses
    }

    pub const fn rasterized_glyphs(self) -> u32 {
        self.rasterized_glyphs
    }

    pub const fn rasterized_texels(self) -> u64 {
        self.rasterized_texels
    }

    pub const fn rasterized_pixels(self) -> u64 {
        self.rasterized_texels
    }

    pub const fn produced_bytes(self) -> u64 {
        self.produced_bytes
    }
}

fn lane_cost_mut(
    cost: &mut UiGlyphRasterCost,
    lane: UiGlyphRasterLane,
) -> &mut UiGlyphRasterLaneCost {
    match lane {
        UiGlyphRasterLane::Ordinary => &mut cost.ordinary,
        UiGlyphRasterLane::Reconstruction => &mut cost.reconstructive,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_vocabulary_separates_lanes_and_names_observed_work() {
        let ordinary = UiGlyphRasterLaneCost::from_text_mechanics(UiGlyphRasterLaneCostInput {
            layout_visits: 7,
            outer_traversals: 7,
            validation_checks: 4,
            provenance_checks: 3,
            demanded_glyphs: 4,
            face_resource_lookups: 4,
            outline_evaluations: 3,
            bitmap_source_evaluations: 2,
            retained_scans: 0,
            cache_hits: 1,
            cache_misses: 3,
            rasterized_glyphs: 3,
            rasterized_texels: 48,
            produced_bytes: 48,
        });
        let cost = UiGlyphRasterCost::from_text_mechanics(ordinary, Default::default());
        assert_eq!(cost.ordinary().layout_visits(), 7);
        assert_eq!(cost.ordinary().outer_traversals(), 7);
        assert_eq!(cost.ordinary().validation_checks(), 4);
        assert_eq!(cost.ordinary().provenance_checks(), 3);
        assert_eq!(cost.ordinary().demanded_glyphs(), 4);
        assert_eq!(cost.ordinary().outline_evaluations(), 3);
        assert_eq!(cost.ordinary().bitmap_source_evaluations(), 2);
        assert_eq!(cost.ordinary().cache_misses(), 3);
        assert_eq!(cost.ordinary().rasterized_texels(), 48);
        assert_eq!(cost.reconstructive().rasterized_glyphs(), 0);
    }
}
