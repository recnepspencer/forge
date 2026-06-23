use crate::workload_platform::planar_boolean_events::{
    group_interval_events, group_point_events, PlanarBooleanCollinearRelation,
    PlanarBooleanCollinearRelationKind, PlanarBooleanCollinearRelationReceipt,
    PlanarBooleanEventGroupingCounters, PlanarBooleanEventPredicateBinding,
    PlanarBooleanIntervalEventExtractionReceipt, PlanarBooleanPointEventExtractionReceipt,
    PlanarBooleanSegmentCarrierSet, PlanarBooleanSegmentPairEnumerationReceipt,
};

use super::counters::PlanarBooleanEventLedgerCounters;
use super::denial::{PlanarBooleanEventLedgerDenial, PlanarBooleanEventLedgerDenialKind};
use super::identity::{
    downstream_consumption_identity, event_ledger_identity, EventLedgerIdentityBasis,
};
use super::ordered_events::PlanarBooleanOrderedEventSet;
use super::receipt::{PlanarBooleanEventLedgerReceipt, PlanarBooleanEventLedgerReceiptInput};
use super::{validate_receipt_chain, EventLedgerReceiptChain};

pub struct PlanarBooleanEventLedger;

#[derive(Clone, Debug, Default)]
pub struct PlanarBooleanEventLedgerAssemblyPlan<'a> {
    reduced_pair_identity: String,
    event_extraction_request_identity: String,
    segment_carriers: Option<&'a PlanarBooleanSegmentCarrierSet>,
    segment_pair_enumeration: Option<&'a PlanarBooleanSegmentPairEnumerationReceipt>,
    predicate_binding: Option<&'a PlanarBooleanEventPredicateBinding>,
    point_events: Option<&'a PlanarBooleanPointEventExtractionReceipt>,
    collinear_relations: Option<&'a PlanarBooleanCollinearRelationReceipt>,
    interval_events: Option<&'a PlanarBooleanIntervalEventExtractionReceipt>,
}

#[derive(Clone, Debug)]
pub struct PlanarBooleanEventLedgerAssemblyCompiledPlan<'a> {
    reduced_pair_identity: String,
    event_extraction_request_identity: String,
    segment_carriers: &'a PlanarBooleanSegmentCarrierSet,
    segment_pair_enumeration: &'a PlanarBooleanSegmentPairEnumerationReceipt,
    predicate_binding: &'a PlanarBooleanEventPredicateBinding,
    point_events: &'a PlanarBooleanPointEventExtractionReceipt,
    collinear_relations: &'a PlanarBooleanCollinearRelationReceipt,
    interval_events: &'a PlanarBooleanIntervalEventExtractionReceipt,
}

impl PlanarBooleanEventLedger {
    pub fn assemble() -> PlanarBooleanEventLedgerAssemblyPlan<'static> {
        PlanarBooleanEventLedgerAssemblyPlan::default()
    }
}

impl<'a> PlanarBooleanEventLedgerAssemblyPlan<'a> {
    pub fn for_reduced_pair_identity(mut self, identity: impl Into<String>) -> Self {
        self.reduced_pair_identity = identity.into();
        self
    }

    pub fn for_event_extraction_request_identity(mut self, identity: impl Into<String>) -> Self {
        self.event_extraction_request_identity = identity.into();
        self
    }

    pub fn with_segment_carriers(
        mut self,
        segment_carriers: &'a PlanarBooleanSegmentCarrierSet,
    ) -> Self {
        self.segment_carriers = Some(segment_carriers);
        self
    }

    pub fn with_segment_pair_enumeration(
        mut self,
        segment_pair_enumeration: &'a PlanarBooleanSegmentPairEnumerationReceipt,
    ) -> Self {
        self.segment_pair_enumeration = Some(segment_pair_enumeration);
        self
    }

    pub fn with_predicate_binding(
        mut self,
        predicate_binding: &'a PlanarBooleanEventPredicateBinding,
    ) -> Self {
        self.predicate_binding = Some(predicate_binding);
        self
    }

    pub fn with_point_events(
        mut self,
        point_events: &'a PlanarBooleanPointEventExtractionReceipt,
    ) -> Self {
        self.point_events = Some(point_events);
        self
    }

    pub fn with_collinear_relations(
        mut self,
        collinear_relations: &'a PlanarBooleanCollinearRelationReceipt,
    ) -> Self {
        self.collinear_relations = Some(collinear_relations);
        self
    }

    pub fn with_interval_events(
        mut self,
        interval_events: &'a PlanarBooleanIntervalEventExtractionReceipt,
    ) -> Self {
        self.interval_events = Some(interval_events);
        self
    }

