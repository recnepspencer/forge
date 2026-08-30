use worth_foundational::facade::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
    CanonicalDigestId, CanonicalDigestWorkBudget, CanonicalEquivalenceBasis,
};
use worth_query_declaration::facade::application_query::ApplicationQueryOrderingDirection;

use super::{definition_with_sequence_slot, ActivitySequenceSlot};
use worth_query_declaration::worth_query_portable_type;

worth_query_portable_type!(AlternativeActivitySequenceSlot => "worth.query.test.installation.alternative-sequence-slot.v1");

struct AlternativeActivitySequenceSlot;

#[test]
fn schema_basis_changes_normalized_planning_identity() {
    let definition = definition_with_sequence_slot::<ActivitySequenceSlot>(
        ApplicationQueryOrderingDirection::Descending,
        "sequence",
    )
    .into_erased();
    let schema = CanonicalDigestId::new([1; 32]);
    let foreign_schema = CanonicalDigestId::new([2; 32]);
    let first =
        super::super::WorthQueryInstalledGraphReadContract::compile(&definition, &schema, budget())
            .expect("the test graph fits its canonical budget");
    let foreign = super::super::WorthQueryInstalledGraphReadContract::compile(
        &definition,
        &foreign_schema,
        budget(),
    )
    .expect("the foreign-schema test graph fits its canonical budget");

    assert_ne!(
        first.canonical_planning_basis().digest(),
        foreign.canonical_planning_basis().digest()
    );
    assert!(matches!(
        compare(
            first.canonical_planning_basis().basis(),
            foreign.canonical_planning_basis().basis()
        ),
        CanonicalComparisonOutcome::Mismatched(_)
    ));
}

#[test]
fn selector_slot_changes_installed_identity_but_not_normalized_planning_meaning() {
    let first = definition_with_sequence_slot::<ActivitySequenceSlot>(
        ApplicationQueryOrderingDirection::Descending,
        "sequence",
    )
    .into_erased();
    let changed = definition_with_sequence_slot::<AlternativeActivitySequenceSlot>(
        ApplicationQueryOrderingDirection::Descending,
        "sequence",
    )
    .into_erased();
    let package = CanonicalDigestId::new([3; 32]);
    let schema = CanonicalDigestId::new([1; 32]);
    let first_graph =
        super::super::WorthQueryInstalledGraphReadContract::compile(&first, &schema, budget())
            .expect("the first test graph fits its canonical budget");
    let changed_graph =
        super::super::WorthQueryInstalledGraphReadContract::compile(&changed, &schema, budget())
            .expect("the changed test graph fits its canonical budget");
    let first_canonical = super::super::canonical_basis::prepare_installed_query_basis(
        &package,
        &schema,
        &first,
        &first_graph,
        budget(),
    )
    .expect("the first installed query fits its canonical budget");
    let changed_canonical = super::super::canonical_basis::prepare_installed_query_basis(
        &package,
        &schema,
        &changed,
        &changed_graph,
        budget(),
    )
    .expect("the changed installed query fits its canonical budget");
    let first_query =
        super::super::WorthQueryInstalledApplicationQueryIdentity::from_canonical(&first_canonical);
    let changed_query = super::super::WorthQueryInstalledApplicationQueryIdentity::from_canonical(
        &changed_canonical,
    );

    assert_ne!(first.canonical_basis(), changed.canonical_basis());
    assert!(matches!(
        compare(
            first.canonical_basis().basis(),
            changed.canonical_basis().basis()
        ),
        CanonicalComparisonOutcome::Mismatched(_)
    ));
    assert_ne!(first_graph.digest(), changed_graph.digest());
    assert!(matches!(
        compare(
            first_graph.canonical_basis().basis(),
            changed_graph.canonical_basis().basis()
        ),
        CanonicalComparisonOutcome::Mismatched(_)
    ));
    assert_ne!(first_query, changed_query);
    assert_eq!(
        first_graph.canonical_planning_basis().digest(),
        changed_graph.canonical_planning_basis().digest(),
    );
    assert!(matches!(
        compare(
            first_graph.canonical_planning_basis().basis(),
            changed_graph.canonical_planning_basis().basis()
        ),
        CanonicalComparisonOutcome::Equivalent(_)
    ));
}

fn budget() -> CanonicalDigestWorkBudget {
    CanonicalDigestWorkBudget::new(4_096, 1024 * 1024)
        .expect("the test canonical budget is nonzero")
}

fn compare(
    left: &worth_foundational::facade::CanonicalBasisReadyArtifact,
    right: &worth_foundational::facade::CanonicalBasisReadyArtifact,
) -> CanonicalComparisonOutcome {
    let ready = prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left.clone(),
        right.clone(),
    )
    .into_result()
    .expect("application-query basis comparison is supported");
    compare_canonical_basis(&ready)
}
