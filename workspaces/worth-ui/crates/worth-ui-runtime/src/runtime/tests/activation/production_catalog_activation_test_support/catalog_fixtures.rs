use super::{activate_catalog, activation_staging_inputs};

pub(crate) fn runtime_with_scroll_catalog() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::evidence::UiMeasurementResult,
    crate::evidence::UiProjectionFactReceipt,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiCommittedAllocationEvidenceSet,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (runtime, roots, planning, _, _, receipt, unrelated_receipt, query, evidence) =
        activate_catalog(runtime, pending, 2, true, false, None);
    let basis = planning.measurement_basis();
    let result = basis
        .host_allocation_requests()
        .find_map(|request| basis.host_measurement_result(request))
        .expect("scroll basis carries host evidence")
        .clone();
    (
        runtime,
        roots,
        result,
        query.expect("scroll catalog carries Query content extent"),
        receipt,
        unrelated_receipt,
        evidence.expect("scroll activation retains committed evidence"),
    )
}

pub(crate) fn runtime_with_portal_catalog() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    Box<[crate::graph::UiGraphNodeIdentity]>,
    crate::runtime::UiAllocationReceipt,
    crate::runtime::UiCommittedAllocationEvidenceSet,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (runtime, roots, _, _, _, receipt, _, _, evidence) =
        activate_catalog(runtime, pending, 2, false, true, None);
    (
        runtime,
        roots,
        receipt,
        evidence.expect("portal activation retains committed evidence"),
    )
}
