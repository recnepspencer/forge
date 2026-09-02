/// Closed Runtime World dependency vocabulary for one exact component-basis
/// pin. A count is semantic usage, not an owner lease or a history record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ComponentBasisDependencyClass {
    ProductBranchHead,
    RetainedCompositeHistory,
    AdmittedObservation,
    ActivePublicationAttempt,
    ProductUnpublishedOwnerEffects,
    HistoricalInspection,
}

impl ComponentBasisDependencyClass {
    pub(crate) const ALL: [Self; 6] = [
        Self::ProductBranchHead,
        Self::RetainedCompositeHistory,
        Self::AdmittedObservation,
        Self::ActivePublicationAttempt,
        Self::ProductUnpublishedOwnerEffects,
        Self::HistoricalInspection,
    ];
}

/// Per-class usage carried by the future unique-pin registry. The registry
/// owns mutation and capacity checks; this value freezes the six accounting
/// slots without introducing a second retention authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ComponentBasisDependencyCounts {
    counts: [usize; 6],
}

impl ComponentBasisDependencyCounts {
    pub(crate) const fn zero() -> Self {
        Self { counts: [0; 6] }
    }

    pub(crate) const fn get(self, class: ComponentBasisDependencyClass) -> usize {
        self.counts[class.index()]
    }

    pub(crate) const fn total(self) -> usize {
        let mut total = 0;
        let mut index = 0;
        while index < self.counts.len() {
            total += self.counts[index];
            index += 1;
        }
        total
    }

    pub(crate) fn increment(&mut self, class: ComponentBasisDependencyClass) -> Option<usize> {
        let count = self.counts[class.index()];
        let next = count.checked_add(1)?;
        self.counts[class.index()] = next;
        Some(next)
    }

    pub(crate) fn decrement(&mut self, class: ComponentBasisDependencyClass) -> Option<usize> {
        let count = self.counts[class.index()];
        let next = count.checked_sub(1)?;
        self.counts[class.index()] = next;
        Some(next)
    }
}

impl ComponentBasisDependencyClass {
    const fn index(self) -> usize {
        match self {
            Self::ProductBranchHead => 0,
            Self::RetainedCompositeHistory => 1,
            Self::AdmittedObservation => 2,
            Self::ActivePublicationAttempt => 3,
            Self::ProductUnpublishedOwnerEffects => 4,
            Self::HistoricalInspection => 5,
        }
    }
}
