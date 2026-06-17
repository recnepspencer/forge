use crate::planar_contracts::predicate_consumption::PredicateCertificateConsumptionReceipt;
use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentPairEnumerationReceipt;

use super::bound_pair::PlanarBooleanPredicateBoundPair;
use super::counters::PlanarBooleanEventPredicateBindingCounters;
use super::denial::{
    PlanarBooleanEventPredicateBindingDenial, PlanarBooleanEventPredicateBindingDenialKind,
};
use super::identity::predicate_binding_identity;
use super::{aligned_segment_contracts, validate_predicate_consumption_alignment};

#[derive(Clone, Debug)]
pub struct PlanarBooleanEventPredicateBindingPlan<'a> {
    reduced_pair_identity: String,
    pair_enumeration: &'a PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    predicate_consumption: Option<PredicateCertificateConsumptionReceipt>,
}

#[derive(Clone, Debug)]
pub struct PlanarBooleanEventPredicateBindingCompiledPlan<'a> {
    reduced_pair_identity: String,
    pair_enumeration: &'a PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
    predicate_consumption: PredicateCertificateConsumptionReceipt,
}

impl<'a> PlanarBooleanEventPredicateBindingPlan<'a> {
    pub fn for_reduced_pair(mut self, identity: impl Into<String>) -> Self {
        self.reduced_pair_identity = identity.into();
        self
    }

    pub fn with_segment_segment_receipts<I>(mut self, receipts: I) -> Self
    where
        I: IntoIterator<Item = CertifiedSegmentSegment2DReceipt>,
    {
        self.segment_receipts = receipts.into_iter().collect();
        self
    }

    pub fn with_predicate_consumption_receipt(
        mut self,
        receipt: PredicateCertificateConsumptionReceipt,
    ) -> Self {
        self.predicate_consumption = Some(receipt);
        self
    }

    pub fn required_segment_contracts(&self) -> usize {
        self.pair_enumeration.candidate_rows().len()
    }

    pub fn supplied_segment_contracts(&self) -> usize {
        self.segment_receipts.len()
    }

    pub fn required_predicate_receipts(&self) -> usize {
        self.required_segment_contracts().saturating_mul(4)
    }

    pub fn compile(
        self,
    ) -> Result<
        PlanarBooleanEventPredicateBindingCompiledPlan<'a>,
        PlanarBooleanEventPredicateBindingDenial,
    > {
        let predicate_consumption = self.predicate_consumption.ok_or_else(|| {
            denial(
                PlanarBooleanEventPredicateBindingDenialKind::PredicateConsumptionSegmentSetMismatch,
                &self.reduced_pair_identity,
                "",
                counters(self.pair_enumeration, &self.segment_receipts, 0, 0),
                "predicate binding requires a predicate-consumption receipt",
            )
        })?;
        if self.reduced_pair_identity.is_empty() {
            return Err(denial(
                PlanarBooleanEventPredicateBindingDenialKind::MissingReducedPairIdentity,
                "",
                "",
                counters(
                    self.pair_enumeration,
                    &self.segment_receipts,
                    0,
                    predicate_consumption.certified_predicate_rows(),
                ),
                "predicate binding requires the reduced-pair identity",
            ));
        }
        Ok(PlanarBooleanEventPredicateBindingCompiledPlan {
            reduced_pair_identity: self.reduced_pair_identity,
            pair_enumeration: self.pair_enumeration,
            segment_receipts: self.segment_receipts,
            predicate_consumption,
        })
    }
}

