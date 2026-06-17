#[path = "public_api_planar_boolean_collinear_relations_support/mod.rs"]
#[allow(dead_code)]
mod collinear_relation_support;
#[path = "public_api_planar_boolean_edge_splitting_endpoint_boundary_support.rs"]
mod edge_splitting_endpoint_boundary_support;
#[path = "public_api_planar_boolean_edge_splitting_interval_subdivision_support.rs"]
mod edge_splitting_interval_subdivision_support;
#[path = "public_api_planar_boolean_edge_splitting_normalized_schedule_support.rs"]
mod edge_splitting_normalized_schedule_support;
#[path = "public_api_planar_boolean_edge_splitting_ordered_schedule_support.rs"]
mod edge_splitting_ordered_schedule_support;
#[path = "public_api_planar_boolean_edge_splitting_overlap_chain_support.rs"]
mod edge_splitting_overlap_chain_support;
#[path = "public_api_planar_boolean_edge_splitting_persistent_naming_support.rs"]
mod edge_splitting_persistent_naming_support;
#[path = "public_api_planar_boolean_edge_splitting_posture_support.rs"]
mod edge_splitting_posture_support;
#[path = "public_api_planar_boolean_edge_splitting_raw_schedule_support.rs"]
mod edge_splitting_raw_schedule_support;
#[path = "public_api_planar_boolean_edge_splitting_split_chain_validation_support.rs"]
mod edge_splitting_split_chain_validation_support;
#[path = "public_api_planar_boolean_edge_splitting_split_fragment_support.rs"]
mod edge_splitting_split_fragment_support;
#[path = "public_api_planar_boolean_edge_splitting_split_vertex_identity_support.rs"]
mod edge_splitting_split_vertex_identity_support;
#[path = "public_api_planar_boolean_edge_splitting_support.rs"]
mod edge_splitting_support;
#[path = "public_api_planar_boolean_event_ledger_support.rs"]
#[allow(dead_code)]
mod event_ledger_support;
#[path = "public_api_planar_boolean_event_extraction_metaboss_support/mod.rs"]
#[allow(dead_code, unused_imports)]
mod metaboss_support;
#[path = "public_api_planar_boolean_point_events_support/mod.rs"]
#[allow(dead_code)]
mod point_event_support;
#[path = "public_api_planar_boolean_event_predicate_binding_support.rs"]
#[allow(dead_code)]
mod predicate_binding_support;
#[path = "public_api_planar_boolean_common_plane_reduced_operand_pair_support.rs"]
mod reduced_pair_support;

use edge_splitting_endpoint_boundary_support::assert_endpoint_boundary_normalization_matches_metaboss;
use edge_splitting_interval_subdivision_support::assert_interval_subdivision_normalization_matches_metaboss;
use edge_splitting_normalized_schedule_support::assert_normalized_edge_split_schedule_matches_metaboss;
use edge_splitting_ordered_schedule_support::assert_ordered_edge_split_schedule_matches_metaboss;
use edge_splitting_overlap_chain_support::assert_overlap_edge_chains_match_metaboss;
use edge_splitting_persistent_naming_support::assert_split_persistent_naming_matches_metaboss;
use edge_splitting_posture_support::assert_point_split_postures_match_admitted_events;
use edge_splitting_raw_schedule_support::assert_raw_edge_split_schedule_matches_metaboss;
use edge_splitting_split_chain_validation_support::assert_split_chain_validation_matches_metaboss;
use edge_splitting_split_fragment_support::assert_split_edge_fragments_match_metaboss;
use edge_splitting_split_vertex_identity_support::assert_split_vertex_identities_match_metaboss;
use edge_splitting_support::{
    assert_interval_candidates_match_ledger_facts,
    assert_interval_domain_admission_matches_candidates,
    assert_point_domain_admission_matches_candidates, expected_interval_candidate_facts,
    expected_point_candidate_facts, recovered_carriers_for,
};
use metaboss_support::MetabossEventExtractionSubject;
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanCandidateIndexConsumptionDenialKind, PlanarBooleanCandidateIndexConsumptionGate,
    PlanarBooleanCandidateIndexConsumptionInput, PlanarBooleanSplitEventParticipationIndex,
};
use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanCandidateIndexFallbackPosture, PlanarBooleanCandidateIndexLifecycleOutcome,
    PlanarBooleanCandidateIndexStrategy, PlanarBooleanIntervalEventKind,
    PlanarBooleanSourceIntervalSense,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

