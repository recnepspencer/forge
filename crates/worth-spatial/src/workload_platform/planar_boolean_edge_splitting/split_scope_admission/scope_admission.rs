use super::counters::PlanarBooleanEdgeSplitScopeAdmissionCounters;
use super::denial::PlanarBooleanEdgeSplitScopeAdmissionDenial;
use super::identity::split_scope_admission_identity;
use super::input::PlanarBooleanEdgeSplitScopeAdmissionInput;
use super::policy::{
    PlanarBooleanEdgeSplitDegeneracyPolicy, PlanarBooleanEdgeSplitDeterminismPolicy,
    PlanarBooleanEdgeSplitOverlapPolicy,
};
use super::policy_outcome::PlanarBooleanEdgeSplitPolicyOutcome;
use super::scope_class::PlanarBooleanEdgeSplitScopeClass;
use super::validation::classify_edge_split_scope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitScopeAdmission {
    scope_admission_identity: String,
    split_request_identity: String,
    event_ledger_identity: String,
    downstream_consumption_identity: String,
    reduced_pair_identity: String,
    segment_carrier_set_identity: String,
    candidate_index_product_identity: String,
    query_index_plan_digest: String,
    scope_class: PlanarBooleanEdgeSplitScopeClass,
    degeneracy_policy: PlanarBooleanEdgeSplitDegeneracyPolicy,
    determinism_policy: PlanarBooleanEdgeSplitDeterminismPolicy,
    overlap_policy: PlanarBooleanEdgeSplitOverlapPolicy,
    policy_outcome: PlanarBooleanEdgeSplitPolicyOutcome,
    counters: PlanarBooleanEdgeSplitScopeAdmissionCounters,
}

impl PlanarBooleanEdgeSplitScopeAdmission {
    pub fn admit(
        input: PlanarBooleanEdgeSplitScopeAdmissionInput<'_>,
    ) -> Result<Self, PlanarBooleanEdgeSplitScopeAdmissionDenial> {
        let scope_class = classify_edge_split_scope(&input)?;
        let split_request = input.split_request();
        let request_counters = split_request.counters();
        let admission = Self {
            scope_admission_identity: String::new(),
            split_request_identity: split_request.split_request_identity().to_string(),
            event_ledger_identity: split_request.event_ledger_identity().to_string(),
            downstream_consumption_identity: split_request
                .downstream_consumption_identity()
                .to_string(),
            reduced_pair_identity: split_request.reduced_pair_identity().to_string(),
            segment_carrier_set_identity: split_request.segment_carrier_set_identity().to_string(),
            candidate_index_product_identity: split_request
                .candidate_index_product_identity()
                .to_string(),
            query_index_plan_digest: split_request.query_index_plan_digest().to_string(),
            scope_class,
            degeneracy_policy: input.degeneracy_policy(),
            determinism_policy: input.determinism_policy(),
            overlap_policy: input.overlap_policy(),
            policy_outcome: PlanarBooleanEdgeSplitPolicyOutcome::admitted(
                split_request.event_ledger_identity(),
                split_request.split_request_identity(),
            ),
            counters: PlanarBooleanEdgeSplitScopeAdmissionCounters::new(
                request_counters.segment_carrier_count(),
                request_counters.point_event_count(),
                request_counters.interval_event_count(),
                request_counters.event_group_count(),
                1,
            ),
        };
        Ok(Self {
            scope_admission_identity: split_scope_admission_identity(&admission),
            ..admission
        })
    }

    pub fn scope_admission_identity(&self) -> &str {
        &self.scope_admission_identity
    }

    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
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

    pub fn segment_carrier_set_identity(&self) -> &str {
        &self.segment_carrier_set_identity
    }

    pub fn candidate_index_product_identity(&self) -> &str {
        &self.candidate_index_product_identity
    }

    pub fn query_index_plan_digest(&self) -> &str {
        &self.query_index_plan_digest
    }

    pub fn scope_class(&self) -> PlanarBooleanEdgeSplitScopeClass {
        self.scope_class
    }

    pub fn degeneracy_policy(&self) -> PlanarBooleanEdgeSplitDegeneracyPolicy {
        self.degeneracy_policy
    }

    pub fn determinism_policy(&self) -> PlanarBooleanEdgeSplitDeterminismPolicy {
        self.determinism_policy
    }

    pub fn overlap_policy(&self) -> PlanarBooleanEdgeSplitOverlapPolicy {
        self.overlap_policy
    }

    pub fn policy_outcome(&self) -> &PlanarBooleanEdgeSplitPolicyOutcome {
        &self.policy_outcome
    }

    pub fn counters(&self) -> PlanarBooleanEdgeSplitScopeAdmissionCounters {
        self.counters
    }
}
