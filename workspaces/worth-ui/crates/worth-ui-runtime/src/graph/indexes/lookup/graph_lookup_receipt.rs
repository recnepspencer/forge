#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphLookupFamily {
    NodeIdentity,
    DeclarationCorrespondence,
    TopologyNode,
    ParentChild,
    SlotOccupancy,
    PageMembership,
    RegionMembership,
    MosaicMembership,
    PageParticipation,
    PublishedAspect,
    ConsumedAspect,
    MountedReceiptSlot,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiGraphLookupCostClass {
    IndexedScalar,
    IndexedSet,
    IndexedNeighborhood,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphLookupReceipt {
    family: UiGraphLookupFamily,
    cost_class: UiGraphLookupCostClass,
}

impl UiGraphLookupReceipt {
    pub const fn new(family: UiGraphLookupFamily, cost_class: UiGraphLookupCostClass) -> Self {
        Self { family, cost_class }
    }

    pub const fn family(self) -> UiGraphLookupFamily {
        self.family
    }

    pub const fn cost_class(self) -> UiGraphLookupCostClass {
        self.cost_class
    }
}
