mod admission;
mod aftermath;
mod continuity;
mod continuity_correspondence;
mod explanation;
mod invariant_capability;
mod support;
mod workflow;
mod workflow_inspection;

use super::proof_integration::{
    create_requested_domain_capability_contribution, AllowedContributionBinding,
    ForgeQueryRequestedDomainCapabilityContribution,
};
use super::targets::ForgeQueryDomainCapabilityTargetBinding;
pub use admission::*;
pub use aftermath::*;
pub use continuity::*;
pub use explanation::*;
pub use invariant_capability::*;
pub use support::*;
pub use workflow::*;

fn bind_requested<P, T>(
    payload: P,
    target: T,
) -> ForgeQueryRequestedDomainCapabilityContribution<P, T>
where
    P: super::payloads::ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
    (P, T): AllowedContributionBinding<P, T>,
{
    create_requested_domain_capability_contribution(target, payload)
}
