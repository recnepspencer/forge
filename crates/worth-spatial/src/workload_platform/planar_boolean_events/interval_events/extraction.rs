use crate::workload_platform::planar_boolean_events::{
    interval_has_collapsed, normalized_parameter_range,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCollinearIntervalBasis, PlanarBooleanCollinearRelation,
    PlanarBooleanCollinearRelationKind, PlanarBooleanCollinearRelationReceipt,
};

use super::counters::PlanarBooleanIntervalEventExtractionCounters;
use super::denial::{
    PlanarBooleanIntervalEventExtractionDenial, PlanarBooleanIntervalEventExtractionDenialKind,
};
use super::event::PlanarBooleanIntervalEvent;
use super::event_kind::PlanarBooleanIntervalEventKind;
use super::identity::extraction_identity;
use super::normalized_interval::PlanarBooleanNormalizedInterval;
use super::source_interval::PlanarBooleanSourceInterval;

pub struct PlanarBooleanIntervalEventExtraction;

#[derive(Clone, Debug)]
pub struct PlanarBooleanIntervalEventExtractionPlan<'a> {
    collinear_relations: &'a PlanarBooleanCollinearRelationReceipt,
}

#[derive(Clone, Debug)]
pub struct PlanarBooleanIntervalEventExtractionCompiledPlan<'a> {
    collinear_relations: &'a PlanarBooleanCollinearRelationReceipt,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanIntervalEventExtractionReceipt {
    collinear_relation_receipt_identity: String,
    interval_events: Vec<PlanarBooleanIntervalEvent>,
    counters: PlanarBooleanIntervalEventExtractionCounters,
    extraction_identity: String,
}

impl PlanarBooleanIntervalEventExtraction {
    pub fn from_collinear_relations(
        collinear_relations: &PlanarBooleanCollinearRelationReceipt,
    ) -> PlanarBooleanIntervalEventExtractionPlan<'_> {
        PlanarBooleanIntervalEventExtractionPlan {
            collinear_relations,
        }
    }
}

impl<'a> PlanarBooleanIntervalEventExtractionPlan<'a> {
    pub fn required_collinear_relations(&self) -> usize {
        self.collinear_relations.relations().len()
    }

    pub fn compile(
        self,
    ) -> Result<
        PlanarBooleanIntervalEventExtractionCompiledPlan<'a>,
        PlanarBooleanIntervalEventExtractionDenial,
    > {
        if self.collinear_relations.receipt_identity().is_empty() {
            return Err(PlanarBooleanIntervalEventExtractionDenial::new(
                PlanarBooleanIntervalEventExtractionDenialKind::MissingCollinearRelationReceiptIdentity,
                "",
                "",
                "",
                PlanarBooleanIntervalEventExtractionCounters::default(),
                "interval-event extraction requires a certified collinear relation receipt identity",
            ));
        }
        Ok(PlanarBooleanIntervalEventExtractionCompiledPlan {
            collinear_relations: self.collinear_relations,
        })
    }
}

impl PlanarBooleanIntervalEventExtractionCompiledPlan<'_> {
    pub fn certify(
        self,
    ) -> Result<
        PlanarBooleanIntervalEventExtractionReceipt,
        PlanarBooleanIntervalEventExtractionDenial,
    > {
        let mut counters = PlanarBooleanIntervalEventExtractionCounters::default();
        let mut interval_events = Vec::new();
        for relation in self.collinear_relations.relations() {
            counters.inspect_collinear_relation();
            match event_for_relation(
                self.collinear_relations.receipt_identity(),
                relation,
                &mut counters,
            )? {
                Some(event) => interval_events.push(event),
                None => {}
            }
        }
        interval_events.sort_by(|left, right| left.event_identity().cmp(right.event_identity()));
        let extraction_identity = extraction_identity(
            self.collinear_relations.receipt_identity(),
            interval_events
                .iter()
                .map(PlanarBooleanIntervalEvent::event_identity),
        );
        Ok(PlanarBooleanIntervalEventExtractionReceipt {
            collinear_relation_receipt_identity: self
                .collinear_relations
                .receipt_identity()
                .to_string(),
            interval_events,
            counters,
            extraction_identity,
        })
    }
}

