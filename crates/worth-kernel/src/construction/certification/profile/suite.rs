use crate::construction::digest::digest_owned_parts;
use crate::construction::{PrimitiveConstructionContinuityCase, PrimitiveConstructionPreviewCase};
use std::collections::BTreeSet;

use super::{
    prepare_primitive_construction_policy_profile_report, PrimitiveConstructionPolicyProfileCase,
};
use crate::construction::certification::continuity::{
    prepare_primitive_construction_continuity_surface_report, PrimitiveConstructionContinuityRow,
    PrimitiveConstructionContinuitySurfaceReportError,
};
use crate::construction::certification::preview::{
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewRow,
    PrimitiveConstructionPreviewSurfaceReportError,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreviewContinuityHostilityCase {
    OverlapBlockedPendingChoice,
    HostFaceBimAttach,
    GrazingAggressiveSnap,
    GrazingAskFirst,
    OverlapHighFidelity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewContinuityHostilityRow {
    case: PrimitiveConstructionPreviewContinuityHostilityCase,
    profile_case: PrimitiveConstructionPolicyProfileCase,
    preview_case: PrimitiveConstructionPreviewCase,
    preview_row: PrimitiveConstructionPreviewRow,
    continuity_case: Option<PrimitiveConstructionContinuityCase>,
    continuity_row: Option<PrimitiveConstructionContinuityRow>,
    row_digest: String,
}

impl PrimitiveConstructionPreviewContinuityHostilityRow {
    fn new(
        case: PrimitiveConstructionPreviewContinuityHostilityCase,
        profile_case: PrimitiveConstructionPolicyProfileCase,
        preview_case: PrimitiveConstructionPreviewCase,
        preview_row: PrimitiveConstructionPreviewRow,
        continuity_case: Option<PrimitiveConstructionContinuityCase>,
        continuity_row: Option<PrimitiveConstructionContinuityRow>,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            format!("{case:?}"),
            format!("{profile_case:?}"),
            preview_row.row_digest().to_string(),
            continuity_row
                .as_ref()
                .map(|row| row.row_digest().to_string())
                .unwrap_or_else(|| "no-continuity-row".to_string()),
        ]);
        Self {
            case,
            profile_case,
            preview_case,
            preview_row,
            continuity_case,
            continuity_row,
            row_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPreviewContinuityHostilityCase {
        self.case
    }

    pub fn profile_case(&self) -> PrimitiveConstructionPolicyProfileCase {
        self.profile_case
    }

    pub fn preview_case(&self) -> PrimitiveConstructionPreviewCase {
        self.preview_case
    }

    pub fn continuity_case(&self) -> Option<PrimitiveConstructionContinuityCase> {
        self.continuity_case
    }

    pub fn continuity_row(&self) -> Option<&PrimitiveConstructionContinuityRow> {
        self.continuity_row.as_ref()
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewContinuityHostilitySuiteReport {
    rows: Vec<PrimitiveConstructionPreviewContinuityHostilityRow>,
    suite_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPreviewContinuityHostilitySuiteReport {
    fn new(rows: Vec<PrimitiveConstructionPreviewContinuityHostilityRow>) -> Self {
        let suite_verified = !rows.is_empty()
            && rows
                .iter()
                .map(|row| row.row_digest())
                .collect::<BTreeSet<_>>()
                .len()
                == rows.len();
        let report_digest = digest_owned_parts(
            &rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            suite_verified,
            report_digest,
        }
    }

    pub fn rows(&self) -> &[PrimitiveConstructionPreviewContinuityHostilityRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionPreviewContinuityHostilityCase,
    ) -> Option<&PrimitiveConstructionPreviewContinuityHostilityRow> {
        self.rows.iter().find(|row| row.case() == case)
    }

    pub fn suite_verified(&self) -> bool {
        self.suite_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPreviewContinuityHostilitySuiteError {
    Preview(PrimitiveConstructionPreviewSurfaceReportError),
    Continuity(PrimitiveConstructionContinuitySurfaceReportError),
    MissingProfileRow(PrimitiveConstructionPolicyProfileCase),
    MissingPreviewRow(PrimitiveConstructionPreviewCase),
    MissingContinuityRow(PrimitiveConstructionContinuityCase),
}

impl std::fmt::Display for PrimitiveConstructionPreviewContinuityHostilitySuiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Preview(error) => write!(f, "{error}"),
            Self::Continuity(error) => write!(f, "{error}"),
            Self::MissingProfileRow(case) => write!(f, "missing policy profile row for {case:?}"),
            Self::MissingPreviewRow(case) => write!(f, "missing preview row for {case:?}"),
            Self::MissingContinuityRow(case) => {
                write!(f, "missing continuity row for {case:?}")
            }
        }
    }
}

impl std::error::Error for PrimitiveConstructionPreviewContinuityHostilitySuiteError {}

pub fn prepare_primitive_construction_preview_continuity_hostility_suite_report() -> Result<
    PrimitiveConstructionPreviewContinuityHostilitySuiteReport,
    PrimitiveConstructionPreviewContinuityHostilitySuiteError,
> {
    let preview_report = prepare_primitive_construction_preview_surface_report()
        .map_err(PrimitiveConstructionPreviewContinuityHostilitySuiteError::Preview)?;
    let continuity_report = prepare_primitive_construction_continuity_surface_report()
        .map_err(PrimitiveConstructionPreviewContinuityHostilitySuiteError::Continuity)?;
    let profile_report = prepare_primitive_construction_policy_profile_report();
    let cases = [
        (
            PrimitiveConstructionPreviewContinuityHostilityCase::OverlapBlockedPendingChoice,
            PrimitiveConstructionPolicyProfileCase::ConservativeExactModeling,
        ),
        (
            PrimitiveConstructionPreviewContinuityHostilityCase::HostFaceBimAttach,
            PrimitiveConstructionPolicyProfileCase::BimHostFriendly,
        ),
        (
            PrimitiveConstructionPreviewContinuityHostilityCase::GrazingAggressiveSnap,
            PrimitiveConstructionPolicyProfileCase::AggressiveSnap,
        ),
        (
            PrimitiveConstructionPreviewContinuityHostilityCase::GrazingAskFirst,
            PrimitiveConstructionPolicyProfileCase::AskFirstArbitration,
        ),
        (
            PrimitiveConstructionPreviewContinuityHostilityCase::OverlapHighFidelity,
            PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
        ),
    ];
    let rows = cases
        .into_iter()
        .map(|(case, profile_case)| {
            let profile_row = profile_report.row(profile_case).ok_or(
                PrimitiveConstructionPreviewContinuityHostilitySuiteError::MissingProfileRow(
                    profile_case,
                ),
            )?;
            let preview_case = profile_row.representative_preview_case();
            let preview_row = preview_report
                .row(preview_case)
                .cloned()
                .ok_or(PrimitiveConstructionPreviewContinuityHostilitySuiteError::MissingPreviewRow(
                    preview_case,
                ))?;
            let continuity_case = profile_row.representative_continuity_case();
            let continuity_row = continuity_case
                .map(|current_case| {
                    continuity_report.row(current_case).cloned().ok_or(
                        PrimitiveConstructionPreviewContinuityHostilitySuiteError::MissingContinuityRow(
                            current_case,
                        ),
                    )
                })
                .transpose()?;
            Ok(PrimitiveConstructionPreviewContinuityHostilityRow::new(
                case,
                profile_case,
                preview_case,
                preview_row,
                continuity_case,
                continuity_row,
            ))
        })
        .collect::<Result<Vec<_>, PrimitiveConstructionPreviewContinuityHostilitySuiteError>>()?;
    Ok(PrimitiveConstructionPreviewContinuityHostilitySuiteReport::new(rows))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_preview_continuity_hostility_suite_report,
        PrimitiveConstructionPreviewContinuityHostilityCase,
    };
    use crate::construction::PrimitiveConstructionPolicyProfileCase;

    #[test]
    fn combined_preview_continuity_hostility_suite_binds_representative_rows() {
        let suite = prepare_primitive_construction_preview_continuity_hostility_suite_report()
            .expect("suite");
        let row = suite
            .row(PrimitiveConstructionPreviewContinuityHostilityCase::HostFaceBimAttach)
            .expect("row");

        assert!(suite.suite_verified());
        assert_eq!(
            row.profile_case(),
            PrimitiveConstructionPolicyProfileCase::BimHostFriendly
        );
        assert!(row.continuity_row().is_some());
        assert_eq!(suite.rows().len(), 5);
        assert_ne!(suite.report_digest(), suite.rows()[0].row_digest());
    }
}
