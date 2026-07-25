use std::collections::BTreeSet;

use crate::maintenance::{
    exact_btree_publication_cases, layout_exact_publication, ExactBTreePublicationRequest,
    IndexMaintenanceMode,
};
use crate::strategy::tests_support::root_manifest_scope;

use super::mutation_support::executed_btree_mutation;

#[test]
fn exact_btree_publication_declares_exactly_ordinary_owner_cases() {
    let catalog = crate::bootstrap::test_support::bootstrap_catalog_read_admission();
    let exact = executed_btree_mutation(IndexMaintenanceMode::SynchronousExact, 1_101);
    let exact_materialization = materialization_for(&catalog, &exact);

    let lagged = executed_btree_mutation(IndexMaintenanceMode::AsynchronousLagged, 1_102);
    let lagged_materialization = materialization_for(&catalog, &lagged);

    let other_publication = executed_btree_mutation(IndexMaintenanceMode::SynchronousExact, 1_103);
    let other_materialization = materialization_for(&catalog, &other_publication);

    let (other_family, _) = root_manifest_scope();
    let other_family_materialization = crate::access_planning()
        .admit_current_catalog_root_materialization(other_family, &catalog)
        .expect("ordinary root family should materialize from the admitted catalog");

    let outcomes = [
        layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
            &exact,
            &exact_materialization,
        )),
        layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
            &lagged,
            &lagged_materialization,
        )),
        layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
            &exact,
            &other_family_materialization,
        )),
        layout_exact_publication().observe_btree(ExactBTreePublicationRequest::new(
            &exact,
            &other_materialization,
        )),
    ];
    let observed = outcomes
        .iter()
        .map(|outcome| outcome.case_id())
        .collect::<BTreeSet<_>>();

    assert_eq!(
        observed,
        exact_btree_publication_cases().collect::<BTreeSet<_>>()
    );
    let published = outcomes
        .into_iter()
        .next()
        .unwrap()
        .into_published()
        .expect("matching synchronous publication should remain exact");
    assert_eq!(published.counters().intent_validations(), 1);
    assert_eq!(published.counters().readiness_joins(), 1);
    assert_eq!(published.counters().root_swaps(), 1);
}

fn materialization_for(
    catalog: &crate::BootstrapCatalogReadAdmission,
    execution: &crate::CopyOnWriteLayoutMutationReceipt,
) -> crate::AdmittedLayoutMaterialization {
    crate::access_planning()
        .admit_btree_publication_materialization(
            execution.admitted_family(),
            catalog,
            execution.publication().new_root_validation(),
        )
        .into_result()
        .expect("executed publication should admit its exact materialization")
}
