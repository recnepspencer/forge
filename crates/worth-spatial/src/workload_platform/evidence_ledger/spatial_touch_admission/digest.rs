use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{
    SpatialGeometryEvidenceTouchCounterHonesty, SpatialGeometryEvidenceTouchOperatingWorld,
};
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceStageKind, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
    WorkloadEvidenceStageLookupCounters, WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialGeometryEvidenceTouchDigest(String);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialGeometryEvidenceParticipantDigest(String);

pub(super) struct SpatialGeometryEvidenceTouchDigestParts<'a> {
    pub boolean_stage: BooleanEvidenceStageKind,
    pub evidence_stage: WorkloadEvidenceStage,
    pub evidence_identity: &'a str,
    pub support: WorkloadEvidenceSupport,
    pub evidence_counters: WorkloadEvidenceStageCounters,
    pub lookup_counters: WorkloadEvidenceStageLookupCounters,
    pub stage_index_identity: &'a str,
    pub stage_link_set_identity: &'a str,
    pub counter_honesty: SpatialGeometryEvidenceTouchCounterHonesty,
    pub operating_world: SpatialGeometryEvidenceTouchOperatingWorld,
}

pub(super) fn spatial_geometry_evidence_touch_digest(
    parts: SpatialGeometryEvidenceTouchDigestParts<'_>,
) -> SpatialGeometryEvidenceTouchDigest {
    SpatialGeometryEvidenceTouchDigest(truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "spatial-geometry-evidence-touch-authority".to_string(),
            format!("boolean-stage:{:?}", parts.boolean_stage),
            format!("evidence-stage:{}", parts.evidence_stage.human_name()),
            format!("evidence-identity:{}", parts.evidence_identity),
            support_digest_key(parts.support),
            evidence_counter_digest_key(parts.evidence_counters),
            lookup_counter_digest_key(parts.lookup_counters),
            // The touch digest must follow the linked locality basis, not unrelated
            // complete-ledger residue that only perturbs the broader stage index.
            format!("stage-link-set-identity:{}", parts.stage_link_set_identity),
            parts.counter_honesty.digest_key(),
            parts.operating_world.digest_key(),
        ],
    ))
}

fn support_digest_key(support: WorkloadEvidenceSupport) -> String {
    let support_key = match support {
        WorkloadEvidenceSupport::Admitted => "admitted",
        WorkloadEvidenceSupport::Unsupported => "unsupported",
        WorkloadEvidenceSupport::Blocked => "blocked",
        WorkloadEvidenceSupport::Manual => "manual",
    };
    format!("support:{support_key}")
}

fn evidence_counter_digest_key(counters: WorkloadEvidenceStageCounters) -> String {
    format!(
        "evidence-counter-total:{}|typed-counters:{:?}",
        counters.total_receipt_backed_counters(),
        counters
    )
}

fn lookup_counter_digest_key(counters: WorkloadEvidenceStageLookupCounters) -> String {
    format!(
        "lookup-counters|required:{}|indexed:{}|raw-scans:{}|rejected-raw-scans:{}|rejected-string-links:{}",
        counters.required_stage_count(),
        counters.indexed_lookup_count(),
        counters.raw_row_scan_count(),
        counters.rejected_raw_row_scan_count(),
        counters.rejected_string_prefix_stage_link_count()
    )
}

impl SpatialGeometryEvidenceTouchDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SpatialGeometryEvidenceTouchDigest {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl SpatialGeometryEvidenceParticipantDigest {
    pub(super) fn new(identity: String) -> Self {
        Self(identity)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SpatialGeometryEvidenceParticipantDigest {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
