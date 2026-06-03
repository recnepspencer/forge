use crate::construction::digest::digest_owned_parts;
use crate::construction::{PrimitiveConstructionContinuityCase, PrimitiveConstructionPreviewCase};
use worth_spatial::facade::arbitration::{
    SpatialArbitrationPosture, SpatialIntentPolicyProfile, SpatialPreviewRichness,
    SpatialThresholdPosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPolicyProfileCase {
    ConservativeExactModeling,
    BimHostFriendly,
    AskFirstArbitration,
    AggressiveSnap,
    HighFidelityPreview,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyProfileRow {
    case: PrimitiveConstructionPolicyProfileCase,
    profile_name: &'static str,
    proximity_posture: SpatialThresholdPosture,
    alignment_posture: SpatialThresholdPosture,
    arbitration_posture: SpatialArbitrationPosture,
    preview_richness: SpatialPreviewRichness,
    representative_preview_case: PrimitiveConstructionPreviewCase,
    representative_continuity_case: Option<PrimitiveConstructionContinuityCase>,
    row_digest: String,
}

impl PrimitiveConstructionPolicyProfileRow {
    fn new(case: PrimitiveConstructionPolicyProfileCase) -> Self {
        let (profile, representative_preview_case, representative_continuity_case) = case.fixture();
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            profile.name().to_string(),
            format!("{:?}", profile.proximity_posture()),
            format!("{:?}", profile.alignment_posture()),
            format!("{:?}", profile.arbitration_posture()),
            format!("{:?}", profile.preview_richness()),
            format!("{representative_preview_case:?}"),
            format!("{representative_continuity_case:?}"),
        ]);
        Self {
            case,
            profile_name: profile.name(),
            proximity_posture: profile.proximity_posture(),
            alignment_posture: profile.alignment_posture(),
            arbitration_posture: profile.arbitration_posture(),
            preview_richness: profile.preview_richness(),
            representative_preview_case,
            representative_continuity_case,
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPolicyProfileCase {
        self.case
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

    pub fn representative_preview_case(&self) -> PrimitiveConstructionPreviewCase {
        self.representative_preview_case
    }

    pub fn representative_continuity_case(&self) -> Option<PrimitiveConstructionContinuityCase> {
        self.representative_continuity_case
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl PrimitiveConstructionPolicyProfileCase {
    fn fixture(
        &self,
    ) -> (
        SpatialIntentPolicyProfile,
        PrimitiveConstructionPreviewCase,
        Option<PrimitiveConstructionContinuityCase>,
    ) {
        match self {
            Self::ConservativeExactModeling => (
                SpatialIntentPolicyProfile::conservative_exact_modeling(),
                PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
                Some(PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice),
            ),
            Self::BimHostFriendly => (
                SpatialIntentPolicyProfile::bim_host_friendly(),
                PrimitiveConstructionPreviewCase::HostFaceBimAttach,
                Some(PrimitiveConstructionContinuityCase::HostAttachReinterpreted),
            ),
            Self::AskFirstArbitration => (
                SpatialIntentPolicyProfile::ask_first_arbitration(),
                PrimitiveConstructionPreviewCase::GrazingAskFirst,
                None,
            ),
            Self::AggressiveSnap => (
                SpatialIntentPolicyProfile::aggressive_snap(),
                PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
                Some(PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity),
            ),
            Self::HighFidelityPreview => (
                SpatialIntentPolicyProfile::high_fidelity_preview(),
                PrimitiveConstructionPreviewCase::OverlapHighFidelity,
                None,
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyProfileSurfaceReport {
    rows: Vec<PrimitiveConstructionPolicyProfileRow>,
    report_digest: String,
}

impl PrimitiveConstructionPolicyProfileSurfaceReport {
    fn new(rows: Vec<PrimitiveConstructionPolicyProfileRow>) -> Self {
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

    pub fn rows(&self) -> &[PrimitiveConstructionPolicyProfileRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionPolicyProfileCase,
    ) -> Option<&PrimitiveConstructionPolicyProfileRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(crate) fn prepare_primitive_construction_policy_profile_row(
    case: PrimitiveConstructionPolicyProfileCase,
) -> PrimitiveConstructionPolicyProfileRow {
    PrimitiveConstructionPolicyProfileRow::new(case)
}

pub fn prepare_primitive_construction_policy_profile_report(
) -> PrimitiveConstructionPolicyProfileSurfaceReport {
    PrimitiveConstructionPolicyProfileSurfaceReport::new(vec![
        prepare_primitive_construction_policy_profile_row(
            PrimitiveConstructionPolicyProfileCase::ConservativeExactModeling,
        ),
        prepare_primitive_construction_policy_profile_row(
            PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
        ),
        prepare_primitive_construction_policy_profile_row(
            PrimitiveConstructionPolicyProfileCase::AskFirstArbitration,
        ),
        prepare_primitive_construction_policy_profile_row(
            PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
        ),
        prepare_primitive_construction_policy_profile_row(
            PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
        ),
    ])
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_policy_profile_report,
        PrimitiveConstructionPolicyProfileCase,
    };
    use worth_spatial::facade::arbitration::{SpatialArbitrationPosture, SpatialPreviewRichness};

    #[test]
    fn policy_profile_report_preserves_shared_profile_posture_truth() {
        let report = prepare_primitive_construction_policy_profile_report();
        let high_fidelity = report
            .row(PrimitiveConstructionPolicyProfileCase::HighFidelityPreview)
            .expect("high fidelity row");
        let aggressive = report
            .row(PrimitiveConstructionPolicyProfileCase::AggressiveSnap)
            .expect("aggressive row");

        assert_eq!(
            high_fidelity.preview_richness(),
            SpatialPreviewRichness::HighFidelity
        );
        assert_eq!(
            aggressive.arbitration_posture(),
            SpatialArbitrationPosture::PreferSnap
        );
        assert_eq!(
            aggressive.representative_continuity_case(),
            Some(
                crate::construction::PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity
            )
        );
    }
}
