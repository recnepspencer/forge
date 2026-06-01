use crate::contribution_composed_orchestration::ForgeQueryContributionComposedClassification;
use crate::ordinary_outcome::ForgeQueryOrdinaryPosture;

use super::mapping::kind_for_contribution_posture;
use crate::recovery_boundary::{ForgeQueryRecoveryConflictPosture, ForgeQueryRecoveryExplanation};

pub(crate) fn enrich_contribution_explanation(
    explanation: ForgeQueryRecoveryExplanation,
    posture: &ForgeQueryOrdinaryPosture,
) -> ForgeQueryRecoveryExplanation {
    let Some(kind) = posture.checked_topology().contribution_composed_kind() else {
        return explanation;
    };
    let conflict_posture = match kind_for_contribution_posture(kind) {
        ForgeQueryContributionComposedClassification::PartiallyAdmitted
        | ForgeQueryContributionComposedClassification::MaterializationFailedAfterAdmission => {
            ForgeQueryRecoveryConflictPosture::MixedContributionFailure
        }
        ForgeQueryContributionComposedClassification::FullyAdmitted
        | ForgeQueryContributionComposedClassification::NoContributionAdmitted => {
            ForgeQueryRecoveryConflictPosture::None
        }
    };
    explanation.with_conflict_posture(conflict_posture)
}
