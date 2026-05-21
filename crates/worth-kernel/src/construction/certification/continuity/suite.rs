use crate::construction::digest::digest_owned_parts;
use std::collections::BTreeSet;

use super::{
    prepare_primitive_construction_continuity_surface_report, PrimitiveConstructionContinuityCase,
    PrimitiveConstructionContinuityRow, PrimitiveConstructionContinuitySurfaceReportError,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionContinuityHostilitySuiteReport {
    rows: Vec<PrimitiveConstructionContinuityRow>,
    suite_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionContinuityHostilitySuiteReport {
    fn new(rows: Vec<PrimitiveConstructionContinuityRow>) -> Self {
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

    pub fn rows(&self) -> &[PrimitiveConstructionContinuityRow] {
        &self.rows
    }

    pub fn row(
        &self,
        case: PrimitiveConstructionContinuityCase,
    ) -> Option<&PrimitiveConstructionContinuityRow> {
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
pub enum PrimitiveConstructionContinuityHostilitySuiteError {
    Surface(PrimitiveConstructionContinuitySurfaceReportError),
    MissingRow(PrimitiveConstructionContinuityCase),
}

impl std::fmt::Display for PrimitiveConstructionContinuityHostilitySuiteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surface(error) => write!(f, "{error}"),
            Self::MissingRow(case) => write!(f, "missing continuity hostility row for {case:?}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionContinuityHostilitySuiteError {}

pub fn prepare_primitive_construction_continuity_hostility_suite_report() -> Result<
    PrimitiveConstructionContinuityHostilitySuiteReport,
    PrimitiveConstructionContinuityHostilitySuiteError,
> {
    let report = prepare_primitive_construction_continuity_surface_report()
        .map_err(PrimitiveConstructionContinuityHostilitySuiteError::Surface)?;
    let rows = [
        PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity,
        PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
        PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
        PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
        PrimitiveConstructionContinuityCase::ExplicitCutOpeningIdentitySplit,
    ]
    .into_iter()
    .map(|case| {
        report
            .row(case)
            .cloned()
            .ok_or(PrimitiveConstructionContinuityHostilitySuiteError::MissingRow(case))
    })
    .collect::<Result<Vec<_>, _>>()?;
    Ok(PrimitiveConstructionContinuityHostilitySuiteReport::new(
        rows,
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_continuity_hostility_suite_report;
    use crate::construction::PrimitiveConstructionContinuityCase;

    #[test]
    fn continuity_hostility_suite_fails_closed_on_exact_hostile_case_coverage() {
        let suite =
            prepare_primitive_construction_continuity_hostility_suite_report().expect("suite");

        assert!(suite.suite_verified());
        assert_eq!(suite.rows().len(), 5);
        assert_eq!(
            suite
                .rows()
                .iter()
                .map(|row| row.case())
                .collect::<Vec<_>>(),
            vec![
                PrimitiveConstructionContinuityCase::GrazingSnapAnchorContinuity,
                PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
                PrimitiveConstructionContinuityCase::OverlapBlockedPendingChoice,
                PrimitiveConstructionContinuityCase::ExplicitMergeIdentityMerged,
                PrimitiveConstructionContinuityCase::ExplicitCutOpeningIdentitySplit,
            ]
        );
        assert_ne!(suite.report_digest(), suite.rows()[0].row_digest());
    }
}
