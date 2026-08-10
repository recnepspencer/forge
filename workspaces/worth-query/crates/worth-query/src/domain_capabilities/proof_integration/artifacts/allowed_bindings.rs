use crate::domain_capabilities::payloads::{
    WorthQueryAdmissionContributionPayload, WorthQueryAftermathContributionPayload,
    WorthQueryContinuityContributionPayload, WorthQueryDomainCapabilityPayload,
    WorthQueryExplanationContributionPayload, WorthQueryInvariantCapabilityContributionPayload,
    WorthQuerySupportContributionPayload, WorthQueryWorkflowContributionPayload,
};
use crate::domain_capabilities::targets::{
    WorthQueryAdmittedPlanBoundContributionTarget, WorthQueryDeclarationBoundContributionTarget,
    WorthQueryDomainCapabilityTargetBinding, WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
};

pub trait AllowedContributionBinding<P, T>: Sealed
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
{
}

pub trait Sealed {}

macro_rules! allow {
    ($payload:ty => [$($target:ty),+ $(,)?]) => {
        $(impl Sealed for ($payload, $target) {}
        impl AllowedContributionBinding<$payload, $target> for ($payload, $target) {})+
    };
}

allow!(
    WorthQueryAdmissionContributionPayload => [
        WorthQueryDeclarationBoundContributionTarget,
        WorthQueryAdmittedPlanBoundContributionTarget
    ]
);
allow!(
    WorthQuerySupportContributionPayload => [
        WorthQueryDeclarationBoundContributionTarget,
        WorthQueryAdmittedPlanBoundContributionTarget,
        WorthQueryLowerRuntimeBoundaryBoundContributionTarget
    ]
);
allow!(
    WorthQueryInvariantCapabilityContributionPayload => [
        WorthQueryDeclarationBoundContributionTarget,
        WorthQueryLowerRuntimeBoundaryBoundContributionTarget
    ]
);
allow!(
    WorthQueryWorkflowContributionPayload => [
        WorthQueryDeclarationBoundContributionTarget,
        WorthQueryAdmittedPlanBoundContributionTarget
    ]
);
allow!(
    WorthQueryContinuityContributionPayload => [
        WorthQueryDeclarationBoundContributionTarget,
        WorthQueryAdmittedPlanBoundContributionTarget
    ]
);
allow!(
    WorthQueryAftermathContributionPayload => [
        WorthQueryAdmittedPlanBoundContributionTarget,
        WorthQueryLowerRuntimeBoundaryBoundContributionTarget
    ]
);
allow!(
    WorthQueryExplanationContributionPayload => [
        WorthQueryDeclarationBoundContributionTarget,
        WorthQueryAdmittedPlanBoundContributionTarget,
        WorthQueryLowerRuntimeBoundaryBoundContributionTarget
    ]
);

impl<P, T> Sealed
    for (
        P,
        crate::domain_capabilities::WorthQueryInstalledDomainContributionTarget<T>,
    )
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
    (P, T): AllowedContributionBinding<P, T>,
{
}

impl<P, T>
    AllowedContributionBinding<
        P,
        crate::domain_capabilities::WorthQueryInstalledDomainContributionTarget<T>,
    >
    for (
        P,
        crate::domain_capabilities::WorthQueryInstalledDomainContributionTarget<T>,
    )
where
    P: WorthQueryDomainCapabilityPayload,
    T: WorthQueryDomainCapabilityTargetBinding,
    (P, T): AllowedContributionBinding<P, T>,
{
}
