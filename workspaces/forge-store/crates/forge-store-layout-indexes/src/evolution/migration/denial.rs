use crate::PhysicalArtifactFamilyDeclaration;

use super::{LayoutPlanFingerprint, LayoutVersion};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutEvolutionDenial {
    FamilyMismatch {
        declared: &'static PhysicalArtifactFamilyDeclaration,
        binding: &'static PhysicalArtifactFamilyDeclaration,
    },
    IncompatibleSourceVersion {
        source: LayoutVersion,
        minimum_readable: LayoutVersion,
        maximum_readable: LayoutVersion,
    },
    UndeclaredCompatibleLayoutVersion {
        source: LayoutVersion,
    },
    UnsupportedMigrationTarget {
        source: LayoutVersion,
        target: LayoutVersion,
    },
    UnsupportedRollbackTarget {
        source: LayoutVersion,
        target: LayoutVersion,
    },
    InterruptStateDoesNotMatchPlan {
        expected: LayoutPlanFingerprint,
        actual: LayoutPlanFingerprint,
    },
}