impl PlanarBooleanEventPredicateBindingCompiledPlan<'_> {
    pub fn required_segment_contracts(&self) -> usize {
        self.pair_enumeration.candidate_rows().len()
    }

    pub fn supplied_segment_contracts(&self) -> usize {
        self.segment_receipts.len()
    }

    pub fn required_predicate_receipts(&self) -> usize {
        self.required_segment_contracts().saturating_mul(4)
    }

    pub fn certify(
        self,
    ) -> Result<PlanarBooleanEventPredicateBinding, PlanarBooleanEventPredicateBindingDenial> {
        validate_predicate_consumption_alignment(
            &self.reduced_pair_identity,
            self.pair_enumeration,
            &self.segment_receipts,
            &self.predicate_consumption,
        )?;
        let bound_pairs = aligned_segment_contracts(
            &self.reduced_pair_identity,
            self.pair_enumeration,
            &self.segment_receipts,
            self.predicate_consumption.fact_digest(),
        )?;
        let counters = counters(
            self.pair_enumeration,
            &self.segment_receipts,
            bound_pairs.len(),
            self.predicate_consumption.certified_predicate_rows(),
        );
        let binding_identity = predicate_binding_identity(
            &self.reduced_pair_identity,
            self.pair_enumeration,
            &self.predicate_consumption,
            counters,
            &bound_pairs,
        );
        let bound_pairs = bound_pairs
            .into_iter()
            .map(|pair| pair.with_predicate_binding_identity(&binding_identity))
            .collect();
        Ok(PlanarBooleanEventPredicateBinding {
            reduced_pair_identity: self.reduced_pair_identity,
            segment_pair_enumeration_identity: self
                .pair_enumeration
                .segment_pair_enumeration_identity()
                .to_string(),
            canonical_segment_set_identity: self
                .pair_enumeration
                .canonical_segment_set_identity()
                .to_string(),
            predicate_consumption_fact_digest: self.predicate_consumption.fact_digest().to_string(),
            predicate_consumption_declaration_digest: self
                .predicate_consumption
                .declaration_digest()
                .to_string(),
            predicate_consumption_envelope_digest: self
                .predicate_consumption
                .envelope_digest()
                .to_string(),
            counters,
            bound_pairs,
            binding_identity,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanEventPredicateBinding {
    reduced_pair_identity: String,
    segment_pair_enumeration_identity: String,
    canonical_segment_set_identity: String,
    predicate_consumption_fact_digest: String,
    predicate_consumption_declaration_digest: String,
    predicate_consumption_envelope_digest: String,
    counters: PlanarBooleanEventPredicateBindingCounters,
    bound_pairs: Vec<PlanarBooleanPredicateBoundPair>,
    binding_identity: String,
}

impl PlanarBooleanEventPredicateBinding {
    pub fn plan(
        pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    ) -> PlanarBooleanEventPredicateBindingPlan<'_> {
        PlanarBooleanEventPredicateBindingPlan {
            reduced_pair_identity: String::new(),
            pair_enumeration,
            segment_receipts: Vec::new(),
            predicate_consumption: None,
        }
    }

    pub fn reduced_pair_identity(&self) -> &str {
        &self.reduced_pair_identity
    }

    pub fn segment_pair_enumeration_identity(&self) -> &str {
        &self.segment_pair_enumeration_identity
    }

    pub fn canonical_segment_set_identity(&self) -> &str {
        &self.canonical_segment_set_identity
    }

    pub fn predicate_consumption_fact_digest(&self) -> &str {
        &self.predicate_consumption_fact_digest
    }

    pub fn predicate_consumption_declaration_digest(&self) -> &str {
        &self.predicate_consumption_declaration_digest
    }

    pub fn predicate_consumption_envelope_digest(&self) -> &str {
        &self.predicate_consumption_envelope_digest
    }

    pub fn counters(&self) -> PlanarBooleanEventPredicateBindingCounters {
        self.counters
    }

    pub fn bound_pairs(&self) -> &[PlanarBooleanPredicateBoundPair] {
        &self.bound_pairs
    }

    pub fn bound_pair(
        &self,
        segment_pair_identity: &str,
    ) -> Option<&PlanarBooleanPredicateBoundPair> {
        self.bound_pairs
            .iter()
            .find(|pair| pair.segment_pair_identity() == segment_pair_identity)
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.binding_identity
    }
}

fn counters(
    pair_enumeration: &PlanarBooleanSegmentPairEnumerationReceipt,
    segment_receipts: &[CertifiedSegmentSegment2DReceipt],
    bound_segment_pairs: usize,
    certified_predicate_rows: usize,
) -> PlanarBooleanEventPredicateBindingCounters {
    PlanarBooleanEventPredicateBindingCounters::new(
        pair_enumeration.candidate_rows().len(),
        segment_receipts.len(),
        bound_segment_pairs,
        certified_predicate_rows,
    )
}

fn denial(
    kind: PlanarBooleanEventPredicateBindingDenialKind,
    reduced_pair_identity: impl Into<String>,
    segment_pair_identity: impl Into<String>,
    counters: PlanarBooleanEventPredicateBindingCounters,
    human_reason: impl Into<String>,
) -> PlanarBooleanEventPredicateBindingDenial {
    PlanarBooleanEventPredicateBindingDenial::new(
        kind,
        reduced_pair_identity,
        segment_pair_identity,
        counters,
        human_reason,
    )
}
