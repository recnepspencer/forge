use worth_query::facade::{
    WorthQueryRuntimeFacadeFamily, WorthQueryRuntimeFamilySupport,
    WorthQueryRuntimeFamilySupportStatus,
};

fn main() {
    let _ = WorthQueryRuntimeFamilySupport {
        family: WorthQueryRuntimeFacadeFamily::Read,
        status: WorthQueryRuntimeFamilySupportStatus::Supported,
        authority_lanes: Vec::new(),
        effect_policies: Vec::new(),
        evidence: Vec::new(),
        denial_reason: None,
    };
}
