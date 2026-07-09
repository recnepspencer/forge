use worth_query::facade::runtime::{
    WorthQueryAdmissionContributionPayload, WorthQueryAdmittedDomainCapabilityContribution,
};

fn main() {
    let _ = WorthQueryAdmittedDomainCapabilityContribution::<
        WorthQueryAdmissionContributionPayload,
    >(todo!());
}
