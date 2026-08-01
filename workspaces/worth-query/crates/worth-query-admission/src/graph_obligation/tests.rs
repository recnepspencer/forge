use super::{
    admit_application_query_graph_work, require_selected_graph_work,
    select_installed_graph_obligations, WorthQueryGraphObligationSelectionDenialKind as DenialKind,
    WorthQueryGraphWorkIntent, WorthQueryGraphWorkRequirementDenialKind,
};

use std::sync::Arc;

use crate::domain_computation::execution_resource_admission::{
    WorthQueryExecutionResourceSupport, WorthQueryExecutionResourceSupportSnapshot,
    WorthQueryFixedExecutionCapacity,
};
use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily, WorthQueryExecutionMode, WorthQueryResourceLimitRequest,
    WorthQuerySemanticScaleRequest,
};
use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily, WorthQueryExecutionAllocatorFamily,
    WorthQueryExecutionProviderFamily, WorthQueryExecutionResourceEnvelope,
};

#[test]
fn installed_query_selection_retains_exact_rows_without_canonical_work() {
    let installed = crate::application_query::installed_query_obligations();
    let selected = select_installed_graph_obligations(
        &installed,
        WorthQueryGraphWorkIntent::application_query_read(),
    )
    .unwrap();

    assert_eq!(selected.identity(), installed.identity());
    assert_eq!(selected.rows(), installed.rows());
    assert_eq!(selected.counters().installed_subject_checks(), 1);
    assert_eq!(
        selected.counters().installed_rows_examined(),
        installed.rows().len()
    );
    assert_eq!(selected.counters().selected_rows(), installed.rows().len());
    assert_eq!(selected.counters().canonical_preparations(), 0);
    assert_eq!(selected.counters().digest_derivations(), 0);
}

#[test]
fn requirements_consume_selection_and_validate_every_owner_route_without_hashing() {
    let (installed, authority) =
        crate::application_query::installed_query_obligations_with_authority();
    let selected = select_installed_graph_obligations(
        &installed,
        WorthQueryGraphWorkIntent::application_query_read(),
    )
    .unwrap();
    let required = require_selected_graph_work(selected, &authority).unwrap();
    let counters = required.counters();

    assert_eq!(required.identity(), installed.identity());
    assert_eq!(counters.selected_rows_consumed(), installed.rows().len());
    assert_eq!(
        counters.owner_progressions_checked(),
        installed.rows().len()
    );
    assert_eq!(counters.requirement_rows(), installed.rows().len());
    assert_eq!(counters.canonical_preparations(), 0);
    assert_eq!(counters.digest_derivations(), 0);
}

#[test]
fn foreign_installation_authority_cannot_progress_selected_obligations() {
    let (installed, _authority) =
        crate::application_query::installed_query_obligations_with_authority();
    let (_foreign, foreign_authority) =
        crate::application_query::installed_query_obligations_with_authority();
    let selected = select_installed_graph_obligations(
        &installed,
        WorthQueryGraphWorkIntent::application_query_read(),
    )
    .unwrap();

    let denial = require_selected_graph_work(selected, &foreign_authority).unwrap_err();
    assert_eq!(
        denial.kind(),
        WorthQueryGraphWorkRequirementDenialKind::ForeignAdmissionAuthority
    );
}

#[test]
fn query_obligations_cannot_be_selected_as_operation_authority() {
    let installed = crate::application_query::installed_query_obligations();
    for intent in [
        WorthQueryGraphWorkIntent::application_operation_read(),
        WorthQueryGraphWorkIntent::application_operation_mutation(),
    ] {
        let denial = select_installed_graph_obligations(&installed, intent).unwrap_err();
        assert_eq!(denial.kind(), DenialKind::SubjectKindMismatch);
    }
}

