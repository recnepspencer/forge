use worth_query::facade::{
    BundleResolvedBasisDigest, FrontierAwarePlan, ParallelAdmissionEvidence, PlannedWorkPacket,
    SerialFallbackBundleEvidence, SerialFallbackEvidence, SignalFrontierBundleEvidence,
};

fn main() {
    let _ = std::any::type_name::<FrontierAwarePlan>();
    let _ = std::any::type_name::<PlannedWorkPacket>();
    let _ = std::any::type_name::<BundleResolvedBasisDigest>();
    let _ = std::any::type_name::<ParallelAdmissionEvidence>();
    let _ = std::any::type_name::<SerialFallbackEvidence>();
    let _ = std::any::type_name::<SerialFallbackBundleEvidence>();
    let _ = std::any::type_name::<SignalFrontierBundleEvidence>();
}
