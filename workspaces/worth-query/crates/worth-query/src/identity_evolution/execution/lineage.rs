use super::classification::{
    authority_state_for_lineage, branch_locality_class_for_lineage, execution_family_for_lineage,
    outcome_family_for_lineage,
};
use super::{IdentityEvolutionExecutionArtifact, IdentityEvolutionExecutionCounters};

use super::super::{
    admission::{AdmittedIdentityEvolutionQuery, IdentityEvolutionAdmissionError},
    contracts::IdentityEvolutionComplexityStatus,
    families::{
        IdentityEvolutionDenialReason, IdentityEvolutionIdentityBreakReason, LineageTraversalFamily,
    },
    metadata::{IdentityEvolutionComplexityReport, IdentityEvolutionMetadata},
    performance::IdentityEvolutionPredictionDriftOutcome,
    results::{
        IdentityEvolutionDeniedBundle, IdentityEvolutionIdentityBreakBundle,
        IdentityEvolutionResultBundle, IdentityLifecycleResult, PluralIdentitySuccessorSet,
        SingularIdentityContinuityResult,
    },
    synthetic::IdentityEvolutionSyntheticScenario,
};

pub(super) fn execute(
    admitted_query: &AdmittedIdentityEvolutionQuery,
    descriptor: &super::super::request::LineageTraversalDescriptor,
) -> Result<IdentityEvolutionExecutionArtifact, IdentityEvolutionAdmissionError> {
    let family = execution_family_for_lineage(descriptor.family());
    let lineage_digest = descriptor.family().digest();
    let complexity_report = IdentityEvolutionComplexityReport::from_contract(
        admitted_query.complexity_contract().clone(),
    );
    let metadata = IdentityEvolutionMetadata::from_authority_parts(
        admitted_query.query_context().query_authority().clone(),
        admitted_query.query_context().basis_proof().clone(),
        lineage_digest.clone(),
        outcome_family_for_lineage(descriptor.family(), admitted_query.synthetic_scenario()),
        admitted_query.query_context().basis_digest().clone(),
        admitted_query.query_context().basis_digest().clone(),
        admitted_query.query_context().basis_digest().clone(),
        branch_locality_class_for_lineage(descriptor.family(), admitted_query.synthetic_scenario()),
        authority_state_for_lineage(descriptor.family(), admitted_query.synthetic_scenario()),
        complexity_report,
    );

    let mut counters = IdentityEvolutionExecutionCounters {
        declared_lineage_complexity_contract_count: 1,
        predicted_lineage_width: 1,
        realized_lineage_width: 1,
        identity_evolution_metadata_attachment_count: 1,
        identity_evolution_replay_parity_count: 1,
        identity_evolution_basis_rediscovery_count: 0,
        executor_rediscovery_count: 0,
        complexity_status_debt_count: usize::from(
            admitted_query
                .complexity_contract()
                .verified_or_debt_status()
                != IdentityEvolutionComplexityStatus::Verified,
        ),
        ..IdentityEvolutionExecutionCounters::default()
    };

    let scenario = admitted_query.synthetic_scenario();
    let (result_bundle, prediction_drift_outcome) = match (descriptor.family(), scenario) {
        #[cfg(test)]
        (
            LineageTraversalFamily::DirectPredecessor,
            IdentityEvolutionSyntheticScenario::BroadLineageScanDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.broad_lineage_scan_denial_count = 1;
            counters.realized_lineage_width = 0;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::BroadLineageScanRequired,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (
            LineageTraversalFamily::DirectPredecessor,
            IdentityEvolutionSyntheticScenario::ComplexityContractViolationDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.realized_lineage_width = 0;
            counters.complexity_contract_violation_denial_count = 1;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::ComplexityContractViolationDenied,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        #[cfg(test)]
        (
            LineageTraversalFamily::DirectPredecessor,
            IdentityEvolutionSyntheticScenario::LineageToCorrespondenceFallbackDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.realized_lineage_width = 0;
            counters.unsupported_lineage_denial_count = 1;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::LineageToCorrespondenceFallbackForbidden,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectPredecessor, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("predecessor:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectSuccessor, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        descriptor
                            .exact_result_identities()
                            .and_then(|identities| identities.first())
                            .cloned()
                            .unwrap_or_else(|| {
                                format!("successor:{}", descriptor.anchor_identity())
                            }),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectReplacement, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("replacement:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::DirectSplitSuccessors, _) => {
            let successor_identities = descriptor
                .exact_result_identities()
                .map(<[String]>::to_vec)
                .unwrap_or_else(|| {
                    vec![
                        format!("split-a:{}", descriptor.anchor_identity()),
                        format!("split-b:{}", descriptor.anchor_identity()),
                    ]
                });
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = successor_identities.len();
            counters.predicted_lineage_width = 1;
            counters.realized_lineage_width = successor_identities.len();
            counters.lineage_width_drift_count = usize::from(successor_identities.len() != 1);
            counters.split_successor_fanout_width = successor_identities.len();
            (
                IdentityEvolutionResultBundle::plural_identity_successor_set(
                    PluralIdentitySuccessorSet::new(metadata, successor_identities),
                ),
                if counters.lineage_width_drift_count == 0 {
                    IdentityEvolutionPredictionDriftOutcome::WithinBudget
                } else {
                    IdentityEvolutionPredictionDriftOutcome::WidthDriftDetected
                },
            )
        }
        (LineageTraversalFamily::DirectMergeSuccessor, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            counters.promotion_or_merge_authority_proof_check_count = 1;
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        descriptor
                            .exact_result_identities()
                            .and_then(|identities| identities.first())
                            .cloned()
                            .unwrap_or_else(|| {
                                format!("merge-successor:{}", descriptor.anchor_identity())
                            }),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::GeneratedIdentity, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.lineage_step_count = 1;
            (
                IdentityEvolutionResultBundle::generated_identity(IdentityLifecycleResult::new(
                    metadata,
                    descriptor.anchor_identity(),
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::RetiredIdentity, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.identity_break_count = 1;
            counters.realized_lineage_width = 0;
            (
                IdentityEvolutionResultBundle::retired_identity(IdentityLifecycleResult::new(
                    metadata,
                    descriptor.anchor_identity(),
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (
            LineageTraversalFamily::BranchLocalDirectEvolution,
            IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.branch_local_boundary_check_count = 1;
            (
                IdentityEvolutionResultBundle::denied(IdentityEvolutionDeniedBundle::new(
                    metadata,
                    IdentityEvolutionDenialReason::BranchCrossingLineageWithoutAdmittedBasisPairing,
                )),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (
            LineageTraversalFamily::BranchLocalDirectEvolution,
            IdentityEvolutionSyntheticScenario::IdentityBreak,
        ) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.branch_local_boundary_check_count = 1;
            counters.identity_break_count = 1;
            counters.realized_lineage_width = 0;
            (
                IdentityEvolutionResultBundle::identity_break(
                    IdentityEvolutionIdentityBreakBundle::new(
                        metadata,
                        IdentityEvolutionIdentityBreakReason::ExplicitIdentityBreak,
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
        (LineageTraversalFamily::BranchLocalDirectEvolution, _) => {
            counters.lineage_anchor_lookup_count = 1;
            counters.branch_local_boundary_check_count = 1;
            counters.lineage_step_count = 1;
            counters.branch_local_divergence_count =
                usize::from(scenario == IdentityEvolutionSyntheticScenario::BranchLocalDivergence);
            (
                IdentityEvolutionResultBundle::singular_identity_continuity(
                    SingularIdentityContinuityResult::new(
                        metadata,
                        format!("branch-local:{}", descriptor.anchor_identity()),
                    ),
                ),
                IdentityEvolutionPredictionDriftOutcome::WithinBudget,
            )
        }
    };

    if super::result_bundle_is_denied(&result_bundle)
        && descriptor.family() == LineageTraversalFamily::BranchLocalDirectEvolution
        && scenario == IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied
    {
        counters.unsupported_lineage_denial_count = 1;
        counters.branch_crossing_denial_count = 1;
        counters.realized_lineage_width = 0;
    }

    let result_digest = super::classification::execution_result_digest(
        admitted_query,
        family.as_str(),
        result_bundle.metadata().metadata_digest().as_str(),
        result_bundle.outcome_family().as_str(),
    );
    Ok(IdentityEvolutionExecutionArtifact::new(
        admitted_query
            .query_context()
            .query_digest()
            .as_str()
            .to_string(),
        admitted_query
            .query_context()
            .basis_digest()
            .as_str()
            .to_string(),
        lineage_digest.as_str().to_string(),
        result_digest.as_str().to_string(),
        family,
        prediction_drift_outcome,
        result_bundle,
        counters,
    ))
}
