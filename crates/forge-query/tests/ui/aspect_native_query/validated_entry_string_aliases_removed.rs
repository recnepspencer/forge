use forge_query::facade::{
    ValidatedOrderingEntry, ValidatedPredicateEntry, ValidatedProjectionEntry,
    ValidatedResultShapeBinding, ValidatedTraversalEntry,
};

fn main() {
    let projection = projection_fixture();
    let _ = projection.aspect();
    let _ = projection.field();

    let binding = binding_fixture();
    let _ = binding.source_aspect();
    let _ = binding.source_field();

    let predicate = predicate_fixture();
    let _ = predicate.aspect();
    let _ = predicate.field();

    let ordering = ordering_fixture();
    let _ = ordering.aspect();
    let _ = ordering.field();

    let traversal = traversal_fixture();
    let _ = traversal.relation();
}

fn projection_fixture() -> ValidatedProjectionEntry {
    panic!("fixture only")
}

fn binding_fixture() -> ValidatedResultShapeBinding {
    panic!("fixture only")
}

fn predicate_fixture() -> ValidatedPredicateEntry {
    panic!("fixture only")
}

fn ordering_fixture() -> ValidatedOrderingEntry {
    panic!("fixture only")
}

fn traversal_fixture() -> ValidatedTraversalEntry {
    panic!("fixture only")
}
