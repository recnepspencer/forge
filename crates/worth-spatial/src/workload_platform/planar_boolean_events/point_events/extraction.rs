use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanDeduplicatedPointEventSet, PlanarBooleanEventClassifierInput,
    PlanarBooleanEventPredicateBinding, PlanarBooleanPointEventExtractionCounters,
    PlanarBooleanPointEventKind,
};

use super::contact_classification::{
    classify_point_contact, denial_for_kind, PointContactClassification,
};
use super::denial::{
    PlanarBooleanPointEventExtractionDenial, PlanarBooleanPointEventExtractionDenialKind,
};
use super::event::PlanarBooleanPointEvent;
use super::identity::extraction_identity;

pub struct PlanarBooleanPointEventExtraction;

#[derive(Clone, Debug)]
pub struct PlanarBooleanPointEventExtractionPlan<'a> {
    predicate_binding: &'a PlanarBooleanEventPredicateBinding,
}

#[derive(Clone, Debug)]
pub struct PlanarBooleanPointEventExtractionCompiledPlan<'a> {
    predicate_binding: &'a PlanarBooleanEventPredicateBinding,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanPointEventExtractionReceipt {
    predicate_binding_identity: String,
    point_events: Vec<PlanarBooleanPointEvent>,
    counters: PlanarBooleanPointEventExtractionCounters,
    extraction_identity: String,
}

impl PlanarBooleanPointEventExtraction {
    pub fn from_predicate_binding(
        predicate_binding: &PlanarBooleanEventPredicateBinding,
    ) -> PlanarBooleanPointEventExtractionPlan<'_> {
        PlanarBooleanPointEventExtractionPlan { predicate_binding }
    }
}

impl<'a> PlanarBooleanPointEventExtractionPlan<'a> {
    pub fn required_bound_pairs(&self) -> usize {
        self.predicate_binding.bound_pairs().len()
    }

    pub fn compile(
        self,
    ) -> Result<
        PlanarBooleanPointEventExtractionCompiledPlan<'a>,
        PlanarBooleanPointEventExtractionDenial,
    > {
        if self
            .predicate_binding
            .predicate_binding_identity()
            .is_empty()
        {
            return Err(PlanarBooleanPointEventExtractionDenial::new(
                PlanarBooleanPointEventExtractionDenialKind::MissingPredicateBindingIdentity,
                "",
                "",
                PlanarBooleanPointEventExtractionCounters::default(),
                "point-event extraction requires a certified predicate binding identity",
            ));
        }
        Ok(PlanarBooleanPointEventExtractionCompiledPlan {
            predicate_binding: self.predicate_binding,
        })
    }
}

impl PlanarBooleanPointEventExtractionCompiledPlan<'_> {
    pub fn certify(
        self,
    ) -> Result<PlanarBooleanPointEventExtractionReceipt, PlanarBooleanPointEventExtractionDenial>
    {
        let mut counters = PlanarBooleanPointEventExtractionCounters::default();
        let mut point_events = Vec::new();
        for bound_pair in self.predicate_binding.bound_pairs() {
            counters.inspect_bound_pair();
            let input = PlanarBooleanEventClassifierInput::from_predicate_bound_pair(bound_pair);
            match classify_point_contact(input) {
                Ok(PointContactClassification::Emit(event)) => {
                    counters.candidate_point_relation();
                    if event.kind() == PlanarBooleanPointEventKind::SharedEndpoint {
                        counters.shared_endpoint_candidate();
                    }
                    counters.emitted_point_event();
                    point_events.push(*event);
                }
                Ok(PointContactClassification::SkipNonPoint) => {
                    counters.skipped_non_point_relation();
                }
                Err(kind) => {
                    if kind
                        == PlanarBooleanPointEventExtractionDenialKind::AmbiguousPredicateRelation
                    {
                        counters.ambiguous_relation();
                    }
                    return Err(denial_for_kind(kind, input, counters));
                }
            }
        }
        let deduplicated = PlanarBooleanDeduplicatedPointEventSet::from_point_reports(point_events);
        counters
            .suppress_duplicate_point_reports(deduplicated.duplicate_point_reports_suppressed());
        counters
            .detect_high_valence_point_groups(deduplicated.high_valence_point_groups_detected());
        let point_events = deduplicated.point_events();
        for event in &point_events {
            if event.kind() == PlanarBooleanPointEventKind::SharedEndpoint {
                counters.emitted_shared_endpoint_event();
            }
        }
        let extraction_identity = extraction_identity(
            self.predicate_binding.predicate_binding_identity(),
            point_events
                .iter()
                .map(PlanarBooleanPointEvent::event_identity),
        );
        Ok(PlanarBooleanPointEventExtractionReceipt {
            predicate_binding_identity: self
                .predicate_binding
                .predicate_binding_identity()
                .to_string(),
            point_events,
            counters,
            extraction_identity,
        })
    }
}

impl PlanarBooleanPointEventExtractionReceipt {
    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn point_events(&self) -> &[PlanarBooleanPointEvent] {
        &self.point_events
    }

    pub fn counters(&self) -> PlanarBooleanPointEventExtractionCounters {
        self.counters
    }

    pub fn extraction_identity(&self) -> &str {
        &self.extraction_identity
    }
}
