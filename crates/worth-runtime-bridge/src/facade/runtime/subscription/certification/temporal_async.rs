use super::*;

impl RuntimeBridge {
    /// Builds a Phase 16 temporal/async certification bundle draft from
    /// already-admitted retained artifacts rather than re-reading live state.
    pub fn build_temporal_async_certification_bundle(
        &self,
        request: BridgeTemporalAsyncCertificationBundleRequest,
    ) -> Result<
        BridgeTemporalAsyncCertificationBundleDraft,
        BridgeTemporalAsyncCertificationBundleRejection,
    > {
        let _ = self;
        BridgeTemporalAsyncCertificationBundleDraft::build(request)
    }

    /// Seals a temporal/async certification bundle draft so later parity and
    /// export phases consume one canonical composed artifact.
    pub fn seal_temporal_async_certification_bundle(
        &self,
        draft: BridgeTemporalAsyncCertificationBundleDraft,
    ) -> BridgeTemporalAsyncCertificationBundleSealed {
        let _ = self;
        draft.seal()
    }

    /// Compares two sealed temporal/async certification bundles without
    /// consulting live diagnostics state.
    pub fn compare_temporal_async_certification_bundles(
        &self,
        left: &BridgeTemporalAsyncCertificationBundleSealed,
        right: &BridgeTemporalAsyncCertificationBundleSealed,
    ) -> BridgeTemporalAsyncCertificationBundleComparison {
        let _ = self;
        BridgeTemporalAsyncCertificationBundleComparison::compare(left, right)
    }

    /// Exports the canonical naming surface for a sealed temporal/async
    /// certification bundle.
    pub fn export_temporal_async_certification_bundle(
        &self,
        bundle: &BridgeTemporalAsyncCertificationBundleSealed,
    ) -> BridgeTemporalAsyncCertificationBundleExport {
        let _ = self;
        BridgeTemporalAsyncCertificationBundleExport::export(bundle)
    }

    /// Projects a narrow inspection view over a sealed temporal/async
    /// certification bundle.
    pub fn inspect_temporal_async_certification_bundle(
        &self,
        bundle: &BridgeTemporalAsyncCertificationBundleSealed,
    ) -> BridgeTemporalAsyncCertificationBundleInspection {
        let _ = self;
        BridgeTemporalAsyncCertificationBundleInspection::inspect(bundle)
    }
}
