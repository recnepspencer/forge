use worth_kernel::workload_composition::{
    PlanarBooleanEventExtractionRequest, WorkloadCompositionError,
};
use worth_spatial::certification::workload_evidence::{
    certification_only_admitted_stage_row, complete_ledger_stage_snapshot,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCollinearRelationKind, PlanarBooleanEventLedger,
    PlanarBooleanEventLedgerDenialKind,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

use super::collinear_relation_support;
use super::collinear_relation_support::SyntheticCollinearRelation;
use super::event_ledger_support::{
    certified_inputs_for_collinear_relation, ledger_for_collinear_relation,
    ledger_for_point_relation, pair_and_ledger_for_collinear_relation,
};
use super::point_event_support::SyntheticPointRelation;
use super::reduced_pair_support;

#[test]
fn event_ledger_orders_point_and_interval_events_canonically_across_replay() {
    reduced_pair_support::run_with_large_stack(|| {
        let first = ledger_for_collinear_relation(
            "phase7.2 event ledger canonical replay",
            SyntheticCollinearRelation::DiagonalPartialOverlapWithSecondReversed,
        );
        let second = ledger_for_collinear_relation(
            "phase7.2 event ledger canonical replay",
            SyntheticCollinearRelation::DiagonalPartialOverlapWithSecondReversed,
        );

        assert_eq!(
            first.ordered_events().interval_event_identities(),
            second.ordered_events().interval_event_identities()
        );
        assert_eq!(
            first
                .interval_events()
                .iter()
                .map(|event| event.event_identity())
                .collect::<Vec<_>>(),
            second
                .interval_events()
                .iter()
                .map(|event| event.event_identity())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            first.ordered_events().event_group_identities(),
            second.ordered_events().event_group_identities()
        );
        assert_eq!(
            first.event_ledger_identity(),
            second.event_ledger_identity()
        );
        assert_eq!(
            first.downstream_consumption_identity(),
            second.downstream_consumption_identity()
        );
    });
}

#[test]
fn event_ledger_groups_coincident_point_reports_without_losing_provenance() {
    reduced_pair_support::run_with_large_stack(|| {
        let ledger = ledger_for_point_relation(
            "phase7.2 event ledger point grouping",
            SyntheticPointRelation::SharedEndpointWithDifferentFreeEndpoints,
        );

        assert!(
            ledger.counters().point_events_consumed()
                + ledger.counters().duplicate_point_reports_suppressed()
                > 1
        );
        assert_eq!(
            ledger.counters().point_events_consumed(),
            ledger.point_events().len()
        );
        assert!(ledger.counters().point_groups_emitted() > 0);
        assert!(
            ledger.counters().point_groups_emitted()
                < ledger.counters().point_events_consumed()
                    + ledger.counters().duplicate_point_reports_suppressed(),
            "coincident point grouping must compress duplicate reports"
        );
        let group = ledger
            .event_groups()
            .iter()
            .filter(|group| !group.point_event_identities().is_empty())
            .max_by_key(|group| group.participating_carrier_identities().len())
            .expect("ledger should contain a point event group");
        assert!(group.point_event_identities().len() >= 1);
        assert!(group.participating_carrier_identities().len() >= 2);
        assert!(group.source_endpoint_identities().len() >= 2);
        assert!(ledger.counters().duplicate_point_reports_suppressed() > 0);
        assert_eq!(
            ledger.ordered_events().event_group_identities().len(),
            ledger.counters().total_grouped_event_count()
        );
    });
}

#[test]
fn event_ledger_groups_interval_events_without_erasing_source_sense() {
    reduced_pair_support::run_with_large_stack(|| {
        let ledger = ledger_for_collinear_relation(
            "phase7.2 event ledger interval grouping",
            SyntheticCollinearRelation::IdenticalAntiParallel,
        );

        assert!(ledger.counters().interval_events_consumed() > 1);
        assert_eq!(
            ledger.counters().interval_events_consumed(),
            ledger.interval_events().len()
        );
        assert!(ledger.counters().interval_groups_emitted() > 0);
        assert!(
            ledger.counters().interval_groups_emitted()
                < ledger.counters().interval_events_consumed(),
            "coincident interval grouping must compress interval reports"
        );
        let interval_group = ledger
            .event_groups()
            .iter()
            .filter(|group| !group.interval_event_identities().is_empty())
            .max_by_key(|group| group.source_interval_identities().len())
            .expect("ledger should contain an interval event group");
        assert!(interval_group.interval_event_identities().len() >= 2);
        assert!(interval_group.source_interval_identities().len() >= 2);
        assert!(interval_group.participating_carrier_identities().len() >= 2);
        assert!(ledger.counters().duplicate_interval_group_reports_merged() > 0);
    });
}

#[test]
fn event_ledger_rejects_missing_pair_enumeration_or_predicate_binding_receipts() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = collinear_relation_support::binding_subject_with_relation(
            "phase7.2 event ledger missing receipt",
            SyntheticCollinearRelation::PartialOverlap,
        );
        let event_request = PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(
            subject.reduced_pair.clone(),
        );
        let carriers = subject
            .reduced_pair
            .segment_carrier_set()
            .expect("segment carriers should certify");

        let denial = PlanarBooleanEventLedger::assemble()
            .for_reduced_pair_identity(event_request.reduced_operand_pair_identity())
            .for_event_extraction_request_identity(
                event_request.event_extraction_request_identity(),
            )
            .with_segment_carriers(&carriers)
            .compile()
            .expect_err("missing pair enumeration must deny before ledger certification");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEventLedgerDenialKind::MissingSegmentPairEnumerationIdentity
        );
    });
}

