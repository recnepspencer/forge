use super::counter_honesty::{
    spatial_touch_counter_honesty, SpatialGeometryEvidenceTouchCounterHonesty,
};
use super::digest::{
    spatial_geometry_evidence_touch_digest, SpatialGeometryEvidenceTouchDigest,
    SpatialGeometryEvidenceTouchDigestParts,
};
use super::operating_world::SpatialGeometryEvidenceTouchOperatingWorld;
use crate::workload_platform::evidence_ledger::{
    BooleanEvidenceStageKind, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
    WorkloadEvidenceStageLookupCounters, WorkloadEvidenceSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialGeometryEvidenceTouchAuthority {
    digest: SpatialGeometryEvidenceTouchDigest,
    boolean_stage: BooleanEvidenceStageKind,
    evidence_stage: WorkloadEvidenceStage,
    evidence_identity: String,
    support: WorkloadEvidenceSupport,
    evidence_counters: WorkloadEvidenceStageCounters,
    lookup_counters: WorkloadEvidenceStageLookupCounters,
    stage_index_identity: String,
    stage_link_set_identity: String,
    counter_honesty: SpatialGeometryEvidenceTouchCounterHonesty,
    operating_world: SpatialGeometryEvidenceTouchOperatingWorld,
}

struct SpatialGeometryEvidenceTouchAuthorityParts {
    boolean_stage: BooleanEvidenceStageKind,
    evidence_stage: WorkloadEvidenceStage,
    evidence_identity: String,
    support: WorkloadEvidenceSupport,
    evidence_counters: WorkloadEvidenceStageCounters,
    lookup_counters: WorkloadEvidenceStageLookupCounters,
    stage_index_identity: String,
    stage_link_set_identity: String,
}

pub(super) fn admit_spatial_geometry_evidence_touch_authority(
    boolean_stage: BooleanEvidenceStageKind,
    evidence_stage: WorkloadEvidenceStage,
    evidence_identity: String,
    support: WorkloadEvidenceSupport,
    evidence_counters: WorkloadEvidenceStageCounters,
    lookup_counters: WorkloadEvidenceStageLookupCounters,
    stage_index_identity: String,
    stage_link_set_identity: String,
) -> SpatialGeometryEvidenceTouchAuthority {
    SpatialGeometryEvidenceTouchAuthority::from_parts(SpatialGeometryEvidenceTouchAuthorityParts {
        boolean_stage,
        evidence_stage,
        evidence_identity,
        support,
        evidence_counters,
        lookup_counters,
        stage_index_identity,
        stage_link_set_identity,
    })
}

impl SpatialGeometryEvidenceTouchAuthority {
    fn from_parts(parts: SpatialGeometryEvidenceTouchAuthorityParts) -> Self {
        let counter_honesty =
            spatial_touch_counter_honesty(parts.evidence_stage, parts.evidence_counters);
        let operating_world = SpatialGeometryEvidenceTouchOperatingWorld::current_head();
        let digest =
            spatial_geometry_evidence_touch_digest(SpatialGeometryEvidenceTouchDigestParts {
                boolean_stage: parts.boolean_stage,
                evidence_stage: parts.evidence_stage,
                evidence_identity: &parts.evidence_identity,
                support: parts.support,
                evidence_counters: parts.evidence_counters,
                lookup_counters: parts.lookup_counters,
                stage_index_identity: &parts.stage_index_identity,
                stage_link_set_identity: &parts.stage_link_set_identity,
                counter_honesty,
                operating_world,
            });
        Self {
            digest,
            boolean_stage: parts.boolean_stage,
            evidence_stage: parts.evidence_stage,
            evidence_identity: parts.evidence_identity,
            support: parts.support,
            evidence_counters: parts.evidence_counters,
            lookup_counters: parts.lookup_counters,
            stage_index_identity: parts.stage_index_identity,
            stage_link_set_identity: parts.stage_link_set_identity,
            counter_honesty,
            operating_world,
        }
    }

    pub fn digest(&self) -> &SpatialGeometryEvidenceTouchDigest {
        &self.digest
    }

    pub fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        self.boolean_stage
    }

    pub fn evidence_stage(&self) -> WorkloadEvidenceStage {
        self.evidence_stage
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn support(&self) -> WorkloadEvidenceSupport {
        self.support
    }

    pub fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        self.evidence_counters
    }

    pub fn lookup_counters(&self) -> WorkloadEvidenceStageLookupCounters {
        self.lookup_counters
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }

    pub fn stage_link_set_identity(&self) -> &str {
        &self.stage_link_set_identity
    }

    pub fn counter_honesty(&self) -> SpatialGeometryEvidenceTouchCounterHonesty {
        self.counter_honesty
    }

    pub fn operating_world(&self) -> SpatialGeometryEvidenceTouchOperatingWorld {
        self.operating_world
    }
}
