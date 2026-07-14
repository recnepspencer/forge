use crate::evidence::UiAllocationNeighborhoodClass;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationNeighborhoodMembershipRule {
    RootOnly,
    ParentSlotPeerGroup,
}

impl UiAllocationNeighborhoodMembershipRule {
    pub(crate) const fn default_for_class(
        neighborhood_class: UiAllocationNeighborhoodClass,
    ) -> Self {
        match neighborhood_class {
            UiAllocationNeighborhoodClass::ContainerPeerGroup => Self::ParentSlotPeerGroup,
            UiAllocationNeighborhoodClass::LocalIntrinsicContent
            | UiAllocationNeighborhoodClass::Viewport
            | UiAllocationNeighborhoodClass::ScrollContainer
            | UiAllocationNeighborhoodClass::PortalAnchor => Self::RootOnly,
        }
    }
}
