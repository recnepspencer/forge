use forge_query::facade::runtime::{
    ForgeQueryDomainCapabilityCategory, ForgeQueryDomainCapabilityCertifiedSurfaceRow,
};

fn main() {
    let _ = ForgeQueryDomainCapabilityCertifiedSurfaceRow {
        category: ForgeQueryDomainCapabilityCategory::Admission,
        ordinary_lane: "ordinary",
        inspectable_lane: "inspectable",
        proof_lane: "proof",
        raw_lane: "raw",
        implementation_path: "path",
    };
}
