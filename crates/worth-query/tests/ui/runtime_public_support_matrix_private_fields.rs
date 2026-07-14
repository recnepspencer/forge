use worth_query::facade::runtime::{WorthQueryRuntimeBackendPosture, WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupportStatus, WorthQueryRuntimePublicSupportMatrix, WorthQueryRuntimePublicSupportMatrixRow};

fn main() {
    let _row = WorthQueryRuntimePublicSupportMatrixRow {
        surface: String::from("temporal"),
        facade_family: Some(WorthQueryRuntimeFacadeFamily::Temporal),
        status: WorthQueryRuntimeFamilySupportStatus::DeferredDebt,
        owner_milestone: String::from("Milestone 9.4"),
        extension_rule: String::from("shortcut"),
        parallel_api_forbidden: false,
        admission_fail_closed: false,
        support_contract_digest: None,
        row_digest: String::new(),
    };

    let _matrix = WorthQueryRuntimePublicSupportMatrix {
        backend_posture: WorthQueryRuntimeBackendPosture::Scaffold,
        rows: Vec::new(),
        stable_row_count: 0,
        deferred_row_count: 0,
        unsupported_row_count: 0,
        fail_closed_row_count: 0,
        parallel_api_forbidden_row_count: 0,
        matrix_digest: String::new(),
    };
}
