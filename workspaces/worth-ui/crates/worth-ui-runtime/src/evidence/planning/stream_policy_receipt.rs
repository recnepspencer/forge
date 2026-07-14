use crate::runtime::{
    UiAllocationFrameCadenceVerdict, UiAllocationFrameDuplicateWitness, UiAllocationFrameEpoch,
    UiAllocationFrameIngressKey, UiAllocationFrameResolutionDenial,
    UiAllocationFrameSourceFactPosture, UiAllocationIngressPolicyVerdict,
    UiAllocationIntermediatePolicyVerdict, UiAllocationInvalidationIntent,
    UiAllocationSourceOrderVerdict, UiAllocationStreamCompositionCounters,
    UiAllocationStreamFamily, UiResolvedAllocationPolicyBranch, UiResolvedAllocationStreamPolicy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationStreamPolicyEvidenceReceipt {
    epoch: UiAllocationFrameEpoch,
    families: Box<[UiAllocationStreamFamily]>,
    order_verdicts: Box<[UiAllocationSourceOrderVerdict]>,
    duplicate_witness: UiAllocationFrameDuplicateWitness,
    invalidations: Box<[UiAllocationInvalidationIntent]>,
    policy: UiResolvedAllocationStreamPolicy,
    intermediate: Box<[UiAllocationIntermediatePolicyVerdict]>,
    branches: Box<[UiResolvedAllocationPolicyBranch]>,
    ingress_policy_verdicts: Box<[UiAllocationIngressPolicyVerdict]>,
    cadence: UiAllocationFrameCadenceVerdict,
    composition_counters: UiAllocationStreamCompositionCounters,
    payload_counters: UiAllocationStreamPolicyPayloadCounters,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiAllocationStreamPolicyPayloadCounters {
    vector_capacity_reservations: u8,
    boxed_slice_conversions: u8,
    denial_source_posture_copies: u16,
    pair_contract_evaluations: u8,
    pair_policy_joins: u8,
    n_way_policy_joins: u8,
    branch_policy_joins: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationStreamPolicyDenialEvidenceReceipt {
    epoch: UiAllocationFrameEpoch,
    ingress: Box<[UiAllocationFrameSourceFactPosture]>,
    duplicate_witness: UiAllocationFrameDuplicateWitness,
    order_verdicts: Box<[UiAllocationSourceOrderVerdict]>,
    ingress_policy_verdicts: Box<[UiAllocationIngressPolicyVerdict]>,
    denial: UiAllocationFrameResolutionDenial,
    payload_counters: UiAllocationStreamPolicyPayloadCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAllocationStreamPolicyEvidenceOutcome {
    Resolved(UiAllocationStreamPolicyEvidenceReceipt),
    Denied(UiAllocationStreamPolicyDenialEvidenceReceipt),
}

impl UiAllocationStreamPolicyEvidenceReceipt {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        epoch: UiAllocationFrameEpoch,
        families: Box<[UiAllocationStreamFamily]>,
        order_verdicts: Box<[UiAllocationSourceOrderVerdict]>,
        duplicate_witness: UiAllocationFrameDuplicateWitness,
        invalidations: Box<[UiAllocationInvalidationIntent]>,
        policy: UiResolvedAllocationStreamPolicy,
        intermediate: Box<[UiAllocationIntermediatePolicyVerdict]>,
        branches: Box<[UiResolvedAllocationPolicyBranch]>,
        ingress_policy_verdicts: Box<[UiAllocationIngressPolicyVerdict]>,
        cadence: UiAllocationFrameCadenceVerdict,
        composition_counters: UiAllocationStreamCompositionCounters,
        payload_counters: UiAllocationStreamPolicyPayloadCounters,
    ) -> Self {
        Self {
            epoch,
            families,
            order_verdicts,
            duplicate_witness,
            invalidations,
            policy,
            intermediate,
            branches,
            ingress_policy_verdicts,
            cadence,
            composition_counters,
            payload_counters,
        }
    }

    pub fn epoch(&self) -> UiAllocationFrameEpoch {
        self.epoch
    }
    pub fn ingress_keys(&self) -> &[UiAllocationFrameIngressKey] {
        self.duplicate_witness.canonical_ingress_keys()
    }
    pub fn families(&self) -> &[UiAllocationStreamFamily] {
        &self.families
    }
    pub fn order_verdicts(&self) -> &[UiAllocationSourceOrderVerdict] {
        &self.order_verdicts
    }
    pub fn duplicate_witness(&self) -> &UiAllocationFrameDuplicateWitness {
        &self.duplicate_witness
    }
    pub fn invalidations(&self) -> &[UiAllocationInvalidationIntent] {
        &self.invalidations
    }
    pub fn policy(&self) -> UiResolvedAllocationStreamPolicy {
        self.policy
    }
    pub fn intermediate(&self) -> &[UiAllocationIntermediatePolicyVerdict] {
        &self.intermediate
    }
    pub fn branches(&self) -> &[UiResolvedAllocationPolicyBranch] {
        &self.branches
    }
    pub fn ingress_policy_verdicts(&self) -> &[UiAllocationIngressPolicyVerdict] {
        &self.ingress_policy_verdicts
    }
    pub fn cadence(&self) -> UiAllocationFrameCadenceVerdict {
        self.cadence
    }
    pub fn composition_counters(&self) -> UiAllocationStreamCompositionCounters {
        self.composition_counters
    }
    pub fn payload_counters(&self) -> UiAllocationStreamPolicyPayloadCounters {
        self.payload_counters
    }
}

impl UiAllocationStreamPolicyPayloadCounters {
    pub(crate) fn reserve_vector_capacity(&mut self, capacity: usize) {
        self.vector_capacity_reservations = self
            .vector_capacity_reservations
            .saturating_add(u8::from(capacity > 0));
    }
    pub(crate) fn convert_boxed_slice(&mut self) {
        self.boxed_slice_conversions = self.boxed_slice_conversions.saturating_add(1);
    }
    pub(crate) fn copy_denial_source_posture(&mut self) {
        self.denial_source_posture_copies = self.denial_source_posture_copies.saturating_add(1);
    }
    pub(crate) fn evaluate_pair_contract(&mut self) {
        self.pair_contract_evaluations = self.pair_contract_evaluations.saturating_add(1);
    }
    pub(crate) fn join_pair_policy(&mut self) {
        self.pair_policy_joins = self.pair_policy_joins.saturating_add(1);
    }
    pub(crate) fn join_n_way_policy(&mut self) {
        self.n_way_policy_joins = self.n_way_policy_joins.saturating_add(1);
    }
    pub(crate) fn join_branch_policy(&mut self) {
        self.branch_policy_joins = self.branch_policy_joins.saturating_add(1);
    }
    pub fn vector_capacity_reservations(self) -> u8 {
        self.vector_capacity_reservations
    }
    pub fn boxed_slice_conversions(self) -> u8 {
        self.boxed_slice_conversions
    }
    pub fn denial_source_posture_copies(self) -> u16 {
        self.denial_source_posture_copies
    }
    pub fn pair_contract_evaluations(self) -> u8 {
        self.pair_contract_evaluations
    }
    pub fn pair_policy_joins(self) -> u8 {
        self.pair_policy_joins
    }
    pub fn n_way_policy_joins(self) -> u8 {
        self.n_way_policy_joins
    }
    pub fn branch_policy_joins(self) -> u8 {
        self.branch_policy_joins
    }
}

impl UiAllocationStreamPolicyDenialEvidenceReceipt {
    pub(crate) fn new(
        epoch: UiAllocationFrameEpoch,
        ingress: Box<[UiAllocationFrameSourceFactPosture]>,
        duplicate_witness: UiAllocationFrameDuplicateWitness,
        order_verdicts: Box<[UiAllocationSourceOrderVerdict]>,
        ingress_policy_verdicts: Box<[UiAllocationIngressPolicyVerdict]>,
        denial: UiAllocationFrameResolutionDenial,
        payload_counters: UiAllocationStreamPolicyPayloadCounters,
    ) -> Self {
        Self {
            epoch,
            ingress,
            duplicate_witness,
            order_verdicts,
            ingress_policy_verdicts,
            denial,
            payload_counters,
        }
    }

    pub fn epoch(&self) -> UiAllocationFrameEpoch {
        self.epoch
    }
    pub fn ingress(&self) -> &[UiAllocationFrameSourceFactPosture] {
        &self.ingress
    }
    pub fn ingress_keys(&self) -> &[UiAllocationFrameIngressKey] {
        self.duplicate_witness.canonical_ingress_keys()
    }
    pub fn duplicate_witness(&self) -> &UiAllocationFrameDuplicateWitness {
        &self.duplicate_witness
    }
    pub fn order_verdicts(&self) -> &[UiAllocationSourceOrderVerdict] {
        &self.order_verdicts
    }
    pub fn ingress_policy_verdicts(&self) -> &[UiAllocationIngressPolicyVerdict] {
        &self.ingress_policy_verdicts
    }
    pub fn denial(&self) -> UiAllocationFrameResolutionDenial {
        self.denial
    }
    pub fn payload_counters(&self) -> UiAllocationStreamPolicyPayloadCounters {
        self.payload_counters
    }
}

impl UiAllocationStreamPolicyEvidenceOutcome {
    pub(crate) fn resolved(&self) -> Option<&UiAllocationStreamPolicyEvidenceReceipt> {
        match self {
            Self::Resolved(receipt) => Some(receipt),
            Self::Denied(_) => None,
        }
    }
    pub(crate) fn denied(&self) -> Option<&UiAllocationStreamPolicyDenialEvidenceReceipt> {
        match self {
            Self::Denied(receipt) => Some(receipt),
            Self::Resolved(_) => None,
        }
    }
}
