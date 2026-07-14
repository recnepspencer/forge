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
fn public_orientation_teaches_the_certified_authority_contract_without_history() {
    assert_contains_all(
        AI_README,
        &[
            "Graph Touch Obligation Authority",
            "Consumer Kit",
            "The covered lane vocabulary is:",
            "BudgetExceeded",
        ],
    );
    assert!(!AI_README.contains("Milestone 9.9"));
}

fn assert_contains_all(haystack: &str, needles: &[&str]) {
    for needle in needles {
        assert!(
            haystack.contains(needle),
            "expected closeout docs to contain `{needle}`"
        );
    }
}
