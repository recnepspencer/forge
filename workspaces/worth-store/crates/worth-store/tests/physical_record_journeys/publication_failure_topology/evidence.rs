use super::super::scenario_evidence::{ScenarioEvidence, ScenarioPredicate};
use super::scenario::ObservedPublicationDeath;

pub(super) fn adjudicate_and_emit(observed: &ObservedPublicationDeath) {
    assert_observation(observed);
    let expected_partial_artifacts = usize::from(observed.case.directive == "prefix");
    let expected_partial_role = expected_partial_role(observed);
    let predicates = [
        ScenarioPredicate::equality(
            "runtime_root",
            observed.case.expected_generation,
            observed.reopened.generation,
        ),
        ScenarioPredicate::equality(
            "offline_root",
            observed.case.expected_generation,
            observed.offline.generation,
        ),
        ScenarioPredicate::equality("runtime_inspection_fence", true, observed.reopened.fenced),
        ScenarioPredicate::equality(
            "typed_recovery_obligation",
            1_u64,
            observed.reopened.recovery_count as u64,
        ),
        ScenarioPredicate::equality(
            "offline_record_count",
            observed.case.expected_records as u64,
            observed.offline.records as u64,
        ),
        ScenarioPredicate::equality(
            "residue_posture",
            observed.case.expected_residue,
            observed.reopened.residue,
        ),
        ScenarioPredicate::equality(
            "close_added_no_publication_effect",
            observed.catalog_before_reopen.clone(),
            observed.catalog_after_reopen.clone(),
        ),
        ScenarioPredicate::equality(
            "interposer_role",
            observed.case.role.metric_name(),
            observed.boundary.role.as_str(),
        ),
        ScenarioPredicate::equality(
            "interposer_identified_ordinal",
            observed.case.append_ordinal,
            observed.boundary.identified_ordinal,
        ),
        ScenarioPredicate::equality(
            "completed_byte_prefix",
            expected_partial_artifacts as u64,
            observed.partial_artifacts.len() as u64,
        ),
        ScenarioPredicate::equality(
            "completed_prefix_role",
            expected_partial_role,
            observed.observed_partial_role,
        ),
    ];
    super::super::scenario_evidence::emit(ScenarioEvidence {
        courtroom: "publication_cutover_never_invents_current_truth",
        world: observed.case.name,
        root: &observed.root,
        seed: 0xC5C5_0000_0000_0001,
        action_trace: &[
            "seed-prior",
            "kill-at-interposer",
            "fresh-reopen",
            "offline-process",
        ],
        authority_transitions: &[
            "prior-root-published",
            "writer-process-died",
            "fresh-runtime-readmitted",
        ],
        walk: &observed.walk,
        placement: observed.placement,
        publication_identity: None,
        processes: &observed.processes,
        counters: serde_json::json!({
            "killed_role": observed.case.role.metric_name(),
            "killed_raw_ordinal": observed.boundary.raw_ordinal,
            "killed_identified_ordinal": observed.boundary.identified_ordinal,
            "payload_bytes": observed.case.payload_bytes,
            "requested_bytes_at_death": observed.boundary.requested_bytes,
            "completed_prefix_bytes": expected_partial_artifacts,
            "completed_prefix_artifacts": observed.partial_artifacts,
        }),
        runtime_result: serde_json::json!({
            "root_generation": observed.reopened.generation,
            "records": observed.reopened.records,
            "residue": observed.reopened.residue,
            "fenced": observed.reopened.fenced,
            "recovery_count": observed.reopened.recovery_count,
        }),
        oracle_result: serde_json::json!({
            "root_generation": observed.case.expected_generation,
            "records": observed.case.expected_records,
            "residue": observed.case.expected_residue,
            "fenced": true,
            "recovery_count": 1,
        }),
        mutant_posture: "production-interposer-process-death",
        predicates: &predicates,
    });
}

fn assert_observation(observed: &ObservedPublicationDeath) {
    let case = observed.case;
    assert_eq!(
        observed.boundary.role,
        case.role.metric_name(),
        "{}",
        case.name
    );
    assert_eq!(
        observed.boundary.identified_ordinal, case.append_ordinal,
        "{}",
        case.name
    );
    let expected_partial_artifacts = usize::from(case.directive == "prefix");
    assert_eq!(
        observed.partial_artifacts.len(),
        expected_partial_artifacts,
        "{} completed byte prefix: {:?}",
        case.name,
        observed.partial_artifacts,
    );
    assert_eq!(
        observed.observed_partial_role,
        expected_partial_role(observed),
        "{} partial write reached the wrong artifact family",
        case.name
    );
    assert_eq!(
        observed.catalog_after_reopen, observed.catalog_before_reopen,
        "{}",
        case.name
    );
    assert_eq!(
        observed.reopened.generation, case.expected_generation,
        "C5_PREDICATE:independent-decision-path {}",
        case.name
    );
    assert_eq!(observed.reopened.records, 0, "{}", case.name);
    assert_eq!(
        observed.reopened.residue, case.expected_residue,
        "{} artifacts={:?}",
        case.name, observed.artifacts_after_death,
    );
    assert!(observed.reopened.fenced, "{}", case.name);
    assert_eq!(observed.reopened.recovery_count, 1, "{}", case.name);
    assert_eq!(
        observed.offline.generation, case.expected_generation,
        "{}",
        case.name
    );
    assert_eq!(
        observed.offline.records, case.expected_records,
        "{}",
        case.name
    );
    assert_eq!(
        observed.walk.root_generation(),
        case.expected_generation,
        "{}",
        case.name
    );
    assert_eq!(
        observed.walk.placements().len(),
        case.expected_records,
        "{}",
        case.name
    );
}

fn expected_partial_role(observed: &ObservedPublicationDeath) -> Option<&'static str> {
    match (
        observed.case.directive,
        observed.case.payload_bytes > 8 * 1024,
    ) {
        ("prefix", false) => Some("segment-page"),
        ("prefix", true) => Some("extent-data"),
        _ => None,
    }
}
