use worth_query::facade::runtime::{
    WorthQueryAdmissionContributionPayload,
    WorthQueryMaterializationReadyDomainCapabilityContribution,
};

fn main() {
    let _ = WorthQueryMaterializationReadyDomainCapabilityContribution::<
        WorthQueryAdmissionContributionPayload,
    >(todo!());
}
