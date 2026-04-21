use crate::{StoreError, StoreErrorKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlacementPolicyClass {
    Conservative(ConservativePlacementPolicy),
    AdaptiveDebt(AdaptivePlacementDebtMarker),
}

impl PlacementPolicyClass {
    pub fn require_conservative(&self) -> Result<&ConservativePlacementPolicy, StoreError> {
        match self {
            Self::Conservative(policy) => Ok(policy),
            Self::AdaptiveDebt(marker) => Err(StoreError::new(
                StoreErrorKind::PlacementPolicyUnsupported,
                format!(
                    "adaptive placement policy `{}` is explicit debt in milestone 13 phase 1",
                    marker.label()
                ),
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ColdDerivedFamilyPolicy {
    SnapshotFamily,
    BranchDeltaFamily,
    Milestone6LayoutFamily,
}

impl ColdDerivedFamilyPolicy {
    pub fn label(self) -> &'static str {
        match self {
            Self::SnapshotFamily => "snapshot_family",
            Self::BranchDeltaFamily => "branch_delta_family",
            Self::Milestone6LayoutFamily => "milestone6_layout_family",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PlacementObservationScopeClass {
    Branch,
    RetainedBasis,
    ArtifactFamily,
}

impl PlacementObservationScopeClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::Branch => "branch",
            Self::RetainedBasis => "retained_basis",
            Self::ArtifactFamily => "artifact_family",
        }
    }

    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "branch" => Some(Self::Branch),
            "retained_basis" => Some(Self::RetainedBasis),
            "artifact_family" => Some(Self::ArtifactFamily),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConservativePlacementPolicy {
    allow_active_branch_head_hot_or_warm: bool,
    allow_retained_authority_warm: bool,
    cold_derived_families: Vec<ColdDerivedFamilyPolicy>,
    observation_scopes: Vec<PlacementObservationScopeClass>,
}

impl ConservativePlacementPolicy {
    pub fn new(
        mut cold_derived_families: Vec<ColdDerivedFamilyPolicy>,
        mut observation_scopes: Vec<PlacementObservationScopeClass>,
    ) -> Result<Self, StoreError> {
        if cold_derived_families.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::PlacementPolicyUnsupported,
                "conservative placement policy must admit at least one cold derived family",
            ));
        }
        if observation_scopes.is_empty() {
            return Err(StoreError::new(
                StoreErrorKind::PlacementPolicyUnsupported,
                "conservative placement policy must admit at least one observation scope",
            ));
        }
        cold_derived_families.sort();
        cold_derived_families.dedup();
        observation_scopes.sort();
        observation_scopes.dedup();
        Ok(Self {
            allow_active_branch_head_hot_or_warm: true,
            allow_retained_authority_warm: true,
            cold_derived_families,
            observation_scopes,
        })
    }

    pub fn allow_active_branch_head_hot_or_warm(&self) -> bool {
        self.allow_active_branch_head_hot_or_warm
    }

    pub fn allow_retained_authority_warm(&self) -> bool {
        self.allow_retained_authority_warm
    }

    pub fn cold_derived_families(&self) -> &[ColdDerivedFamilyPolicy] {
        &self.cold_derived_families
    }

    pub fn observation_scopes(&self) -> &[PlacementObservationScopeClass] {
        &self.observation_scopes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdaptivePlacementDebtMarker {
    CrossBranchGlobalHeatBalancing,
    PredictivePrefetchPromotion,
    SchedulerDrivenTierMutation,
    AggressiveColdAuthorityPlacement,
}

impl AdaptivePlacementDebtMarker {
    pub fn label(self) -> &'static str {
        match self {
            Self::CrossBranchGlobalHeatBalancing => "cross_branch_global_heat_balancing",
            Self::PredictivePrefetchPromotion => "predictive_prefetch_promotion",
            Self::SchedulerDrivenTierMutation => "scheduler_driven_tier_mutation",
            Self::AggressiveColdAuthorityPlacement => "aggressive_cold_authority_placement",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conservative_policy_normalizes_duplicates() {
        let policy = ConservativePlacementPolicy::new(
            vec![
                ColdDerivedFamilyPolicy::SnapshotFamily,
                ColdDerivedFamilyPolicy::SnapshotFamily,
                ColdDerivedFamilyPolicy::Milestone6LayoutFamily,
            ],
            vec![
                PlacementObservationScopeClass::Branch,
                PlacementObservationScopeClass::ArtifactFamily,
                PlacementObservationScopeClass::Branch,
            ],
        )
        .unwrap();

        assert!(policy.allow_active_branch_head_hot_or_warm());
        assert!(policy.allow_retained_authority_warm());
        assert_eq!(policy.cold_derived_families().len(), 2);
        assert_eq!(policy.observation_scopes().len(), 2);
    }

    #[test]
    fn conservative_policy_requires_admitted_surface() {
        let error = ConservativePlacementPolicy::new(Vec::new(), Vec::new()).unwrap_err();
        assert_eq!(error.kind(), &StoreErrorKind::PlacementPolicyUnsupported);
    }

    #[test]
    fn adaptive_marker_is_explicit_debt() {
        let policy = PlacementPolicyClass::AdaptiveDebt(
            AdaptivePlacementDebtMarker::PredictivePrefetchPromotion,
        );

        let error = policy.require_conservative().unwrap_err();
        assert_eq!(error.kind(), &StoreErrorKind::PlacementPolicyUnsupported);
    }
}
