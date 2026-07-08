use forge_store_physical_isolation::{CompactionReadInterlockDenial, CompactionReadInterlockPlan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobCompactionPhysicalInterlock {
    Admitted(CompactionReadInterlockPlan),
    Denied(CompactionReadInterlockDenial),
}

impl BlobCompactionPhysicalInterlock {
    pub(crate) const fn admitted(&self) -> Option<&CompactionReadInterlockPlan> {
        match self {
            Self::Admitted(plan) => Some(plan),
            Self::Denied(_) => None,
        }
    }

    pub(crate) const fn denial(&self) -> Option<CompactionReadInterlockDenial> {
        match self {
            Self::Admitted(_) => None,
            Self::Denied(denial) => Some(*denial),
        }
    }
}

#[allow(dead_code)]
fn _physical_interlock_is_part_of_the_boundary(_: BlobCompactionPhysicalInterlock) {}
