use worth_query::facade::runtime::{WorthQueryHandleContract, WorthQueryHandleContractFamily, WorthQueryHandleContractRow};

fn main() {
    let _row = WorthQueryHandleContractRow {
        family: WorthQueryHandleContractFamily::LiveView,
        authority_lanes: Vec::new(),
        basis_lanes: Vec::new(),
        support_status: worth_query::facade::runtime::WorthQueryRuntimeFamilySupportStatus::Supported,
        inspection_sections: Vec::new(),
        retained_artifact_required: true,
        deferred_future_posture: false,
        contract_digest: String::new(),
    };
    let _contract = WorthQueryHandleContract {
        rows: Vec::new(),
        support_contract_digest: String::new(),
        inspectable_family_count: 0,
        retained_artifact_family_count: 0,
        deferred_future_family_count: 0,
        contract_digest: String::new(),
    };
}
