use super::BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection;
use crate::subscription::certification::{
    BridgeSubscriptionCertificationBundleInsufficiencyReport,
    BridgeSubscriptionCertificationCostPostureReport,
    BridgeSubscriptionCertificationDeniedContinuationReport,
    BridgeSubscriptionCertificationFanoutReport,
    BridgeSubscriptionCertificationHistoricalBasisReport,
    BridgeSubscriptionCertificationMultiFailurePrecedenceReport,
    BridgeSubscriptionCertificationOrderingHostilityReport,
    BridgeSubscriptionCertificationSchemaParityReport,
    BridgeSubscriptionCertificationStaleCheckpointReport,
    BridgeSubscriptionCertificationStrategyLoweringReport,
    BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet,
    BridgeSubscriptionReferenceWorkloadSufficiency,
    BridgeTemporalAsyncCertificationBundleComparison,
};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionTemporalAsyncCertificationCloseoutRequest {
    cost_posture: BridgeSubscriptionCertificationCostPostureReport,
    schema_parity: BridgeSubscriptionCertificationSchemaParityReport,
    multi_failure: BridgeSubscriptionCertificationMultiFailurePrecedenceReport,
    ordering_hostility: BridgeSubscriptionCertificationOrderingHostilityReport,
    stale_checkpoint: BridgeSubscriptionCertificationStaleCheckpointReport,
    bundle_insufficiency: BridgeSubscriptionCertificationBundleInsufficiencyReport,
    historical_basis: BridgeSubscriptionCertificationHistoricalBasisReport,
    strategy_lowering: BridgeSubscriptionCertificationStrategyLoweringReport,
    fanout: BridgeSubscriptionCertificationFanoutReport,
    denied_continuation: BridgeSubscriptionCertificationDeniedContinuationReport,
    temporal_async_equivalent: BridgeTemporalAsyncCertificationBundleComparison,
    temporal_async_diagnostics_delta: BridgeTemporalAsyncCertificationBundleComparison,
    temporal_async_divergent: BridgeTemporalAsyncCertificationBundleComparison,
    workload_sufficiency: BridgeSubscriptionReferenceWorkloadSufficiency,
}

