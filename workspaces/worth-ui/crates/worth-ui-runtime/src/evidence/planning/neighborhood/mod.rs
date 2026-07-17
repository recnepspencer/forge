mod allocation_neighborhood;
mod class;
mod identity;
#[cfg(test)]
mod identity_tests;
mod member;
mod membership_rule;
mod scope;

pub use allocation_neighborhood::UiAllocationNeighborhood;
pub(crate) use allocation_neighborhood::UiAllocationNeighborhoodInput;
#[cfg(test)]
pub(crate) use allocation_neighborhood::UiAllocationNeighborhoodTestInput;
pub use class::UiAllocationNeighborhoodClass;
pub use identity::UiAllocationNeighborhoodIdentity;
use identity::UiAllocationNeighborhoodIdentityInput;
pub use member::{UiAllocationNeighborhoodMember, UiAllocationNeighborhoodMemberRole};
pub use membership_rule::UiAllocationNeighborhoodMembershipRule;
pub use scope::UiAllocationNeighborhoodScope;

#[cfg(test)]
pub(crate) struct UiAllocationNeighborhoodEvidenceTestAuthority(());

#[cfg(test)]
impl UiAllocationNeighborhoodEvidenceTestAuthority {
    pub(in crate::evidence) const fn mint() -> Self {
        Self(())
    }
}
