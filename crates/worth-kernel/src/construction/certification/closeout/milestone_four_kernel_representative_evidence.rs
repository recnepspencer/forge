use forge_query::facade::ForgeQueryWorkspace;

use crate::construction::certification::continuity::{
    PrimitiveConstructionContinuityCase, PrimitiveConstructionContinuityHostilitySuiteReport,
    PrimitiveConstructionContinuityRow,
};
use crate::construction::certification::preview::{
    PrimitiveConstructionPreviewCase, PrimitiveConstructionPreviewHostilitySuiteReport,
    PrimitiveConstructionPreviewRow,
};
use crate::construction::certification::profile::{
    PrimitiveConstructionPolicyProfileCase, PrimitiveConstructionPolicyProfileRow,
    PrimitiveConstructionPreviewContinuityHostilityRow,
    PrimitiveConstructionPreviewContinuityHostilitySuiteReport,
};
use crate::construction::continuity_branch_runtime::{
    prepare_primitive_construction_continuity_branch_preview_runtime_report,
    PrimitiveConstructionContinuityBranchPreviewRuntimeError,
    PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
};
use crate::construction::continuity_replay::{
    prepare_primitive_construction_continuity_replay_parity_report,
    PrimitiveConstructionContinuityReplayParityError,
    PrimitiveConstructionContinuityReplayParityReport,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::preview_branch_runtime::{
    prepare_primitive_construction_preview_branch_preview_runtime_report,
    PrimitiveConstructionPreviewBranchPreviewRuntimeError,
    PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
};
use crate::construction::preview_replay::{
    prepare_primitive_construction_preview_replay_parity_report,
    PrimitiveConstructionPreviewReplayParityError, PrimitiveConstructionPreviewReplayParityReport,
};
use crate::construction::profile_branch_runtime::{
    prepare_primitive_construction_policy_profile_branch_preview_runtime_report,
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError,
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
};
use crate::construction::profile_replay::{
    prepare_primitive_construction_policy_profile_replay_parity_report,
    PrimitiveConstructionPolicyProfileReplayParityError,
    PrimitiveConstructionPolicyProfileReplayParityReport,
};
use crate::construction::query::continuity::{
    prepare_primitive_construction_query_continuity_inspection_parity_report,
    prepare_primitive_construction_query_continuity_projection_consumption_receipt_report,
    PrimitiveConstructionQueryContinuityParityError,
    PrimitiveConstructionQueryContinuityParityReport,
};
use crate::construction::query::preview::{
    prepare_primitive_construction_query_preview_inspection_parity_report,
    prepare_primitive_construction_query_preview_projection_consumption_receipt_report,
    PrimitiveConstructionQueryPreviewParityError, PrimitiveConstructionQueryPreviewParityReport,
};
use crate::construction::query::profile::{
    prepare_primitive_construction_query_policy_profile_inspection_parity_report,
    prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report,
    PrimitiveConstructionQueryPolicyProfileParityError,
    PrimitiveConstructionQueryPolicyProfileParityReport,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct PrimitiveConstructionPreviewRepresentativeEvidence {
    case: PrimitiveConstructionPreviewCase,
    preview_row: PrimitiveConstructionPreviewRow,
    replay_report: PrimitiveConstructionPreviewReplayParityReport,
    inspection_report: PrimitiveConstructionQueryPreviewParityReport,
    projection_report: PrimitiveConstructionQueryPreviewParityReport,
    branch_runtime_report: PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPreviewRepresentativeEvidence {
    fn new(
        case: PrimitiveConstructionPreviewCase,
        preview_row: PrimitiveConstructionPreviewRow,
        replay_report: PrimitiveConstructionPreviewReplayParityReport,
        inspection_report: PrimitiveConstructionQueryPreviewParityReport,
        projection_report: PrimitiveConstructionQueryPreviewParityReport,
        branch_runtime_report: PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
    ) -> Self {
        let parity_verified = replay_report.parity_verified()
            && inspection_report.parity_verified()
            && projection_report.parity_verified()
            && inspection_report.candidates() == projection_report.candidates()
            && inspection_report.blocked_candidates() == projection_report.blocked_candidates()
            && inspection_report.warnings() == projection_report.warnings();
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            preview_row.row_digest().to_string(),
            replay_report.report_digest().to_string(),
            branch_runtime_report.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            case,
            preview_row,
            replay_report,
            inspection_report,
            projection_report,
            branch_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    pub(super) fn case(&self) -> PrimitiveConstructionPreviewCase {
        self.case
    }

    #[cfg(test)]
    pub(super) fn preview_row(&self) -> &PrimitiveConstructionPreviewRow {
        &self.preview_row
    }

    pub(super) fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub(super) fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(super) fn prepare_preview_representative_evidence(
    suite: &PrimitiveConstructionPreviewHostilitySuiteReport,
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPreviewCase,
) -> Result<
    PrimitiveConstructionPreviewRepresentativeEvidence,
    PrimitiveConstructionPreviewRepresentativeEvidenceError,
> {
    let preview_row = suite
        .row(case)
        .cloned()
        .ok_or(PrimitiveConstructionPreviewRepresentativeEvidenceError::MissingRow(case))?;
    let replay_report = prepare_primitive_construction_preview_replay_parity_report(case)
        .map_err(PrimitiveConstructionPreviewRepresentativeEvidenceError::Replay)?;
    let inspection_report = prepare_primitive_construction_query_preview_inspection_parity_report(
        workspace,
        preview_row.clone(),
    )
    .map_err(PrimitiveConstructionPreviewRepresentativeEvidenceError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_preview_projection_consumption_receipt_report(
            workspace,
            preview_row.clone(),
        )
        .map_err(PrimitiveConstructionPreviewRepresentativeEvidenceError::Projection)?;
    let branch_runtime_report =
        prepare_primitive_construction_preview_branch_preview_runtime_report(workspace, case)
            .map_err(PrimitiveConstructionPreviewRepresentativeEvidenceError::BranchRuntime)?;
    Ok(PrimitiveConstructionPreviewRepresentativeEvidence::new(
        case,
        preview_row,
        replay_report,
        inspection_report,
        projection_report,
        branch_runtime_report,
    ))
}

#[derive(Debug)]
pub(super) enum PrimitiveConstructionPreviewRepresentativeEvidenceError {
    MissingRow(PrimitiveConstructionPreviewCase),
    Replay(PrimitiveConstructionPreviewReplayParityError),
    Inspection(PrimitiveConstructionQueryPreviewParityError),
    Projection(PrimitiveConstructionQueryPreviewParityError),
    BranchRuntime(PrimitiveConstructionPreviewBranchPreviewRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionPreviewRepresentativeEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRow(case) => write!(f, "missing preview hostility row for {case:?}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::BranchRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPreviewRepresentativeEvidenceError {}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct PrimitiveConstructionContinuityRepresentativeEvidence {
    case: PrimitiveConstructionContinuityCase,
    continuity_row: PrimitiveConstructionContinuityRow,
    replay_report: PrimitiveConstructionContinuityReplayParityReport,
    inspection_report: PrimitiveConstructionQueryContinuityParityReport,
    projection_report: PrimitiveConstructionQueryContinuityParityReport,
    branch_runtime_report: PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionContinuityRepresentativeEvidence {
    fn new(
        case: PrimitiveConstructionContinuityCase,
        continuity_row: PrimitiveConstructionContinuityRow,
        replay_report: PrimitiveConstructionContinuityReplayParityReport,
        inspection_report: PrimitiveConstructionQueryContinuityParityReport,
        projection_report: PrimitiveConstructionQueryContinuityParityReport,
        branch_runtime_report: PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
    ) -> Self {
        let parity_verified = replay_report.parity_verified()
            && inspection_report.parity_verified()
            && projection_report.parity_verified()
            && inspection_report.continuity_class() == projection_report.continuity_class()
            && inspection_report.source() == projection_report.source()
            && inspection_report.candidate() == projection_report.candidate()
            && inspection_report.blocked_capability() == projection_report.blocked_capability()
            && inspection_report.preserves_subject_identity()
                == projection_report.preserves_subject_identity()
            && inspection_report.preserves_anchor_identity()
                == projection_report.preserves_anchor_identity();
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            continuity_row.row_digest().to_string(),
            replay_report.report_digest().to_string(),
            branch_runtime_report.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            case,
            continuity_row,
            replay_report,
            inspection_report,
            projection_report,
            branch_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    pub(super) fn case(&self) -> PrimitiveConstructionContinuityCase {
        self.case
    }

    #[cfg(test)]
    pub(super) fn continuity_row(&self) -> &PrimitiveConstructionContinuityRow {
        &self.continuity_row
    }

    pub(super) fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub(super) fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(super) fn prepare_continuity_representative_evidence(
    suite: &PrimitiveConstructionContinuityHostilitySuiteReport,
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionContinuityCase,
) -> Result<
    PrimitiveConstructionContinuityRepresentativeEvidence,
    PrimitiveConstructionContinuityRepresentativeEvidenceError,
> {
    let continuity_row = suite
        .row(case)
        .cloned()
        .ok_or(PrimitiveConstructionContinuityRepresentativeEvidenceError::MissingRow(case))?;
    let replay_report = prepare_primitive_construction_continuity_replay_parity_report(case)
        .map_err(PrimitiveConstructionContinuityRepresentativeEvidenceError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_continuity_inspection_parity_report(
            workspace,
            continuity_row.clone(),
        )
        .map_err(PrimitiveConstructionContinuityRepresentativeEvidenceError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_continuity_projection_consumption_receipt_report(
            workspace,
            continuity_row.clone(),
        )
        .map_err(PrimitiveConstructionContinuityRepresentativeEvidenceError::Projection)?;
    let branch_runtime_report =
        prepare_primitive_construction_continuity_branch_preview_runtime_report(workspace, case)
            .map_err(PrimitiveConstructionContinuityRepresentativeEvidenceError::BranchRuntime)?;
    Ok(PrimitiveConstructionContinuityRepresentativeEvidence::new(
        case,
        continuity_row,
        replay_report,
        inspection_report,
        projection_report,
        branch_runtime_report,
    ))
}

#[derive(Debug)]
pub(super) enum PrimitiveConstructionContinuityRepresentativeEvidenceError {
    MissingRow(PrimitiveConstructionContinuityCase),
    Replay(PrimitiveConstructionContinuityReplayParityError),
    Inspection(PrimitiveConstructionQueryContinuityParityError),
    Projection(PrimitiveConstructionQueryContinuityParityError),
    BranchRuntime(PrimitiveConstructionContinuityBranchPreviewRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionContinuityRepresentativeEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRow(case) => write!(f, "missing continuity hostility row for {case:?}"),
            Self::Replay(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::BranchRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionContinuityRepresentativeEvidenceError {}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct PrimitiveConstructionPolicyProfileRepresentativeEvidence {
    case: PrimitiveConstructionPolicyProfileCase,
    profile_row: PrimitiveConstructionPolicyProfileRow,
    hostility_row: PrimitiveConstructionPreviewContinuityHostilityRow,
    replay_report: PrimitiveConstructionPolicyProfileReplayParityReport,
    inspection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
    projection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
    branch_runtime_report: PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPolicyProfileRepresentativeEvidence {
    fn new(
        case: PrimitiveConstructionPolicyProfileCase,
        profile_row: PrimitiveConstructionPolicyProfileRow,
        hostility_row: PrimitiveConstructionPreviewContinuityHostilityRow,
        replay_report: PrimitiveConstructionPolicyProfileReplayParityReport,
        inspection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
        projection_report: PrimitiveConstructionQueryPolicyProfileParityReport,
        branch_runtime_report: PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
    ) -> Self {
        let parity_verified = replay_report.parity_verified()
            && inspection_report.parity_verified()
            && projection_report.parity_verified()
            && inspection_report.profile_name() == projection_report.profile_name()
            && inspection_report.proximity_posture() == projection_report.proximity_posture()
            && inspection_report.alignment_posture() == projection_report.alignment_posture()
            && inspection_report.arbitration_posture() == projection_report.arbitration_posture()
            && inspection_report.preview_richness() == projection_report.preview_richness()
            && hostility_row.profile_case() == profile_row.case()
            && hostility_row.preview_case() == profile_row.representative_preview_case()
            && hostility_row.continuity_case() == profile_row.representative_continuity_case()
            && branch_runtime_report.profile_row().profile_name() == profile_row.profile_name();
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            profile_row.row_digest().to_string(),
            hostility_row.row_digest().to_string(),
            replay_report.report_digest().to_string(),
            inspection_report.report_digest().to_string(),
            projection_report.report_digest().to_string(),
            branch_runtime_report.report_digest().to_string(),
            parity_verified.to_string(),
        ]);
        Self {
            case,
            profile_row,
            hostility_row,
            replay_report,
            inspection_report,
            projection_report,
            branch_runtime_report,
            parity_verified,
            report_digest,
        }
    }

    pub(super) fn case(&self) -> PrimitiveConstructionPolicyProfileCase {
        self.case
    }

    #[cfg(test)]
    pub(super) fn profile_row(&self) -> &PrimitiveConstructionPolicyProfileRow {
        &self.profile_row
    }

    #[cfg(test)]
    pub(super) fn hostility_row(&self) -> &PrimitiveConstructionPreviewContinuityHostilityRow {
        &self.hostility_row
    }

    pub(super) fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub(super) fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub(super) fn prepare_policy_profile_representative_evidence(
    suite: &PrimitiveConstructionPreviewContinuityHostilitySuiteReport,
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPolicyProfileCase,
    profile_row: PrimitiveConstructionPolicyProfileRow,
) -> Result<
    PrimitiveConstructionPolicyProfileRepresentativeEvidence,
    PrimitiveConstructionPolicyProfileRepresentativeEvidenceError,
> {
    let hostility_row = suite
        .rows()
        .iter()
        .find(|row| row.profile_case() == case)
        .cloned()
        .ok_or(PrimitiveConstructionPolicyProfileRepresentativeEvidenceError::MissingRow(case))?;
    let replay_report = prepare_primitive_construction_policy_profile_replay_parity_report(case)
        .map_err(PrimitiveConstructionPolicyProfileRepresentativeEvidenceError::Replay)?;
    let inspection_report =
        prepare_primitive_construction_query_policy_profile_inspection_parity_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileRepresentativeEvidenceError::Inspection)?;
    let projection_report =
        prepare_primitive_construction_query_policy_profile_projection_consumption_receipt_report(
            workspace,
            profile_row.clone(),
        )
        .map_err(PrimitiveConstructionPolicyProfileRepresentativeEvidenceError::Projection)?;
    let branch_runtime_report =
        prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
            workspace, case,
        )
        .map_err(PrimitiveConstructionPolicyProfileRepresentativeEvidenceError::BranchRuntime)?;
    Ok(
        PrimitiveConstructionPolicyProfileRepresentativeEvidence::new(
            case,
            profile_row,
            hostility_row,
            replay_report,
            inspection_report,
            projection_report,
            branch_runtime_report,
        ),
    )
}

#[derive(Debug)]
pub(super) enum PrimitiveConstructionPolicyProfileRepresentativeEvidenceError {
    MissingRow(PrimitiveConstructionPolicyProfileCase),
    Replay(PrimitiveConstructionPolicyProfileReplayParityError),
    Inspection(PrimitiveConstructionQueryPolicyProfileParityError),
    Projection(PrimitiveConstructionQueryPolicyProfileParityError),
    BranchRuntime(PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionPolicyProfileRepresentativeEvidenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRow(case) => {
                write!(f, "missing policy profile hostility row for {case:?}")
            }
            Self::Replay(error) => write!(f, "{error}"),
            Self::Inspection(error) => write!(f, "{error}"),
            Self::Projection(error) => write!(f, "{error}"),
            Self::BranchRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyProfileRepresentativeEvidenceError {}

#[cfg(test)]
mod tests {
    use super::{
        prepare_continuity_representative_evidence, prepare_policy_profile_representative_evidence,
        prepare_preview_representative_evidence,
    };
    use crate::construction::certification::continuity::{
        prepare_primitive_construction_continuity_hostility_suite_report,
        PrimitiveConstructionContinuityCase,
    };
    use crate::construction::certification::preview::{
        prepare_primitive_construction_preview_hostility_suite_report,
        PrimitiveConstructionPreviewCase,
    };
    use crate::construction::certification::profile::{
        prepare_primitive_construction_policy_profile_report,
        prepare_primitive_construction_preview_continuity_hostility_suite_report,
        PrimitiveConstructionPolicyProfileCase,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn representative_evidence_surfaces_replace_preview_and_continuity_bundles() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.representative-evidence".to_string(),
        )
        .expect("workspace");

        let preview = prepare_preview_representative_evidence(
            &prepare_primitive_construction_preview_hostility_suite_report()
                .expect("preview suite"),
            &mut workspace,
            PrimitiveConstructionPreviewCase::OverlapHighFidelity,
        )
        .expect("preview evidence");
        let continuity = prepare_continuity_representative_evidence(
            &prepare_primitive_construction_continuity_hostility_suite_report()
                .expect("continuity suite"),
            &mut workspace,
            PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
        )
        .expect("continuity evidence");

        assert!(preview.parity_verified());
        assert!(continuity.parity_verified());
        assert_ne!(preview.report_digest(), preview.preview_row().row_digest());
        assert_ne!(
            continuity.report_digest(),
            continuity.continuity_row().row_digest()
        );
    }

    #[test]
    fn representative_evidence_surfaces_replace_policy_profile_bundle() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.representative-policy-profile".to_string(),
        )
        .expect("workspace");
        let profile_row = prepare_primitive_construction_policy_profile_report()
            .row(PrimitiveConstructionPolicyProfileCase::HighFidelityPreview)
            .expect("profile row")
            .clone();

        let evidence = prepare_policy_profile_representative_evidence(
            &prepare_primitive_construction_preview_continuity_hostility_suite_report()
                .expect("combined suite"),
            &mut workspace,
            PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
            profile_row,
        )
        .expect("policy evidence");

        assert!(evidence.parity_verified());
        assert_eq!(
            evidence.hostility_row().preview_case(),
            PrimitiveConstructionPreviewCase::OverlapHighFidelity
        );
        assert_ne!(
            evidence.report_digest(),
            evidence.profile_row().row_digest()
        );
    }
}