impl PlanarBooleanIntervalEventExtractionReceipt {
    pub fn collinear_relation_receipt_identity(&self) -> &str {
        &self.collinear_relation_receipt_identity
    }

    pub fn interval_events(&self) -> &[PlanarBooleanIntervalEvent] {
        &self.interval_events
    }

    pub fn counters(&self) -> PlanarBooleanIntervalEventExtractionCounters {
        self.counters
    }

    pub fn extraction_identity(&self) -> &str {
        &self.extraction_identity
    }
}

fn event_for_relation(
    receipt_identity: &str,
    relation: &PlanarBooleanCollinearRelation,
    counters: &mut PlanarBooleanIntervalEventExtractionCounters,
) -> Result<Option<PlanarBooleanIntervalEvent>, PlanarBooleanIntervalEventExtractionDenial> {
    match relation.kind() {
        PlanarBooleanCollinearRelationKind::Disjoint => {
            counters.skip_disjoint_relation();
            Ok(None)
        }
        PlanarBooleanCollinearRelationKind::EndpointTouch => {
            counters.skip_endpoint_touch_relation();
            Ok(None)
        }
        PlanarBooleanCollinearRelationKind::PartialOverlap => interval_event_for_interval_relation(
            receipt_identity,
            PlanarBooleanIntervalEventKind::PartialOverlap,
            relation,
            counters,
        )
        .map(Some),
        PlanarBooleanCollinearRelationKind::ContainmentOverlap => {
            interval_event_for_interval_relation(
                receipt_identity,
                PlanarBooleanIntervalEventKind::ContainmentOverlap,
                relation,
                counters,
            )
            .map(Some)
        }
        PlanarBooleanCollinearRelationKind::IdenticalSameDirection => {
            interval_event_for_interval_relation(
                receipt_identity,
                PlanarBooleanIntervalEventKind::IdenticalSameDirection,
                relation,
                counters,
            )
            .map(Some)
        }
        PlanarBooleanCollinearRelationKind::IdenticalAntiParallel => {
            interval_event_for_interval_relation(
                receipt_identity,
                PlanarBooleanIntervalEventKind::IdenticalAntiParallel,
                relation,
                counters,
            )
            .map(Some)
        }
    }
}

fn interval_event_for_interval_relation(
    receipt_identity: &str,
    event_kind: PlanarBooleanIntervalEventKind,
    relation: &PlanarBooleanCollinearRelation,
    counters: &mut PlanarBooleanIntervalEventExtractionCounters,
) -> Result<PlanarBooleanIntervalEvent, PlanarBooleanIntervalEventExtractionDenial> {
    let interval_basis = required_interval_basis(receipt_identity, relation, counters)?;
    let event = build_interval_event(
        receipt_identity,
        event_kind,
        relation,
        interval_basis,
        counters,
    )?;
    counters.emit_interval_event(event_kind);
    Ok(event)
}

fn required_interval_basis<'a>(
    receipt_identity: &str,
    relation: &'a PlanarBooleanCollinearRelation,
    counters: &mut PlanarBooleanIntervalEventExtractionCounters,
) -> Result<&'a PlanarBooleanCollinearIntervalBasis, PlanarBooleanIntervalEventExtractionDenial> {
    relation.interval_basis().ok_or_else(|| {
        counters.missing_interval_basis_relation();
        PlanarBooleanIntervalEventExtractionDenial::new(
            PlanarBooleanIntervalEventExtractionDenialKind::MissingIntervalBasisForIntervalRelation,
            receipt_identity,
            relation.relation_identity(),
            relation.segment_pair_identity(),
            *counters,
            "interval-event extraction requires interval-bearing collinear relations",
        )
    })
}

