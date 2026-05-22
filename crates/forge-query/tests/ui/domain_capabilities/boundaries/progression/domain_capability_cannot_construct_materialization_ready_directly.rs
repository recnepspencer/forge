use forge_query::facade::runtime::{
    ForgeQueryAdmissionContributionPayload,
    ForgeQueryMaterializationReadyDomainCapabilityContribution,
};

fn main() {
    let _ = ForgeQueryMaterializationReadyDomainCapabilityContribution::<
        ForgeQueryAdmissionContributionPayload,
    >(todo!());
}
