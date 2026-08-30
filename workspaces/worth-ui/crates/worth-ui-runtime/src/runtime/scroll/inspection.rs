#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiScrollOwnerInspectionRecord {
    owner: super::UiScrollOwnerIdentity,
    owners_visited: u16,
    owners_changed: u16,
    remainder_present: bool,
    revision: u64,
}

impl UiScrollOwnerInspectionRecord {
    pub(super) fn from_receipt(receipt: &super::UiScrollRouteReceipt) -> Option<Self> {
        let owner = receipt
            .transitions()
            .iter()
            .find(|transition| transition.previous() != transition.current())
            .or_else(|| receipt.transitions().first())?
            .owner();
        let owners_changed = receipt
            .transitions()
            .iter()
            .filter(|transition| transition.previous() != transition.current())
            .count();
        Some(Self {
            owner,
            owners_visited: receipt.owners_visited(),
            owners_changed: u16::try_from(owners_changed).unwrap_or(u16::MAX),
            remainder_present: !receipt.remainder().is_zero(),
            revision: receipt.revision(),
        })
    }

    pub(crate) const fn owner(self) -> super::UiScrollOwnerIdentity {
        self.owner
    }
    pub(crate) const fn owners_visited(self) -> u16 {
        self.owners_visited
    }
    pub(crate) const fn owners_changed(self) -> u16 {
        self.owners_changed
    }
    pub(crate) const fn remainder_present(self) -> bool {
        self.remainder_present
    }
    pub(crate) const fn revision(self) -> u64 {
        self.revision
    }
}