#[test]
fn event_ledger_rejects_missing_predicate_binding_after_pair_enumeration_receipt() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject = collinear_relation_support::binding_subject_with_relation(
            "phase7.2 event ledger missing predicate binding",
            SyntheticCollinearRelation::PartialOverlap,
        );
        let event_request =
            PlanarBooleanEventExtractionRequest::from_reduced_operand_pair(subject.reduced_pair);
        let carriers = event_request
            .reduced_operand_pair_request()
            .segment_carrier_set()
            .expect("segment carriers should certify");
        let pair_worklist = carriers
            .canonical_segment_set()
            .expect("canonical segments should certify")
            .segment_pair_enumeration_receipt()
            .expect("pair worklist should certify");

        let denial = PlanarBooleanEventLedger::assemble()
            .for_reduced_pair_identity(event_request.reduced_operand_pair_identity())
            .for_event_extraction_request_identity(
                event_request.event_extraction_request_identity(),
            )
            .with_segment_carriers(&carriers)
            .with_segment_pair_enumeration(&pair_worklist)
            .compile()
            .expect_err("missing predicate binding must deny after pair enumeration");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEventLedgerDenialKind::MissingPredicateBindingIdentity
        );
    });
}

#[test]
fn event_ledger_rejects_foreign_point_event_receipt_chain() {
    reduced_pair_support::run_with_large_stack(|| {
        let honest = certified_inputs_for_collinear_relation(
            "phase7.2 event ledger honest chain",
            SyntheticCollinearRelation::PartialOverlap,
        );
        let foreign = certified_inputs_for_collinear_relation(
            "phase7.2 event ledger foreign point chain",
            SyntheticCollinearRelation::ContainmentOverlap,
        );

        let denial = PlanarBooleanEventLedger::assemble()
            .for_reduced_pair_identity(honest.event_request.reduced_operand_pair_identity())
            .for_event_extraction_request_identity(
                honest.event_request.event_extraction_request_identity(),
            )
            .with_segment_carriers(&honest.carriers)
            .with_segment_pair_enumeration(&honest.pair_worklist)
            .with_predicate_binding(&honest.binding)
            .with_point_events(&foreign.point_events)
            .with_collinear_relations(&honest.collinear_relations)
            .with_interval_events(&honest.interval_events)
            .compile()
            .expect("ledger assembly should compile before chain validation")
            .certify()
            .expect_err("foreign point event extraction must deny");

        assert_eq!(
            denial.kind(),
            PlanarBooleanEventLedgerDenialKind::MismatchedPredicateBindingForPointEvents
        );
    });
}

#[test]
fn event_ledger_retains_no_event_relation_diagnostics_for_consumers() {
    reduced_pair_support::run_with_large_stack(|| {
        let ledger = ledger_for_collinear_relation(
            "phase7.2 event ledger no-event diagnostics",
            SyntheticCollinearRelation::Disjoint,
        );

        assert!(ledger.point_events().is_empty());
        assert!(ledger.interval_events().is_empty());
        assert!(!ledger.relation_diagnostics().is_empty());
        assert!(ledger
            .relation_diagnostics()
            .iter()
            .all(|relation| relation.kind() == PlanarBooleanCollinearRelationKind::Disjoint));
        let relation_diagnostic_identities = ledger
            .relation_diagnostics()
            .iter()
            .map(|relation| relation.relation_identity().to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            ledger.ordered_events().relation_diagnostic_identities(),
            &relation_diagnostic_identities
        );
        assert_eq!(
            ledger.counters().relation_diagnostics_retained(),
            ledger.relation_diagnostics().len()
        );
    });
}

#[test]
fn worth_workload_requires_real_event_ledger_evidence() {
    reduced_pair_support::run_with_large_stack(|| {
        let (pair, ledger) = pair_and_ledger_for_collinear_relation(
            "phase7.2 event ledger workload evidence",
            SyntheticCollinearRelation::PartialOverlap,
        );
        let bare = pair.left().workload().clone();

        assert_eq!(
            bare.require_boolean_event_ledger(&ledger)
                .expect_err("bare workload must reject missing event-ledger evidence"),
            WorkloadCompositionError::MissingEvidenceStage(
                WorkloadEvidenceStage::BooleanEventLedger
            )
        );

        let admitted = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::from_boolean_evidence_receipt(&ledger)],
        );
        admitted
            .require_boolean_event_ledger(&ledger)
            .expect("real event-ledger receipt evidence should pass");
        let counters = complete_ledger_stage_snapshot(
            admitted.evidence_ledger(),
            WorkloadEvidenceStage::BooleanEventLedger,
        )
        .expect("event-ledger row should exist")
        .counters();
        assert_eq!(counters.boolean_event_ledger_count(), 1);
        assert_eq!(
            counters.boolean_event_ledger_group_count(),
            ledger.counters().total_grouped_event_count()
        );

        let manual = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![WorkloadEvidenceRow::new(
                WorkloadEvidenceStage::BooleanEventLedger,
                ledger.event_ledger_identity(),
            )],
        );
        assert_eq!(
            manual
                .require_boolean_event_ledger(&ledger)
                .expect_err("manual event-ledger row must fail"),
            WorkloadCompositionError::ManualEvidenceStage(
                WorkloadEvidenceStage::BooleanEventLedger
            )
        );

        let counterless = reduced_pair_support::rebuild_left_workload(
            &pair,
            vec![certification_only_admitted_stage_row(
                WorkloadEvidenceStage::BooleanEventLedger,
                ledger.event_ledger_identity(),
                WorkloadEvidenceStageCounters::default(),
            )],
        );
        assert_eq!(
            counterless
                .require_boolean_event_ledger(&ledger)
                .expect_err("counterless event-ledger row must fail"),
            WorkloadCompositionError::CounterlessEvidenceStage(
                WorkloadEvidenceStage::BooleanEventLedger
            )
        );
    });
}
