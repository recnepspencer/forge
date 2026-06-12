use crate::workload_platform::surface_support::SurfaceFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MixedSurfaceKillBoxDenial {
    MissingDeclaration,
    MissingFamilyRun {
        family: SurfaceFamily,
    },
    DuplicateFamilyRun {
        family: SurfaceFamily,
    },
    MissingSurfaceSupportEvidence {
        family: SurfaceFamily,
    },
    SurfaceFamilyReceiptMismatch {
        target_family: SurfaceFamily,
        receipt_family: SurfaceFamily,
    },
    UnsupportedFamilyReadinessAttempt {
        family: SurfaceFamily,
    },
    KernelSummarySubstitution,
    WrongFamilyUserResponse {
        target_family: SurfaceFamily,
        response_family: SurfaceFamily,
    },
    GeneratedFeatureSmugglingAttempt,
}

impl MixedSurfaceKillBoxDenial {
    pub fn human_reason(&self) -> String {
        match self {
            Self::MissingDeclaration => {
                "mixed surface kill box requires a human-readable declaration".to_string()
            }
            Self::MissingFamilyRun { family } => format!(
                "mixed surface kill box requires a surface-support receipt for {}",
                family.human_label()
            ),
            Self::DuplicateFamilyRun { family } => format!(
                "mixed surface kill box received duplicate surface-family evidence for {}",
                family.human_label()
            ),
            Self::MissingSurfaceSupportEvidence { family } => format!(
                "mixed surface kill box cannot classify {} without its surface-support receipt",
                family.human_label()
            ),
            Self::SurfaceFamilyReceiptMismatch {
                target_family,
                receipt_family,
            } => format!(
                "{} cannot consume a surface-support receipt certified for {}",
                target_family.human_label(),
                receipt_family.human_label()
            ),
            Self::UnsupportedFamilyReadinessAttempt { family } => format!(
                "{} is not acceptable M7 input because M6.5 only certifies plane surface support",
                family.human_label()
            ),
            Self::KernelSummarySubstitution => {
                "kernel summary substitution cannot satisfy mixed surface kill box readiness evidence"
                    .to_string()
            }
            Self::WrongFamilyUserResponse {
                target_family,
                response_family,
            } => format!(
                "{} cannot consume user-response evidence produced for {}",
                target_family.human_label(),
                response_family.human_label()
            ),
            Self::GeneratedFeatureSmugglingAttempt => {
                "generated feature surface support is not partially admitted in M6.5".to_string()
            }
        }
    }
}
