use worth_kernel::workload_composition::{PlanarBooleanFamily, PlanarBooleanSupportReceipt};

fn main() {
    let _ = PlanarBooleanSupportReceipt {
        family: PlanarBooleanFamily::PlanarRegions,
        posture: panic!("support posture is private"),
        query_support_digest: "digest".to_string(),
        human_reason: "reason".to_string(),
    };
}
