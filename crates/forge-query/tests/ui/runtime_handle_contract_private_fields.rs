use forge_query::facade::{
    ForgeQueryHandleContract, ForgeQueryHandleContractFamily, ForgeQueryHandleContractRow,
};

fn main() {
    let _row = ForgeQueryHandleContractRow {
        family: ForgeQueryHandleContractFamily::LiveView,
        authority_lanes: Vec::new(),
        basis_lanes: Vec::new(),
        support_status: forge_query::facade::ForgeQueryRuntimeFamilySupportStatus::Supported,
        inspection_sections: Vec::new(),
        retained_artifact_required: true,
        deferred_future_posture: false,
        contract_digest: String::new(),
    };
    let _contract = ForgeQueryHandleContract {
        rows: Vec::new(),
        support_contract_digest: String::new(),
        inspectable_family_count: 0,
        retained_artifact_family_count: 0,
        deferred_future_family_count: 0,
        contract_digest: String::new(),
    };
}
