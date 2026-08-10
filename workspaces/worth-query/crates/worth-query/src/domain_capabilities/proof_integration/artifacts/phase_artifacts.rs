use super::contribution::{
    AdmittedContributionArtifact, EligibleContributionArtifact,
    MaterializationReadyContributionArtifact, RequestedContributionArtifact,
    WorthQueryDomainCapabilityContribution,
};
use crate::domain_capabilities::payloads::WorthQueryDomainCapabilityPayload;
use crate::domain_capabilities::targets::WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryRequestedDomainCapabilityContribution<P, T>(
    pub(crate) RequestedContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryEligibleDomainCapabilityContribution<P, T>(
    pub(crate) EligibleContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryAdmittedDomainCapabilityContribution<P, T>(
    pub(crate) AdmittedContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

pub struct WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>(
    pub(crate) MaterializationReadyContributionArtifact<P, T>,
)
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding;

macro_rules! impl_wrapper_accessors {
    ($name:ident, $inner:ident) => {
        impl<P, T> $name<P, T>
        where
            P: WorthQueryDomainCapabilityPayload,
            T: WorthQueryDomainCapabilityTargetBinding,
        {
            pub fn payload(&self) -> &WorthQueryDomainCapabilityContribution<P, T> {
                self.0.payload()
            }

            pub(crate) fn into_inner(self) -> $inner<P, T> {
                self.0
            }
        }
    };
}

impl_wrapper_accessors!(
    WorthQueryRequestedDomainCapabilityContribution,
    RequestedContributionArtifact
);
impl_wrapper_accessors!(
    WorthQueryEligibleDomainCapabilityContribution,
    EligibleContributionArtifact
);
impl_wrapper_accessors!(
    WorthQueryAdmittedDomainCapabilityContribution,
    AdmittedContributionArtifact
);

impl<P, T> WorthQueryMaterializationReadyDomainCapabilityContribution<P, T>
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
    pub fn payload(&self) -> &WorthQueryDomainCapabilityContribution<P, T> {
        self.0.payload()
    }
}
