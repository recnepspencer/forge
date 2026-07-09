use crate::access_shape::{
    S8AccessAuthorityPosture, S8AccessLaneClassification, S8AccessShapeDetail,
    S8AccessStaleDisposition, S8ExpectedCounterClass,
};
use crate::budget::S8PlannedCounterEnvelope;
use crate::key_domain::PhysicalKeyDomainWitness;
use crate::maintenance::S8PhysicalMutationShape;
use crate::strategy::S8LayoutStrategyFamily;
use worth_store_budgets::S8PreExecutionPlanBinding;

use super::selection_basis::S8DeterministicSelectionRule;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PlanFingerprint {
    family: S8LayoutStrategyFamily,
    detail: S8AccessShapeDetail,
    lane: S8AccessLaneClassification,
    authority_posture: S8AccessAuthorityPosture,
    stale_disposition: S8AccessStaleDisposition,
    key_domain: PhysicalKeyDomainWitness,
    expected_counters: S8ExpectedCounterClass,
    mutation_shape: Option<S8PhysicalMutationShape>,
    budget_rows: Option<u64>,
    planned_counter_envelope: S8PlannedCounterEnvelope,
    selection_rule: S8DeterministicSelectionRule,
}

impl S8PlanFingerprint {
    pub(crate) const fn new(
        family: S8LayoutStrategyFamily,
        detail: S8AccessShapeDetail,
        lane: S8AccessLaneClassification,
        authority_posture: S8AccessAuthorityPosture,
        stale_disposition: S8AccessStaleDisposition,
        key_domain: PhysicalKeyDomainWitness,
        expected_counters: S8ExpectedCounterClass,
        mutation_shape: Option<S8PhysicalMutationShape>,
        budget_rows: Option<u64>,
        planned_counter_envelope: S8PlannedCounterEnvelope,
        selection_rule: S8DeterministicSelectionRule,
    ) -> Self {
        Self {
            family,
            detail,
            lane,
            authority_posture,
            stale_disposition,
            key_domain,
            expected_counters,
            mutation_shape,
            budget_rows,
            planned_counter_envelope,
            selection_rule,
        }
    }

    pub const fn family(self) -> S8LayoutStrategyFamily {
        self.family
    }

    pub const fn detail(self) -> S8AccessShapeDetail {
        self.detail
    }

    pub const fn shape(self) -> crate::access_shape::S8AccessShape {
        self.detail.shape()
    }

    pub const fn lane(self) -> S8AccessLaneClassification {
        self.lane
    }

    pub const fn authority_posture(self) -> S8AccessAuthorityPosture {
        self.authority_posture
    }

    pub const fn stale_disposition(self) -> S8AccessStaleDisposition {
        self.stale_disposition
    }

    pub const fn key_domain(self) -> PhysicalKeyDomainWitness {
        self.key_domain
    }

    pub const fn planned_counter_envelope(self) -> S8PlannedCounterEnvelope {
        self.planned_counter_envelope
    }

    pub const fn selection_rule(self) -> S8DeterministicSelectionRule {
        self.selection_rule
    }

    pub(crate) fn plan_binding(self) -> S8PreExecutionPlanBinding {
        S8PreExecutionPlanBinding::new(
            mix_values(&[
                family_code(self.family) as u64,
                detail_code(self.detail) as u64,
                lane_code(self.lane) as u64,
                authority_posture_code(self.authority_posture) as u64,
                stale_code(self.stale_disposition) as u64,
                expected_counter_code(self.expected_counters) as u64,
                mutation_code(self.mutation_shape) as u64,
                rule_code(self.selection_rule) as u64,
                domain_code(self.key_domain.domain()) as u64,
            ]),
            mix_snapshot(self.planned_counter_envelope.lookup()),
            mix_snapshot(self.planned_counter_envelope.publication()),
            mix_snapshot(self.planned_counter_envelope.recovery()),
            self.budget_rows.unwrap_or(0),
        )
    }
}

