use std::sync::Arc;

use worth_store_security::{
    StoreAuthenticityRequirement, StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope,
};

use super::{QueueDurabilityClass, QueueGroupingDenial, QueueWorkClass};

const MAX_QUEUE_LOCALITY_RANGES: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueLocalityIdentity {
    digest: [u8; 32],
    ranges: Arc<[QueueLocalityRange]>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueLocalityRange {
    artifact: [u8; 32],
    start: u64,
    end: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueLocalityRelation {
    Exact,
    Adjacent,
    Disjoint,
    OverlappingOrInterleaved,
    StructurallyUnknown,
}

impl QueueLocalityIdentity {
    pub fn from_digest(digest: [u8; 32]) -> Self {
        Self {
            digest,
            ranges: Arc::from([]),
        }
    }

    pub fn from_ranges(
        digest: [u8; 32],
        ranges: impl IntoIterator<Item = QueueLocalityRange>,
    ) -> Option<Self> {
        let mut exact = Vec::new();
        for range in ranges {
            if exact.len() == MAX_QUEUE_LOCALITY_RANGES {
                return None;
            }
            exact.push(range);
        }
        if exact.is_empty() {
            return None;
        }
        exact.sort_unstable_by_key(|range| (range.artifact, range.start, range.end));
        if exact.iter().any(|range| range.start >= range.end)
            || exact
                .windows(2)
                .any(|pair| pair[0].artifact == pair[1].artifact && pair[0].end > pair[1].start)
        {
            return None;
        }
        Some(Self {
            digest,
            ranges: exact.into(),
        })
    }

    pub const fn as_bytes(&self) -> [u8; 32] {
        self.digest
    }

    pub fn relation(&self, other: &Self) -> QueueLocalityRelation {
        if self == other {
            return QueueLocalityRelation::Exact;
        }
        if self.ranges.is_empty() || other.ranges.is_empty() {
            return QueueLocalityRelation::StructurallyUnknown;
        }
        relation_between_sorted_ranges(&self.ranges, &other.ranges)
    }
}

impl QueueLocalityRange {
    pub const fn new(artifact: [u8; 32], start: u64, end: u64) -> Option<Self> {
        if start >= end {
            return None;
        }
        Some(Self {
            artifact,
            start,
            end,
        })
    }
}

fn relation_between_sorted_ranges(
    left: &[QueueLocalityRange],
    right: &[QueueLocalityRange],
) -> QueueLocalityRelation {
    let mut adjacent = false;
    let mut left_index = 0;
    let mut right_index = 0;
    while left_index < left.len() && right_index < right.len() {
        let l = left[left_index];
        let r = right[right_index];
        if l.artifact < r.artifact {
            left_index += 1;
        } else if r.artifact < l.artifact {
            right_index += 1;
        } else if l.end < r.start {
            left_index += 1;
        } else if r.end < l.start {
            right_index += 1;
        } else if l.end == r.start || r.end == l.start {
            adjacent = true;
            if l.end <= r.end {
                left_index += 1;
            } else {
                right_index += 1;
            }
        } else {
            return QueueLocalityRelation::OverlappingOrInterleaved;
        }
    }
    if adjacent {
        QueueLocalityRelation::Adjacent
    } else {
        QueueLocalityRelation::Disjoint
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueRecoveryOrdering {
    NotRecoveryCritical,
    WalBeforeData,
    RecoveryReadOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueWritebackPolicy {
    None,
    Immediate,
    DeferredWithinFlushEpoch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueGroupingBasis {
    security_scope_identity: StoreSecurityScopeIdentity,
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    authenticity_requirement: StoreAuthenticityRequirement,
    durability_class: QueueDurabilityClass,
    flush_epoch: u64,
    work_class: QueueWorkClass,
    recovery_ordering: QueueRecoveryOrdering,
    writeback_policy: QueueWritebackPolicy,
    locality: Option<QueueLocalityIdentity>,
}

impl QueueGroupingBasis {
    #[allow(clippy::too_many_arguments)]
    pub(crate) const fn new(
        security_scope_identity: StoreSecurityScopeIdentity,
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
        authenticity_requirement: StoreAuthenticityRequirement,
        durability_class: QueueDurabilityClass,
        flush_epoch: u64,
        work_class: QueueWorkClass,
        recovery_ordering: QueueRecoveryOrdering,
        writeback_policy: QueueWritebackPolicy,
    ) -> Self {
        Self {
            security_scope_identity,
            tenant_scope,
            key_scope,
            authenticity_requirement,
            durability_class,
            flush_epoch,
            work_class,
            recovery_ordering,
            writeback_policy,
            locality: None,
        }
    }

    pub(crate) fn with_locality(mut self, locality: QueueLocalityIdentity) -> Self {
        self.locality = Some(locality);
        self
    }

    pub fn compatible_with(&self, other: &Self) -> Result<(), QueueGroupingDenial> {
        if self.security_scope_identity != other.security_scope_identity {
            return Err(QueueGroupingDenial::SecurityScopeMismatch);
        }
        if self.tenant_scope != other.tenant_scope {
            return Err(QueueGroupingDenial::TenantScopeMismatch);
        }
        if self.key_scope != other.key_scope {
            return Err(QueueGroupingDenial::KeyScopeMismatch);
        }
        if self.authenticity_requirement != other.authenticity_requirement {
            return Err(QueueGroupingDenial::AuthenticityRequirementMismatch);
        }
        if self.durability_class != other.durability_class {
            return Err(QueueGroupingDenial::DurabilityClassMismatch);
        }
        if self.flush_epoch != other.flush_epoch {
            return Err(QueueGroupingDenial::FlushEpochMismatch);
        }
        if self.work_class != other.work_class {
            return Err(QueueGroupingDenial::WorkClassMismatch);
        }
        if self.recovery_ordering != other.recovery_ordering {
            return Err(QueueGroupingDenial::RecoveryOrderingMismatch);
        }
        if self.writeback_policy != other.writeback_policy {
            return Err(QueueGroupingDenial::WritebackPolicyMismatch);
        }
        if self.locality != other.locality {
            return Err(QueueGroupingDenial::LocalityMismatch);
        }
        Ok(())
    }

    pub const fn security_scope_identity(&self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn tenant_scope(&self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn key_scope(&self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn authenticity_requirement(&self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }

    pub const fn durability_class(&self) -> QueueDurabilityClass {
        self.durability_class
    }

    pub const fn flush_epoch(&self) -> u64 {
        self.flush_epoch
    }

    pub const fn work_class(&self) -> QueueWorkClass {
        self.work_class
    }

    pub const fn recovery_ordering(&self) -> QueueRecoveryOrdering {
        self.recovery_ordering
    }

    pub const fn writeback_policy(&self) -> QueueWritebackPolicy {
        self.writeback_policy
    }

    pub fn locality(&self) -> Option<&QueueLocalityIdentity> {
        self.locality.as_ref()
    }
}

#[cfg(test)]
mod locality_tests {
    use super::{QueueLocalityIdentity, QueueLocalityRange, QueueLocalityRelation};

    #[test]
    fn multi_artifact_ranges_preserve_overlap_and_disjoint_truth() {
        let left = locality(1, [range(1, 0, 8), range(2, 32, 40), range(3, 64, 72)]);
        let overlapping = locality(2, [range(2, 36, 44)]);
        let disjoint = locality(3, [range(1, 8, 16), range(4, 0, 8)]);

        assert_eq!(
            left.relation(&overlapping),
            QueueLocalityRelation::OverlappingOrInterleaved
        );
        assert_eq!(left.relation(&disjoint), QueueLocalityRelation::Adjacent);
    }

    #[test]
    fn malformed_or_overlapping_structural_ranges_are_rejected() {
        assert!(
            QueueLocalityIdentity::from_ranges([9; 32], [range(1, 0, 8), range(1, 4, 12)])
                .is_none()
        );
        assert!(QueueLocalityIdentity::from_ranges([9; 32], [range(1, 0, 8); 257]).is_none());
        assert!(QueueLocalityRange::new([1; 32], 8, 8).is_none());
    }

    #[test]
    fn equal_caller_digests_cannot_override_contradictory_structural_ranges() {
        let left = locality(9, [range(1, 0, 8)]);
        let different_artifact = locality(9, [range(2, 0, 8)]);
        let overlapping_offset = locality(9, [range(1, 4, 12)]);

        assert_eq!(left.relation(&left), QueueLocalityRelation::Exact);
        assert_eq!(
            left.relation(&different_artifact),
            QueueLocalityRelation::Disjoint
        );
        assert_eq!(
            left.relation(&overlapping_offset),
            QueueLocalityRelation::OverlappingOrInterleaved
        );
    }

    fn locality<const N: usize>(
        digest: u8,
        ranges: [QueueLocalityRange; N],
    ) -> QueueLocalityIdentity {
        QueueLocalityIdentity::from_ranges([digest; 32], ranges).unwrap()
    }

    fn range(artifact: u8, start: u64, end: u64) -> QueueLocalityRange {
        QueueLocalityRange::new([artifact; 32], start, end).unwrap()
    }
}