#[test]
fn candidate_index_consumption_gate_proves_metaboss_query_indexed_discovery() {
    reduced_pair_support::run_with_large_stack(move || {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 candidate index consumption gate");
        let segment_pairs = &subject.inputs().pair_worklist;
        let ledger = subject.ledger();
        let evidence = WorkloadEvidenceLedger::from_rows(vec![
            WorkloadEvidenceRow::from_boolean_evidence_receipt(segment_pairs),
            WorkloadEvidenceRow::from_boolean_evidence_receipt(ledger),
        ])
        .expect("metaboss boolean receipts should build an indexed evidence product");

        let gate = PlanarBooleanCandidateIndexConsumptionGate::admit(
            PlanarBooleanCandidateIndexConsumptionInput::new(
                ledger,
                segment_pairs,
                evidence.stage_index(),
            ),
        )
        .expect("metaboss event ledger must consume the Query-owned candidate-index product");

        assert_eq!(gate.event_ledger_identity(), ledger.event_ledger_identity());
        assert_eq!(
            gate.segment_pair_enumeration_identity(),
            ledger.segment_pair_enumeration_identity()
        );
        assert_eq!(
            gate.candidate_index_product_identity(),
            segment_pairs.candidate_index_product_identity()
        );
        assert_eq!(
            gate.query_index_plan_digest(),
            segment_pairs.query_index_plan_digest()
        );
        assert_eq!(
            gate.candidate_index_strategy(),
            PlanarBooleanCandidateIndexStrategy::AabbSweep
        );
        assert_eq!(
            gate.fallback_posture(),
            PlanarBooleanCandidateIndexFallbackPosture::NotUsed
        );
        assert_eq!(
            gate.lifecycle_outcome(),
            PlanarBooleanCandidateIndexLifecycleOutcome::Bound
        );
        assert_eq!(
            gate.counters().expected_pair_breadth(),
            subject.expected().expected_possible_segment_pair_breadth()
        );
        assert_eq!(
            gate.counters().indexed_candidate_pair_count(),
            subject.expected().expected_segment_pair_breadth()
        );
        assert_eq!(
            gate.counters().culled_pair_count(),
            subject.expected().expected_query_index_culled_pair_count()
        );
        assert!(gate.certifies_production_candidate_discovery());
    });
}

fn assert_metaboss_candidate_index_consumption_denial(
    certification_label: &'static str,
    evidence_rows: impl FnOnce(&MetabossEventExtractionSubject) -> Vec<WorkloadEvidenceRow>
        + Send
        + 'static,
    expected_kind: PlanarBooleanCandidateIndexConsumptionDenialKind,
    expected_denial_message: &'static str,
) {
    reduced_pair_support::run_with_large_stack(move || {
        let subject = MetabossEventExtractionSubject::certify(certification_label);
        let segment_pairs = &subject.inputs().pair_worklist;
        let ledger = subject.ledger();
        let evidence = WorkloadEvidenceLedger::from_rows(evidence_rows(&subject))
            .expect("hostile evidence rows should be indexable before gate admission");

        let denial = PlanarBooleanCandidateIndexConsumptionGate::admit(
            PlanarBooleanCandidateIndexConsumptionInput::new(
                ledger,
                segment_pairs,
                evidence.stage_index(),
            ),
        )
        .expect_err(expected_denial_message);

        assert_eq!(denial.kind(), expected_kind);
    });
}

include!("public_api_planar_boolean_edge_splitting_candidate_index_denial_tests.rs");

#[test]
fn split_event_participation_index_covers_every_event_carrier_reference() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 split participation index coverage");
        let ledger = subject.ledger();
        let recovered = recovered_carriers_for(&subject);
        let index =
            PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(&recovered, ledger)
                .expect("split participation index should consume recovered source-edge carriers");
        let counters = index.counters();

        assert_eq!(
            index.event_ledger_identity(),
            ledger.event_ledger_identity()
        );
        assert_eq!(
            index.recovered_carrier_set_identity(),
            recovered.carrier_set_identity()
        );
        assert_eq!(counters.carriers_indexed(), ledger.segment_carriers().len());
        assert_eq!(
            counters.point_event_references(),
            ledger
                .point_events()
                .iter()
                .map(|event| event.participating_carrier_identities().len())
                .sum::<usize>()
        );
        assert_eq!(
            counters.interval_event_references(),
            ledger.interval_events().len() * 2
        );
        assert_eq!(
            counters.event_group_references(),
            ledger
                .event_groups()
                .iter()
                .map(|group| group.participating_carrier_identities().len())
                .sum::<usize>()
        );
        assert_eq!(counters.rejected_orphan_references(), 0);
        assert!(index.rows().iter().all(|row| {
            !row.participation_row_identity().is_empty()
                && !row.carrier_identity().is_empty()
                && !row.source_edge_identity().is_empty()
                && !row.start_source_endpoint_identity().is_empty()
                && !row.start_projected_endpoint_fact_identity().is_empty()
                && !row.end_source_endpoint_identity().is_empty()
                && !row.end_projected_endpoint_fact_identity().is_empty()
        }));
    });
}