fn mix_snapshot(snapshot: crate::execution::S8AccessPathCounterSnapshot) -> u64 {
    mix_values(&[
        snapshot.point_lookups() as u64,
        snapshot.range_lookups() as u64,
        snapshot.wal_replays() as u64,
        snapshot.publications() as u64,
        snapshot.maintenance_reads() as u64,
        snapshot.page_touches() as u64,
        snapshot.index_probes() as u64,
        snapshot.key_comparisons() as u64,
        snapshot.range_steps() as u64,
        snapshot.prefix_steps() as u64,
        snapshot.chunk_tree_node_reads() as u64,
        snapshot.manifest_reads() as u64,
        snapshot.bytes_read(),
        snapshot.bytes_written(),
        snapshot.write_fanout() as u64,
        snapshot.read_amplification() as u64,
        snapshot.write_amplification() as u64,
    ])
}

fn mix_values(values: &[u64]) -> u64 {
    let mut acc = 0xcbf29ce484222325_u64;
    for value in values {
        acc ^= *value;
        acc = acc.wrapping_mul(0x100000001b3_u64);
    }
    acc
}

const fn family_code(family: S8LayoutStrategyFamily) -> u8 {
    match family {
        S8LayoutStrategyFamily::AppendLog => 1,
        S8LayoutStrategyFamily::HeapFile => 2,
        S8LayoutStrategyFamily::PageTable => 3,
        S8LayoutStrategyFamily::BaselineBTreeRange => 4,
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized => 5,
        S8LayoutStrategyFamily::SparseIndex => 6,
        S8LayoutStrategyFamily::ChunkTree => 7,
        S8LayoutStrategyFamily::ManifestTable => 8,
        S8LayoutStrategyFamily::BitmapAllocationMap => 9,
        S8LayoutStrategyFamily::HashEqualityIndex => 10,
        S8LayoutStrategyFamily::RangeMap => 11,
        S8LayoutStrategyFamily::QuarantineMap => 12,
        S8LayoutStrategyFamily::StreamingCursorIndex => 13,
        S8LayoutStrategyFamily::ExactScan => 14,
    }
}

const fn detail_code(detail: S8AccessShapeDetail) -> u8 {
    match detail {
        S8AccessShapeDetail::PointLookup => 1,
        S8AccessShapeDetail::BatchPointLookup(_) => 2,
        S8AccessShapeDetail::SortedBatchLookup(_) => 3,
        S8AccessShapeDetail::RangeLookup(_) => 4,
        S8AccessShapeDetail::MultiRangeLookup(_) => 5,
        S8AccessShapeDetail::PrefixLookup(_) => 6,
        S8AccessShapeDetail::GroupedPrefixLookup(_) => 7,
        S8AccessShapeDetail::CoalescedPageRead(_) => 8,
        S8AccessShapeDetail::ChunkTreeWalk(_) => 9,
        S8AccessShapeDetail::ManifestGraphWalk(_) => 10,
        S8AccessShapeDetail::BoundedScan(_) => 11,
        S8AccessShapeDetail::FullDeclaredScan(_) => 12,
        S8AccessShapeDetail::StreamingRead(_) => 13,
        S8AccessShapeDetail::StreamingContinuationRead(_) => 14,
        S8AccessShapeDetail::Append(_) => 15,
        S8AccessShapeDetail::CompactionRead(_) => 16,
        S8AccessShapeDetail::RebuildRead(_) => 17,
        S8AccessShapeDetail::VerifierRead(_) => 18,
        S8AccessShapeDetail::RepairRead(_) => 19,
        S8AccessShapeDetail::QuarantineRead(_) => 20,
        S8AccessShapeDetail::DegradedExactScan(_) => 21,
    }
}

const fn lane_code(lane: S8AccessLaneClassification) -> u8 {
    match lane {
        S8AccessLaneClassification::Foreground => 1,
        S8AccessLaneClassification::Maintenance => 2,
        S8AccessLaneClassification::Verifier => 3,
        S8AccessLaneClassification::Terminal => 4,
    }
}

