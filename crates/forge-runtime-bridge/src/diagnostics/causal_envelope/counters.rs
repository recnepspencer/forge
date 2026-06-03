use std::sync::Arc;

use super::{causal_envelope_digest, digest_basis::BridgeCausalEnvelopeDigestArtifact};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeCausalEnvelopeCounters {
    evidence_reference_count: usize,
    lower_runtime_family_count: usize,
    bridge_retained_lookup_count: usize,
    retained_bridge_binding_count: usize,
    external_authority_reference_count: usize,
    materialized_detail_count: usize,
    missing_bridge_record_count: usize,
    bridge_record_unindexed_scan_count: usize,
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
        let evidence_reference_count_text = evidence_reference_count.to_string();
        let lower_runtime_family_count_text = lower_runtime_family_count.to_string();
        let bridge_retained_lookup_count_text = bridge_retained_lookup_count.to_string();
        let retained_bridge_binding_count_text = retained_bridge_binding_count.to_string();
        let external_authority_reference_count_text =
            external_authority_reference_count.to_string();
        let materialized_detail_count_text = materialized_detail_count.to_string();
        let missing_bridge_record_count_text = missing_bridge_record_count.to_string();
        let counter_digest = causal_envelope_digest(
            BridgeCausalEnvelopeDigestArtifact::Counters,
            &[
                evidence_reference_count_text.as_str(),
                lower_runtime_family_count_text.as_str(),
                bridge_retained_lookup_count_text.as_str(),
                retained_bridge_binding_count_text.as_str(),
                external_authority_reference_count_text.as_str(),
                materialized_detail_count_text.as_str(),
                missing_bridge_record_count_text.as_str(),
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
            bridge_record_unindexed_scan_count: 0,
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

    pub fn bridge_record_unindexed_scan_count(&self) -> usize {
        self.bridge_record_unindexed_scan_count
    }

    pub fn counter_digest(&self) -> &str {
        self.counter_digest.as_ref()
    }
}
