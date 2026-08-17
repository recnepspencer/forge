//! Portable runtime transaction joining mounted text meaning to the native
//! host's private atlas effect facade.

use worth_ui_host_contract::{
    UiGlyphRasterPinTransitionView, UiGlyphRasterTransactionOutcome, UiMountedFrameConsumptionView,
};

use crate::facade::UiHostEffectPort;

use super::{
    UiNativeTextMissRasterizer, UiNativeTextPresentationPrepared, UiNativeTextRasterWorkReport,
};

pub(crate) struct UiNativeTextAtlasTransaction<'layout> {
    demand_views: Box<[worth_ui_host_contract::UiGlyphRasterDemandBatchView<'layout>]>,
    rasterizer: UiNativeTextMissRasterizer<'layout>,
}

pub(crate) struct UiNativeTextAtlasTransactionObservation {
    pub(crate) outcome: UiGlyphRasterTransactionOutcome,
    pub(crate) raster_work: UiNativeTextRasterWorkReport,
}

impl<'layout> UiNativeTextAtlasTransaction<'layout> {
    pub(crate) fn prepare(
        prepared: &'layout UiNativeTextPresentationPrepared,
        resolve: impl Fn(
            worth_ui_host_contract::UiQualifiedTextLayoutIdentity,
        ) -> Option<&'layout worth_ui_text::UiQualifiedTextLayout>,
    ) -> Option<Self> {
        let demand_views = prepared
            .demand_batches()
            .iter()
            .map(worth_ui_text::UiGlyphRasterDemandBatch::as_view)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let rasterizer = UiNativeTextMissRasterizer::for_prepared(prepared, resolve)?;
        Some(Self {
            demand_views,
            rasterizer,
        })
    }

    pub(crate) fn with_mounted_work<Output>(
        &mut self,
        pins: UiGlyphRasterPinTransitionView<'_>,
        binding_pins: &[worth_ui_host_contract::UiGlyphRasterPinRequest],
        operation: impl FnOnce(&worth_ui_host_contract::UiMountedTextRasterWork<'_>) -> Output,
    ) -> Output {
        struct Callback<'rasterizer> {
            rasterizer: std::cell::RefCell<
                &'rasterizer mut dyn worth_ui_host_contract::UiGlyphRasterMissRasterizer,
            >,
        }
        impl worth_ui_host_contract::UiMountedTextRasterCallback for Callback<'_> {
            fn rasterize(
                &self,
                misses: worth_ui_host_contract::UiGlyphRasterMissSelectionView<'_>,
                sink: &mut dyn worth_ui_host_contract::UiGlyphRasterBatchSink,
            ) -> Result<(), worth_ui_host_contract::UiGlyphRasterCallbackDenial> {
                self.rasterizer.borrow_mut().rasterize(misses, sink)
            }
        }
        let callback = Callback {
            rasterizer: std::cell::RefCell::new(&mut self.rasterizer),
        };
        let work = worth_ui_host_contract::UiMountedTextRasterWork::from_text_mechanics(
            &self.demand_views,
            pins,
            binding_pins,
            &callback,
        );
        operation(&work)
    }

    pub(crate) fn execute(
        mut self,
        host: UiHostEffectPort<'_>,
        view: &UiMountedFrameConsumptionView<'_>,
        pins: UiGlyphRasterPinTransitionView<'_>,
    ) -> UiNativeTextAtlasTransactionObservation {
        let outcome = host.adapter().prepare_mounted_text_raster(
            host.authority(),
            view,
            &self.demand_views,
            pins,
            &mut self.rasterizer,
        );
        UiNativeTextAtlasTransactionObservation {
            outcome,
            raster_work: self.rasterizer.report(),
        }
    }
}
