//! Runtime-owned join from native-admitted misses to text-owned raster bytes.

use worth_ui_host_contract::{
    UiGlyphRasterBatchSink, UiGlyphRasterCallbackDenial, UiGlyphRasterMissRasterizer,
    UiGlyphRasterMissSelectionView,
};

use super::UiNativeTextPresentationPrepared;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiNativeTextRasterWorkReport {
    rasterized_glyphs: u32,
    rasterized_texels: u64,
    produced_bytes: u64,
}

impl UiNativeTextRasterWorkReport {
    pub(crate) const fn not_admitted() -> Self {
        Self {
            rasterized_glyphs: 0,
            rasterized_texels: 0,
            produced_bytes: 0,
        }
    }

    pub(crate) const fn rasterized_glyphs(self) -> u32 {
        self.rasterized_glyphs
    }

    pub(crate) const fn rasterized_texels(self) -> u64 {
        self.rasterized_texels
    }

    pub(crate) const fn produced_bytes(self) -> u64 {
        self.produced_bytes
    }

    fn add(&mut self, cost: worth_ui_text::UiGlyphRasterCost) {
        for lane in [cost.ordinary(), cost.reconstructive()] {
            self.rasterized_glyphs = self
                .rasterized_glyphs
                .saturating_add(lane.rasterized_glyphs());
            self.rasterized_texels = self
                .rasterized_texels
                .saturating_add(lane.rasterized_texels());
            self.produced_bytes = self.produced_bytes.saturating_add(lane.produced_bytes());
        }
    }
}

struct UiNativeTextRasterSource<'layout> {
    layout: &'layout worth_ui_text::UiQualifiedTextLayout,
    demand: &'layout worth_ui_text::UiGlyphRasterDemandBatch,
}

pub(crate) struct UiNativeTextMissRasterizer<'layout> {
    sources: Box<[UiNativeTextRasterSource<'layout>]>,
    report: UiNativeTextRasterWorkReport,
}

impl<'layout> UiNativeTextMissRasterizer<'layout> {
    pub(crate) fn for_prepared(
        prepared: &'layout UiNativeTextPresentationPrepared,
        resolve: impl Fn(
            worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
        ) -> Option<&'layout worth_ui_text::UiQualifiedTextLayout>,
    ) -> Option<Self> {
        let sources = prepared
            .demand_batches()
            .iter()
            .map(|demand| {
                Some(UiNativeTextRasterSource {
                    layout: resolve(demand.layout_identity())?,
                    demand,
                })
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self {
            sources: sources.into_boxed_slice(),
            report: UiNativeTextRasterWorkReport::default(),
        })
    }

    pub(crate) const fn report(&self) -> UiNativeTextRasterWorkReport {
        self.report
    }

    fn source_for(
        &self,
        misses: UiGlyphRasterMissSelectionView<'_>,
    ) -> Option<&UiNativeTextRasterSource<'layout>> {
        self.sources.iter().find(|source| {
            source.demand.identity() == misses.demand_identity()
                && source.demand.layout_identity() == misses.layout_identity()
                && source.demand.lane() == misses.lane()
        })
    }
}

impl UiGlyphRasterMissRasterizer for UiNativeTextMissRasterizer<'_> {
    fn rasterize(
        &mut self,
        misses: UiGlyphRasterMissSelectionView<'_>,
        sink: &mut dyn UiGlyphRasterBatchSink,
    ) -> Result<(), UiGlyphRasterCallbackDenial> {
        let source = self
            .source_for(misses)
            .ok_or(UiGlyphRasterCallbackDenial::DemandMismatch)?;
        let keys = misses
            .records()
            .iter()
            .map(|record| record.key())
            .collect::<Vec<_>>();
        let alpha =
            worth_ui_text::rasterize_alpha_outline_selection(source.layout, source.demand, &keys)
                .map_err(|_| UiGlyphRasterCallbackDenial::RasterizationDenied)?;
        let color =
            worth_ui_text::rasterize_intrinsic_color_selection(source.layout, source.demand, &keys)
                .map_err(|_| UiGlyphRasterCallbackDenial::RasterizationDenied)?;
        if !alpha.batch().records().is_empty() {
            alpha
                .batch()
                .with_view(|batch| sink.submit_alpha(batch))
                .map_err(UiGlyphRasterCallbackDenial::BatchRejected)?;
        }
        if !color.batch().records().is_empty() {
            color
                .batch()
                .with_view(|batch| sink.submit_color(batch))
                .map_err(UiGlyphRasterCallbackDenial::BatchRejected)?;
        }
        self.report.add(alpha.cost());
        self.report.add(color.cost());
        Ok(())
    }
}
