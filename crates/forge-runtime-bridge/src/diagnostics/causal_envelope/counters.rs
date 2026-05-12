use std::sync::Arc;

use super::digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEnvelopeCounters {
    evidence_reference_count: usize,
    lower_runtime_family_count: usize,
    bridge_retained_lookup_count: usize,
    retained_bridge_binding_count: usize,
    external_authority_reference_count: usize,
    materialized_detail_count: usize,
    missing_bridge_record_count: usize,
    bridge_record_scan_fallback_count: usize,
    counter_digest: Arc<str>,
}

impl BridgeCausalEnvelopeCounters {
    pub(crate) fn empty() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0)
    }

    pub(crate) fn new(
        evidence_reference_count: usize,
        lower_runtime_family_count: usize,
        bridge_retained_lookup_count: usize,
        retained_bridge_binding_count: usize,
        external_authority_reference_count: usize,
        materialized_detail_count: usize,
        missing_bridge_record_count: usize,
    ) -> Self {
        let counter_digest = digest(
            "bridge-causal-envelope-counters",
            &[
                &evidence_reference_count.to_string(),
                &lower_runtime_family_count.to_string(),
                &bridge_retained_lookup_count.to_string(),
                &retained_bridge_binding_count.to_string(),
                &external_authority_reference_count.to_string(),
                &materialized_detail_count.to_string(),
                &missing_bridge_record_count.to_string(),
                "0",
            ],
        );
        Self {
            evidence_reference_count,
            lower_runtime_family_count,
            bridge_retained_lookup_count,
            retained_bridge_binding_count,
            external_authority_reference_count,
            materialized_detail_count,
            missing_bridge_record_count,
            bridge_record_scan_fallback_count: 0,
            counter_digest: Arc::from(counter_digest),
        }
    }

    pub fn evidence_reference_count(&self) -> usize {
        self.evidence_reference_count
    }

    pub fn lower_runtime_family_count(&self) -> usize {
        self.lower_runtime_family_count
    }

    pub fn bridge_retained_lookup_count(&self) -> usize {
        self.bridge_retained_lookup_count
    }

    pub fn retained_bridge_binding_count(&self) -> usize {
        self.retained_bridge_binding_count
    }

    pub fn external_authority_reference_count(&self) -> usize {
        self.external_authority_reference_count
    }

    pub fn materialized_detail_count(&self) -> usize {
        self.materialized_detail_count
    }

    pub fn missing_bridge_record_count(&self) -> usize {
        self.missing_bridge_record_count
    }

    pub fn bridge_record_scan_fallback_count(&self) -> usize {
        self.bridge_record_scan_fallback_count
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }
}
