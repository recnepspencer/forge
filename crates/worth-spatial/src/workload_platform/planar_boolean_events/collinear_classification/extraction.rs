use worth_math::sign::TriSign;

use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DClassification;
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventPredicateBinding, PlanarBooleanPointEventCoordinateFact,
    PlanarBooleanPredicateBoundPair,
};

use super::counters::PlanarBooleanCollinearRelationCounters;
use super::denial::{
    PlanarBooleanCollinearRelationDenial, PlanarBooleanCollinearRelationDenialKind,
};
use super::identity::receipt_identity;
use super::interval_basis::PlanarBooleanCollinearIntervalBasis;
use super::overlap_parameterization::PlanarBooleanCollinearOverlapParameterization;
use super::relation::PlanarBooleanCollinearRelation;
use super::relation_kind::PlanarBooleanCollinearRelationKind;
use super::touch_point::PlanarBooleanCollinearTouchPoint;

pub struct PlanarBooleanCollinearRelationExtraction;

#[derive(Clone, Debug)]
pub struct PlanarBooleanCollinearRelationExtractionPlan<'a> {
    predicate_binding: &'a PlanarBooleanEventPredicateBinding,
}

#[derive(Clone, Debug)]
pub struct PlanarBooleanCollinearRelationExtractionCompiledPlan<'a> {
    predicate_binding: &'a PlanarBooleanEventPredicateBinding,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanCollinearRelationReceipt {
    predicate_binding_identity: String,
    relations: Vec<PlanarBooleanCollinearRelation>,
    counters: PlanarBooleanCollinearRelationCounters,
    receipt_identity: String,
}

impl PlanarBooleanCollinearRelationExtraction {
    pub fn from_predicate_binding(
        predicate_binding: &PlanarBooleanEventPredicateBinding,
    ) -> PlanarBooleanCollinearRelationExtractionPlan<'_> {
        PlanarBooleanCollinearRelationExtractionPlan { predicate_binding }
    }
}

impl<'a> PlanarBooleanCollinearRelationExtractionPlan<'a> {
    pub fn required_bound_pairs(&self) -> usize {
        self.predicate_binding.bound_pairs().len()
    }

    pub fn compile(
        self,
    ) -> Result<
        PlanarBooleanCollinearRelationExtractionCompiledPlan<'a>,
        PlanarBooleanCollinearRelationDenial,
    > {
        if self
            .predicate_binding
            .predicate_binding_identity()
            .is_empty()
        {
            return Err(PlanarBooleanCollinearRelationDenial::new(
                PlanarBooleanCollinearRelationDenialKind::MissingPredicateBindingIdentity,
                "",
                "",
                PlanarBooleanCollinearRelationCounters::default(),
                "collinear relation extraction requires a certified predicate binding identity",
            ));
        }
        Ok(PlanarBooleanCollinearRelationExtractionCompiledPlan {
            predicate_binding: self.predicate_binding,
        })
    }
}

impl PlanarBooleanCollinearRelationExtractionCompiledPlan<'_> {
    pub fn certify(
        self,
    ) -> Result<PlanarBooleanCollinearRelationReceipt, PlanarBooleanCollinearRelationDenial> {
        let mut counters = PlanarBooleanCollinearRelationCounters::default();
        let mut relations = Vec::new();
        for bound_pair in self.predicate_binding.bound_pairs() {
            counters.inspect_bound_pair();
            match relation_for_bound_pair(bound_pair, &mut counters) {
                Ok(Some(relation)) => relations.push(relation),
                Ok(None) => counters.skip_non_collinear_pair(),
                Err(denial) => return Err(denial),
            }
        }
        relations.sort_by(|left, right| left.relation_identity().cmp(right.relation_identity()));
        let receipt_identity = receipt_identity(
            self.predicate_binding.predicate_binding_identity(),
            relations
                .iter()
                .map(PlanarBooleanCollinearRelation::relation_identity),
        );
        Ok(PlanarBooleanCollinearRelationReceipt {
            predicate_binding_identity: self
                .predicate_binding
                .predicate_binding_identity()
                .to_string(),
            relations,
            counters,
            receipt_identity,
        })
    }
}

impl PlanarBooleanCollinearRelationReceipt {
    #[cfg(test)]
    pub(crate) fn from_interval_event_test_relations(
        relations: Vec<PlanarBooleanCollinearRelation>,
    ) -> Self {
        Self {
            predicate_binding_identity: "test-predicate-binding".to_string(),
            relations,
            counters: PlanarBooleanCollinearRelationCounters::default(),
            receipt_identity: "test-collinear-relation-receipt".to_string(),
        }
    }

    pub fn predicate_binding_identity(&self) -> &str {
        &self.predicate_binding_identity
    }

    pub fn relations(&self) -> &[PlanarBooleanCollinearRelation] {
        &self.relations
    }

    pub fn counters(&self) -> PlanarBooleanCollinearRelationCounters {
        self.counters
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }
}