const fn domain_code(domain: crate::key_domain::PhysicalKeyDomain) -> u8 {
    match domain {
        crate::key_domain::PhysicalKeyDomain::RootManifestKey => 1,
        crate::key_domain::PhysicalKeyDomain::PageAddressKey => 2,
        crate::key_domain::PhysicalKeyDomain::SegmentAddressKey => 3,
        crate::key_domain::PhysicalKeyDomain::ExtentAddressKey => 4,
        crate::key_domain::PhysicalKeyDomain::PhysicalReferenceKey => 5,
        crate::key_domain::PhysicalKeyDomain::WalRecordKey => 6,
        crate::key_domain::PhysicalKeyDomain::BlobIdentityKey => 7,
    }
}

const fn authority_posture_code(authority: S8AccessAuthorityPosture) -> u8 {
    match authority {
        S8AccessAuthorityPosture::ExactMaterialized => 1,
        S8AccessAuthorityPosture::ExplicitDegradedExactScan => 2,
        S8AccessAuthorityPosture::MaintenanceMutation => 3,
    }
}

const fn stale_code(stale: S8AccessStaleDisposition) -> u8 {
    match stale {
        S8AccessStaleDisposition::ExactOnly => 1,
        S8AccessStaleDisposition::RebindBeforeExecution => 2,
        S8AccessStaleDisposition::ExplicitDegradedFallback => 3,
    }
}

const fn expected_counter_code(expected: S8ExpectedCounterClass) -> u8 {
    match expected {
        S8ExpectedCounterClass::PointLookup => 1,
        S8ExpectedCounterClass::BatchPointLookup => 2,
        S8ExpectedCounterClass::SortedBatchLookup => 3,
        S8ExpectedCounterClass::RangeLookup => 4,
        S8ExpectedCounterClass::MultiRangeLookup => 5,
        S8ExpectedCounterClass::PrefixLookup => 6,
        S8ExpectedCounterClass::GroupedPrefixLookup => 7,
        S8ExpectedCounterClass::CoalescedPageRead => 8,
        S8ExpectedCounterClass::ChunkTreeWalk => 9,
        S8ExpectedCounterClass::ManifestGraphWalk => 10,
        S8ExpectedCounterClass::BoundedScan => 11,
        S8ExpectedCounterClass::FullDeclaredScan => 12,
        S8ExpectedCounterClass::StreamingRead => 13,
        S8ExpectedCounterClass::StreamingContinuationRead => 14,
        S8ExpectedCounterClass::AppendTraversal => 15,
        S8ExpectedCounterClass::CompactionTraversal => 16,
        S8ExpectedCounterClass::RebuildTraversal => 17,
        S8ExpectedCounterClass::VerifierTraversal => 18,
        S8ExpectedCounterClass::RepairTraversal => 19,
        S8ExpectedCounterClass::QuarantineTraversal => 20,
        S8ExpectedCounterClass::DegradedExactScan => 21,
    }
}

const fn mutation_code(mutation: Option<S8PhysicalMutationShape>) -> u8 {
    match mutation {
        Some(S8PhysicalMutationShape::ObservationOnly) => 1,
        Some(S8PhysicalMutationShape::PointRewrite) => 2,
        Some(S8PhysicalMutationShape::LogStructuredAppend) => 3,
        Some(S8PhysicalMutationShape::CompactionRewrite) => 4,
        None => 0,
    }
}

const fn rule_code(rule: S8DeterministicSelectionRule) -> u8 {
    match rule {
        S8DeterministicSelectionRule::SoleEligibleCandidate => 1,
        S8DeterministicSelectionRule::OrderedIndexReadsPreferBTree => 2,
        S8DeterministicSelectionRule::BufferedOrTraversalReadsPreferLsm => 3,
        S8DeterministicSelectionRule::ExplicitDegradedExactScan => 4,
    }
}
