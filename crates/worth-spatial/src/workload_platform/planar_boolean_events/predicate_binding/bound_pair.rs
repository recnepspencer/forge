use crate::planar_contracts::segment_segment_2d::{
    CertifiedSegmentSegment2DClassification, CertifiedSegmentSegment2DReceipt,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentCandidateRowReceipt;

use super::bound_pair_basis::PlanarBooleanPredicateBoundPairBasis;
use super::identity::{bound_pair_identity, BoundPairIdentityBasis};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPredicateBoundPair {
    reduced_pair_identity: String,
    segment_pair_identity: String,
    left_segment_identity: String,
    right_segment_identity: String,
    left_carrier_identity: String,
    right_carrier_identity: String,
    segment_contract_fact_digest: String,
    segment_contract_declaration_digest: String,
    segment_contract_envelope_digest: String,
    predicate_consumption_fact_digest: String,
    local_frame_identity: String,
    precision_basis_identity: String,
    segment_basis: PlanarBooleanPredicateBoundPairBasis,
    classification: CertifiedSegmentSegment2DClassification,
    predicate_binding_identity: String,
    bound_pair_identity: String,
}

impl PlanarBooleanPredicateBoundPair {
    pub(crate) fn new(
        reduced_pair_identity: &str,
        candidate_row: &PlanarBooleanSegmentCandidateRowReceipt,
        segment_receipt: &CertifiedSegmentSegment2DReceipt,
        predicate_consumption_fact_digest: &str,
    ) -> Self {
        let left_segment_identity = candidate_row
            .left()
            .canonical_segment_identity()
            .to_string();
        let right_segment_identity = candidate_row
            .right()
            .canonical_segment_identity()
            .to_string();
        let left_carrier_identity = candidate_row.left().carrier_identity().to_string();
        let right_carrier_identity = candidate_row.right().carrier_identity().to_string();
        let local_frame_identity = candidate_row.local_frame_identity().to_string();
        let precision_basis_identity = candidate_row.precision_basis_identity().to_string();
        let segment_contract_fact_digest = segment_receipt.fact_digest().to_string();
        let segment_basis =
            PlanarBooleanPredicateBoundPairBasis::from_segment_receipt(segment_receipt);
        let binding = Self {
            reduced_pair_identity: reduced_pair_identity.to_string(),
            segment_pair_identity: candidate_row.candidate_identity().to_string(),
            left_segment_identity,
            right_segment_identity,
            left_carrier_identity,
            right_carrier_identity,
            segment_contract_fact_digest,
            segment_contract_declaration_digest: segment_receipt.declaration_digest().to_string(),
            segment_contract_envelope_digest: segment_receipt.envelope_digest().to_string(),
            predicate_consumption_fact_digest: predicate_consumption_fact_digest.to_string(),
            local_frame_identity,
            precision_basis_identity,
            segment_basis,
            classification: segment_receipt.classification(),
            predicate_binding_identity: String::new(),
            bound_pair_identity: String::new(),
        };
        Self {
            bound_pair_identity: bound_pair_identity(BoundPairIdentityBasis {
                reduced_pair_identity: &binding.reduced_pair_identity,
                segment_pair_identity: &binding.segment_pair_identity,
                left_segment_identity: &binding.left_segment_identity,
                right_segment_identity: &binding.right_segment_identity,
                left_carrier_identity: &binding.left_carrier_identity,
                right_carrier_identity: &binding.right_carrier_identity,
                segment_contract_fact_digest: &binding.segment_contract_fact_digest,
                predicate_consumption_fact_digest: &binding.predicate_consumption_fact_digest,
                local_frame_identity: &binding.local_frame_identity,
                precision_basis_identity: &binding.precision_basis_identity,
            }),
            ..binding
        }
    }

    pub(crate) fn with_predicate_binding_identity(
        mut self,
        predicate_binding_identity: &str,
    ) -> Self {
        self.predicate_binding_identity = predicate_binding_identity.to_string();
        self
    }

    pub fn reduced_pair_identity(&self) -> &str {
        &self.reduced_pair_identity
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }

    pub fn left_segment_identity(&self) -> &str {
        &self.left_segment_identity
    }

    pub fn right_segment_identity(&self) -> &str {
        &self.right_segment_identity
    }

    pub fn left_carrier_identity(&self) -> &str {
        &self.left_carrier_identity
    }

    pub fn right_carrier_identity(&self) -> &str {
        &self.right_carrier_identity
    }

    pub fn segment_contract_fact_digest(&self) -> &str {
        &self.segment_contract_fact_digest
    }

    pub fn segment_contract_declaration_digest(&self) -> &str {
        &self.segment_contract_declaration_digest
    }

    pub fn segment_contract_envelope_digest(&self) -> &str {
        &self.segment_contract_envelope_digest
    }

    pub fn predicate_consumption_fact_digest(&self) -> &str {
        &self.predicate_consumption_fact_digest
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn precision_basis_identity(&self) -> &str {
        &self.precision_basis_identity
    }

    pub fn classification(&self) -> CertifiedSegmentSegment2DClassification {
        self.classification
    }

    pub(crate) fn segment_basis(&self) -> &PlanarBooleanPredicateBoundPairBasis {
        &self.segment_basis
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn bound_pair_identity(&self) -> &str {
        &self.bound_pair_identity
    }
}
