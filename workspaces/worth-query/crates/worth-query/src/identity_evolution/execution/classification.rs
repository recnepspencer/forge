use crate::identity::ResultDigest;

use super::super::{
    admission::AdmittedIdentityEvolutionQuery,
    families::{IdentityEvolutionOutcomeFamily, LineageTraversalFamily},
    metadata::{BranchLocalityClass, PromotionOrMergeAuthorityState},
    request::{
        CorrespondenceIdentityComparison, IdentityComparisonIntent,
        IdentityEvolutionComparisonBasisFamily,
    },
    synthetic::IdentityEvolutionSyntheticScenario,
};
use super::IdentityEvolutionExecutionFamily;

pub(super) fn execution_family_for_lineage(
    family: LineageTraversalFamily,
) -> IdentityEvolutionExecutionFamily {
    match family {
        LineageTraversalFamily::DirectPredecessor => {
            IdentityEvolutionExecutionFamily::DirectPredecessor
        }
        LineageTraversalFamily::DirectSuccessor => {
            IdentityEvolutionExecutionFamily::DirectSuccessor
        }
        LineageTraversalFamily::DirectReplacement => {
            IdentityEvolutionExecutionFamily::DirectReplacement
        }
        LineageTraversalFamily::DirectSplitSuccessors => {
            IdentityEvolutionExecutionFamily::DirectSplitSuccessors
        }
        LineageTraversalFamily::DirectMergeSuccessor => {
            IdentityEvolutionExecutionFamily::DirectMergeSuccessor
        }
        LineageTraversalFamily::GeneratedIdentity => {
            IdentityEvolutionExecutionFamily::GeneratedIdentity
        }
        LineageTraversalFamily::RetiredIdentity => {
            IdentityEvolutionExecutionFamily::RetiredIdentity
        }
        LineageTraversalFamily::BranchLocalDirectEvolution => {
            IdentityEvolutionExecutionFamily::BranchLocalDirectEvolution
        }
    }
}

pub(super) fn execution_family_for_comparison(
    family: IdentityEvolutionComparisonBasisFamily,
) -> IdentityEvolutionExecutionFamily {
    match family {
        IdentityEvolutionComparisonBasisFamily::InstalledOperation => {
            IdentityEvolutionExecutionFamily::InstalledOperationComparison
        }
        IdentityEvolutionComparisonBasisFamily::BranchToBranch => {
            IdentityEvolutionExecutionFamily::BranchToBranchComparison
        }
        IdentityEvolutionComparisonBasisFamily::CurrentToHistorical => {
            IdentityEvolutionExecutionFamily::CurrentToHistoricalComparison
        }
        IdentityEvolutionComparisonBasisFamily::HistoricalToHistorical => {
            IdentityEvolutionExecutionFamily::HistoricalToHistoricalComparison
        }
        IdentityEvolutionComparisonBasisFamily::PreviewToAuthoritative => {
            IdentityEvolutionExecutionFamily::PreviewToAuthoritativeComparison
        }
    }
}

pub(super) fn branch_locality_class_for_lineage(
    family: LineageTraversalFamily,
    scenario: IdentityEvolutionSyntheticScenario,
) -> BranchLocalityClass {
    match family {
        LineageTraversalFamily::BranchLocalDirectEvolution => {
            if scenario == IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied {
                BranchLocalityClass::CrossBranchDenied
            } else {
                BranchLocalityClass::BranchLocalOnly
            }
        }
        LineageTraversalFamily::DirectPredecessor
        | LineageTraversalFamily::DirectSuccessor
        | LineageTraversalFamily::DirectReplacement
        | LineageTraversalFamily::DirectSplitSuccessors
        | LineageTraversalFamily::DirectMergeSuccessor
        | LineageTraversalFamily::GeneratedIdentity
        | LineageTraversalFamily::RetiredIdentity => BranchLocalityClass::CrossBranchAuthoritative,
    }
}

pub(super) fn authority_state_for_lineage(
    family: LineageTraversalFamily,
    scenario: IdentityEvolutionSyntheticScenario,
) -> PromotionOrMergeAuthorityState {
    match family {
        LineageTraversalFamily::DirectMergeSuccessor => {
            PromotionOrMergeAuthorityState::AuthorityWitnessed
        }
        LineageTraversalFamily::BranchLocalDirectEvolution => {
            if scenario == IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied {
                PromotionOrMergeAuthorityState::RequiredButUnavailable
            } else {
                PromotionOrMergeAuthorityState::NotRequired
            }
        }
        LineageTraversalFamily::DirectPredecessor
        | LineageTraversalFamily::DirectSuccessor
        | LineageTraversalFamily::DirectReplacement
        | LineageTraversalFamily::DirectSplitSuccessors
        | LineageTraversalFamily::GeneratedIdentity
        | LineageTraversalFamily::RetiredIdentity => PromotionOrMergeAuthorityState::NotRequired,
    }
}

