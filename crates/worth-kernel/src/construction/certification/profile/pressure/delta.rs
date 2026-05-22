use super::report::{
    prepare_primitive_construction_policy_pressure_report, PrimitiveConstructionPolicyPressureCase,
    PrimitiveConstructionPolicyPressureRow, PrimitiveConstructionPolicyPressureSetup,
    PrimitiveConstructionPolicyPressureSurfaceReport,
    PrimitiveConstructionPolicyPressureSurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;
use crate::spatial_intent::{
    SpatialIntentEscalation, SpatialIntentPreviewCommitDisposition, SpatialPreviewRichness,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyPressureDeltaCase {
    GrazingClarificationVsPreservedAmbiguity,
    GrazingClarificationVsAggressiveSnap,
    GrazingAggressiveSnapVsHighFidelity,
    HostFaceAskFirstVsBimHostFriendly,
    HostFaceBimHostFriendlyVsHighFidelityAskFirst,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureDeltaRow {
    case: PrimitiveConstructionPolicyPressureDeltaCase,
    setup: PrimitiveConstructionPolicyPressureSetup,
    left_case: PrimitiveConstructionPolicyPressureCase,
    right_case: PrimitiveConstructionPolicyPressureCase,
    left_row: PrimitiveConstructionPolicyPressureRow,
    right_row: PrimitiveConstructionPolicyPressureRow,
    delta_verified: bool,
    row_digest: String,
}

impl PrimitiveConstructionPolicyPressureDeltaRow {
    fn new(
        case: PrimitiveConstructionPolicyPressureDeltaCase,
        report: &PrimitiveConstructionPolicyPressureSurfaceReport,
    ) -> Result<Self, PrimitiveConstructionPolicyPressureDeltaReportError> {
        let (left_case, right_case) = case.fixture();
        let left_row = report
            .row(left_case)
            .cloned()
            .ok_or(PrimitiveConstructionPolicyPressureDeltaReportError::MissingRow(left_case))?;
        let right_row = report
            .row(right_case)
            .cloned()
            .ok_or(PrimitiveConstructionPolicyPressureDeltaReportError::MissingRow(right_case))?;
        let setup = left_row.setup();
        let delta_verified = setup == right_row.setup()
            && left_row.setup_digest() == right_row.setup_digest()
            && case.delta_holds(&left_row, &right_row);
        if !delta_verified {
            return Err(PrimitiveConstructionPolicyPressureDeltaReportError::DeltaDrift(case));
        }
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            left_row.row_digest().to_string(),
            right_row.row_digest().to_string(),
            delta_verified.to_string(),
        ]);
        Ok(Self {
            case,
            setup,
            left_case,
            right_case,
            left_row,
            right_row,
            delta_verified,
            row_digest,
        })
    }

    pub fn case(&self) -> PrimitiveConstructionPolicyPressureDeltaCase {
        self.case
    }

    pub fn setup(&self) -> PrimitiveConstructionPolicyPressureSetup {
        self.setup
    }

    pub fn left_case(&self) -> PrimitiveConstructionPolicyPressureCase {
        self.left_case
    }

    pub fn right_case(&self) -> PrimitiveConstructionPolicyPressureCase {
        self.right_case
    }

    pub fn left_row(&self) -> &PrimitiveConstructionPolicyPressureRow {
        &self.left_row
    }

    pub fn right_row(&self) -> &PrimitiveConstructionPolicyPressureRow {
        &self.right_row
    }

    pub fn delta_verified(&self) -> bool {
        self.delta_verified
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl PrimitiveConstructionPolicyPressureDeltaCase {
    fn fixture(
        &self,
    ) -> (
        PrimitiveConstructionPolicyPressureCase,
        PrimitiveConstructionPolicyPressureCase,
    ) {
        match self {
            Self::GrazingClarificationVsPreservedAmbiguity => (
                PrimitiveConstructionPolicyPressureCase::GrazingAskFirst,
                PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity,
            ),
            Self::GrazingClarificationVsAggressiveSnap => (
                PrimitiveConstructionPolicyPressureCase::GrazingAskFirst,
                PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap,
            ),
            Self::GrazingAggressiveSnapVsHighFidelity => (
                PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap,
                PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity,
            ),
            Self::HostFaceAskFirstVsBimHostFriendly => (
                PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst,
                PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly,
            ),
            Self::HostFaceBimHostFriendlyVsHighFidelityAskFirst => (
                PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly,
                PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst,
            ),
        }
    }

    fn delta_holds(
        &self,
        left: &PrimitiveConstructionPolicyPressureRow,
        right: &PrimitiveConstructionPolicyPressureRow,
    ) -> bool {
        match self {
            Self::GrazingClarificationVsPreservedAmbiguity => {
                left.escalation() == SpatialIntentEscalation::AskForClarification
                    && right.escalation() == SpatialIntentEscalation::PreserveCandidates
                    && left.arbitration_posture() != right.arbitration_posture()
                    && left.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldRequireClarification
                    && right.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldPreserveCandidates
                    && left.continuity_class() == right.continuity_class()
                    && left.preview_richness() == right.preview_richness()
            }
            Self::GrazingClarificationVsAggressiveSnap => {
                left.escalation() == SpatialIntentEscalation::AskForClarification
                    && right.escalation()
                        == SpatialIntentEscalation::AutoResolve(
                            crate::spatial_intent::SpatialIntentCandidate::SnapFlush,
                        )
                    && left.arbitration_posture() != right.arbitration_posture()
                    && left.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldRequireClarification
                    && right.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
                            crate::spatial_intent::SpatialIntentCandidate::SnapFlush,
                        )
                    && right.policy_resolution_authority()
                        == Some(
                            crate::spatial_intent::SpatialChosenIntentAuthority::PolicyAutoResolve,
                        )
                    && left.continuity_class()
                        == crate::spatial_intent::SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
                    && right.continuity_class()
                        == crate::spatial_intent::SpatialIdentityContinuityClass::AnchorContinuityPreserved
            }
            Self::GrazingAggressiveSnapVsHighFidelity => {
                left.escalation() == right.escalation()
                    && left.commit_disposition() == right.commit_disposition()
                    && left.continuity_class() == right.continuity_class()
                    && left.proximity_posture() == right.proximity_posture()
                    && left.alignment_posture() == right.alignment_posture()
                    && left.preview_richness() == SpatialPreviewRichness::Standard
                    && right.preview_richness() == SpatialPreviewRichness::HighFidelity
                    && !left.warnings().contains(
                        &crate::spatial_intent::SpatialIntentPreviewWarning::HighFidelityPreview,
                    )
                    && right.warnings().contains(
                        &crate::spatial_intent::SpatialIntentPreviewWarning::HighFidelityPreview,
                    )
            }
            Self::HostFaceAskFirstVsBimHostFriendly => {
                left.escalation()
                    == SpatialIntentEscalation::BlockedByMissingCapability(
                        crate::spatial_intent::SpatialBlockedCapability::Join,
                    )
                    && left.clarification_blocked_capability()
                        == Some(crate::spatial_intent::SpatialBlockedCapability::Join)
                    && left.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(
                            crate::spatial_intent::SpatialBlockedCapability::Join,
                        )
                    && left.continuity_class()
                        == crate::spatial_intent::SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
                    && right.escalation()
                    == SpatialIntentEscalation::AutoResolve(
                        crate::spatial_intent::SpatialIntentCandidate::AttachRelationally,
                    )
                    && right.policy_resolution_authority()
                        == Some(
                            crate::spatial_intent::SpatialChosenIntentAuthority::PolicyAutoResolve,
                        )
                    && left.arbitration_posture() != right.arbitration_posture()
                    && right.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
                            crate::spatial_intent::SpatialIntentCandidate::AttachRelationally,
                        )
                    && right.continuity_class()
                        == crate::spatial_intent::SpatialIdentityContinuityClass::IdentityReinterpreted
                    && left.preview_richness() == SpatialPreviewRichness::Standard
                    && right.preview_richness() == SpatialPreviewRichness::Standard
            }
            Self::HostFaceBimHostFriendlyVsHighFidelityAskFirst => {
                left.escalation()
                    == SpatialIntentEscalation::AutoResolve(
                        crate::spatial_intent::SpatialIntentCandidate::AttachRelationally,
                    )
                    && left.policy_resolution_authority()
                        == Some(
                            crate::spatial_intent::SpatialChosenIntentAuthority::PolicyAutoResolve,
                        )
                    && left.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
                            crate::spatial_intent::SpatialIntentCandidate::AttachRelationally,
                        )
                    && left.continuity_class()
                        == crate::spatial_intent::SpatialIdentityContinuityClass::IdentityReinterpreted
                    && right.escalation()
                        == SpatialIntentEscalation::BlockedByMissingCapability(
                            crate::spatial_intent::SpatialBlockedCapability::Join,
                        )
                    && right.clarification_blocked_capability()
                        == Some(crate::spatial_intent::SpatialBlockedCapability::Join)
                    && left.arbitration_posture() != right.arbitration_posture()
                    && right.commit_disposition()
                        == SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(
                            crate::spatial_intent::SpatialBlockedCapability::Join,
                        )
                    && right.continuity_class()
                        == crate::spatial_intent::SpatialIdentityContinuityClass::IdentityBlockedPendingChoice
                    && left.preview_richness() == SpatialPreviewRichness::Standard
                    && right.preview_richness() == SpatialPreviewRichness::HighFidelity
                    && !left.warnings().contains(
                        &crate::spatial_intent::SpatialIntentPreviewWarning::HighFidelityPreview,
                    )
                    && right.warnings().contains(
                        &crate::spatial_intent::SpatialIntentPreviewWarning::HighFidelityPreview,
                    )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureDeltaReport {
    direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
    rows: Vec<PrimitiveConstructionPolicyPressureDeltaRow>,
    delta_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPolicyPressureDeltaReport {
    fn new(
        direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
        rows: Vec<PrimitiveConstructionPolicyPressureDeltaRow>,
    ) -> Self {
        let delta_verified = direct_report.pressure_verified()
            && rows
                .iter()
                .all(PrimitiveConstructionPolicyPressureDeltaRow::delta_verified);
        let report_digest = digest_owned_parts(&[
            direct_report.report_digest().to_string(),
            rows.iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>()
                .join("|"),
            delta_verified.to_string(),
        ]);
        Self {
            direct_report,
            rows,
            delta_verified,
            report_digest,
        }
    }

    pub fn direct_report(&self) -> &PrimitiveConstructionPolicyPressureSurfaceReport {
        &self.direct_report
    }

    pub fn rows(&self) -> &[PrimitiveConstructionPolicyPressureDeltaRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionPolicyPressureDeltaCase,
    ) -> Option<&PrimitiveConstructionPolicyPressureDeltaRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn delta_verified(&self) -> bool {
        self.delta_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPolicyPressureDeltaReportError {
    Pressure(PrimitiveConstructionPolicyPressureSurfaceReportError),
    MissingRow(PrimitiveConstructionPolicyPressureCase),
    DeltaDrift(PrimitiveConstructionPolicyPressureDeltaCase),
}

impl std::fmt::Display for PrimitiveConstructionPolicyPressureDeltaReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pressure(error) => write!(f, "{error}"),
            Self::MissingRow(case) => write!(f, "missing policy pressure row for {case:?}"),
            Self::DeltaDrift(case) => write!(f, "policy pressure delta drift for {case:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyPressureDeltaReportError {}

pub fn prepare_primitive_construction_policy_pressure_delta_report() -> Result<
    PrimitiveConstructionPolicyPressureDeltaReport,
    PrimitiveConstructionPolicyPressureDeltaReportError,
> {
    let direct_report = prepare_primitive_construction_policy_pressure_report()
        .map_err(PrimitiveConstructionPolicyPressureDeltaReportError::Pressure)?;
    prepare_primitive_construction_policy_pressure_delta_report_from_direct_report(direct_report)
}

pub(super) fn prepare_primitive_construction_policy_pressure_delta_report_from_direct_report(
    direct_report: PrimitiveConstructionPolicyPressureSurfaceReport,
) -> Result<
    PrimitiveConstructionPolicyPressureDeltaReport,
    PrimitiveConstructionPolicyPressureDeltaReportError,
> {
    let rows = [
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
    ]
    .into_iter()
    .map(|case| PrimitiveConstructionPolicyPressureDeltaRow::new(case, &direct_report))
    .collect::<Result<Vec<_>, _>>()?;
    Ok(PrimitiveConstructionPolicyPressureDeltaReport::new(
        direct_report,
        rows,
    ))
}
