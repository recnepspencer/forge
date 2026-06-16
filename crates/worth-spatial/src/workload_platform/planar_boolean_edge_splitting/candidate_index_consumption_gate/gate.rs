use super::counters::PlanarBooleanCandidateIndexConsumptionCounters;
use super::denial::{
    PlanarBooleanCandidateIndexConsumptionDenial, PlanarBooleanCandidateIndexConsumptionDenialKind,
};
use super::identity::consumption_gate_identity;
use super::input::PlanarBooleanCandidateIndexConsumptionInput;
use super::validation::validate_candidate_index_consumption;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCandidateIndexConsumptionGate {
    gate_identity: String,
    event_ledger_identity: String,
    downstream_consumption_identity: String,
    reduced_pair_identity: String,
    segment_pair_enumeration_identity: String,
    candidate_index_product_identity: String,
    query_index_declaration_digest: String,
    query_index_plan_digest: String,
    query_index_envelope_digest: String,
    candidate_index_strategy: PlanarBooleanCandidateIndexStrategy,
    fallback_posture: PlanarBooleanCandidateIndexFallbackPosture,
    lifecycle_outcome: PlanarBooleanCandidateIndexLifecycleOutcome,
    counters: PlanarBooleanCandidateIndexConsumptionCounters,
}

impl PlanarBooleanCandidateIndexConsumptionGate {
    pub fn admit(
        input: PlanarBooleanCandidateIndexConsumptionInput<'_>,
    ) -> Result<Self, PlanarBooleanCandidateIndexConsumptionDenial> {
        validate_candidate_index_consumption(&input)?;
        let event_ledger = input.event_ledger();
        let segment_pair_enumeration = input.segment_pair_enumeration();
        let product = segment_pair_enumeration.candidate_index_product();
        let gate = Self {
            gate_identity: String::new(),
            event_ledger_identity: event_ledger.event_ledger_identity().to_string(),
            downstream_consumption_identity: event_ledger
                .downstream_consumption_identity()
                .to_string(),
            reduced_pair_identity: event_ledger.reduced_pair_identity().to_string(),
            segment_pair_enumeration_identity: segment_pair_enumeration
                .segment_pair_enumeration_identity()
                .to_string(),
            candidate_index_product_identity: segment_pair_enumeration
                .candidate_index_product_identity()
                .to_string(),
            query_index_declaration_digest: segment_pair_enumeration
                .query_index_declaration_digest()
                .to_string(),
            query_index_plan_digest: segment_pair_enumeration
                .query_index_plan_digest()
                .to_string(),
            query_index_envelope_digest: segment_pair_enumeration
                .query_index_envelope_digest()
                .to_string(),
            candidate_index_strategy: product.strategy(),
            fallback_posture: product.fallback_posture(),
            lifecycle_outcome: product.lifecycle_outcome(),
            counters: PlanarBooleanCandidateIndexConsumptionCounters::from_segment_pair_counters(
                segment_pair_enumeration.counters(),
            ),
        };
        Ok(Self {
            gate_identity: consumption_gate_identity(&gate),
            ..gate
        })
    }

    pub fn gate_identity(&self) -> &str {
        &self.gate_identity
    }

    pub fn event_ledger_identity(&self) -> &str {
        &self.event_ledger_identity
    }

    pub fn downstream_consumption_identity(&self) -> &str {
        &self.downstream_consumption_identity
    }

    pub fn reduced_pair_identity(&self) -> &str {
        &self.reduced_pair_identity
    }

    pub fn segment_pair_enumeration_identity(&self) -> &str {
        &self.segment_pair_enumeration_identity
    }

    pub fn candidate_index_product_identity(&self) -> &str {
        &self.candidate_index_product_identity
    }

    pub fn query_index_declaration_digest(&self) -> &str {
        &self.query_index_declaration_digest
    }

    pub fn query_index_plan_digest(&self) -> &str {
        &self.query_index_plan_digest
    }

    pub fn query_index_envelope_digest(&self) -> &str {
        &self.query_index_envelope_digest
    }

    pub fn candidate_index_strategy(&self) -> PlanarBooleanCandidateIndexStrategy {
        self.candidate_index_strategy
    }

    pub fn fallback_posture(&self) -> PlanarBooleanCandidateIndexFallbackPosture {
        self.fallback_posture
    }

    pub fn lifecycle_outcome(&self) -> PlanarBooleanCandidateIndexLifecycleOutcome {
        self.lifecycle_outcome
    }

    pub fn counters(&self) -> PlanarBooleanCandidateIndexConsumptionCounters {
        self.counters
    }

    pub fn certifies_production_candidate_discovery(&self) -> bool {
        self.fallback_posture == PlanarBooleanCandidateIndexFallbackPosture::NotUsed
            && self.lifecycle_outcome == PlanarBooleanCandidateIndexLifecycleOutcome::Bound
            && !self.counters.fallback_used()
            && self.counters.indexed_candidate_pair_count() == self.counters.emitted_pair_count()
            && self
                .counters
                .indexed_candidate_pair_count()
                .saturating_add(self.counters.culled_pair_count())
                == self.counters.expected_pair_breadth()
    }
}

pub(crate) fn non_production_fallback_denial(
    evidence_identity: impl Into<String>,
) -> PlanarBooleanCandidateIndexConsumptionDenial {
    PlanarBooleanCandidateIndexConsumptionDenial::new(
        PlanarBooleanCandidateIndexConsumptionDenialKind::NonProductionCandidateIndexFallback,
        evidence_identity,
        "candidate-index consumption requires Query-owned production discovery without full-breadth fallback",
    )
}

pub(crate) fn unsupported_lifecycle_denial(
    evidence_identity: impl Into<String>,
) -> PlanarBooleanCandidateIndexConsumptionDenial {
    PlanarBooleanCandidateIndexConsumptionDenial::new(
        PlanarBooleanCandidateIndexConsumptionDenialKind::UnsupportedCandidateIndexLifecycleOutcome,
        evidence_identity,
        "candidate-index consumption requires a bound Query-owned candidate-index product lifecycle outcome",
    )
}
