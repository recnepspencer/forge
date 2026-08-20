/// Borrowed pure-text input attached to an ordinary mounted presentation.
///
/// This value carries no atlas, Signal, device, or settlement authority. The
/// native host may invoke the raster callback only while executing the
/// authority-checked mounted-surface operation that borrowed it.
pub struct UiMountedTextRasterWork<'work> {
    demands: &'work [crate::UiGlyphRasterDemandBatchView<'work>],
    glyph_runs: &'work [crate::UiGlyphRunView],
    pins: crate::UiGlyphRasterPinTransitionView<'work>,
    binding_pins: &'work [crate::UiGlyphRasterPinRequest],
    rasterizer: &'work dyn UiMountedTextRasterCallback,
}

pub trait UiMountedTextRasterCallback {
    fn rasterize(
        &self,
        misses: crate::UiGlyphRasterMissSelectionView<'_>,
        sink: &mut dyn crate::UiGlyphRasterBatchSink,
    ) -> Result<(), crate::UiGlyphRasterCallbackDenial>;
}

#[doc(hidden)]
impl<'work> UiMountedTextRasterWork<'work> {
    pub fn from_text_mechanics(
        demands: &'work [crate::UiGlyphRasterDemandBatchView<'work>],
        glyph_runs: &'work [crate::UiGlyphRunView],
        pins: crate::UiGlyphRasterPinTransitionView<'work>,
        binding_pins: &'work [crate::UiGlyphRasterPinRequest],
        rasterizer: &'work dyn UiMountedTextRasterCallback,
    ) -> Self {
        Self {
            demands,
            glyph_runs,
            pins,
            binding_pins,
            rasterizer,
        }
    }

    pub fn demands(&self) -> &[crate::UiGlyphRasterDemandBatchView<'work>] {
        self.demands
    }

    pub const fn glyph_runs(&self) -> &'work [crate::UiGlyphRunView] {
        self.glyph_runs
    }

    pub const fn pins(&self) -> crate::UiGlyphRasterPinTransitionView<'work> {
        self.pins
    }

    pub const fn binding_pins(&self) -> &'work [crate::UiGlyphRasterPinRequest] {
        self.binding_pins
    }

    pub fn rasterize(
        &self,
        misses: crate::UiGlyphRasterMissSelectionView<'_>,
        sink: &mut dyn crate::UiGlyphRasterBatchSink,
    ) -> Result<(), crate::UiGlyphRasterCallbackDenial> {
        self.rasterizer.rasterize(misses, sink)
    }
}
