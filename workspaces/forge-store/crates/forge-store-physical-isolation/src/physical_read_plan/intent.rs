use crate::CurrentPhysicalRoot;

use super::{PhysicalReadPlanReleaseSemantics, ProtectedPhysicalReferenceSet};

#[derive(Debug, Clone)]
pub struct UnprotectedReadIntent {
    root: CurrentPhysicalRoot,
    protected_references: ProtectedPhysicalReferenceSet,
    resident_bytes: u64,
    release: Option<PhysicalReadPlanReleaseSemantics>,
}

impl UnprotectedReadIntent {
    pub fn for_known_footprint(
        root: CurrentPhysicalRoot,
        protected_references: ProtectedPhysicalReferenceSet,
        resident_bytes: u64,
    ) -> Self {
        Self {
            root,
            protected_references,
            resident_bytes,
            release: None,
        }
    }

    pub fn with_release_semantics(mut self, release: PhysicalReadPlanReleaseSemantics) -> Self {
        self.release = Some(release);
        self
    }

    pub const fn root(&self) -> CurrentPhysicalRoot {
        self.root
    }

    pub const fn resident_bytes(&self) -> u64 {
        self.resident_bytes
    }

    pub const fn release(&self) -> Option<PhysicalReadPlanReleaseSemantics> {
        self.release
    }

    pub fn protected_references(&self) -> &ProtectedPhysicalReferenceSet {
        &self.protected_references
    }
}