    pub fn compile(
        self,
    ) -> Result<PlanarBooleanEventLedgerAssemblyCompiledPlan<'a>, PlanarBooleanEventLedgerDenial>
    {
        if self.reduced_pair_identity.is_empty() {
            return Err(denial(
                PlanarBooleanEventLedgerDenialKind::MissingReducedPairIdentity,
                "",
                "event ledger assembly requires a reduced-pair identity",
            ));
        }
        if self.event_extraction_request_identity.is_empty() {
            return Err(denial(
                PlanarBooleanEventLedgerDenialKind::MissingEventExtractionRequestIdentity,
                "",
                "event ledger assembly requires an event extraction request identity",
            ));
        }
        let segment_carriers = require_receipt(
            self.segment_carriers,
            PlanarBooleanEventLedgerDenialKind::MissingSegmentCarrierSetIdentity,
            "event ledger assembly requires a segment-carrier set",
        )?;
        let segment_pair_enumeration = require_receipt(
            self.segment_pair_enumeration,
            PlanarBooleanEventLedgerDenialKind::MissingSegmentPairEnumerationIdentity,
            "event ledger assembly requires segment-pair enumeration",
        )?;
        let predicate_binding = require_receipt(
            self.predicate_binding,
            PlanarBooleanEventLedgerDenialKind::MissingPredicateBindingIdentity,
            "event ledger assembly requires predicate binding",
        )?;
        let point_events = require_receipt(
            self.point_events,
            PlanarBooleanEventLedgerDenialKind::MissingPointEventExtractionIdentity,
            "event ledger assembly requires point-event extraction",
        )?;
        let collinear_relations = require_receipt(
            self.collinear_relations,
            PlanarBooleanEventLedgerDenialKind::MissingCollinearRelationReceiptIdentity,
            "event ledger assembly requires collinear relation extraction",
        )?;
        let interval_events = require_receipt(
            self.interval_events,
            PlanarBooleanEventLedgerDenialKind::MissingIntervalEventExtractionIdentity,
            "event ledger assembly requires interval-event extraction",
        )?;
        ensure_nonempty(
            segment_carriers.segment_carrier_set_identity(),
            PlanarBooleanEventLedgerDenialKind::MissingSegmentCarrierSetIdentity,
            "event ledger assembly requires a segment-carrier set identity",
        )?;
        ensure_nonempty(
            segment_pair_enumeration.segment_pair_enumeration_identity(),
            PlanarBooleanEventLedgerDenialKind::MissingSegmentPairEnumerationIdentity,
            "event ledger assembly requires a segment-pair enumeration identity",
        )?;
        ensure_nonempty(
            predicate_binding.predicate_binding_identity(),
            PlanarBooleanEventLedgerDenialKind::MissingPredicateBindingIdentity,
            "event ledger assembly requires a predicate-binding identity",
        )?;
        ensure_nonempty(
            point_events.extraction_identity(),
            PlanarBooleanEventLedgerDenialKind::MissingPointEventExtractionIdentity,
            "event ledger assembly requires a point-event extraction identity",
        )?;
        ensure_nonempty(
            collinear_relations.receipt_identity(),
            PlanarBooleanEventLedgerDenialKind::MissingCollinearRelationReceiptIdentity,
            "event ledger assembly requires a collinear relation receipt identity",
        )?;
        ensure_nonempty(
            interval_events.extraction_identity(),
            PlanarBooleanEventLedgerDenialKind::MissingIntervalEventExtractionIdentity,
            "event ledger assembly requires an interval-event extraction identity",
        )?;
        Ok(PlanarBooleanEventLedgerAssemblyCompiledPlan {
            reduced_pair_identity: self.reduced_pair_identity,
            event_extraction_request_identity: self.event_extraction_request_identity,
            segment_carriers,
            segment_pair_enumeration,
            predicate_binding,
            point_events,
            collinear_relations,
            interval_events,
        })
    }
}

