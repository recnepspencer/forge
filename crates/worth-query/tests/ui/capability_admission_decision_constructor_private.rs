use worth_query::facade::{
    CapabilityAdmissionDecision, WorthQueryCapabilityFamily, WorthQueryCapabilityStatus,
    WorthQueryConfigSectionFamily, WorthQueryFacadeCounters, WorthQuerySubsystemOwner,
};

fn main() {
    let _ = CapabilityAdmissionDecision {
        descriptor: worth_query::facade::WorthQueryCapabilityDescriptor {
            family: WorthQueryCapabilityFamily::QueryRead,
            status: WorthQueryCapabilityStatus::Admitted,
            owner: WorthQuerySubsystemOwner::Query,
            config_section: WorthQueryConfigSectionFamily::Query,
            reason: "forbidden",
        },
        validated_config_digest: String::new(),
        counters: WorthQueryFacadeCounters::default(),
        decision_digest: String::new(),
    };
}
