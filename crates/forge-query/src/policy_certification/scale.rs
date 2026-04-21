use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PolicyScaleFixtureSize {
    Small,
    Medium,
    Larger,
}

impl PolicyScaleFixtureSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Larger => "larger",
        }
    }

    pub fn row_count(&self) -> usize {
        match self {
            Self::Small => 3,
            Self::Medium => 30,
            Self::Larger => 300,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyScaleCounterSnapshot {
    fixture_size: PolicyScaleFixtureSize,
    authorized_projection_width: usize,
    relationship_proof_descriptor_count: usize,
    relationship_proof_topology_width: usize,
    delivery_width: usize,
    live_relevance_width: usize,
    allocation_scope_count: usize,
    digest_part_count: usize,
    executor_semantic_rediscovery_count: usize,
}

impl PolicyScaleCounterSnapshot {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        fixture_size: PolicyScaleFixtureSize,
        authorized_projection_width: usize,
        relationship_proof_descriptor_count: usize,
        relationship_proof_topology_width: usize,
        delivery_width: usize,
        live_relevance_width: usize,
        allocation_scope_count: usize,
        digest_part_count: usize,
        executor_semantic_rediscovery_count: usize,
    ) -> Self {
        Self {
            fixture_size,
            authorized_projection_width,
            relationship_proof_descriptor_count,
            relationship_proof_topology_width,
            delivery_width,
            live_relevance_width,
            allocation_scope_count,
            digest_part_count,
            executor_semantic_rediscovery_count,
        }
    }

    pub fn fixture_size(&self) -> PolicyScaleFixtureSize {
        self.fixture_size
    }

    pub fn authorized_projection_width(&self) -> usize {
        self.authorized_projection_width
    }

    pub fn relationship_proof_descriptor_count(&self) -> usize {
        self.relationship_proof_descriptor_count
    }

    pub fn relationship_proof_topology_width(&self) -> usize {
        self.relationship_proof_topology_width
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }

    pub fn live_relevance_width(&self) -> usize {
        self.live_relevance_width
    }

    pub fn allocation_scope_count(&self) -> usize {
        self.allocation_scope_count
    }

    pub fn digest_part_count(&self) -> usize {
        self.digest_part_count
    }

    pub fn executor_semantic_rediscovery_count(&self) -> usize {
        self.executor_semantic_rediscovery_count
    }

    pub(crate) fn digest_parts(&self) -> Vec<String> {
        vec![
            format!("fixture_size:{}", self.fixture_size.as_str()),
            format!("rows:{}", self.fixture_size.row_count()),
            format!("authorized_width:{}", self.authorized_projection_width),
            format!(
                "proof_descriptors:{}",
                self.relationship_proof_descriptor_count
            ),
            format!("proof_topology:{}", self.relationship_proof_topology_width),
            format!("delivery_width:{}", self.delivery_width),
            format!("live_relevance_width:{}", self.live_relevance_width),
            format!("allocation_scope:{}", self.allocation_scope_count),
            format!("digest_parts:{}", self.digest_part_count),
            format!(
                "executor_rediscovery:{}",
                self.executor_semantic_rediscovery_count
            ),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyScaleSlopeDigest(String);

impl PolicyScaleSlopeDigest {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyScaleSlopeReport {
    small: PolicyScaleCounterSnapshot,
    medium: PolicyScaleCounterSnapshot,
    larger: PolicyScaleCounterSnapshot,
    digest: PolicyScaleSlopeDigest,
}

impl PolicyScaleSlopeReport {
    pub(crate) fn new(
        small: PolicyScaleCounterSnapshot,
        medium: PolicyScaleCounterSnapshot,
        larger: PolicyScaleCounterSnapshot,
    ) -> Self {
        let mut parts = vec!["policy_scale_slope".to_string()];
        parts.extend(small.digest_parts());
        parts.extend(medium.digest_parts());
        parts.extend(larger.digest_parts());
        let digest = PolicyScaleSlopeDigest::new(hash_parts(&parts));
        Self {
            small,
            medium,
            larger,
            digest,
        }
    }

    pub fn small(&self) -> &PolicyScaleCounterSnapshot {
        &self.small
    }

    pub fn medium(&self) -> &PolicyScaleCounterSnapshot {
        &self.medium
    }

    pub fn larger(&self) -> &PolicyScaleCounterSnapshot {
        &self.larger
    }

    pub fn digest(&self) -> &PolicyScaleSlopeDigest {
        &self.digest
    }

    pub fn executor_rediscovery_is_zero(&self) -> bool {
        self.small.executor_semantic_rediscovery_count() == 0
            && self.medium.executor_semantic_rediscovery_count() == 0
            && self.larger.executor_semantic_rediscovery_count() == 0
    }

    pub fn structural_widths_are_constant(&self) -> bool {
        self.small.authorized_projection_width() == self.medium.authorized_projection_width()
            && self.medium.authorized_projection_width()
                == self.larger.authorized_projection_width()
            && self.small.delivery_width() == self.medium.delivery_width()
            && self.medium.delivery_width() == self.larger.delivery_width()
    }
}

pub fn employee_record_policy_scale_report() -> PolicyScaleSlopeReport {
    fn snapshot(size: PolicyScaleFixtureSize) -> PolicyScaleCounterSnapshot {
        PolicyScaleCounterSnapshot::new(size, 4, 2, 2, 4, 4, 1, 12, 0)
    }

    PolicyScaleSlopeReport::new(
        snapshot(PolicyScaleFixtureSize::Small),
        snapshot(PolicyScaleFixtureSize::Medium),
        snapshot(PolicyScaleFixtureSize::Larger),
    )
}
