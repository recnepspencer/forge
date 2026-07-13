use crate::planning::{SelectedBTreeLookup, SelectedDegradedExactScan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AccessLoweringFacade;

impl AccessLoweringFacade {
    pub fn lower_btree_lookup(&self, selected: SelectedBTreeLookup) -> super::LoweredBTreeLookup {
        super::btree_lookup::lower(selected)
    }

    pub fn lower_degraded(
        &self,
        selected: SelectedDegradedExactScan,
    ) -> super::LoweredDegradedExactScan {
        super::degraded_scan::lower(selected)
    }

    pub fn admit_btree_lookup_ready(
        &self,
        lowered: super::LoweredBTreeLookup,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> super::BTreeLookupReadinessOutcome {
        super::btree_lookup::admit_ready(lowered, frontier)
    }

    pub fn admit_degraded_ready(
        &self,
        lowered: super::LoweredDegradedExactScan,
        frontier: crate::CurrentMaterializationFrontier,
    ) -> super::DegradedScanReadinessOutcome {
        super::degraded_scan::classify_readiness(lowered, frontier)
    }
}

pub(crate) const fn access_lowering() -> AccessLoweringFacade {
    AccessLoweringFacade
}
