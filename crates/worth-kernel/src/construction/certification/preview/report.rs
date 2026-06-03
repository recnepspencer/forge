use crate::construction::digest::digest_owned_parts;
use crate::spatial_intent::{
    PrimitiveIntentPreviewAssessment, SpatialAuthoredActKind, SpatialBlockedCapability,
    SpatialIntentCandidate, SpatialIntentCapabilitySet, SpatialIntentConflictClass,
    SpatialIntentPolicyProfile, SpatialIntentPreviewCommitDisposition, SpatialIntentPreviewWarning,
    SpatialObservedRelationFact, SpatialPreviewRichness,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreviewCase {
    GrazingAskFirst,
    GrazingAggressiveSnap,
    HostFaceBimAttach,
    OverlapBlockedMerge,
    OverlapHighFidelity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewRow {
    case: PrimitiveConstructionPreviewCase,
    profile_name: &'static str,
    authored_act: SpatialAuthoredActKind,
    proximity_posture: worth_spatial::facade::arbitration::SpatialThresholdPosture,
    alignment_posture: worth_spatial::facade::arbitration::SpatialThresholdPosture,
    conflict_class: SpatialIntentConflictClass,
    commit_disposition: SpatialIntentPreviewCommitDisposition,
    preview_richness: SpatialPreviewRichness,
    candidates: Vec<SpatialIntentCandidate>,
    blocked_candidates: Vec<(SpatialIntentCandidate, SpatialBlockedCapability)>,
    warnings: Vec<SpatialIntentPreviewWarning>,
    row_digest: String,
}

impl PrimitiveConstructionPreviewRow {
    fn new(case: PrimitiveConstructionPreviewCase) -> Self {
        let (authored_act, observed_relations, capabilities, profile) = case.fixture();
        let preview = PrimitiveIntentPreviewAssessment::analyze_with_capabilities(
            authored_act,
            &observed_relations,
            capabilities,
            profile,
        );
        let analysis = preview.analysis();
        let candidates = analysis
            .candidates()
            .iter()
            .map(|candidate| candidate.candidate())
            .collect::<Vec<_>>();
        let blocked_candidates = analysis
            .candidates()
            .iter()
            .filter_map(|candidate| match candidate.availability() {
                worth_spatial::facade::arbitration::SpatialIntentCandidateAvailability::Available => None,
                worth_spatial::facade::arbitration::SpatialIntentCandidateAvailability::Blocked(blocked) => {
                    Some((candidate.candidate(), blocked))
                }
            })
            .collect::<Vec<_>>();
        let warnings = preview.warnings().to_vec();
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            profile.name().to_string(),
            authored_act.as_str().to_string(),
            format!("{:?}", analysis.conflict_class()),
            format!("{:?}", preview.commit_disposition()),
            format!("{:?}", preview.preview_richness()),
            format!("{candidates:?}"),
            format!("{blocked_candidates:?}"),
            format!("{warnings:?}"),
        ]);
        Self {
            case,
            profile_name: profile.name(),
            authored_act,
            proximity_posture: profile.proximity_posture(),
            alignment_posture: profile.alignment_posture(),
            conflict_class: analysis.conflict_class(),
            commit_disposition: preview.commit_disposition(),
            preview_richness: preview.preview_richness(),
            candidates,
            blocked_candidates,
            warnings,
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPreviewCase {
        self.case
    }

    pub fn profile_name(&self) -> &'static str {
        self.profile_name
    }

    pub fn authored_act(&self) -> SpatialAuthoredActKind {
        self.authored_act
    }

    pub fn proximity_posture(&self) -> worth_spatial::facade::arbitration::SpatialThresholdPosture {
        self.proximity_posture
    }

    pub fn alignment_posture(&self) -> worth_spatial::facade::arbitration::SpatialThresholdPosture {
        self.alignment_posture
    }

    pub fn conflict_class(&self) -> SpatialIntentConflictClass {
        self.conflict_class
    }

    pub fn commit_disposition(&self) -> SpatialIntentPreviewCommitDisposition {
        self.commit_disposition
    }

    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.preview_richness
    }

    pub fn candidates(&self) -> &[SpatialIntentCandidate] {
        &self.candidates
    }

    pub fn blocked_candidates(&self) -> &[(SpatialIntentCandidate, SpatialBlockedCapability)] {
        &self.blocked_candidates
    }

    pub fn warnings(&self) -> &[SpatialIntentPreviewWarning] {
        &self.warnings
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(crate) fn prepare_primitive_construction_preview_row(
    case: PrimitiveConstructionPreviewCase,
) -> PrimitiveConstructionPreviewRow {
    PrimitiveConstructionPreviewRow::new(case)
}

impl PrimitiveConstructionPreviewCase {
    fn fixture(
        &self,
    ) -> (
        SpatialAuthoredActKind,
        Vec<SpatialObservedRelationFact>,
        SpatialIntentCapabilitySet,
        SpatialIntentPolicyProfile,
    ) {
        match self {
            Self::GrazingAskFirst => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::GrazingContact],
                SpatialIntentCapabilitySet::blocked_defaults(),
                SpatialIntentPolicyProfile::ask_first_arbitration(),
            ),
            Self::GrazingAggressiveSnap => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::GrazingContact],
                SpatialIntentCapabilitySet::blocked_defaults(),
                SpatialIntentPolicyProfile::aggressive_snap(),
            ),
            Self::HostFaceBimAttach => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::HostFaceContact],
                SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
                SpatialIntentPolicyProfile::bim_host_friendly(),
            ),
            Self::OverlapBlockedMerge => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::Overlap],
                SpatialIntentCapabilitySet::blocked_defaults(),
                SpatialIntentPolicyProfile::conservative_exact_modeling(),
            ),
            Self::OverlapHighFidelity => (
                SpatialAuthoredActKind::Move,
                vec![SpatialObservedRelationFact::Overlap],
                SpatialIntentCapabilitySet::blocked_defaults().with_merge_boolean(),
                SpatialIntentPolicyProfile::high_fidelity_preview(),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewSurfaceReport {
    rows: Vec<PrimitiveConstructionPreviewRow>,
    report_digest: String,
}

impl PrimitiveConstructionPreviewSurfaceReport {
    fn new(rows: Vec<PrimitiveConstructionPreviewRow>) -> Self {
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

    pub fn rows(&self) -> &[PrimitiveConstructionPreviewRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionPreviewCase,
    ) -> Option<&PrimitiveConstructionPreviewRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPreviewSurfaceReportError {}

impl std::fmt::Display for PrimitiveConstructionPreviewSurfaceReportError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {}
    }
}

impl std::error::Error for PrimitiveConstructionPreviewSurfaceReportError {}

pub fn prepare_primitive_construction_preview_surface_report(
) -> Result<PrimitiveConstructionPreviewSurfaceReport, PrimitiveConstructionPreviewSurfaceReportError>
{
    Ok(PrimitiveConstructionPreviewSurfaceReport::new(vec![
        prepare_primitive_construction_preview_row(
            PrimitiveConstructionPreviewCase::GrazingAskFirst,
        ),
        prepare_primitive_construction_preview_row(
            PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
        ),
        prepare_primitive_construction_preview_row(
            PrimitiveConstructionPreviewCase::HostFaceBimAttach,
        ),
        prepare_primitive_construction_preview_row(
            PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
        ),
        prepare_primitive_construction_preview_row(
            PrimitiveConstructionPreviewCase::OverlapHighFidelity,
        ),
    ]))
}
