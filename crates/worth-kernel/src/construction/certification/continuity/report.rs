use crate::construction::digest::digest_owned_parts;
use crate::spatial_intent::{
    PrimitiveIntentConflict, PrimitiveIntentPreviewAssessment, SpatialAuthoredActKind,
    SpatialBlockedCapability, SpatialIdentityContinuityClass,
    SpatialIdentityContinuityExplanationClass, SpatialIntentCandidate, SpatialIntentCapabilitySet,
    SpatialIntentConflictClass, SpatialIntentPolicyProfile, SpatialObservedRelationFact,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionContinuityCase {
    MoveOnlyPreserved,
    GrazingSnapAnchorContinuity,
    HostAttachReinterpreted,
    OverlapBlockedPendingChoice,
    ExplicitMergeIdentityMerged,
    ExplicitCutOpeningIdentitySplit,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionContinuityResolutionSource {
    PreviewAnalysis,
    PolicyAutoResolve,
    ExplicitChoice,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionContinuityRow {
    case: PrimitiveConstructionContinuityCase,
    profile_name: &'static str,
    source: PrimitiveConstructionContinuityResolutionSource,
    conflict_class: SpatialIntentConflictClass,
    continuity_class: SpatialIdentityContinuityClass,
    explanation_class: SpatialIdentityContinuityExplanationClass,
    candidate: Option<SpatialIntentCandidate>,
    blocked_capability: Option<SpatialBlockedCapability>,
    preserves_subject_identity: bool,
    preserves_anchor_identity: bool,
    row_digest: String,
}

impl PrimitiveConstructionContinuityRow {
    fn new(
        case: PrimitiveConstructionContinuityCase,
    ) -> Result<Self, PrimitiveConstructionContinuitySurfaceReportError> {
        let (authored_act, observed_relations, capabilities, profile, explicit_choice) =
            case.fixture();
        let preview = PrimitiveIntentPreviewAssessment::analyze_with_capabilities(
            authored_act,
            &observed_relations,
            capabilities,
            profile,
        );
        let analysis = preview.analysis().clone();
        let (source, continuity) = match explicit_choice {
            Some(choice) => {
                let resolution = PrimitiveIntentConflict::analyze_with_capabilities_and_profile(
                    authored_act,
                    &observed_relations,
                    capabilities,
                    profile,
                )
                .resolve_by_choice(choice)
                .map_err(PrimitiveConstructionContinuitySurfaceReportError::Resolution)?;
                (
                    PrimitiveConstructionContinuityResolutionSource::ExplicitChoice,
                    resolution.identity_continuity_assessment(),
                )
            }
            None => {
                let continuity = preview.continuity().clone();
                let source = if analysis.chosen_candidate().is_some() {
                    PrimitiveConstructionContinuityResolutionSource::PolicyAutoResolve
                } else {
                    PrimitiveConstructionContinuityResolutionSource::PreviewAnalysis
                };
                (source, continuity)
            }
        };
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            profile.name().to_string(),
            format!("{source:?}"),
            format!("{:?}", analysis.conflict_class()),
            format!("{:?}", continuity.continuity_class()),
            format!("{:?}", continuity.explanation_class()),
            format!("{:?}", continuity.candidate()),
            format!("{:?}", continuity.blocked_capability()),
            continuity.preserves_subject_identity().to_string(),
            continuity.preserves_anchor_identity().to_string(),
        ]);
        Ok(Self {
            case,
            profile_name: profile.name(),
            source,
            conflict_class: analysis.conflict_class(),
            continuity_class: continuity.continuity_class(),
            explanation_class: continuity.explanation_class(),
            candidate: continuity.candidate(),
            blocked_capability: continuity.blocked_capability(),
            preserves_subject_identity: continuity.preserves_subject_identity(),
            preserves_anchor_identity: continuity.preserves_anchor_identity(),
            row_digest,
        })
    }

    pub fn case(&self) -> PrimitiveConstructionContinuityCase {
        self.case
    }

    pub fn profile_name(&self) -> &'static str {
        self.profile_name
    }

    pub fn source(&self) -> PrimitiveConstructionContinuityResolutionSource {
        self.source
    }

    pub fn conflict_class(&self) -> SpatialIntentConflictClass {
        self.conflict_class
    }

    pub fn continuity_class(&self) -> SpatialIdentityContinuityClass {
        self.continuity_class
    }

    pub fn explanation_class(&self) -> SpatialIdentityContinuityExplanationClass {
        self.explanation_class
    }

    pub fn candidate(&self) -> Option<SpatialIntentCandidate> {
        self.candidate
    }

    pub fn blocked_capability(&self) -> Option<SpatialBlockedCapability> {
        self.blocked_capability
    }

    pub fn preserves_subject_identity(&self) -> bool {
        self.preserves_subject_identity
    }

    pub fn preserves_anchor_identity(&self) -> bool {
        self.preserves_anchor_identity
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn prepare_primitive_construction_continuity_row(
    case: PrimitiveConstructionContinuityCase,
) -> Result<PrimitiveConstructionContinuityRow, PrimitiveConstructionContinuitySurfaceReportError> {
    PrimitiveConstructionContinuityRow::new(case)
}

impl PrimitiveConstructionContinuityCase {
    fn fixture(
        &self,
    ) -> (
        SpatialAuthoredActKind,
        Vec<SpatialObservedRelationFact>,
        SpatialIntentCapabilitySet,
        SpatialIntentPolicyProfile,
        Option<SpatialIntentCandidate>,
    ) {
        match self {
            Self::MoveOnlyPreserved => (
                SpatialAuthoredActKind::Move,
                vec![],
                SpatialIntentCapabilitySet::blocked_defaults(),
                SpatialIntentPolicyProfile::conservative_exact_modeling(),
                None,
            ),
            Self::GrazingSnapAnchorContinuity => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::GrazingContact],
                SpatialIntentCapabilitySet::blocked_defaults(),
                SpatialIntentPolicyProfile::aggressive_snap(),
                None,
            ),
            Self::HostAttachReinterpreted => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::HostFaceContact],
                SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
                SpatialIntentPolicyProfile::bim_host_friendly(),
                None,
            ),
            Self::OverlapBlockedPendingChoice => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::Overlap],
                SpatialIntentCapabilitySet::blocked_defaults(),
                SpatialIntentPolicyProfile::conservative_exact_modeling(),
                None,
            ),
            Self::ExplicitMergeIdentityMerged => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::Overlap],
                SpatialIntentCapabilitySet::blocked_defaults().with_merge_boolean(),
                SpatialIntentPolicyProfile::conservative_exact_modeling(),
                Some(SpatialIntentCandidate::MergeCandidate),
            ),
            Self::ExplicitCutOpeningIdentitySplit => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::HostPenetration],
                SpatialIntentCapabilitySet::blocked_defaults().with_cut_opening(),
                SpatialIntentPolicyProfile::conservative_exact_modeling(),
                Some(SpatialIntentCandidate::CutOpeningCandidate),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionContinuitySurfaceReport {
    rows: Vec<PrimitiveConstructionContinuityRow>,
    report_digest: String,
}

