mod class;
mod identity;
#[cfg(test)]
mod identity_tests;
mod member;
mod membership_rule;
mod neighborhood;
mod scope;

pub use class::UiAllocationNeighborhoodClass;
pub use identity::UiAllocationNeighborhoodIdentity;
pub use member::{UiAllocationNeighborhoodMember, UiAllocationNeighborhoodMemberRole};
pub use membership_rule::UiAllocationNeighborhoodMembershipRule;
pub use neighborhood::UiAllocationNeighborhood;
pub use scope::UiAllocationNeighborhoodScope;

#[cfg(test)]
pub(crate) struct UiAllocationNeighborhoodEvidenceTestAuthority(());

#[cfg(test)]
impl UiAllocationNeighborhoodEvidenceTestAuthority {
    pub(in crate::evidence) const fn mint() -> Self {
        Self(())
    }
}