impl PlanarBooleanEventLedgerAssemblyCompiledPlan<'_> {
    pub fn certify(
        self,
    ) -> Result<PlanarBooleanEventLedgerReceipt, PlanarBooleanEventLedgerDenial> {
        validate_receipt_chain(EventLedgerReceiptChain {
            reduced_pair_identity: &self.reduced_pair_identity,
            segment_pair_enumeration: self.segment_pair_enumeration,
            predicate_binding: self.predicate_binding,
            point_events: self.point_events,
            collinear_relations: self.collinear_relations,
            interval_events: self.interval_events,
        })?;
        let (point_groups, point_group_counters) =
            group_point_events(self.point_events.point_events());
        let (interval_groups, interval_group_counters) =
            group_interval_events(self.interval_events.interval_events());
        let mut grouping_counters = PlanarBooleanEventGroupingCounters::default();
        grouping_counters.merge(point_group_counters);
        grouping_counters.merge(interval_group_counters);
        let mut event_groups = point_groups;
        event_groups.extend(interval_groups);
        event_groups.sort_by(|left, right| left.group_identity().cmp(right.group_identity()));
        let relation_diagnostics = relation_diagnostics(self.collinear_relations);
        let relation_diagnostic_identities = relation_diagnostics
            .iter()
            .map(|relation| relation.relation_identity().to_string())
            .collect::<Vec<_>>();
        let ordered_events = PlanarBooleanOrderedEventSet::from_events_and_groups(
            self.point_events.point_events(),
            self.interval_events.interval_events(),
            &event_groups,
            relation_diagnostic_identities,
        );
        let counters = PlanarBooleanEventLedgerCounters::new(
            self.point_events.point_events().len(),
            self.interval_events.interval_events().len(),
            self.point_events.counters(),
            self.interval_events.counters(),
            grouping_counters,
            self.collinear_relations.relations().len(),
            ordered_events.relation_diagnostic_identities().len(),
        );
        let event_ledger_identity = event_ledger_identity(EventLedgerIdentityBasis {
            reduced_pair_identity: &self.reduced_pair_identity,
            event_extraction_request_identity: &self.event_extraction_request_identity,
            segment_carrier_set_identity: self.segment_carriers.segment_carrier_set_identity(),
            segment_pair_enumeration_identity: self
                .segment_pair_enumeration
                .segment_pair_enumeration_identity(),
            predicate_binding_identity: self.predicate_binding.predicate_binding_identity(),
            point_event_extraction_identity: self.point_events.extraction_identity(),
            collinear_relation_receipt_identity: self.collinear_relations.receipt_identity(),
            interval_event_extraction_identity: self.interval_events.extraction_identity(),
            ordered_events: &ordered_events,
        });
        let downstream_consumption_identity =
            downstream_consumption_identity(&event_ledger_identity, &ordered_events);
        Ok(PlanarBooleanEventLedgerReceipt::new(
            PlanarBooleanEventLedgerReceiptInput {
                reduced_pair_identity: self.reduced_pair_identity,
                event_extraction_request_identity: self.event_extraction_request_identity,
                segment_carrier_set_identity: self
                    .segment_carriers
                    .segment_carrier_set_identity()
                    .to_string(),
                segment_carriers: segment_carriers(self.segment_carriers),
                segment_pair_enumeration_identity: self
                    .segment_pair_enumeration
                    .segment_pair_enumeration_identity()
                    .to_string(),
                predicate_binding_identity: self
                    .predicate_binding
                    .predicate_binding_identity()
                    .to_string(),
                point_event_extraction_identity: self
                    .point_events
                    .extraction_identity()
                    .to_string(),
                collinear_relation_receipt_identity: self
                    .collinear_relations
                    .receipt_identity()
                    .to_string(),
                interval_event_extraction_identity: self
                    .interval_events
                    .extraction_identity()
                    .to_string(),
                point_events: self.point_events.point_events().to_vec(),
                interval_events: self.interval_events.interval_events().to_vec(),
                relation_diagnostics,
                event_groups,
                ordered_events,
                counters,
                event_ledger_identity,
                downstream_consumption_identity,
            },
        ))
    }
}

fn segment_carriers(
    carriers: &PlanarBooleanSegmentCarrierSet,
) -> Vec<crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentCarrier> {
    let mut rows = carriers.left().to_vec();
    rows.extend(carriers.right().to_vec());
    rows.sort_by(|left, right| left.carrier_identity().cmp(right.carrier_identity()));
    rows
}

fn relation_diagnostics(
    collinear_relations: &PlanarBooleanCollinearRelationReceipt,
) -> Vec<PlanarBooleanCollinearRelation> {
    collinear_relations
        .relations()
        .iter()
        .filter(|relation| {
            matches!(
                relation.kind(),
                PlanarBooleanCollinearRelationKind::Disjoint
                    | PlanarBooleanCollinearRelationKind::EndpointTouch
            )
        })
        .cloned()
        .collect()
}

fn require_receipt<'a, T>(
    receipt: Option<&'a T>,
    kind: PlanarBooleanEventLedgerDenialKind,
    human_reason: &'static str,
) -> Result<&'a T, PlanarBooleanEventLedgerDenial> {
    receipt.ok_or_else(|| denial(kind, "", human_reason))
}

fn ensure_nonempty(
    identity: &str,
    kind: PlanarBooleanEventLedgerDenialKind,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanEventLedgerDenial> {
    if identity.is_empty() {
        Err(denial(kind, identity, human_reason))
    } else {
        Ok(())
    }
}

fn denial(
    kind: PlanarBooleanEventLedgerDenialKind,
    evidence_identity: impl Into<String>,
    human_reason: impl Into<String>,
) -> PlanarBooleanEventLedgerDenial {
    PlanarBooleanEventLedgerDenial::new(kind, evidence_identity, human_reason)
}
