use worth_query::facade::runtime::{
    WorthQueryDomainCapabilityCategory, WorthQueryDomainCapabilityCertifiedSurfaceRow,
};

fn main() {
    let _ = WorthQueryDomainCapabilityCertifiedSurfaceRow {
        category: WorthQueryDomainCapabilityCategory::Admission,
        ordinary_lane: "ordinary",
        inspectable_lane: "inspectable",
        proof_lane: "proof",
        raw_lane: "raw",
        implementation_path: "path",
    };
}