fn build_interval_event(
    receipt_identity: &str,
    event_kind: PlanarBooleanIntervalEventKind,
    relation: &PlanarBooleanCollinearRelation,
    interval_basis: &PlanarBooleanCollinearIntervalBasis,
    counters: &mut PlanarBooleanIntervalEventExtractionCounters,
) -> Result<PlanarBooleanIntervalEvent, PlanarBooleanIntervalEventExtractionDenial> {
    let normalized_range = normalized_parameter_range(interval_basis);
    if interval_has_collapsed(normalized_range) {
        counters.collapsed_interval_denial();
        return Err(PlanarBooleanIntervalEventExtractionDenial::new(
            PlanarBooleanIntervalEventExtractionDenialKind::CollapsedIntervalAfterNormalization,
            receipt_identity,
            relation.relation_identity(),
            relation.segment_pair_identity(),
            *counters,
            "interval-event extraction rejects zero-width interval relations",
        ));
    }
    let normalized_interval = PlanarBooleanNormalizedInterval::new(
        normalized_range,
        relation.local_frame_identity(),
        relation.precision_basis_identity(),
    );
    let left_source_interval = PlanarBooleanSourceInterval::new(
        relation.left_segment_identity(),
        relation.left_carrier_identity(),
        interval_basis.left_source_parameter_range(),
    );
    let right_source_interval = PlanarBooleanSourceInterval::new(
        relation.right_segment_identity(),
        relation.right_carrier_identity(),
        interval_basis.right_source_parameter_range(),
    );
    Ok(PlanarBooleanIntervalEvent::new(
        event_kind,
        relation,
        normalized_interval,
        left_source_interval,
        right_source_interval,
    ))
}

#[cfg(test)]
mod tests {
    use crate::workload_platform::planar_boolean_events::{
        PlanarBooleanCollinearIntervalBasis, PlanarBooleanCollinearRelation,
        PlanarBooleanCollinearRelationKind, PlanarBooleanCollinearRelationReceipt,
    };

    use super::{
        PlanarBooleanIntervalEventExtraction, PlanarBooleanIntervalEventExtractionDenialKind,
    };

    #[test]
    fn collapsed_interval_after_normalization_denies_instead_of_becoming_overlap() {
        let collapsed_relation = PlanarBooleanCollinearRelation::from_interval_event_test_parts(
            PlanarBooleanCollinearRelationKind::PartialOverlap,
            Some(PlanarBooleanCollinearIntervalBasis::from_source_ranges(
                [0.5, 0.5],
                [0.0, -0.0],
            )),
        );
        let collinear_relations =
            PlanarBooleanCollinearRelationReceipt::from_interval_event_test_relations(vec![
                collapsed_relation,
            ]);

        let denial =
            PlanarBooleanIntervalEventExtraction::from_collinear_relations(&collinear_relations)
                .compile()
                .expect("test collinear relation receipt should compile")
                .certify()
                .expect_err("collapsed interval must deny before event construction");

        assert_eq!(
            denial.kind(),
            PlanarBooleanIntervalEventExtractionDenialKind::CollapsedIntervalAfterNormalization
        );
        assert_eq!(denial.counters().inspected_collinear_relations(), 1);
        assert_eq!(denial.counters().collapsed_interval_denials(), 1);
    }

    #[test]
    fn interval_relation_without_interval_basis_denies_before_event_construction() {
        let malformed_relation = PlanarBooleanCollinearRelation::from_interval_event_test_parts(
            PlanarBooleanCollinearRelationKind::ContainmentOverlap,
            None,
        );
        let collinear_relations =
            PlanarBooleanCollinearRelationReceipt::from_interval_event_test_relations(vec![
                malformed_relation,
            ]);

        let denial =
            PlanarBooleanIntervalEventExtraction::from_collinear_relations(&collinear_relations)
                .compile()
                .expect("test collinear relation receipt should compile")
                .certify()
                .expect_err("interval relation without interval basis must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanIntervalEventExtractionDenialKind::MissingIntervalBasisForIntervalRelation
        );
        assert_eq!(denial.counters().inspected_collinear_relations(), 1);
        assert_eq!(denial.counters().missing_interval_basis_relations(), 1);
    }
}
