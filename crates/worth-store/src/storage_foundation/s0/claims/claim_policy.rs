use super::super::artifacts::S0ArtifactRowStatus;
use super::super::milestones::{
    MilestonePhysicalStatusRow, S0PhysicalStatus, SemanticPhysicalClaimFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SemanticPhysicalClaimStatus {
    SemanticProven,
    NotApplicable,
    NotStarted,
    SemanticOnly,
    BootstrapPhysical,
    PhysicalDebt,
    PartiallyFoundationBacked,
    FoundationBacked,
    PlatformGrade,
}

pub(super) fn claim_status_for(
    row: &MilestonePhysicalStatusRow,
    family: SemanticPhysicalClaimFamily,
) -> SemanticPhysicalClaimStatus {
    match family {
        SemanticPhysicalClaimFamily::SemanticAuthority
        | SemanticPhysicalClaimFamily::RecoverySemantics
        | SemanticPhysicalClaimFamily::RetentionSemantics
        | SemanticPhysicalClaimFamily::SubscriptionSupport
        | SemanticPhysicalClaimFamily::CompatibilitySemantics
        | SemanticPhysicalClaimFamily::TieringPlacement
        | SemanticPhysicalClaimFamily::ReplicationSemantics => {
            SemanticPhysicalClaimStatus::SemanticProven
        }
        _ => match row.physical_status_for_claim_family(family) {
            S0PhysicalStatus::NotApplicable => SemanticPhysicalClaimStatus::NotApplicable,
            S0PhysicalStatus::NotStarted => SemanticPhysicalClaimStatus::NotStarted,
            S0PhysicalStatus::SemanticOnly => SemanticPhysicalClaimStatus::SemanticOnly,
            S0PhysicalStatus::BootstrapPhysical => SemanticPhysicalClaimStatus::BootstrapPhysical,
            S0PhysicalStatus::PhysicalDebt => SemanticPhysicalClaimStatus::PhysicalDebt,
            S0PhysicalStatus::PartiallyFoundationBacked => {
                SemanticPhysicalClaimStatus::PartiallyFoundationBacked
            }
            S0PhysicalStatus::FoundationBacked => SemanticPhysicalClaimStatus::FoundationBacked,
            S0PhysicalStatus::PlatformGrade => SemanticPhysicalClaimStatus::PlatformGrade,
        },
    }
}

pub(super) fn claim_status_requires_deferred_mapping(
    family: SemanticPhysicalClaimFamily,
    status: SemanticPhysicalClaimStatus,
) -> bool {
    is_physical_family(family)
        && matches!(
            status,
            SemanticPhysicalClaimStatus::NotStarted
                | SemanticPhysicalClaimStatus::SemanticOnly
                | SemanticPhysicalClaimStatus::BootstrapPhysical
                | SemanticPhysicalClaimStatus::PhysicalDebt
                | SemanticPhysicalClaimStatus::PartiallyFoundationBacked
        )
}

fn is_physical_family(family: SemanticPhysicalClaimFamily) -> bool {
    matches!(
        family,
        SemanticPhysicalClaimFamily::PhysicalSubstrate
            | SemanticPhysicalClaimFamily::PhysicalBoundedness
            | SemanticPhysicalClaimFamily::PhysicalIntegrity
            | SemanticPhysicalClaimFamily::PhysicalRecoveryPhysics
            | SemanticPhysicalClaimFamily::PhysicalIsolation
            | SemanticPhysicalClaimFamily::PhysicalIo
            | SemanticPhysicalClaimFamily::PhysicalOperationalSafety
            | SemanticPhysicalClaimFamily::PhysicalSecurity
    )
}

pub(super) fn artifact_status_for(status: SemanticPhysicalClaimStatus) -> S0ArtifactRowStatus {
    match status {
        SemanticPhysicalClaimStatus::NotApplicable => S0ArtifactRowStatus::NotApplicable,
        SemanticPhysicalClaimStatus::FoundationBacked
        | SemanticPhysicalClaimStatus::PlatformGrade
        | SemanticPhysicalClaimStatus::SemanticProven => S0ArtifactRowStatus::Admitted,
        SemanticPhysicalClaimStatus::NotStarted
        | SemanticPhysicalClaimStatus::SemanticOnly
        | SemanticPhysicalClaimStatus::BootstrapPhysical
        | SemanticPhysicalClaimStatus::PhysicalDebt
        | SemanticPhysicalClaimStatus::PartiallyFoundationBacked => S0ArtifactRowStatus::Deferred,
    }
}
