use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DClassification;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedIntervalRow {
    island_identity: String,
    first_segment_identity: String,
    second_segment_identity: String,
    contact_class: CertifiedSegmentSegment2DClassification,
    segment_fact_digest: String,
}

impl SharedIntervalRow {
    pub(crate) fn new(
        island_identity: String,
        first_segment_identity: String,
        second_segment_identity: String,
        contact_class: CertifiedSegmentSegment2DClassification,
        segment_fact_digest: String,
    ) -> Self {
        Self {
            island_identity,
            first_segment_identity,
            second_segment_identity,
            contact_class,
            segment_fact_digest,
        }
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn first_segment_identity(&self) -> &str {
        &self.first_segment_identity
    }

    pub fn second_segment_identity(&self) -> &str {
        &self.second_segment_identity
    }

    pub fn contact_class(&self) -> CertifiedSegmentSegment2DClassification {
        self.contact_class
    }

    pub fn segment_fact_digest(&self) -> &str {
        &self.segment_fact_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlapIslandRow {
    island_identity: String,
    shared_interval_count: usize,
}

impl OverlapIslandRow {
    pub(crate) fn new(island_identity: String, shared_interval_count: usize) -> Self {
        Self {
            island_identity,
            shared_interval_count,
        }
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn shared_interval_count(&self) -> usize {
        self.shared_interval_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainmentRelationRow {
    face_identity: String,
    loop_identity: String,
    containment: String,
    winding_fact_digest: String,
}

impl ContainmentRelationRow {
    pub(crate) fn new(
        face_identity: String,
        loop_identity: String,
        containment: String,
        winding_fact_digest: String,
    ) -> Self {
        Self {
            face_identity,
            loop_identity,
            containment,
            winding_fact_digest,
        }
    }

    pub fn face_identity(&self) -> &str {
        &self.face_identity
    }

    pub fn loop_identity(&self) -> &str {
        &self.loop_identity
    }

    pub fn containment(&self) -> &str {
        &self.containment
    }

    pub fn winding_fact_digest(&self) -> &str {
        &self.winding_fact_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyRequiredExitRow {
    region_identity: String,
    reason: String,
    consumed_fact_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmbiguousContactRow {
    region_identity: String,
    first_segment_identity: String,
    second_segment_identity: String,
    contact_class: CertifiedSegmentSegment2DClassification,
    segment_fact_digest: String,
}

impl AmbiguousContactRow {
    pub(crate) fn new(
        region_identity: String,
        first_segment_identity: String,
        second_segment_identity: String,
        contact_class: CertifiedSegmentSegment2DClassification,
        segment_fact_digest: String,
    ) -> Self {
        Self {
            region_identity,
            first_segment_identity,
            second_segment_identity,
            contact_class,
            segment_fact_digest,
        }
    }

    pub fn region_identity(&self) -> &str {
        &self.region_identity
    }

    pub fn first_segment_identity(&self) -> &str {
        &self.first_segment_identity
    }

    pub fn second_segment_identity(&self) -> &str {
        &self.second_segment_identity
    }

    pub fn contact_class(&self) -> CertifiedSegmentSegment2DClassification {
        self.contact_class
    }

    pub fn segment_fact_digest(&self) -> &str {
        &self.segment_fact_digest
    }
}

impl PolicyRequiredExitRow {
    pub(crate) fn new(
        region_identity: String,
        reason: String,
        consumed_fact_digest: String,
    ) -> Self {
        Self {
            region_identity,
            reason,
            consumed_fact_digest,
        }
    }

    pub fn region_identity(&self) -> &str {
        &self.region_identity
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn consumed_fact_digest(&self) -> &str {
        &self.consumed_fact_digest
    }
}
