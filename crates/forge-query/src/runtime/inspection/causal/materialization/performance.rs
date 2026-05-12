use crate::identity::hash_parts;

use forge_runtime_bridge::facade::{BridgeCausalEnvelopeDenial, BridgeCausalExplanationEnvelope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CausalInspectionPerformanceEnvelope {
    anchor_derivation_count: usize,
    evidence_reference_resolution_count: usize,
    admission_count: usize,
    bridge_envelope_assembly_count: usize,
    redaction_count: usize,
    materialization_count: usize,
    artifact_serialization_count: usize,
    bridge_binding_count: usize,
    bridge_lookup_count: usize,
    bridge_scan_fallback_count: usize,
    materialized_detail_count: usize,
    performance_digest: String,
}

impl CausalInspectionPerformanceEnvelope {
    pub(super) fn for_bridge_envelope(
        envelope: &BridgeCausalExplanationEnvelope,
        redaction_count: usize,
    ) -> Self {
        let counters = envelope.counters();
        Self::new(
            1,
            1,
            1,
            1,
            redaction_count,
            1,
            1,
            counters.evidence_reference_count(),
            counters.bridge_retained_lookup_count(),
            counters.bridge_record_scan_fallback_count(),
            counters.materialized_detail_count(),
        )
    }

    pub(super) fn for_denied_query() -> Self {
        Self::new(1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0)
    }

    pub(super) fn for_bridge_denial(denial: &BridgeCausalEnvelopeDenial) -> Self {
        let counters = denial.counters();
        Self::new(
            1,
            1,
            1,
            1,
            0,
            1,
            1,
            counters.evidence_reference_count(),
            counters.bridge_retained_lookup_count(),
            counters.bridge_record_scan_fallback_count(),
            counters.materialized_detail_count(),
        )
    }

    fn new(
        anchor_derivation_count: usize,
        evidence_reference_resolution_count: usize,
        admission_count: usize,
        bridge_envelope_assembly_count: usize,
        redaction_count: usize,
        materialization_count: usize,
        artifact_serialization_count: usize,
        bridge_binding_count: usize,
        bridge_lookup_count: usize,
        bridge_scan_fallback_count: usize,
        materialized_detail_count: usize,
    ) -> Self {
        let performance_digest = hash_parts(&[
            "causal_inspection_performance_envelope_v1".to_string(),
            format!("anchor:{anchor_derivation_count}"),
            format!("references:{evidence_reference_resolution_count}"),
            format!("admission:{admission_count}"),
            format!("bridge-envelope:{bridge_envelope_assembly_count}"),
            format!("redaction:{redaction_count}"),
            format!("materialization:{materialization_count}"),
            format!("artifact-serialization:{artifact_serialization_count}"),
            format!("bridge-bindings:{bridge_binding_count}"),
            format!("bridge-lookups:{bridge_lookup_count}"),
            format!("bridge-scan-fallback:{bridge_scan_fallback_count}"),
            format!("materialized-detail:{materialized_detail_count}"),
        ]);
        Self {
            anchor_derivation_count,
            evidence_reference_resolution_count,
            admission_count,
            bridge_envelope_assembly_count,
            redaction_count,
            materialization_count,
            artifact_serialization_count,
            bridge_binding_count,
            bridge_lookup_count,
            bridge_scan_fallback_count,
            materialized_detail_count,
            performance_digest,
        }
    }

    pub fn anchor_derivation_count(&self) -> usize {
        self.anchor_derivation_count
    }

    pub fn evidence_reference_resolution_count(&self) -> usize {
        self.evidence_reference_resolution_count
    }

    pub fn admission_count(&self) -> usize {
        self.admission_count
    }

    pub fn bridge_envelope_assembly_count(&self) -> usize {
        self.bridge_envelope_assembly_count
    }

    pub fn redaction_count(&self) -> usize {
        self.redaction_count
    }

    pub fn materialization_count(&self) -> usize {
        self.materialization_count
    }

    pub fn artifact_serialization_count(&self) -> usize {
        self.artifact_serialization_count
    }

    pub fn bridge_binding_count(&self) -> usize {
        self.bridge_binding_count
    }

    pub fn bridge_lookup_count(&self) -> usize {
        self.bridge_lookup_count
    }

    pub fn bridge_scan_fallback_count(&self) -> usize {
        self.bridge_scan_fallback_count
    }

    pub fn materialized_detail_count(&self) -> usize {
        self.materialized_detail_count
    }

    pub fn performance_digest(&self) -> &str {
        &self.performance_digest
    }
}
