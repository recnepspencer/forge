use trybuild::TestCases;

pub(crate) fn register(cases: &TestCases) {
    cases.pass("tests/pass/installed_audience_journey.rs");
    cases.pass("tests/pass/granular_invalidation_public_outcome.rs");
}
