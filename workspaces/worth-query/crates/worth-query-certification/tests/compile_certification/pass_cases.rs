use trybuild::TestCases;

pub(crate) fn register(cases: &TestCases) {
    cases.pass("tests/pass/installed_audience_journey.rs");
}
