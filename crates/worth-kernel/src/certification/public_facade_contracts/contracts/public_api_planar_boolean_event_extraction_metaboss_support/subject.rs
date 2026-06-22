use worth_kernel::workload_composition::{
    BuiltBooleanOperandPairRecipe, PlanarBooleanEventExtractionRequest,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCollinearRelationExtraction, PlanarBooleanEventExtractionPhaseStop,
    PlanarBooleanEventLedgerReceipt, PlanarBooleanEventPredicateBinding,
    PlanarBooleanIntervalEventExtraction, PlanarBooleanPointEventExtraction,
};

use super::event_ledger_support::{ledger_from_certified_inputs, CertifiedEventLedgerInputs};
use super::expected_shape::MetabossExpectedLedgerShape;
use super::predicate_binding_support::{self, BindingSubject};

pub(crate) struct MetabossEventExtractionSubject {
    pair: BuiltBooleanOperandPairRecipe,
    inputs: CertifiedEventLedgerInputs,
    ledger: PlanarBooleanEventLedgerReceipt,
    policy_stop: PlanarBooleanEventExtractionPhaseStop,
    expected: MetabossExpectedLedgerShape,
}

impl MetabossEventExtractionSubject {
    pub(crate) fn certify(readiness_scope: &'static str) -> Self {
        let binding_subject = predicate_binding_support::metaboss_binding_subject(readiness_scope);
        Self::from_binding_subject(binding_subject)
    }

    #[allow(dead_code)]
    pub(crate) fn certify_event_carrier(readiness_scope: &'static str) -> Self {
        let binding_subject = predicate_binding_support::binding_subject(readiness_scope);
        Self::from_binding_subject(binding_subject)
    }

    fn from_binding_subject(binding_subject: BindingSubject) -> Self {
        let expected = MetabossExpectedLedgerShape::new();
        let event_request = PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(
            binding_subject.reduced_pair.clone(),
        );
        let carriers = binding_subject
            .reduced_pair
            .segment_carrier_set()
            .expect("metaboss segment carriers should certify");
        let binding = PlanarBooleanEventPredicateBinding::plan(&binding_subject.pair_worklist)
            .for_reduced_pair(binding_subject.reduced_pair_identity.clone())
            .with_segment_segment_receipts(binding_subject.segment_receipts)
            .with_predicate_consumption_receipt(binding_subject.predicate_consumption)
            .compile()
            .expect("metaboss predicate binding plan should compile")
            .certify()
            .expect("metaboss predicate binding should certify");
        let point_events = PlanarBooleanPointEventExtraction::from_predicate_binding(&binding)
            .compile()
            .expect("metaboss point-event extraction plan should compile")
            .certify()
            .expect("metaboss point events should certify");
        let collinear_relations =
            PlanarBooleanCollinearRelationExtraction::from_predicate_binding(&binding)
                .compile()
                .expect("metaboss collinear relation plan should compile")
                .certify()
                .expect("metaboss collinear relations should certify");
        let interval_events =
            PlanarBooleanIntervalEventExtraction::from_collinear_relations(&collinear_relations)
                .compile()
                .expect("metaboss interval-event extraction plan should compile")
                .certify()
                .expect("metaboss interval events should certify");
        let policy_stop = policy_stop_from_binding(&binding);
        let inputs = CertifiedEventLedgerInputs {
            event_request,
            carriers,
            pair_worklist: binding_subject.pair_worklist,
            binding,
            point_events,
            collinear_relations,
            interval_events,
        };
        let ledger = ledger_from_certified_inputs(&inputs);
        Self {
            pair: binding_subject.pair,
            inputs,
            ledger,
            policy_stop,
            expected,
        }
    }

    pub(crate) fn pair(&self) -> &BuiltBooleanOperandPairRecipe {
        &self.pair
    }

    pub(crate) fn inputs(&self) -> &CertifiedEventLedgerInputs {
        &self.inputs
    }

    pub(crate) fn ledger(&self) -> &PlanarBooleanEventLedgerReceipt {
        &self.ledger
    }

    pub(crate) fn policy_stop(&self) -> &PlanarBooleanEventExtractionPhaseStop {
        &self.policy_stop
    }

    pub(crate) fn expected(&self) -> &MetabossExpectedLedgerShape {
        &self.expected
    }
}

fn policy_stop_from_binding(
    binding: &PlanarBooleanEventPredicateBinding,
) -> PlanarBooleanEventExtractionPhaseStop {
    let bound_pair = binding
        .bound_pairs()
        .iter()
        .find(|pair| pair.segment_pair_identity().contains("pair"))
        .or_else(|| binding.bound_pairs().first())
        .expect("metaboss binding must expose a bound pair");
    PlanarBooleanEventExtractionPhaseStop::policy_exit_for_collinear_overlap(
        binding,
        bound_pair,
        "metaboss closeout includes typed policy-exit evidence for 7.3 handoff",
    )
    .expect("metaboss policy exit should preserve binding provenance")
}
