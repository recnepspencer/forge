use worth_query::facade::runtime::WorthQueryGraphObligationSupportMatrix;

const TEST_REQUIREMENTS: &str = include_str!("../../../../_docs/worth-query/test-requirements.md");
const AI_README: &str = include_str!("../../docs/AI_README.md");

#[test]
fn phase_20_named_suite_is_published_in_test_requirements() {
    assert_contains_all(
        TEST_REQUIREMENTS,
        &[
            "Milestone 9.9 Graph Touch Obligation Authority Hostile Certification Matrix",
            "kind x lane x representative touch",
            "execution budget",
            "Consumer Kit",
        ],
    );
}

#[test]
fn phase_20_closeout_uses_same_public_certification_name_as_docs() {
    assert_contains_all(
        AI_README,
        &[
            WorthQueryGraphObligationSupportMatrix::MILESTONE_9_9_AUTHORITY_CERTIFICATION_MATRIX_NAME,
            "Graph Touch Obligation Authority",
            "Consumer Kit",
        ],
    );
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            haystack.contains(needle),
            "expected closeout docs to contain `{needle}`"
        );
    }
}