#[test]
fn selected_inspection_cannot_widen_or_reconstitute_the_selected_set() {
    let installed = crate::application_query::installed_query_obligations();
    let selected = select_installed_graph_obligations(
        &installed,
        WorthQueryGraphWorkIntent::application_query_read(),
    )
    .unwrap();
    let inspection = selected.inspect();

    assert_eq!(inspection.identity(), installed.identity());
    assert_eq!(inspection.selected_row_count(), installed.rows().len());
    assert_eq!(inspection.counters(), selected.counters());
}

#[test]
fn query_admission_consumes_requirements_support_budget_and_real_capacity() {
    let (installed, authority) =
        crate::application_query::installed_query_obligations_with_authority();
    let required = query_requirements(&installed, &authority);
    let review = crate::application_query::installed_query_graph_read_review(required);
    let support = support_snapshot(Arc::new(
        WorthQueryFixedExecutionCapacity::new("graph-work-query-capacity", 1).unwrap(),
    ));

    let plan = admit_application_query_graph_work(review, &support).unwrap();

    assert_eq!(plan.obligation_identity(), installed.identity());
    assert_eq!(plan.reservation_count(), 1);
    assert_eq!(plan.canonical_work().digest_derivations(), 1);
    assert!(plan.graph_read_review().unwrap().is_admitted());
    let release = plan.release();
    assert_eq!(release.released_reservation_count(), 1);
}

#[test]
fn graph_work_capacity_saturates_and_release_restores_the_exact_baseline() {
    let (installed, authority) =
        crate::application_query::installed_query_obligations_with_authority();
    let capacity = Arc::new(
        WorthQueryFixedExecutionCapacity::new("graph-work-saturation-capacity", 1).unwrap(),
    );
    let support = support_snapshot(capacity);
    let first = admit_application_query_graph_work(
        crate::application_query::installed_query_graph_read_review(query_requirements(
            &installed, &authority,
        )),
        &support,
    )
    .unwrap();
    let denial = match admit_application_query_graph_work(
        crate::application_query::installed_query_graph_read_review(query_requirements(
            &installed, &authority,
        )),
        &support,
    ) {
        Ok(_) => panic!("the saturated provider must not admit a second graph-work plan"),
        Err(denial) => denial,
    };
    assert!(matches!(
        denial,
        super::WorthQueryGraphWorkAdmissionDenial::CapacityUnavailable
    ));

    first.release();
    let admitted_again = admit_application_query_graph_work(
        crate::application_query::installed_query_graph_read_review(query_requirements(
            &installed, &authority,
        )),
        &support,
    )
    .unwrap();
    admitted_again.release();
}

fn query_requirements(
    installed: &worth_query_installation::facade::WorthQueryInstalledGraphObligationSet,
    authority: &worth_query_installation::facade::WorthQueryInstalledGraphAdmissionAuthority,
) -> super::WorthQueryRequiredGraphWork {
    let selected = select_installed_graph_obligations(
        installed,
        WorthQueryGraphWorkIntent::application_query_read(),
    )
    .unwrap();
    require_selected_graph_work(selected, authority).unwrap()
}

fn support_snapshot(
    capacity: Arc<WorthQueryFixedExecutionCapacity>,
) -> WorthQueryExecutionResourceSupportSnapshot {
    let support = WorthQueryExecutionResourceSupport::new(
        WorthQueryExecutionProviderFamily::new("test-provider").unwrap(),
        WorthQueryExecutionAccessProductFamily::new("test-access").unwrap(),
        WorthQueryExecutionAllocatorFamily::new("test-allocator").unwrap(),
        WorthQueryExecutionResourceEnvelope::new(
            WorthQuerySemanticScaleRequest::bounded(4_096),
            WorthQueryResourceLimitRequest::bounded(4_096),
            WorthQueryExecutionMode::Synchronous,
            None,
            WorthQueryCancellationSafePointFamily::new("test-safe-point").unwrap(),
        ),
        capacity,
    );
    WorthQueryExecutionResourceSupportSnapshot::new(
        support.clone(),
        Vec::new(),
        vec![("primary".to_owned(), support.clone())],
        vec![("primary".to_owned(), support)],
        None,
    )
}
