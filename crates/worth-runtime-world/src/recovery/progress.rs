/// Counts and byte accounting retained with a product-unpublished record.
/// These are Runtime World metadata metrics, not relabeled owner byte totals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProductUnpublishedOwnerEffectSummary {
    pub(crate) owner_effect_count: usize,
    pub(crate) metadata_bytes: usize,
}

impl ProductUnpublishedOwnerEffectSummary {
    pub(crate) fn from_progress(
        progress: &crate::publication::CompositeAttemptProgress,
        metadata_bytes: usize,
    ) -> Self {
        Self {
            owner_effect_count: progress.owner_effect_count(),
            metadata_bytes,
        }
    }
}

/// Every retained record is installed into exactly one recovery slot and
/// occupies it until the record is cleaned up.
const RECOVERY_SLOT: usize = 1;

/// A retained record's live obligations, counted once from its own custody
/// when the record is installed and divided by scope. The component half is
/// the exact pin pair the record holds or reserved; the composite half is the
/// recovery slot it occupies plus the successor history protection it holds
/// when its attempt installed a successor occurrence. The halves are one value
/// because they are only meaningful together: nothing outside this type may
/// name a pair that does not sum to the record's own count, and no route may
/// restate the count as a literal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProductUnpublishedLiveObligations {
    component: usize,
    composite: usize,
}

impl ProductUnpublishedLiveObligations {
    pub(crate) fn from_custody(component_pins: usize, successor_history_installed: bool) -> Self {
        Self {
            component: component_pins,
            composite: RECOVERY_SLOT + usize::from(successor_history_installed),
        }
    }

    pub(crate) const fn component(self) -> usize {
        self.component
    }

    pub(crate) const fn composite(self) -> usize {
        self.composite
    }

    pub(crate) const fn total(self) -> usize {
        self.component + self.composite
    }
}
