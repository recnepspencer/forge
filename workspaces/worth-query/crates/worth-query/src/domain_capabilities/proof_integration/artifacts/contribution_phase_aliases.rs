use super::phase_artifacts::{
    WorthQueryAdmittedDomainCapabilityContribution, WorthQueryEligibleDomainCapabilityContribution,
    WorthQueryMaterializationReadyDomainCapabilityContribution,
    WorthQueryRequestedDomainCapabilityContribution,
};
use crate::domain_capabilities::payloads::{
    WorthQueryAdmissionContributionPayload, WorthQueryAftermathContributionPayload,
    WorthQueryContinuityContributionPayload, WorthQueryExplanationContributionPayload,
    WorthQueryInvariantCapabilityContributionPayload, WorthQuerySupportContributionPayload,
    WorthQueryWorkflowContributionPayload,
};

pub type WorthQueryRequestedAdmissionContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQueryEligibleAdmissionContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQueryAdmittedAdmissionContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryAdmissionContributionPayload, T>;
pub type WorthQueryMaterializationReadyAdmissionContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryAdmissionContributionPayload,
        T,
    >;

pub type WorthQueryRequestedSupportContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryEligibleSupportContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryAdmittedSupportContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQuerySupportContributionPayload, T>;
pub type WorthQueryMaterializationReadySupportContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQuerySupportContributionPayload,
        T,
    >;

pub type WorthQueryRequestedInvariantCapabilityContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryEligibleInvariantCapabilityContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryAdmittedInvariantCapabilityContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;
pub type WorthQueryMaterializationReadyInvariantCapabilityContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryInvariantCapabilityContributionPayload,
        T,
    >;

pub type WorthQueryRequestedWorkflowContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryEligibleWorkflowContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryAdmittedWorkflowContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryWorkflowContributionPayload, T>;
pub type WorthQueryMaterializationReadyWorkflowContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryWorkflowContributionPayload,
        T,
    >;

pub type WorthQueryRequestedContinuityContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryEligibleContinuityContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryAdmittedContinuityContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryContinuityContributionPayload, T>;
pub type WorthQueryMaterializationReadyContinuityContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryContinuityContributionPayload,
        T,
    >;

pub type WorthQueryRequestedAftermathContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryEligibleAftermathContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryAdmittedAftermathContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryAftermathContributionPayload, T>;
pub type WorthQueryMaterializationReadyAftermathContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryAftermathContributionPayload,
        T,
    >;

pub type WorthQueryRequestedExplanationContribution<T> =
    WorthQueryRequestedDomainCapabilityContribution<WorthQueryExplanationContributionPayload, T>;
pub type WorthQueryEligibleExplanationContribution<T> =
    WorthQueryEligibleDomainCapabilityContribution<WorthQueryExplanationContributionPayload, T>;
pub type WorthQueryAdmittedExplanationContribution<T> =
    WorthQueryAdmittedDomainCapabilityContribution<WorthQueryExplanationContributionPayload, T>;
pub type WorthQueryMaterializationReadyExplanationContribution<T> =
    WorthQueryMaterializationReadyDomainCapabilityContribution<
        WorthQueryExplanationContributionPayload,
        T,
    >;
