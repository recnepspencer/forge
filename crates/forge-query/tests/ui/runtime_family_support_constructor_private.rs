use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupport,
    ForgeQueryRuntimeFamilySupportStatus,
};

fn main() {
    let _ = ForgeQueryRuntimeFamilySupport {
        family: ForgeQueryRuntimeFacadeFamily::Read,
        status: ForgeQueryRuntimeFamilySupportStatus::Supported,
        authority_lanes: Vec::new(),
        effect_policies: Vec::new(),
        evidence: Vec::new(),
        denial_reason: None,
    };
}
