use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    request::BridgeSubscriptionTemporalAsyncCertificationCloseoutRequest,
    suite_id::*,
    support_matrix::{
        BridgeSubscriptionTemporalAsyncCertificationSupportMatrix,
        BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow,
        BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict,
    },
    BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection,
};
use crate::subscription::certification::BridgeSubscriptionCertificationCounterSnapshot;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionTemporalAsyncCertificationCloseoutArtifact {
    support_matrix: BridgeSubscriptionTemporalAsyncCertificationSupportMatrix,
    workload_sufficiency_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
}

impl BridgeSubscriptionTemporalAsyncCertificationCloseoutArtifact {
    pub(crate) fn seal(
        request: BridgeSubscriptionTemporalAsyncCertificationCloseoutRequest,
    ) -> Result<Self, BridgeSubscriptionTemporalAsyncCertificationCloseoutRejection> {
        request.validate()?;
        let temporal_async_parity_band_digest = request.temporal_async_parity_band_digest();
        let mut rows = vec![
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite38CostPosture,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.cost_posture().digest(),
                None,
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite39SchemaParity,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.schema_parity().digest(),
                Some(request.schema_parity().primary_failure_boundary()),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite40MultiFailurePrecedence,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.multi_failure().digest(),
                Some(request.multi_failure().primary_failure_boundary()),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite41OrderingHostility,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::EquivalentParityProven,
                request.ordering_hostility().digest(),
                None,
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite42StaleCheckpoint,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.stale_checkpoint().digest(),
                Some(request.stale_checkpoint().primary_failure_boundary()),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite43BundleInsufficiency,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.bundle_insufficiency().digest(),
                Some(request.bundle_insufficiency().primary_failure_boundary()),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite44UnsupportedBasis,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.historical_basis().digest(),
                Some(request.historical_basis().primary_failure_boundary()),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite45StrategyLoweringProvenance,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.strategy_lowering().digest(),
                Some(request.strategy_lowering().primary_failure_boundary()),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite46UnsupportedNeighbor,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.fanout().digest(),
                Some(crate::subscription::certification::BridgeSubscriptionCertificationFailureBoundary::IllegalSharingReuse),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite47DeniedContinuation,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::TypedRejectionProven,
                request.denied_continuation().digest(),
                Some(request.denied_continuation().primary_failure_boundary()),
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite48TemporalAsyncBundleParity,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::ParityBandProven,
                temporal_async_parity_band_digest.as_str(),
                None,
            ),
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite49ReferenceWorkloadSufficiency,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::SufficiencyProven,
                request.workload_sufficiency().digest(),
                None,
            ),
        ];
        let provisional_support_matrix =
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrix::from_rows(rows.clone());
        let provisional_counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *request.cost_posture().counters(),
            *request.schema_parity().counters(),
            *request.multi_failure().counters(),
            *request.ordering_hostility().counters(),
            *request.stale_checkpoint().counters(),
            *request.bundle_insufficiency().counters(),
            *request.historical_basis().counters(),
            *request.strategy_lowering().counters(),
            *request.fanout().counters(),
            *request.denied_continuation().counters(),
            *request.workload_sufficiency().report().counters(),
            *provisional_support_matrix.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_phase_18_closeout_artifact(),
        ]);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-temporal-async-certification-closeout-artifact|support-matrix={}|parity-equivalent={}|parity-diagnostics={}|parity-divergent={}|workload={}|counters={}",
            provisional_support_matrix.digest(),
            request.temporal_async_equivalent().digest(),
            request.temporal_async_diagnostics_delta().digest(),
            request.temporal_async_divergent().digest(),
            request.workload_sufficiency().digest(),
            provisional_counters.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        let closeout_digest: Arc<str> = Arc::from(format!(
            "bridge-subscription-temporal-async-certification-closeout-artifact:sha256:{digest:x}"
        ));
        rows.push(
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrixRow::new(
                BridgeSubscriptionTemporalAsyncCertificationCloseoutSuiteId::Suite50MergedCloseout,
                BridgeSubscriptionTemporalAsyncCertificationSupportMatrixVerdict::CloseoutProven,
                closeout_digest.as_ref(),
                None,
            ),
        );
        let support_matrix =
            BridgeSubscriptionTemporalAsyncCertificationSupportMatrix::from_rows(rows);
        let counters = BridgeSubscriptionCertificationCounterSnapshot::combine([
            *request.cost_posture().counters(),
            *request.schema_parity().counters(),
            *request.multi_failure().counters(),
            *request.ordering_hostility().counters(),
            *request.stale_checkpoint().counters(),
            *request.bundle_insufficiency().counters(),
            *request.historical_basis().counters(),
            *request.strategy_lowering().counters(),
            *request.fanout().counters(),
            *request.denied_continuation().counters(),
            *request.workload_sufficiency().report().counters(),
            *support_matrix.counters(),
            BridgeSubscriptionCertificationCounterSnapshot::from_phase_18_closeout_artifact(),
        ]);
        Ok(Self {
            support_matrix,
            workload_sufficiency_digest: Arc::from(request.workload_sufficiency().digest()),
            canonical_basis,
            digest: closeout_digest,
            counters,
        })
    }

    pub fn support_matrix(&self) -> &BridgeSubscriptionTemporalAsyncCertificationSupportMatrix {
        &self.support_matrix
    }

    pub fn workload_sufficiency_digest(&self) -> &str {
        self.workload_sufficiency_digest.as_ref()
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