#[test]
fn split_event_participation_index_orders_events_canonically() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 split participation canonical order");
        let recovered = recovered_carriers_for(&subject);
        let index = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
            &recovered,
            subject.ledger(),
        )
        .expect("split participation index should consume recovered source-edge carriers");

        assert_sorted_by(index.rows(), |row| row.carrier_identity().to_string());
        for row in index.rows() {
            assert_sorted(row.point_event_identities());
            assert_sorted(row.interval_event_identities());
            assert_sorted(row.event_group_identities());
        }
    });
}

#[test]
fn point_split_candidates_preserve_event_kind_coordinate_and_parameter_facts() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 point split candidate extraction");
        let expected_candidate_facts = expected_point_candidate_facts(subject.ledger());
        let recovered = recovered_carriers_for(&subject);
        let index = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
            &recovered,
            subject.ledger(),
        )
        .expect("split participation index should consume recovered source-edge carriers");
        let candidates = index
            .extract_point_split_candidates()
            .expect("point split candidates should lower from participation index");

        assert_eq!(
            candidates.counters().inspected_point_events(),
            subject.ledger().point_events().len()
        );
        assert_eq!(
            candidates.counters().emitted_point_candidates(),
            subject.ledger().point_events().len() * 2
        );
        assert_eq!(
            candidates.candidates().len(),
            expected_candidate_facts.len()
        );
        for candidate in candidates.candidates() {
            let expected = expected_candidate_facts
                .get(&(
                    candidate.point_event_identity().to_string(),
                    candidate.carrier_identity().to_string(),
                ))
                .expect("every point split candidate must bind a ledger point-event parameter");
            assert_eq!(candidate.point_event_kind(), expected.point_event_kind);
            assert_eq!(
                candidate.coordinate_fact().coordinate_fact_identity(),
                expected.coordinate_fact_identity
            );
            assert_eq!(
                candidate.parameter_fact_identity(),
                expected.parameter_fact_identity
            );
            assert_eq!(candidate.segment_identity(), expected.segment_identity);
            assert_eq!(candidate.parameter(), expected.parameter);
            assert!(!candidate.source_edge_identity().is_empty());
        }

        let admitted = candidates
            .admit_parameter_domain()
            .expect("metaboss point split candidates should be in-domain");
        assert_point_domain_admission_matches_candidates(&candidates, &admitted);
        let postures = admitted
            .classify_point_split_postures()
            .expect("metaboss point split postures should classify from admitted candidates");
        assert_point_split_postures_match_admitted_events(&admitted, &postures);
    });
}

#[test]
fn interval_split_candidates_preserve_kind_source_range_and_source_sense() {
    reduced_pair_support::run_with_large_stack(|| {
        let subject =
            MetabossEventExtractionSubject::certify("phase7.3 interval split candidate extraction");
        let recovered = recovered_carriers_for(&subject);
        let expected_candidate_facts =
            expected_interval_candidate_facts(subject.ledger(), &recovered);
        let index = PlanarBooleanSplitEventParticipationIndex::from_recovered_carriers(
            &recovered,
            subject.ledger(),
        )
        .expect("split participation index should consume recovered source-edge carriers");
        let candidates = index
            .extract_interval_split_candidates()
            .expect("interval split candidates should lower from participation index");

        assert_eq!(
            candidates.counters().inspected_interval_events(),
            subject.ledger().interval_events().len()
        );
        assert_eq!(
            candidates.counters().emitted_interval_candidates(),
            subject.ledger().interval_events().len() * 2
        );
        assert_interval_candidates_match_ledger_facts(&candidates, &expected_candidate_facts);
        assert!(candidates.candidates().iter().any(|candidate| {
            candidate.interval_event_kind() == PlanarBooleanIntervalEventKind::IdenticalAntiParallel
                && candidate.source_sense() == PlanarBooleanSourceIntervalSense::Reversed
        }));

        let admitted = candidates
            .admit_parameter_domain()
            .expect("metaboss interval split candidates should be in-domain and non-collapsed");
        assert_interval_domain_admission_matches_candidates(&candidates, &admitted);
        assert_raw_edge_split_schedule_matches_metaboss(&subject);
        assert_ordered_edge_split_schedule_matches_metaboss(&subject);
        assert_normalized_edge_split_schedule_matches_metaboss(&subject);
        assert_endpoint_boundary_normalization_matches_metaboss(&subject);
        assert_interval_subdivision_normalization_matches_metaboss(&subject);
        assert_split_vertex_identities_match_metaboss(&subject);
        assert_split_edge_fragments_match_metaboss(&subject);
        assert_overlap_edge_chains_match_metaboss(&subject);
        assert_split_chain_validation_matches_metaboss(&subject);
        assert_split_persistent_naming_matches_metaboss(&subject);
    });
}

fn assert_sorted(values: &[String]) {
    assert_sorted_by(values, |value| value.clone());
}

fn assert_sorted_by<T, F>(values: &[T], key: F)
where
    F: Fn(&T) -> String,
{
    assert!(
        values
            .windows(2)
            .all(|window| key(&window[0]) < key(&window[1])),
        "values should be strictly sorted and deduplicated"
    );
}
