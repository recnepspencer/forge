#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
    pub(super) readiness_inputs_consumed: usize,
    pub(super) overlap_ledger_receipts_consumed: usize,
    pub(super) replay_rows_verified: usize,
    pub(super) decision_rows_verified: usize,
    pub(super) ledger_rows_verified: usize,
    pub(super) boundary_only_rows_verified: usize,
    pub(super) area_rows_verified: usize,
    pub(super) mixed_boundary_area_rows_verified: usize,
    pub(super) pairwise_rediscovery_attempts: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness {
    pub(super) digest: String,
    pub(super) region_count: usize,
    pub(super) row_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSharedAreaOutcomeWitness {
    pub(super) digest: String,
    pub(super) component_count: usize,
    pub(super) row_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness {
    pub(super) digest: String,
    pub(super) stable_region_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness {
    pub(super) digest: String,
    pub(super) nested_region_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionMixedBoundaryAreaWitness {
    pub(super) digest: String,
    pub(super) boundary_only_rows: usize,
    pub(super) area_rows: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionOrderingParityWitness {
    pub(super) canonical_digest: String,
    pub(super) order_invariant_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionReplayParityWitness {
    pub(super) original_outcome_digest: String,
    pub(super) replayed_outcome_digest: String,
    pub(super) replay_row_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCheckpointOutcomeWitness {
    pub(super) checkpoint_identity: String,
    pub(super) replay_evidence_identity: String,
    pub(super) certified_outcome_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionStormWitness {
    pub(super) identity_rows_examined: usize,
    pub(super) decision_rows_verified: usize,
    pub(super) ledger_rows_verified: usize,
    pub(super) pairwise_rediscovery_attempts: usize,
}

impl PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
    pub fn readiness_inputs_consumed(self) -> usize {
        self.readiness_inputs_consumed
    }
    pub fn overlap_ledger_receipts_consumed(self) -> usize {
        self.overlap_ledger_receipts_consumed
    }
    pub fn replay_rows_verified(self) -> usize {
        self.replay_rows_verified
    }
    pub fn decision_rows_verified(self) -> usize {
        self.decision_rows_verified
    }
    pub fn ledger_rows_verified(self) -> usize {
        self.ledger_rows_verified
    }
    pub fn boundary_only_rows_verified(self) -> usize {
        self.boundary_only_rows_verified
    }
    pub fn area_rows_verified(self) -> usize {
        self.area_rows_verified
    }
    pub fn mixed_boundary_area_rows_verified(self) -> usize {
        self.mixed_boundary_area_rows_verified
    }
    pub fn pairwise_rediscovery_attempts(self) -> usize {
        self.pairwise_rediscovery_attempts
    }
}

macro_rules! witness_accessors {
    ($type_name:ident, $digest:ident, $( $field:ident : $ret:ty ),* $(,)?) => {
        impl $type_name {
            pub fn digest(&self) -> &str { &self.$digest }
            $(pub fn $field(&self) -> $ret { self.$field })*
        }
    };
}

witness_accessors!(PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness, digest, region_count: usize, row_count: usize);
witness_accessors!(PlanarBooleanOverlapRegionSharedAreaOutcomeWitness, digest, component_count: usize, row_count: usize);
witness_accessors!(PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness, digest, stable_region_count: usize);
witness_accessors!(PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness, digest, nested_region_count: usize);
witness_accessors!(PlanarBooleanOverlapRegionMixedBoundaryAreaWitness, digest, boundary_only_rows: usize, area_rows: usize);

impl PlanarBooleanOverlapRegionOrderingParityWitness {
    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
    pub fn order_invariant_digest(&self) -> &str {
        &self.order_invariant_digest
    }
}

impl PlanarBooleanOverlapRegionReplayParityWitness {
    pub fn original_outcome_digest(&self) -> &str {
        &self.original_outcome_digest
    }
    pub fn replayed_outcome_digest(&self) -> &str {
        &self.replayed_outcome_digest
    }
    pub fn replay_row_count(&self) -> usize {
        self.replay_row_count
    }
}

impl PlanarBooleanOverlapRegionCheckpointOutcomeWitness {
    pub fn checkpoint_identity(&self) -> &str {
        &self.checkpoint_identity
    }
    pub fn replay_evidence_identity(&self) -> &str {
        &self.replay_evidence_identity
    }
    pub fn certified_outcome_digest(&self) -> &str {
        &self.certified_outcome_digest
    }
}

impl PlanarBooleanOverlapRegionStormWitness {
    pub fn identity_rows_examined(&self) -> usize {
        self.identity_rows_examined
    }
    pub fn decision_rows_verified(&self) -> usize {
        self.decision_rows_verified
    }
    pub fn ledger_rows_verified(&self) -> usize {
        self.ledger_rows_verified
    }
    pub fn pairwise_rediscovery_attempts(&self) -> usize {
        self.pairwise_rediscovery_attempts
    }
}
