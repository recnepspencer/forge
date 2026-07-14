use super::*;
use crate::diagnostics::{
    BridgeFailureLocalizationRequest, BridgeLocalizedTemporalAsyncFailure,
    BridgeTemporalAsyncFailureBundleComparison, BridgeTemporalAsyncFailureLocalizationMatrix,
    BridgeTemporalAsyncFailureLocalizationRejection,
    BridgeTemporalAsyncOfflineDiagnosisBundleDraft,
    BridgeTemporalAsyncOfflineDiagnosisBundleRejection,
    BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
};

impl RuntimeBridge {
    /// Localizes one typed temporal/async bridge failure into the canonical
    /// bridge-native failure taxonomy.
    pub fn localize_temporal_async_failure(
        &self,
        request: BridgeFailureLocalizationRequest,
    ) -> Result<BridgeLocalizedTemporalAsyncFailure, BridgeTemporalAsyncFailureLocalizationRejection>
    {
        let _ = self;
        BridgeLocalizedTemporalAsyncFailure::localize(request)
    }

    /// Seals one replay-safe offline diagnosis bundle from localized failure
    /// artifacts alone.
    pub fn seal_temporal_async_offline_diagnosis_bundle(
        &self,
        localized_failures: Vec<BridgeLocalizedTemporalAsyncFailure>,
    ) -> Result<
        BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
        BridgeTemporalAsyncOfflineDiagnosisBundleRejection,
    > {
        let _ = self;
        let draft = BridgeTemporalAsyncOfflineDiagnosisBundleDraft::new(localized_failures)?;
        Ok(BridgeTemporalAsyncOfflineDiagnosisBundleSealed::seal(draft))
    }

    /// Compares two sealed offline diagnosis bundles without consulting live
    /// runtime diagnostics state.
    pub fn compare_temporal_async_failure_bundles(
        &self,
        left: &BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
        right: &BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
    ) -> BridgeTemporalAsyncFailureBundleComparison {
        let _ = self;
        BridgeTemporalAsyncFailureBundleComparison::compare(left, right)
    }

    /// Projects a certification-facing localization matrix from a sealed
    /// offline diagnosis bundle.
    pub fn inspect_temporal_async_failure_matrix(
        &self,
        bundle: &BridgeTemporalAsyncOfflineDiagnosisBundleSealed,
    ) -> BridgeTemporalAsyncFailureLocalizationMatrix {
        let _ = self;
        BridgeTemporalAsyncFailureLocalizationMatrix::from_bundle(bundle)
    }
}
