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

pub(crate) const INGRESS_SESSIONS_STARTED: &str = "compat_http.upload.ingress_sessions_started";
pub(crate) const INGRESS_PARTS_PROCESSED: &str = "compat_http.upload.ingress_parts_processed";
pub(crate) const INGRESS_WIRE_BYTES: &str = "compat_http.upload.ingress_wire_bytes";
pub(crate) const INGRESS_AUTHORITATIVE_BYTES: &str =
    "compat_http.upload.ingress_authoritative_bytes";
pub(crate) const INGRESS_UNKNOWN_LENGTH_PARTS: &str =
    "compat_http.upload.ingress_unknown_length_parts";
pub(crate) const INGRESS_COMPRESSED_PARTS: &str = "compat_http.upload.ingress_compressed_parts";
pub(crate) const INGRESS_CHUNKS_OBSERVED: &str = "compat_http.upload.ingress_chunks_observed";
pub(crate) const CLEANUP_OPERATIONS: &str = "compat_http.upload.cleanup_operations";
pub(crate) const CLEANUP_STAGED_BYTES: &str = "compat_http.upload.cleanup_staged_bytes";

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ForgeServerIngressMetricSnapshot {
    pub(crate) sessions_started: u64,
    pub(crate) parts_processed: u64,
    pub(crate) wire_bytes: u64,
    pub(crate) authoritative_bytes: u64,
    pub(crate) unknown_length_parts: u64,
    pub(crate) compressed_parts: u64,
    pub(crate) chunks_observed: u64,
    pub(crate) cleanup_operations: u64,
    pub(crate) cleanup_staged_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerIngressPerformanceReceipt {
    inner: FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl ForgeServerIngressPerformanceReceipt {
    pub(crate) fn build(
        metrics: ForgeServerIngressMetricSnapshot,
        contract_name: &'static str,
    ) -> Result<Self, ForgeServerIngressPerformanceReceiptError> {
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
            .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
            .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
            .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
            .exclude_work(FoundationalPerformanceWorkClass::ForensicParity)
            .finish()
            .map_err(ForgeServerIngressPerformanceReceiptError::PerformanceClaim)?;
        let bundle = performance_bundle(claim)
            .attach_contract_name(
                FoundationalPerformanceContractName::new(contract_name)
                    .expect("static upload ingress contract name should be valid"),
            )
            .attach_counter_spec(counter_spec(
                INGRESS_SESSIONS_STARTED,
                metrics.sessions_started,
            ))
            .attach_counter_spec(counter_spec(
                INGRESS_PARTS_PROCESSED,
                metrics.parts_processed,
            ))
            .attach_counter_spec(counter_spec(INGRESS_WIRE_BYTES, metrics.wire_bytes))
            .attach_counter_spec(counter_spec(
                INGRESS_AUTHORITATIVE_BYTES,
                metrics.authoritative_bytes,
            ))
            .attach_counter_spec(counter_spec(
                INGRESS_UNKNOWN_LENGTH_PARTS,
                metrics.unknown_length_parts,
            ))
            .attach_counter_spec(counter_spec(
                INGRESS_COMPRESSED_PARTS,
                metrics.compressed_parts,
            ))
            .attach_counter_spec(counter_spec(
                INGRESS_CHUNKS_OBSERVED,
                metrics.chunks_observed,
            ))
            .attach_counter_spec(counter_spec(CLEANUP_OPERATIONS, metrics.cleanup_operations))
            .attach_counter_spec(counter_spec(
                CLEANUP_STAGED_BYTES,
                metrics.cleanup_staged_bytes,
            ))
            .finish()
            .map_err(ForgeServerIngressPerformanceReceiptError::PerformanceBundle)?;
        let receipt = counter_backed_performance_receipt(bundle)
            .attach_counter_row(counter_row(
                INGRESS_SESSIONS_STARTED,
                metrics.sessions_started,
            ))
            .attach_counter_row(counter_row(
                INGRESS_PARTS_PROCESSED,
                metrics.parts_processed,
            ))
            .attach_counter_row(counter_row(INGRESS_WIRE_BYTES, metrics.wire_bytes))
            .attach_counter_row(counter_row(
                INGRESS_AUTHORITATIVE_BYTES,
                metrics.authoritative_bytes,
            ))
            .attach_counter_row(counter_row(
                INGRESS_UNKNOWN_LENGTH_PARTS,
                metrics.unknown_length_parts,
            ))
            .attach_counter_row(counter_row(
                INGRESS_COMPRESSED_PARTS,
                metrics.compressed_parts,
            ))
            .attach_counter_row(counter_row(
                INGRESS_CHUNKS_OBSERVED,
                metrics.chunks_observed,
            ))
            .attach_counter_row(counter_row(CLEANUP_OPERATIONS, metrics.cleanup_operations))
            .attach_counter_row(counter_row(
                CLEANUP_STAGED_BYTES,
                metrics.cleanup_staged_bytes,
            ))
            .finish()
            .map_err(ForgeServerIngressPerformanceReceiptError::CounterReceipt)?;
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
            .expect("static upload ingress counter name should be valid"),
        FoundationalPerformanceWorkClass::ValidationPlanning,
        expected_exact_count,
    )
}

fn counter_row(name: &'static str, observed_count: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(
        FoundationalPerformanceCounterName::new(name)
            .expect("static upload ingress counter name should be valid"),
        observed_count,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ForgeServerIngressPerformanceReceiptError {
    PerformanceClaim(FoundationalPerformanceClaimConstructionDenial),
    PerformanceBundle(FoundationalPerformanceBundleConstructionDenial),
    CounterReceipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}
