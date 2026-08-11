use super::super::capability::Roadmap2SequenceId;
use super::super::harness::{S1CompileTimeBoundaryFixture, S1CompileTimeBoundaryStatus};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S1NonPlatformGradeDebtRow {
    pub(super) subject: String,
    pub(super) deferred_s_sequences: Vec<Roadmap2SequenceId>,
    pub(super) required_wording: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S1CompileTimeBoundaryFixtureStatusRow {
    pub(super) fixture: S1CompileTimeBoundaryFixture,
    pub(super) status: S1CompileTimeBoundaryStatus,
}

pub(super) fn compile_time_fixture_rows(
    available_fixtures: &[S1CompileTimeBoundaryFixture],
) -> Vec<S1CompileTimeBoundaryFixtureStatusRow> {
    let available = available_fixtures
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let mut rows = S1CompileTimeBoundaryFixture::required_by_s0()
        .into_iter()
        .map(|fixture| S1CompileTimeBoundaryFixtureStatusRow {
            fixture,
            status: if available.contains(&fixture) {
                S1CompileTimeBoundaryStatus::Present
            } else {
                S1CompileTimeBoundaryStatus::MissingS0Debt
            },
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.fixture);
    rows
}
