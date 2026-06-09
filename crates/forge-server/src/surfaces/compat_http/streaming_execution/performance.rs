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

const CHUNKS_EMITTED: &str = "compat_http.streaming.chunks_emitted";
const BYTES_EMITTED: &str = "compat_http.streaming.bytes_emitted";
const FULL_BUFFER_MATERIALIZATIONS: &str = "compat_http.streaming.full_buffer_materializations";
const FIRST_CHUNK_WITHOUT_FULL_BUFFER: &str =
    "compat_http.streaming.first_chunk_without_full_buffer";
const BACKPRESSURE_EVENTS: &str = "compat_http.streaming.backpressure_events";
const DISCONNECTS: &str = "compat_http.streaming.disconnects";
const CANCELLATIONS: &str = "compat_http.streaming.cancellations";
const BACKGROUND_EXPORT_FALLBACKS: &str = "compat_http.streaming.background_export_fallbacks";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ForgeServerStreamingMetricSnapshot {
    pub(crate) chunks_emitted: u64,
    pub(crate) bytes_emitted: u64,
    pub(crate) full_buffer_materializations: u64,
    pub(crate) first_chunk_without_full_buffer: u64,
    pub(crate) backpressure_events: u64,
    pub(crate) disconnects: u64,
    pub(crate) cancellations: u64,
    pub(crate) background_export_fallbacks: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerStreamingPerformanceReceipt {
    inner: FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl ForgeServerStreamingPerformanceReceipt {
    pub(crate) fn build(
        metrics: ForgeServerStreamingMetricSnapshot,
    ) -> Result<Self, ForgeServerStreamingPerformanceReceiptError> {
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
            .map_err(ForgeServerStreamingPerformanceReceiptError::PerformanceClaim)?;
        let bundle = performance_bundle(claim)
            .attach_contract_name(
                FoundationalPerformanceContractName::new("compat_http.streaming.delivery")
                    .expect("static streaming contract name should be valid"),
            )
            .attach_counter_spec(counter_spec(CHUNKS_EMITTED, metrics.chunks_emitted))
            .attach_counter_spec(counter_spec(BYTES_EMITTED, metrics.bytes_emitted))
            .attach_counter_spec(counter_spec(
                FULL_BUFFER_MATERIALIZATIONS,
                metrics.full_buffer_materializations,
            ))
            .attach_counter_spec(counter_spec(
                FIRST_CHUNK_WITHOUT_FULL_BUFFER,
                metrics.first_chunk_without_full_buffer,
            ))
            .attach_counter_spec(counter_spec(
                BACKPRESSURE_EVENTS,
                metrics.backpressure_events,
            ))
            .attach_counter_spec(counter_spec(DISCONNECTS, metrics.disconnects))
            .attach_counter_spec(counter_spec(CANCELLATIONS, metrics.cancellations))
            .attach_counter_spec(counter_spec(
                BACKGROUND_EXPORT_FALLBACKS,
                metrics.background_export_fallbacks,
            ))
            .finish()
            .map_err(ForgeServerStreamingPerformanceReceiptError::PerformanceBundle)?;
        let receipt = counter_backed_performance_receipt(bundle)
            .attach_counter_row(counter_row(CHUNKS_EMITTED, metrics.chunks_emitted))
            .attach_counter_row(counter_row(BYTES_EMITTED, metrics.bytes_emitted))
            .attach_counter_row(counter_row(
                FULL_BUFFER_MATERIALIZATIONS,
                metrics.full_buffer_materializations,
            ))
            .attach_counter_row(counter_row(
                FIRST_CHUNK_WITHOUT_FULL_BUFFER,
                metrics.first_chunk_without_full_buffer,
            ))
            .attach_counter_row(counter_row(
                BACKPRESSURE_EVENTS,
                metrics.backpressure_events,
            ))
            .attach_counter_row(counter_row(DISCONNECTS, metrics.disconnects))
            .attach_counter_row(counter_row(CANCELLATIONS, metrics.cancellations))
            .attach_counter_row(counter_row(
                BACKGROUND_EXPORT_FALLBACKS,
                metrics.background_export_fallbacks,
            ))
            .finish()
            .map_err(ForgeServerStreamingPerformanceReceiptError::CounterReceipt)?;
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
            .expect("static streaming counter name should be valid"),
        FoundationalPerformanceWorkClass::PublicationDelivery,
        expected_exact_count,
    )
}

fn counter_row(name: &'static str, observed_count: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("static streaming counter name should be valid"),
        observed_count,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ForgeServerStreamingPerformanceReceiptError {
    PerformanceClaim(FoundationalPerformanceClaimConstructionDenial),
    PerformanceBundle(FoundationalPerformanceBundleConstructionDenial),
    CounterReceipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}