fn relation_for_bound_pair(
    bound_pair: &PlanarBooleanPredicateBoundPair,
    counters: &mut PlanarBooleanCollinearRelationCounters,
) -> Result<Option<PlanarBooleanCollinearRelation>, PlanarBooleanCollinearRelationDenial> {
    if !is_collinear_bound_pair(bound_pair) {
        return Ok(None);
    }
    let relation = match bound_pair.classification() {
        CertifiedSegmentSegment2DClassification::CollinearDisjoint => {
            counters.emit_disjoint_relation();
            PlanarBooleanCollinearRelation::new(
                PlanarBooleanCollinearRelationKind::Disjoint,
                bound_pair,
                None,
                None,
            )
        }
        CertifiedSegmentSegment2DClassification::EndpointTouch => {
            let touch = touch_point(bound_pair, counters)?;
            counters.emit_endpoint_touch_relation();
            PlanarBooleanCollinearRelation::new(
                PlanarBooleanCollinearRelationKind::EndpointTouch,
                bound_pair,
                None,
                Some(touch),
            )
        }
        CertifiedSegmentSegment2DClassification::CollinearOverlap => {
            overlap_relation(bound_pair, counters)?
        }
        CertifiedSegmentSegment2DClassification::Identical => {
            counters.emit_identical_same_direction_relation();
            identical_relation(
                bound_pair,
                PlanarBooleanCollinearRelationKind::IdenticalSameDirection,
            )?
        }
        CertifiedSegmentSegment2DClassification::ReverseIdentical => {
            counters.emit_identical_anti_parallel_relation();
            identical_relation(
                bound_pair,
                PlanarBooleanCollinearRelationKind::IdenticalAntiParallel,
            )?
        }
        _ => return Ok(None),
    };
    Ok(Some(relation))
}

fn overlap_relation(
    bound_pair: &PlanarBooleanPredicateBoundPair,
    counters: &mut PlanarBooleanCollinearRelationCounters,
) -> Result<PlanarBooleanCollinearRelation, PlanarBooleanCollinearRelationDenial> {
    let interval =
        PlanarBooleanCollinearOverlapParameterization::from_bound_pair(bound_pair, counters)?;
    let kind = if interval.contains_one_segment() {
        counters.emit_containment_overlap_relation();
        PlanarBooleanCollinearRelationKind::ContainmentOverlap
    } else {
        counters.emit_partial_overlap_relation();
        PlanarBooleanCollinearRelationKind::PartialOverlap
    };
    Ok(PlanarBooleanCollinearRelation::new(
        kind,
        bound_pair,
        Some(interval.into_interval_basis()),
        None,
    ))
}

fn identical_relation(
    bound_pair: &PlanarBooleanPredicateBoundPair,
    kind: PlanarBooleanCollinearRelationKind,
) -> Result<PlanarBooleanCollinearRelation, PlanarBooleanCollinearRelationDenial> {
    let right_source_range = if kind == PlanarBooleanCollinearRelationKind::IdenticalAntiParallel {
        [1.0, 0.0]
    } else {
        [0.0, 1.0]
    };
    let interval =
        PlanarBooleanCollinearIntervalBasis::from_source_ranges([0.0, 1.0], right_source_range);
    debug_assert!(kind.has_interval_basis());
    Ok(PlanarBooleanCollinearRelation::new(
        kind,
        bound_pair,
        Some(interval),
        None,
    ))
}

fn is_collinear_bound_pair(bound_pair: &PlanarBooleanPredicateBoundPair) -> bool {
    bound_pair
        .segment_basis()
        .orientation_signs()
        .iter()
        .all(|sign| *sign == TriSign::Zero)
}

fn touch_point(
    bound_pair: &PlanarBooleanPredicateBoundPair,
    counters: &mut PlanarBooleanCollinearRelationCounters,
) -> Result<PlanarBooleanCollinearTouchPoint, PlanarBooleanCollinearRelationDenial> {
    let interval =
        PlanarBooleanCollinearOverlapParameterization::from_bound_pair(bound_pair, counters)?;
    let left_parameter = interval.overlap_start_on_left();
    let right_parameter = interval.overlap_start_on_right();
    let point = point_on_first_segment(bound_pair, left_parameter);
    let coordinate_fact = PlanarBooleanPointEventCoordinateFact::new(
        point,
        bound_pair.local_frame_identity(),
        bound_pair.precision_basis_identity(),
    );
    Ok(PlanarBooleanCollinearTouchPoint::new(
        coordinate_fact,
        left_parameter,
        right_parameter,
    ))
}

fn point_on_first_segment(
    bound_pair: &PlanarBooleanPredicateBoundPair,
    parameter: f64,
) -> [f64; 2] {
    let basis = bound_pair.segment_basis();
    let start = basis.first_start_point_2d();
    let end = basis.first_end_point_2d();
    [
        start[0] + (end[0] - start[0]) * parameter,
        start[1] + (end[1] - start[1]) * parameter,
    ]
}
