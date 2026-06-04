use crate::construction::digest::digest_owned_parts;
use crate::spatial_intent::{
    PrimitiveIntentClarificationRequestError, PrimitiveIntentConflict,
    PrimitiveIntentPreviewAssessment, SpatialArbitrationPosture, SpatialBlockedCapability,
    SpatialChosenIntentAuthority, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass, SpatialIntentCandidate, SpatialIntentConflictClass,
    SpatialIntentEscalation, SpatialIntentPreviewCommitDisposition, SpatialIntentPreviewWarning,
    SpatialIntentResolutionError, SpatialPreviewRichness, SpatialThresholdPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyPressureSetup {
    GrazingContactMove,
    HostFaceAttachMove,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyPressureCase {
    GrazingAskFirst,
    GrazingPreserveAmbiguity,
    GrazingAggressiveSnap,
    GrazingAggressiveSnapHighFidelity,
    HostFaceAskFirst,
    HostFaceBimHostFriendly,
    HostFaceBimHostHighFidelityAskFirst,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureRow {
    case: PrimitiveConstructionPolicyPressureCase,
    setup: PrimitiveConstructionPolicyPressureSetup,
    setup_digest: String,
    profile_name: &'static str,
    proximity_posture: SpatialThresholdPosture,
    alignment_posture: SpatialThresholdPosture,
    arbitration_posture: SpatialArbitrationPosture,
    preview_richness: SpatialPreviewRichness,
    conflict_class: SpatialIntentConflictClass,
    escalation: SpatialIntentEscalation,
    chosen_candidate: Option<SpatialIntentCandidate>,
    policy_resolution_authority: Option<SpatialChosenIntentAuthority>,
    clarification_blocked_capability: Option<SpatialBlockedCapability>,
    commit_disposition: SpatialIntentPreviewCommitDisposition,
    continuity_class: SpatialIdentityContinuityClass,
    continuity_explanation_class: SpatialIdentityContinuityExplanationClass,
    preserves_subject_identity: bool,
    preserves_anchor_identity: bool,
    warnings: Vec<SpatialIntentPreviewWarning>,
    shared_analysis_verified: bool,
    boundary_verified: bool,
    row_digest: String,
}

impl PrimitiveConstructionPolicyPressureRow {
    fn new(
        case: PrimitiveConstructionPolicyPressureCase,
    ) -> Result<Self, PrimitiveConstructionPolicyPressureSurfaceReportError> {
        let (setup, authored_act, observed_relations, capabilities, profile) = case.fixture();
        let conflict = PrimitiveIntentConflict::analyze_with_capabilities_and_profile(
            authored_act,
            &observed_relations,
            capabilities,
            profile,
        );
        let assessment = PrimitiveIntentPreviewAssessment::analyze_with_capabilities(
            authored_act,
            &observed_relations,
            capabilities,
            profile,
        );
        let shared_analysis_verified = conflict.analysis() == assessment.analysis();
        if !shared_analysis_verified {
            return Err(PrimitiveConstructionPolicyPressureSurfaceReportError::AnalysisDrift(case));
        }

        let escalation = conflict.escalation();
        let chosen_candidate = conflict.analysis().chosen_candidate();
        let (policy_resolution_authority, clarification_blocked_capability, boundary_verified) =
            match escalation {
                SpatialIntentEscalation::AutoResolve(candidate) => {
                    let resolution = conflict.resolve_by_policy().map_err(|error| {
                        PrimitiveConstructionPolicyPressureSurfaceReportError::PolicyResolution(
                            case, error,
                        )
                    })?;
                    let clarification = conflict.clarification_request();
                    (
                        Some(resolution.authority()),
                        None,
                        resolution.chosen_candidate() == candidate
                            && clarification
                                == Err(
                                    PrimitiveIntentClarificationRequestError::NoClarificationBoundary(
                                        escalation,
                                    ),
                                ),
                    )
                }
                SpatialIntentEscalation::PreserveCandidates
                | SpatialIntentEscalation::AskForClarification
                | SpatialIntentEscalation::BlockedByMissingCapability(_) => {
                    let clarification = conflict.clarification_request().map_err(|error| {
                        PrimitiveConstructionPolicyPressureSurfaceReportError::Clarification(
                            case, error,
                        )
                    })?;
                    let expected_error = match escalation {
                        SpatialIntentEscalation::PreserveCandidates => {
                            SpatialIntentResolutionError::CandidateSetPreserved
                        }
                        SpatialIntentEscalation::AskForClarification => {
                            SpatialIntentResolutionError::ClarificationRequired
                        }
                        SpatialIntentEscalation::BlockedByMissingCapability(blocked) => {
                            SpatialIntentResolutionError::BlockedByMissingCapability(blocked)
                        }
                        SpatialIntentEscalation::AutoResolve(_) => unreachable!(),
                    };
                    (
                        None,
                        clarification.blocked_capability(),
                        clarification.escalation() == escalation
                            && conflict.resolve_by_policy() == Err(expected_error),
                    )
                }
            };
        if !boundary_verified {
            return Err(PrimitiveConstructionPolicyPressureSurfaceReportError::BoundaryDrift(case));
        }

        let warnings = assessment.warnings().to_vec();
        let continuity = assessment.continuity();
        let setup_digest = digest_owned_parts(&[
            format!("{authored_act:?}"),
            format!("{observed_relations:?}"),
            format!("{capabilities:?}"),
        ]);
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            format!("{setup:?}"),
            setup_digest.clone(),
            profile.name().to_string(),
            format!("{:?}", conflict.conflict_class()),
            format!("{:?}", escalation),
            format!("{:?}", chosen_candidate),
            format!("{:?}", policy_resolution_authority),
            format!("{:?}", clarification_blocked_capability),
            format!("{:?}", assessment.commit_disposition()),
            format!("{:?}", continuity.continuity_class()),
            format!("{:?}", continuity.explanation_class()),
            continuity.preserves_subject_identity().to_string(),
            continuity.preserves_anchor_identity().to_string(),
            format!("{warnings:?}"),
        ]);

        Ok(Self {
            case,
            setup,
            setup_digest,
            profile_name: profile.name(),
            proximity_posture: profile.proximity_posture(),
            alignment_posture: profile.alignment_posture(),
            arbitration_posture: profile.arbitration_posture(),
            preview_richness: assessment.preview_richness(),
            conflict_class: conflict.conflict_class(),
            escalation,
            chosen_candidate,
            policy_resolution_authority,
            clarification_blocked_capability,
            commit_disposition: assessment.commit_disposition(),
            continuity_class: continuity.continuity_class(),
            continuity_explanation_class: continuity.explanation_class(),
            preserves_subject_identity: continuity.preserves_subject_identity(),
            preserves_anchor_identity: continuity.preserves_anchor_identity(),
            warnings,
            shared_analysis_verified,
            boundary_verified,
            row_digest,
        })
    }

    pub fn case(&self) -> PrimitiveConstructionPolicyPressureCase {
        self.case
    }
    pub fn setup(&self) -> PrimitiveConstructionPolicyPressureSetup {
        self.setup
    }
    pub fn setup_digest(&self) -> &str {
        &self.setup_digest
    }
    pub fn profile_name(&self) -> &'static str {
        self.profile_name
    }
    pub fn proximity_posture(&self) -> SpatialThresholdPosture {
        self.proximity_posture
    }
    pub fn alignment_posture(&self) -> SpatialThresholdPosture {
        self.alignment_posture
    }
    pub fn arbitration_posture(&self) -> SpatialArbitrationPosture {
        self.arbitration_posture
    }
    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.preview_richness
    }
    pub fn conflict_class(&self) -> SpatialIntentConflictClass {
        self.conflict_class
    }
    pub fn escalation(&self) -> SpatialIntentEscalation {
        self.escalation
    }
    pub fn chosen_candidate(&self) -> Option<SpatialIntentCandidate> {
        self.chosen_candidate
    }
    pub fn policy_resolution_authority(&self) -> Option<SpatialChosenIntentAuthority> {
        self.policy_resolution_authority
    }
    pub fn clarification_blocked_capability(&self) -> Option<SpatialBlockedCapability> {
        self.clarification_blocked_capability
    }
    pub fn commit_disposition(&self) -> SpatialIntentPreviewCommitDisposition {
        self.commit_disposition
    }
    pub fn continuity_class(&self) -> SpatialIdentityContinuityClass {
        self.continuity_class
    }
    pub fn continuity_explanation_class(&self) -> SpatialIdentityContinuityExplanationClass {
        self.continuity_explanation_class
    }
    pub fn preserves_subject_identity(&self) -> bool {
        self.preserves_subject_identity
    }
    pub fn preserves_anchor_identity(&self) -> bool {
        self.preserves_anchor_identity
    }
    pub fn warnings(&self) -> &[SpatialIntentPreviewWarning] {
        &self.warnings
    }
    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyPressureSurfaceReport {
    rows: Vec<PrimitiveConstructionPolicyPressureRow>,
    pressure_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPolicyPressureSurfaceReport {
    fn new(rows: Vec<PrimitiveConstructionPolicyPressureRow>) -> Self {
        let pressure_verified = rows
            .iter()
            .all(|row| row.shared_analysis_verified && row.boundary_verified);
        let report_digest = digest_owned_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            pressure_verified,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionPolicyPressureRow] {
        &self.rows
    }
    pub fn row(
        &self,
        case: PrimitiveConstructionPolicyPressureCase,
    ) -> Option<&PrimitiveConstructionPolicyPressureRow> {
        self.rows.iter().find(|row| row.case() == case)
    }
    pub fn pressure_verified(&self) -> bool {
        self.pressure_verified
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPolicyPressureSurfaceReportError {
    AnalysisDrift(PrimitiveConstructionPolicyPressureCase),
    BoundaryDrift(PrimitiveConstructionPolicyPressureCase),
    PolicyResolution(
        PrimitiveConstructionPolicyPressureCase,
        SpatialIntentResolutionError,
    ),
    Clarification(
        PrimitiveConstructionPolicyPressureCase,
        PrimitiveIntentClarificationRequestError,
    ),
}

impl std::fmt::Display for PrimitiveConstructionPolicyPressureSurfaceReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnalysisDrift(case) => write!(f, "policy pressure analysis drift for {case:?}"),
            Self::BoundaryDrift(case) => write!(f, "policy pressure boundary drift for {case:?}"),
            Self::PolicyResolution(case, error) => {
                write!(f, "policy pressure resolution failed for {case:?}: {error}")
            }
            Self::Clarification(case, error) => {
                write!(
                    f,
                    "policy pressure clarification failed for {case:?}: {error}"
                )
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyPressureSurfaceReportError {}

pub fn prepare_primitive_construction_policy_pressure_report() -> Result<
    PrimitiveConstructionPolicyPressureSurfaceReport,
    PrimitiveConstructionPolicyPressureSurfaceReportError,
> {
    Ok(PrimitiveConstructionPolicyPressureSurfaceReport::new(vec![
        PrimitiveConstructionPolicyPressureRow::new(
            PrimitiveConstructionPolicyPressureCase::GrazingAskFirst,
        )?,
        PrimitiveConstructionPolicyPressureRow::new(
            PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity,
        )?,
        PrimitiveConstructionPolicyPressureRow::new(
            PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap,
        )?,
        PrimitiveConstructionPolicyPressureRow::new(
            PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity,
        )?,
        PrimitiveConstructionPolicyPressureRow::new(
            PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst,
        )?,
        PrimitiveConstructionPolicyPressureRow::new(
            PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly,
        )?,
        PrimitiveConstructionPolicyPressureRow::new(
            PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst,
        )?,
    ]))
}
