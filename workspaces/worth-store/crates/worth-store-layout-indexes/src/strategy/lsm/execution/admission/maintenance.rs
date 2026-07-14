use crate::planning::{SelectedLsmCompaction, SelectedLsmRunPublication};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmRunPublicationAdmission {
    selected: SelectedLsmRunPublication,
}

impl BaselineLsmRunPublicationAdmission {
    pub fn admit(selected: SelectedLsmRunPublication) -> Self {
        Self { selected }
    }
    pub const fn selected(&self) -> &SelectedLsmRunPublication {
        &self.selected
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmCompactionAdmission {
    selected: SelectedLsmCompaction,
}

impl BaselineLsmCompactionAdmission {
    pub fn admit(selected: SelectedLsmCompaction) -> Self {
        Self { selected }
    }
    pub const fn selected(&self) -> &SelectedLsmCompaction {
        &self.selected
    }
}
