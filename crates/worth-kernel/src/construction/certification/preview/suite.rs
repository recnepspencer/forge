use crate::construction::digest::digest_owned_parts;
use std::collections::BTreeSet;

use super::{
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewRow, PrimitiveConstructionPreviewSurfaceReportError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewHostilitySuiteReport {
    rows: Vec<PrimitiveConstructionPreviewRow>,
    suite_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionPreviewHostilitySuiteReport {
    fn new(rows: Vec<PrimitiveConstructionPreviewRow>) -> Self {
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

    pub fn rows(&self) -> &[PrimitiveConstructionPreviewRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionPreviewCase,
    ) -> Option<&PrimitiveConstructionPreviewRow> {
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
pub enum PrimitiveConstructionPreviewHostilitySuiteError {
    Surface(PrimitiveConstructionPreviewSurfaceReportError),
    MissingRow(PrimitiveConstructionPreviewCase),
}

impl std::fmt::Display for PrimitiveConstructionPreviewHostilitySuiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surface(error) => write!(f, "{error}"),
            Self::MissingRow(case) => write!(f, "missing preview hostility row for {case:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPreviewHostilitySuiteError {}

pub fn prepare_primitive_construction_preview_hostility_suite_report() -> Result<
    PrimitiveConstructionPreviewHostilitySuiteReport,
    PrimitiveConstructionPreviewHostilitySuiteError,
> {
    let report = prepare_primitive_construction_preview_surface_report()
        .map_err(PrimitiveConstructionPreviewHostilitySuiteError::Surface)?;
    let rows = [
        PrimitiveConstructionPreviewCase::GrazingAskFirst,
        PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
        PrimitiveConstructionPreviewCase::HostFaceBimAttach,
        PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
        PrimitiveConstructionPreviewCase::OverlapHighFidelity,
    ]
    .into_iter()
    .map(|case| {
        report.row(case).cloned().ok_or(
            PrimitiveConstructionPreviewHostilitySuiteError::MissingRow(case),
        )
    })
    .collect::<Result<Vec<_>, _>>()?;
    Ok(PrimitiveConstructionPreviewHostilitySuiteReport::new(rows))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_preview_hostility_suite_report;
    use crate::construction::certification::preview::PrimitiveConstructionPreviewCase;

    #[test]
    fn preview_hostility_suite_fails_closed_on_exact_hostile_case_coverage() {
        let suite = prepare_primitive_construction_preview_hostility_suite_report().expect("suite");

        assert!(suite.suite_verified());
        assert_eq!(suite.rows().len(), 5);
        assert_eq!(
            suite
                .rows()
                .iter()
                .map(|row| row.case())
                .collect::<Vec<_>>(),
            vec![
                PrimitiveConstructionPreviewCase::GrazingAskFirst,
                PrimitiveConstructionPreviewCase::GrazingAggressiveSnap,
                PrimitiveConstructionPreviewCase::HostFaceBimAttach,
                PrimitiveConstructionPreviewCase::OverlapBlockedMerge,
                PrimitiveConstructionPreviewCase::OverlapHighFidelity,
            ]
        );
        assert_ne!(suite.report_digest(), suite.rows()[0].row_digest());
    }
}
