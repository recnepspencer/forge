use forge_foundational::facade::{
    counter_backed_performance_receipt, performance, performance_bundle,
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceClaimConstructionDenial, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

pub(crate) const DOWNLOAD_REQUESTS: &str = "compat_http.download.requests";
pub(crate) const DOWNLOAD_BYTES_EMITTED: &str = "compat_http.download.bytes_emitted";
pub(crate) const DOWNLOAD_FULL_BUFFER_MATERIALIZATIONS: &str =
    "compat_http.download.full_buffer_materializations";
pub(crate) const DOWNLOAD_RANGE_REQUESTS: &str = "compat_http.download.range_requests";
pub(crate) const DOWNLOAD_FULL_REQUESTS: &str = "compat_http.download.full_requests";
pub(crate) const DOWNLOAD_HEAD_REQUESTS: &str = "compat_http.download.head_requests";
pub(crate) const DOWNLOAD_RESUME_REQUESTS: &str = "compat_http.download.resume_requests";
pub(crate) const DOWNLOAD_RESUMED_REQUESTS_ADMITTED: &str =
    "compat_http.download.resumed_requests_admitted";
pub(crate) const DOWNLOAD_INTEGRITY_VERIFICATIONS: &str =
    "compat_http.download.integrity_verifications";
pub(crate) const DOWNLOAD_FORBIDDEN_FALLBACKS: &str = "compat_http.download.forbidden_fallbacks";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ForgeServerBinaryEgressMetricSnapshot {
    pub(crate) requests: u64,
    pub(crate) bytes_emitted: u64,
    pub(crate) full_buffer_materializations: u64,
    pub(crate) range_requests: u64,
    pub(crate) full_requests: u64,
    pub(crate) head_requests: u64,
    pub(crate) resume_requests: u64,
    pub(crate) resumed_requests_admitted: u64,
    pub(crate) integrity_verifications: u64,
    pub(crate) forbidden_fallbacks: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerBinaryEgressPerformanceReceipt {
    inner: FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl ForgeServerBinaryEgressPerformanceReceipt {
    pub(crate) fn build(
        metrics: ForgeServerBinaryEgressMetricSnapshot,
    ) -> Result<Self, ForgeServerBinaryEgressPerformanceReceiptError> {
        let claim = performance()
            .claim()
            .authoritative_execution()
            .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
            .evidence_strength(
                FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            )
            .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
            .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
            .execution_temperature(FoundationalPerformanceExecutionTemperature::WarmPath)
            .freshness_retention(
                FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent,
            )
            .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
            .include_work(FoundationalPerformanceWorkClass::PublicationDelivery)
            .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
            .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
            .exclude_work(FoundationalPerformanceWorkClass::ForensicParity)
            .finish()
            .map_err(ForgeServerBinaryEgressPerformanceReceiptError::PerformanceClaim)?;
        let bundle = performance_bundle(claim)
            .attach_contract_name(
                FoundationalPerformanceContractName::new("compat_http.download.delivery")
                    .expect("static binary download contract name should be valid"),
            )
            .attach_counter_spec(counter_spec(DOWNLOAD_REQUESTS, metrics.requests))
            .attach_counter_spec(counter_spec(DOWNLOAD_BYTES_EMITTED, metrics.bytes_emitted))
            .attach_counter_spec(counter_spec(
                DOWNLOAD_FULL_BUFFER_MATERIALIZATIONS,
                metrics.full_buffer_materializations,
            ))
            .attach_counter_spec(counter_spec(
                DOWNLOAD_RANGE_REQUESTS,
                metrics.range_requests,
            ))
            .attach_counter_spec(counter_spec(DOWNLOAD_FULL_REQUESTS, metrics.full_requests))
            .attach_counter_spec(counter_spec(DOWNLOAD_HEAD_REQUESTS, metrics.head_requests))
            .attach_counter_spec(counter_spec(
                DOWNLOAD_RESUME_REQUESTS,
                metrics.resume_requests,
            ))
            .attach_counter_spec(counter_spec(
                DOWNLOAD_RESUMED_REQUESTS_ADMITTED,
                metrics.resumed_requests_admitted,
            ))
            .attach_counter_spec(counter_spec(
                DOWNLOAD_INTEGRITY_VERIFICATIONS,
                metrics.integrity_verifications,
            ))
            .attach_counter_spec(counter_spec(
                DOWNLOAD_FORBIDDEN_FALLBACKS,
                metrics.forbidden_fallbacks,
            ))
            .finish()
            .map_err(ForgeServerBinaryEgressPerformanceReceiptError::PerformanceBundle)?;
        let receipt = counter_backed_performance_receipt(bundle)
            .attach_counter_row(counter_row(DOWNLOAD_REQUESTS, metrics.requests))
            .attach_counter_row(counter_row(DOWNLOAD_BYTES_EMITTED, metrics.bytes_emitted))
            .attach_counter_row(counter_row(
                DOWNLOAD_FULL_BUFFER_MATERIALIZATIONS,
                metrics.full_buffer_materializations,
            ))
            .attach_counter_row(counter_row(DOWNLOAD_RANGE_REQUESTS, metrics.range_requests))
            .attach_counter_row(counter_row(DOWNLOAD_FULL_REQUESTS, metrics.full_requests))
            .attach_counter_row(counter_row(DOWNLOAD_HEAD_REQUESTS, metrics.head_requests))
            .attach_counter_row(counter_row(
                DOWNLOAD_RESUME_REQUESTS,
                metrics.resume_requests,
            ))
            .attach_counter_row(counter_row(
                DOWNLOAD_RESUMED_REQUESTS_ADMITTED,
                metrics.resumed_requests_admitted,
            ))
            .attach_counter_row(counter_row(
                DOWNLOAD_INTEGRITY_VERIFICATIONS,
                metrics.integrity_verifications,
            ))
            .attach_counter_row(counter_row(
                DOWNLOAD_FORBIDDEN_FALLBACKS,
                metrics.forbidden_fallbacks,
            ))
            .finish()
            .map_err(ForgeServerBinaryEgressPerformanceReceiptError::CounterReceipt)?;
        Ok(Self { inner: receipt })
    }

    pub fn receipt(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.inner
    }

    pub fn counter(&self, name: &str) -> Option<u64> {
        self.inner
            .counter_rows()
            .iter()
            .find(|row| row.name().as_str() == name)
            .map(FoundationalPerformanceCounterRow::observed_count)
    }
}

fn counter_spec(
    name: &'static str,
    expected_exact_count: u64,
) -> FoundationalPerformanceCounterSpec {
    FoundationalPerformanceCounterSpec::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("static binary download counter name should be valid"),
        FoundationalPerformanceWorkClass::PublicationDelivery,
        expected_exact_count,
    )
}

fn counter_row(name: &'static str, observed_count: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("static binary download counter name should be valid"),
        observed_count,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ForgeServerBinaryEgressPerformanceReceiptError {
    PerformanceClaim(FoundationalPerformanceClaimConstructionDenial),
    PerformanceBundle(FoundationalPerformanceBundleConstructionDenial),
    CounterReceipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}
