use crate::certification::support::parity::digest_materialized_topology_view;
use crate::certification::topology_operator_closeout::report::{
    MilestoneThreeHostileOutcomeClass, MilestoneThreeMutationReplayStepRow,
};
use crate::topology_operators::application::TopologyMutationApplicationError;
use crate::topology_operators::application::{
    TopologyDeclarationMutationPayload, TopologyDeclaredMutationArtifact,
};
use crate::topology_operators::{
    NamingMutationContinuityMatrix, TopologyMutationDerivedFallbackPolicy, TopologyMutationDigest,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationSequenceDigest,
};

pub(super) fn accepted_step_row_for_execution(
    step_index: usize,
    execution: &TopologyDeclaredMutationArtifact,
) -> MilestoneThreeMutationReplayStepRow {
    let synopsis = execution.accepted_mutation_projection();
    let semantic_projection = execution.accepted_mutation_projection();
    MilestoneThreeMutationReplayStepRow {
        step_index,
        mutation_families: synopsis.mutation_families().to_vec(),
        topology_mutation_digest: synopsis.topology_mutation_digest().clone(),
        naming_mutation_continuity_matrix: semantic_projection
            .naming_mutation_continuity_matrix()
            .clone(),
        derived_fallback_policy: Some(semantic_projection.derived_fallback_policy()),
        outcome_class: MilestoneThreeHostileOutcomeClass::Accepted,
        rejection_class: None,
        resulting_materialized_topology_digest: Some(digest_materialized_topology_view(
            &execution.materialized(),
        )),
    }
}

pub(super) fn rejected_step_row_for_declaration<D>(
    step_index: usize,
    declaration: &D,
    error: &TopologyMutationApplicationError,
) -> MilestoneThreeMutationReplayStepRow
where
    D: TopologyDeclarationMutationPayload,
{
    MilestoneThreeMutationReplayStepRow {
        step_index,
        mutation_families: declaration.semantic_families(),
        topology_mutation_digest: declaration.topology_mutation_digest(),
        naming_mutation_continuity_matrix: declaration.naming_continuity_matrix(),
        derived_fallback_policy: None,
        outcome_class: MilestoneThreeHostileOutcomeClass::Rejected,
        rejection_class: error.rejection_class(),
        resulting_materialized_topology_digest: None,
    }
}

pub(super) fn aggregate_topology_mutation_digest_from_step_rows(
    step_rows: &[MilestoneThreeMutationReplayStepRow],
) -> TopologyMutationDigest {
    digest_rows(
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.digest.digest_hex.clone()),
    )
    .with_counts(
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.mutation_record_count)
            .sum(),
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.family_count)
            .sum(),
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.changed_scope_count)
            .sum(),
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.naming_scope_count)
            .sum(),
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.derived_region_count)
            .sum(),
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.fallback_policy_count)
            .sum(),
        step_rows
            .iter()
            .map(|row| row.topology_mutation_digest.fallback_rejection_policy_count)
            .sum(),
    )
}

pub(super) fn aggregate_naming_mutation_continuity_matrix_from_step_rows(
    step_rows: &[MilestoneThreeMutationReplayStepRow],
) -> NamingMutationContinuityMatrix {
    naming_mutation_continuity_matrix_from_rows(
        step_rows
            .iter()
            .flat_map(|row| row.naming_mutation_continuity_matrix.rows.iter().cloned())
            .collect(),
    )
}

pub(super) fn aggregate_mutation_families_from_step_rows(
    step_rows: &[MilestoneThreeMutationReplayStepRow],
) -> Vec<TopologyMutationFamily> {
    step_rows
        .iter()
        .flat_map(|row| row.mutation_families.iter().copied())
        .collect()
}

pub(super) fn aggregate_fallback_summary_from_step_rows(
    step_rows: &[MilestoneThreeMutationReplayStepRow],
) -> Option<TopologyMutationDerivedFallbackPolicy> {
    let accepted_fallbacks = step_rows
        .iter()
        .filter_map(|row| row.derived_fallback_policy)
        .collect::<Vec<_>>();
    if accepted_fallbacks.is_empty() {
        return None;
    }
    let policy = if accepted_fallbacks
        .iter()
        .any(|policy| *policy == TopologyMutationDerivedFallbackPolicy::RejectAnyFallback)
    {
        TopologyMutationDerivedFallbackPolicy::RejectAnyFallback
    } else {
        TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback
    };
    Some(policy)
}

fn naming_mutation_continuity_matrix_from_rows(
    rows: Vec<crate::topology_operators::TopologyMutationNamingRow>,
) -> NamingMutationContinuityMatrix {
    let preserved_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyMutationNamingOutcome::Preserved)
        .count();
    let ambiguous_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyMutationNamingOutcome::Ambiguous)
        .count();
    let rejected_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyMutationNamingOutcome::Rejected)
        .count();
    NamingMutationContinuityMatrix {
        rows,
        preserved_count,
        ambiguous_count,
        rejected_count,
    }
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> TopologyMutationSequenceDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    TopologyMutationSequenceDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}

trait WithCounts {
    fn with_counts(
        self,
        mutation_record_count: usize,
        family_count: usize,
        changed_scope_count: usize,
        naming_scope_count: usize,
        derived_region_count: usize,
        fallback_policy_count: usize,
        fallback_rejection_policy_count: usize,
    ) -> TopologyMutationDigest;
}

impl WithCounts for TopologyMutationSequenceDigest {
    fn with_counts(
        self,
        mutation_record_count: usize,
        family_count: usize,
        changed_scope_count: usize,
        naming_scope_count: usize,
        derived_region_count: usize,
        fallback_policy_count: usize,
        fallback_rejection_policy_count: usize,
    ) -> TopologyMutationDigest {
        TopologyMutationDigest {
            digest: self,
            mutation_record_count,
            family_count,
            changed_scope_count,
            naming_scope_count,
            derived_region_count,
            fallback_policy_count,
            fallback_rejection_policy_count,
        }
    }
}