impl PrimitiveConstructionContinuitySurfaceReport {
    fn new(rows: Vec<PrimitiveConstructionContinuityRow>) -> Self {
        let report_digest = digest_owned_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionContinuityRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionContinuityCase,
    ) -> Option<&PrimitiveConstructionContinuityRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionContinuitySurfaceReportError {
    Resolution(crate::spatial_intent::SpatialIntentResolutionError),
}

impl std::fmt::Display for PrimitiveConstructionContinuitySurfaceReportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resolution(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionContinuitySurfaceReportError {}

pub fn prepare_primitive_construction_continuity_surface_report() -> Result<
    PrimitiveConstructionContinuitySurfaceReport,
    PrimitiveConstructionContinuitySurfaceReportError,
> {
    Ok(PrimitiveConstructionContinuitySurfaceReport::new(vec![
        prepare_primitive_construction_continuity_row(
            PrimitiveConstructionContinuityCase::MoveOnlyPreserved,
        )?,
        prepare_primitive_construction_continuity_row(
            PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity,
        )?,
        prepare_primitive_construction_continuity_row(
            PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
        )?,
        prepare_primitive_construction_continuity_row(
            PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
        )?,
        prepare_primitive_construction_continuity_row(
            PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
        )?,
        prepare_primitive_construction_continuity_row(
            PrimitiveConstructionContinuityCase::ExplicitCutOpeningIdentitySplit,
        )?,
    ]))
}
