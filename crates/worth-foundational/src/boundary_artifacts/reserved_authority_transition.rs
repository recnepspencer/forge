use super::planned::FoundationalPlannedWorkBoundaryArtifact;
use super::same_family::FoundationalSameFamilyBoundaryArtifact;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalReservedAuthorityTransitionKind {
    Branch,
    Merge,
    Commit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalReservedAuthorityTransitionDenial {
    PlannedWorkMustRemainDescriptive {
        attempted: FoundationalReservedAuthorityTransitionKind,
    },
    SameFamilyMustRemainDescriptive {
        attempted: FoundationalReservedAuthorityTransitionKind,
    },
}

pub fn evaluate_planned_work_reserved_authority_transition_legality<Surface>(
    _: &FoundationalPlannedWorkBoundaryArtifact<Surface>,
    attempted: FoundationalReservedAuthorityTransitionKind,
) -> Result<(), FoundationalReservedAuthorityTransitionDenial> {
    Err(
        FoundationalReservedAuthorityTransitionDenial::PlannedWorkMustRemainDescriptive {
            attempted,
        },
    )
}

pub fn evaluate_same_family_reserved_authority_transition_legality<Surface>(
    _: &FoundationalSameFamilyBoundaryArtifact<Surface>,
    attempted: FoundationalReservedAuthorityTransitionKind,
) -> Result<(), FoundationalReservedAuthorityTransitionDenial> {
    Err(
        FoundationalReservedAuthorityTransitionDenial::SameFamilyMustRemainDescriptive {
            attempted,
        },
    )
}
