use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};
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
use super::reduced_pair_support;
use worth_kernel::workload_composition::trace_scope;

#[derive(Clone)]
pub(crate) struct MetabossEventExtractionSubject {
    pair: BuiltBooleanOperandPairRecipe,
    inputs: CertifiedEventLedgerInputs,
    ledger: PlanarBooleanEventLedgerReceipt,
    policy_stop: PlanarBooleanEventExtractionPhaseStop,
    expected: MetabossExpectedLedgerShape,
}

impl MetabossEventExtractionSubject {
    pub(crate) fn certify(readiness_scope: &'static str) -> Self {
        cached_subject(("metaboss", readiness_scope), || {
            let binding_subject = trace_scope("metaboss_binding_subject", || {
                let (pair, operand_a, operand_b) = trace_scope(
                    "binding_subject_metaboss_operands",
                    || {
                        reduced_pair_support::metaboss_projected_operand_requests_from_catalog(
                            readiness_scope,
                        )
                    },
                );
                predicate_binding_support::binding_subject_from_projected_operands(
                    readiness_scope,
                    pair,
                    operand_a,
                    operand_b,
                )
            });
            Self::from_binding_subject(binding_subject)
        })
    }

    pub(crate) fn certify_from_pair(
        readiness_scope: &'static str,
        pair: BuiltBooleanOperandPairRecipe,
    ) -> Self {
        let cache_key = format!(
            "custom:{}:{}",
            readiness_scope,
            pair.operand_pair_identity()
        );
        cached_subject_from_owned_key(cache_key, || {
            let binding_subject = trace_scope("custom_binding_subject", || {
                let (pair, operand_a, operand_b) = trace_scope(
                    "binding_subject_custom_pair_operands",
                    || {
                        reduced_pair_support::projected_operand_requests_from_pair(
                            readiness_scope,
                            pair,
                        )
                    },
                );
                predicate_binding_support::binding_subject_from_projected_operands(
                    readiness_scope,
                    pair,
                    operand_a,
                    operand_b,
                )
            });
            Self::from_binding_subject(binding_subject)
        })
    }

    pub(crate) fn certify_event_carrier(readiness_scope: &'static str) -> Self {
        cached_subject(("event-carrier", readiness_scope), || {
            let binding_subject = trace_scope("event_carrier_binding_subject", || {
                predicate_binding_support::binding_subject(readiness_scope)
            });
            Self::from_binding_subject(binding_subject)
        })
    }

    fn from_binding_subject(binding_subject: BindingSubject) -> Self {
        trace_scope("event_subject_from_binding_subject", || {
            let expected = MetabossExpectedLedgerShape::new();
            let event_request = trace_scope("event_subject_request", || {
                PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(
                    binding_subject.reduced_pair.clone(),
                )
            });
            let carriers = trace_scope("event_subject_segment_carriers", || {
                binding_subject
                    .reduced_pair
                    .segment_carrier_set()
                    .expect("metaboss segment carriers should certify")
            });
            let binding = trace_scope("event_subject_predicate_binding", || {
                PlanarBooleanEventPredicateBinding::plan(&binding_subject.pair_worklist)
                    .for_reduced_pair(binding_subject.reduced_pair_identity.clone())
                    .with_segment_segment_receipts(binding_subject.segment_receipts)
                    .with_predicate_consumption_receipt(binding_subject.predicate_consumption)
                    .compile()
                    .expect("metaboss predicate binding plan should compile")
                    .certify()
                    .expect("metaboss predicate binding should certify")
            });
            let point_events = trace_scope("event_subject_point_events", || {
                PlanarBooleanPointEventExtraction::from_predicate_binding(&binding)
                    .compile()
                    .expect("metaboss point-event extraction plan should compile")
                    .certify()
                    .expect("metaboss point events should certify")
            });
            let collinear_relations = trace_scope("event_subject_collinear_relations", || {
                PlanarBooleanCollinearRelationExtraction::from_predicate_binding(&binding)
                    .compile()
                    .expect("metaboss collinear relation plan should compile")
                    .certify()
                    .expect("metaboss collinear relations should certify")
            });
            let interval_events = trace_scope("event_subject_interval_events", || {
                PlanarBooleanIntervalEventExtraction::from_collinear_relations(&collinear_relations)
                    .compile()
                    .expect("metaboss interval-event extraction plan should compile")
                    .certify()
                    .expect("metaboss interval events should certify")
            });
            let policy_stop = trace_scope("event_subject_policy_stop", || {
                policy_stop_from_binding(&binding)
            });
            let inputs = CertifiedEventLedgerInputs {
                event_request,
                carriers,
                pair_worklist: binding_subject.pair_worklist,
                binding,
                point_events,
                collinear_relations,
                interval_events,
            };
            let ledger = trace_scope("event_subject_ledger", || {
                ledger_from_certified_inputs(&inputs)
            });
            Self {
                pair: binding_subject.pair,
                inputs,
                ledger,
                policy_stop,
                expected,
            }
        })
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

fn cached_subject(
    key: (&'static str, &'static str),
    build: impl FnOnce() -> MetabossEventExtractionSubject,
) -> MetabossEventExtractionSubject {
    cached_subject_from_owned_key(format!("{}::{}", key.0, key.1), build)
}

fn cached_subject_from_owned_key(
    key: String,
    build: impl FnOnce() -> MetabossEventExtractionSubject,
) -> MetabossEventExtractionSubject {
    if let Some(subject) = certified_subject_cache()
        .lock()
        .expect("metaboss subject cache should not be poisoned")
        .get(&key)
        .cloned()
    {
        return subject;
    }

    let subject = build();
    let mut cache = certified_subject_cache()
        .lock()
        .expect("metaboss subject cache should not be poisoned");
    cache.entry(key).or_insert(subject).clone()
}

fn certified_subject_cache() -> &'static Mutex<BTreeMap<String, MetabossEventExtractionSubject>> {
    static CACHE: OnceLock<Mutex<BTreeMap<String, MetabossEventExtractionSubject>>> =
        OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(BTreeMap::new()))
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
