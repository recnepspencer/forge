//! Independent field-by-field oracles for portable record projections.

use crate::facade::{
    WorthQueryPortableDomainOperationDefinition, WorthQueryPortableDomainOperationRecord,
};

pub(super) fn assert_domain_operation_projection(
    actual: &WorthQueryPortableDomainOperationRecord,
    source: &WorthQueryPortableDomainOperationDefinition,
) {
    assert_eq!(actual.identity(), source.identity());
    assert_eq!(actual.canonical_identity(), source.canonical_identity());
    let actual = actual.semantics();
    let source = source.semantics();
    macro_rules! assert_ref {
        ($field:ident) => {
            assert_eq!(actual.$field(), &source.$field, stringify!($field));
        };
    }
    macro_rules! assert_copy {
        ($field:ident) => {
            assert_eq!(actual.$field(), source.$field, stringify!($field));
        };
    }
    assert_ref!(parameters);
    assert_ref!(native_projection);
    assert_canonical_query_projection(actual.canonical_query(), &source.canonical_query);
    assert_ref!(collection);
    assert_eq!(actual.required_capabilities(), source.required_capabilities);
    assert_eq!(actual.required_domains(), source.required_domains);
    assert_ref!(workflow);
    assert_ref!(evidence);
    assert_eq!(actual.conditional_nodes(), source.conditional_nodes);
    assert_ref!(graph_reads);
    assert_ref!(decision_facts);
    assert_ref!(touches);
    assert_ref!(effects);
    assert_ref!(invariants);
    assert_ref!(invariant_execution);
    assert_copy!(replay);
    assert!(source.aftermath.is_none());
    assert_copy!(lineage);
    assert_copy!(promotion);
    assert_ref!(publication);
    assert_copy!(projection_consumption);
    assert_ref!(terminal);
    assert_copy!(cost);
    assert_ref!(resources);
    assert_copy!(support);
    assert_ref!(lowering);
}

fn assert_canonical_query_projection(
    actual: &worth_query_declaration::facade::canonicalization::WorthQueryPortableCanonicalQueryBundleRecord,
    source: &worth_query_declaration::facade::canonicalization::CanonicalQueryBundle,
) {
    let actual_query = actual.query();
    let source_query = source.query();
    assert_eq!(actual_query.digest(), source_query.digest());
    assert_eq!(actual_query.family(), source_query.family());
    assert_eq!(actual_query.root(), source_query.root());
    assert_eq!(actual_query.projection(), source_query.projection());
    assert_eq!(actual_query.predicates(), source_query.predicates());
    assert_eq!(actual_query.ordering(), source_query.ordering());
    assert_eq!(actual_query.traversal(), source_query.traversal());
    assert_eq!(
        actual_query.identity_bindings(),
        source_query.identity_bindings()
    );
    let actual_result = actual.result_shape();
    let source_result = source.result_shape();
    assert_eq!(actual_result.digest(), source_result.digest());
    assert_eq!(actual_result.family(), source_result.family());
    assert_eq!(actual_result.fields(), source_result.fields());
    assert_eq!(actual.report(), source.report());
    assert_eq!(actual.counters(), source.counters());
}
