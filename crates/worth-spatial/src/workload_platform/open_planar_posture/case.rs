use topology::facade::TopologySeedKind;

use crate::planar_contracts::clean_fail_boundary::PlanarOpenInputKind;
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticSubjectKind;
use crate::workload_platform::user_response::{WorthUserOutcomeCauseKind, WorthUserOutcomeKind};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpenPlanarPostureCase {
    UnsupportedOpenSheet,
    UnsupportedOpenWire,
    PolicyRequiredHalfSpace,
    PredicateUncertain,
    BoundedOperatorIncompatibility,
    IntegrityMismatch,
    TransformDivergence,
}

impl OpenPlanarPostureCase {
    pub(crate) fn from_topology_kind(kind: TopologySeedKind) -> Option<Self> {
        match kind {
            TopologySeedKind::OpenSheet => Some(Self::UnsupportedOpenSheet),
            TopologySeedKind::OpenWire => Some(Self::UnsupportedOpenWire),
            _ => None,
        }
    }

    pub(crate) fn expected_user_outcome(self) -> (WorthUserOutcomeKind, WorthUserOutcomeCauseKind) {
        match self {
            Self::UnsupportedOpenSheet
            | Self::UnsupportedOpenWire
            | Self::BoundedOperatorIncompatibility => (
                WorthUserOutcomeKind::Unsupported,
                WorthUserOutcomeCauseKind::UnsupportedInput,
            ),
            Self::PolicyRequiredHalfSpace => (
                WorthUserOutcomeKind::PolicyRequired,
                WorthUserOutcomeCauseKind::PolicyRequired,
            ),
            Self::PredicateUncertain => (
                WorthUserOutcomeKind::PredicateUncertain,
                WorthUserOutcomeCauseKind::PredicateUncertain,
            ),
            Self::IntegrityMismatch => (
                WorthUserOutcomeKind::IntegrityMismatch,
                WorthUserOutcomeCauseKind::IntegrityMismatch,
            ),
            Self::TransformDivergence => (
                WorthUserOutcomeKind::Denied,
                WorthUserOutcomeCauseKind::DeniedMovementOrRotation,
            ),
        }
    }

    pub(crate) fn expected_open_input_kind(self) -> Option<PlanarOpenInputKind> {
        match self {
            Self::PolicyRequiredHalfSpace => Some(PlanarOpenInputKind::HalfSpaceGroup),
            Self::UnsupportedOpenSheet
            | Self::UnsupportedOpenWire
            | Self::PredicateUncertain
            | Self::BoundedOperatorIncompatibility
            | Self::IntegrityMismatch
            | Self::TransformDivergence => Some(PlanarOpenInputKind::OpenPlanarDomain),
        }
    }

    pub(crate) fn expected_diagnostic_subject_kind(self) -> PlanarDiagnosticSubjectKind {
        match self {
            Self::UnsupportedOpenSheet
            | Self::UnsupportedOpenWire
            | Self::BoundedOperatorIncompatibility => {
                PlanarDiagnosticSubjectKind::UnsupportedPlanarClass
            }
            Self::PolicyRequiredHalfSpace => PlanarDiagnosticSubjectKind::PolicyRequired,
            Self::PredicateUncertain => PlanarDiagnosticSubjectKind::PredicateFailure,
            Self::IntegrityMismatch | Self::TransformDivergence => {
                PlanarDiagnosticSubjectKind::UnsupportedPlanarClass
            }
        }
    }

    pub fn human_name(self) -> &'static str {
        match self {
            Self::UnsupportedOpenSheet => "unsupported open sheet",
            Self::UnsupportedOpenWire => "unsupported open wire",
            Self::PolicyRequiredHalfSpace => "half-space interpretation requires policy",
            Self::PredicateUncertain => "predicate uncertainty",
            Self::BoundedOperatorIncompatibility => "bounded operator incompatibility",
            Self::IntegrityMismatch => "bounded surrogate integrity mismatch",
            Self::TransformDivergence => "movement or rotation changes open posture",
        }
    }
}