impl BridgeSubscriptionTemporalAsyncCertificationCloseoutRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cost_posture: BridgeSubscriptionCertificationCostPostureReport,
        schema_parity: BridgeSubscriptionCertificationSchemaParityReport,
        multi_failure: BridgeSubscriptionCertificationMultiFailurePrecedenceReport,
        ordering_hostility: BridgeSubscriptionCertificationOrderingHostilityReport,
        stale_checkpoint: BridgeSubscriptionCertificationStaleCheckpointReport,
        bundle_insufficiency: BridgeSubscriptionCertificationBundleInsufficiencyReport,
        historical_basis: BridgeSubscriptionCertificationHistoricalBasisReport,
        strategy_lowering: BridgeSubscriptionCertificationStrategyLoweringReport,
        fanout: BridgeSubscriptionCertificationFanoutReport,
        denied_continuation: BridgeSubscriptionCertificationDeniedContinuationReport,
        temporal_async_equivalent: BridgeTemporalAsyncCertificationBundleComparison,
        temporal_async_diagnostics_delta: BridgeTemporalAsyncCertificationBundleComparison,
        temporal_async_divergent: BridgeTemporalAsyncCertificationBundleComparison,
        workload_sufficiency: BridgeSubscriptionReferenceWorkloadSufficiency,
    ) -> Self {
        Self {
            cost_posture,
            schema_parity,
            multi_failure,
            ordering_hostility,
            stale_checkpoint,
            bundle_insufficiency,
            historical_basis,
            strategy_lowering,
            fanout,
            denied_continuation,
            temporal_async_equivalent,
            temporal_async_diagnostics_delta,
            temporal_async_divergent,
            workload_sufficiency,
        }
    }

    pub(crate) fn validate(
        &self,
    ) -> Result<(), BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection> {
        use crate::facade::BridgeTemporalAsyncCertificationBundleComparisonOutcome;
        if self
            .workload_sufficiency
            .report()
            .coverage_report()
            .covered_required_facets()
            != BridgeSubscriptionReferenceWorkloadRequiredCoverageFacet::all()
        {
            return Err(BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection::new(
                super::BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind::ReferenceWorkloadNotSufficient,
                "phase 17 workload sufficiency must retain all required coverage facets",
            ));
        }
        if !self.historical_basis.retained_basis_is_explicit() {
            return Err(BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection::new(
                super::BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind::UnsupportedBasisNotTyped,
                "unsupported basis lane must stay explicit in historical basis proof",
            ));
        }
        if !self.fanout.divergent_sharing_rejected_before_delivery() {
            return Err(BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection::new(
                super::BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind::UnsupportedNeighborNotTyped,
                "unsupported neighbor lane must reject before delivery drift",
            ));
        }
        if self.temporal_async_equivalent.outcome()
            != BridgeTemporalAsyncCertificationBundleComparisonOutcome::Equivalent
            || self.temporal_async_diagnostics_delta.outcome()
                != BridgeTemporalAsyncCertificationBundleComparisonOutcome::DiagnosticsRichnessOnlyDelta
            || self.temporal_async_divergent.outcome()
                != BridgeTemporalAsyncCertificationBundleComparisonOutcome::Divergent
        {
            return Err(BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection::new(
                super::BridgeSubscriptionTemporalAsyncCertificationCloseoutRejectionKind::TemporalAsyncParityBandIncomplete,
                "phase 16 parity band must retain equivalent, diagnostics delta, and divergent outcomes",
            ));
        }
        Ok(())
    }

    pub fn cost_posture(&self) -> &BridgeSubscriptionCertificationCostPostureReport {
        &self.cost_posture
    }
    pub fn schema_parity(&self) -> &BridgeSubscriptionCertificationSchemaParityReport {
        &self.schema_parity
    }
    pub fn multi_failure(&self) -> &BridgeSubscriptionCertificationMultiFailurePrecedenceReport {
        &self.multi_failure
    }
    pub fn ordering_hostility(&self) -> &BridgeSubscriptionCertificationOrderingHostilityReport {
        &self.ordering_hostility
    }
    pub fn stale_checkpoint(&self) -> &BridgeSubscriptionCertificationStaleCheckpointReport {
        &self.stale_checkpoint
    }
    pub fn bundle_insufficiency(
        &self,
    ) -> &BridgeSubscriptionCertificationBundleInsufficiencyReport {
        &self.bundle_insufficiency
    }
    pub fn historical_basis(&self) -> &BridgeSubscriptionCertificationHistoricalBasisReport {
        &self.historical_basis
    }
    pub fn strategy_lowering(&self) -> &BridgeSubscriptionCertificationStrategyLoweringReport {
        &self.strategy_lowering
    }
    pub fn fanout(&self) -> &BridgeSubscriptionCertificationFanoutReport {
        &self.fanout
    }
    pub fn denied_continuation(&self) -> &BridgeSubscriptionCertificationDeniedContinuationReport {
        &self.denied_continuation
    }
    pub fn temporal_async_equivalent(&self) -> &BridgeTemporalAsyncCertificationBundleComparison {
        &self.temporal_async_equivalent
    }
    pub fn temporal_async_diagnostics_delta(
        &self,
    ) -> &BridgeTemporalAsyncCertificationBundleComparison {
        &self.temporal_async_diagnostics_delta
    }
    pub fn temporal_async_divergent(&self) -> &BridgeTemporalAsyncCertificationBundleComparison {
        &self.temporal_async_divergent
    }
    pub fn workload_sufficiency(&self) -> &BridgeSubscriptionReferenceWorkloadSufficiency {
        &self.workload_sufficiency
    }

    pub fn temporal_async_parity_band_digest(&self) -> String {
        let canonical_basis = format!(
            "bridge-subscription-temporal-async-certification-closeout-parity-band|equivalent={}|diagnostics-delta={}|divergent={}",
            self.temporal_async_equivalent.digest(),
            self.temporal_async_diagnostics_delta.digest(),
            self.temporal_async_divergent.digest(),
        );
        let digest = Sha256::digest(canonical_basis.as_bytes());
        format!(
            "bridge-subscription-temporal-async-certification-closeout-parity-band:sha256:{digest:x}"
        )
    }
}
