use crate::{BufferPoolBudget, S2PhysicalEntryFacts};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BufferPoolAdmission {
    budget: BufferPoolBudget,
    facts: S2PhysicalEntryFacts,
}

impl BufferPoolAdmission {
    pub(crate) const fn new(budget: BufferPoolBudget, facts: S2PhysicalEntryFacts) -> Self {
        Self { budget, facts }
    }

    pub const fn budget(&self) -> BufferPoolBudget {
        self.budget
    }

    pub const fn facts(&self) -> S2PhysicalEntryFacts {
        self.facts
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedBufferPoolEntry {
    admission: BufferPoolAdmission,
}

impl AdmittedBufferPoolEntry {
    pub(crate) const fn new(admission: BufferPoolAdmission) -> Self {
        Self { admission }
    }

    pub const fn admission(&self) -> BufferPoolAdmission {
        self.admission
    }
}
