use worth_relational::facade::history::BranchId;
use worth_signal::facade::branch::ValidatedSignalBranchName;

use crate::identity::{ProductBranchIdentity, ProductBranchIncarnation};

#[path = "custody/registry.rs"]
mod registry;

pub(crate) use registry::{OwnerCreatedComponentCustodyRegistry, ReservedCustodySlot};

/// Which component owner created a branch on this world's behalf.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustodyComponent {
    Relational,
    Signal,
}

/// The exact component branch a Runtime World owner asked a component owner to
/// create. It names a destination; it is not a deletion capability.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentBranchTarget {
    Relational(BranchId),
    Signal(ValidatedSignalBranchName),
}

impl ComponentBranchTarget {
    pub const fn component(&self) -> CustodyComponent {
        match self {
            Self::Relational(_) => CustodyComponent::Relational,
            Self::Signal(_) => CustodyComponent::Signal,
        }
    }
}

/// One owner-created component branch charged against the installed custody
/// budget. It is evidence of custody, not authority to delete anything.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnerCreatedComponentCustodyRecord {
    product_branch: ProductBranchIdentity,
    incarnation: ProductBranchIncarnation,
    target: ComponentBranchTarget,
}

impl OwnerCreatedComponentCustodyRecord {
    pub(crate) const fn new(
        product_branch: ProductBranchIdentity,
        incarnation: ProductBranchIncarnation,
        target: ComponentBranchTarget,
    ) -> Self {
        Self {
            product_branch,
            incarnation,
            target,
        }
    }

    pub const fn component(&self) -> CustodyComponent {
        self.target.component()
    }

    pub const fn product_branch(&self) -> &ProductBranchIdentity {
        &self.product_branch
    }

    pub const fn incarnation(&self) -> ProductBranchIncarnation {
        self.incarnation
    }

    pub const fn target(&self) -> &ComponentBranchTarget {
        &self.target
    }

    pub fn into_retirement_work(self) -> OwnerRetirementWork {
        match self.target {
            ComponentBranchTarget::Relational(target) => {
                OwnerRetirementWork::RelationalBranchRetirement { target }
            }
            ComponentBranchTarget::Signal(target) => {
                OwnerRetirementWork::SignalBranchRetirement { target }
            }
        }
    }
}

/// Typed work the component owner must perform. Runtime World never deletes a
/// component reference itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwnerRetirementWork {
    RelationalBranchRetirement { target: BranchId },
    SignalBranchRetirement { target: ValidatedSignalBranchName },
}

/// Terminal artifact of one product-branch retirement.
#[derive(Debug)]
#[must_use = "retirement work is dispatched or reported, never dropped silently"]
pub struct ProductBranchRetirementReport {
    released_product_reference: ProductBranchIdentity,
    owner_retirement_work: Vec<OwnerRetirementWork>,
}

impl ProductBranchRetirementReport {
    pub(crate) const fn new(
        released_product_reference: ProductBranchIdentity,
        owner_retirement_work: Vec<OwnerRetirementWork>,
    ) -> Self {
        Self {
            released_product_reference,
            owner_retirement_work,
        }
    }

    pub const fn released_product_reference(&self) -> &ProductBranchIdentity {
        &self.released_product_reference
    }

    pub fn owner_retirement_work(&self) -> &[OwnerRetirementWork] {
        &self.owner_retirement_work
    }
}
