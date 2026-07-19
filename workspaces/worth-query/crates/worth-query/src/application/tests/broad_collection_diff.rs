use super::*;

#[test]
fn broad_collection_diff_remains_denied_before_diff_bundle_construction() {
    let facade = WorthQueryApplicationFacade::runtime_backed_default();
    let contexts = facade
        .query_context_capability()
        .expect("query context capability should admit");
    let left_preflight = execution_preflights::ordered_collection_without_traversal_preflight();
    let right_preflight = execution_preflights::alternate_basis_ordered_collection_preflight();
    let left = contexts
        .capability()
        .admit_basis_context(
            basis_lifecycle().current_head(),
            QueryContextBindingSource::RuntimeCurrent(&left_preflight),
        )
        .expect("left context should admit");
    let right = contexts
        .capability()
        .admit_basis_context(
            basis_lifecycle().branch_head("branch:ordered-collection", true),
            QueryContextBindingSource::RuntimeBranch(&right_preflight),
        )
        .expect("right context should admit");
    let diff = contexts
        .capability()
        .bind_diff_context(&left, &right)
        .expect("diff context should bind");
    let left_execution = contexts
        .capability()
        .execute_basis_context(&left)
        .expect("left context should execute");
    let right_execution = contexts
        .capability()
        .execute_basis_context(&right)
        .expect("right context should execute");

    let error = contexts
        .capability()
        .shape_diff_result_bundle(&diff, &left_execution, &right_execution)
        .expect_err("broad collection diff should deny before bundle construction");

    assert_eq!(
        error.failure_class().clone(),
        crate::query_context::QueryContextAdmissionFailureClass::ComparisonBroadeningRequired
    );
}