pub(super) fn outcome_family_for_lineage(
    family: LineageTraversalFamily,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionOutcomeFamily {
    match family {
        #[cfg(test)]
        LineageTraversalFamily::DirectPredecessor
            if matches!(
                scenario,
                IdentityEvolutionSyntheticScenario::BroadLineageScanDenied
                    | IdentityEvolutionSyntheticScenario::ComplexityContractViolationDenied
                    | IdentityEvolutionSyntheticScenario::LineageToCorrespondenceFallbackDenied
            ) =>
        {
            IdentityEvolutionOutcomeFamily::Denied
        }
        LineageTraversalFamily::DirectSplitSuccessors => {
            IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet
        }
        LineageTraversalFamily::GeneratedIdentity => {
            IdentityEvolutionOutcomeFamily::GeneratedIdentity
        }
        LineageTraversalFamily::RetiredIdentity => IdentityEvolutionOutcomeFamily::RetiredIdentity,
        LineageTraversalFamily::BranchLocalDirectEvolution
            if scenario == IdentityEvolutionSyntheticScenario::BranchCrossingLineageDenied =>
        {
            IdentityEvolutionOutcomeFamily::Denied
        }
        LineageTraversalFamily::BranchLocalDirectEvolution
            if scenario == IdentityEvolutionSyntheticScenario::IdentityBreak =>
        {
            IdentityEvolutionOutcomeFamily::IdentityBreak
        }
        LineageTraversalFamily::DirectPredecessor
        | LineageTraversalFamily::DirectSuccessor
        | LineageTraversalFamily::DirectReplacement
        | LineageTraversalFamily::DirectMergeSuccessor
        | LineageTraversalFamily::BranchLocalDirectEvolution => {
            IdentityEvolutionOutcomeFamily::SingularIdentityContinuity
        }
    }
}

pub(super) fn comparison_outcome_family(
    comparison: &CorrespondenceIdentityComparison,
    locality: BranchLocalityClass,
    scenario: IdentityEvolutionSyntheticScenario,
) -> IdentityEvolutionOutcomeFamily {
    if comparison.intent() == IdentityComparisonIntent::ExplicitContinuityBreak
        || scenario == IdentityEvolutionSyntheticScenario::IdentityBreak
    {
        IdentityEvolutionOutcomeFamily::IdentityBreak
    } else if comparison.intent() == IdentityComparisonIntent::AmbiguousCandidateSet
        || scenario == IdentityEvolutionSyntheticScenario::AmbiguousCorrespondence
    {
        IdentityEvolutionOutcomeFamily::Ambiguity
    } else if comparison.intent() == IdentityComparisonIntent::AdvisoryCandidateSet {
        IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet
    } else if locality == BranchLocalityClass::CrossBranchAuthoritative
        && scenario != IdentityEvolutionSyntheticScenario::IdentityBreak
    {
        IdentityEvolutionOutcomeFamily::SingularIdentityContinuity
    } else {
        IdentityEvolutionOutcomeFamily::Denied
    }
}

pub(super) fn branch_locality_class_for_comparison(
    scenario: IdentityEvolutionSyntheticScenario,
) -> BranchLocalityClass {
    match scenario {
        #[cfg(test)]
        IdentityEvolutionSyntheticScenario::BranchLocalComparison => {
            BranchLocalityClass::BranchLocalOnly
        }
        IdentityEvolutionSyntheticScenario::AdvisoryAsAuthoritativeDenied => {
            BranchLocalityClass::CrossBranchDenied
        }
        _ => BranchLocalityClass::CrossBranchAuthoritative,
    }
}

pub(super) fn execution_result_digest(
    admitted_query: &AdmittedIdentityEvolutionQuery,
    execution_family: &str,
    metadata_digest: &str,
    outcome_family: &str,
) -> ResultDigest {
    ResultDigest::from_parts(&[
        format!(
            "query_digest:{}",
            admitted_query.query_context().query_digest().as_str()
        ),
        format!(
            "basis_digest:{}",
            admitted_query.query_context().basis_digest().as_str()
        ),
        format!("execution_family:{execution_family}"),
        format!("metadata_digest:{metadata_digest}"),
        format!("outcome_family:{outcome_family}"),
    ])
}
