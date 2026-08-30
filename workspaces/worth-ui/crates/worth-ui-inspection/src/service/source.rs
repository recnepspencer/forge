#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRuntimeServiceInspectionFamily {
    Portal,
    Focus,
    Motion,
    CommandRouting,
    Scroll,
    Selection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiRuntimeServiceInspectionSource {
    family: UiRuntimeServiceInspectionFamily,
    owner: Option<u64>,
    revision: u64,
}

impl UiRuntimeServiceInspectionSource {
    pub const fn new(
        family: UiRuntimeServiceInspectionFamily,
        owner: Option<u64>,
        revision: u64,
    ) -> Self {
        Self {
            family,
            owner,
            revision,
        }
    }

    pub const fn family(self) -> UiRuntimeServiceInspectionFamily {
        self.family
    }

    pub const fn owner(self) -> Option<u64> {
        self.owner
    }

    pub const fn revision(self) -> u64 {
        self.revision
    }
}
