use forge_query::facade::{
    CapabilityAdmissionDecision, ForgeQueryCapabilityFamily, ForgeQueryCapabilityStatus,
    ForgeQueryConfigSectionFamily, ForgeQueryFacadeCounters, ForgeQuerySubsystemOwner,
};

fn main() {
    let _ = CapabilityAdmissionDecision {
        descriptor: forge_query::facade::ForgeQueryCapabilityDescriptor {
            family: ForgeQueryCapabilityFamily::QueryRead,
            status: ForgeQueryCapabilityStatus::Admitted,
            owner: ForgeQuerySubsystemOwner::Query,
            config_section: ForgeQueryConfigSectionFamily::Query,
            reason: "forbidden",
        },
        validated_config_digest: String::new(),
        counters: ForgeQueryFacadeCounters::default(),
        decision_digest: String::new(),
    };
}
