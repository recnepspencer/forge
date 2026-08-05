use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ApplicationQueryObservableInfluence {
    RowPresence,
    Ordering,
    Pagination,
    Count,
    Aggregate,
    Explanation,
    HistoricalMembership,
    Preview,
    LiveMembership,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationQueryInfluenceContract {
    permitted: BTreeSet<ApplicationQueryObservableInfluence>,
}

impl ApplicationQueryInfluenceContract {
    pub const fn forbid_all() -> Self {
        Self {
            permitted: BTreeSet::new(),
        }
    }

    pub fn permit(surfaces: impl IntoIterator<Item = ApplicationQueryObservableInfluence>) -> Self {
        Self {
            permitted: surfaces.into_iter().collect(),
        }
    }

    pub fn permit_all() -> Self {
        Self::permit([
            ApplicationQueryObservableInfluence::RowPresence,
            ApplicationQueryObservableInfluence::Ordering,
            ApplicationQueryObservableInfluence::Pagination,
            ApplicationQueryObservableInfluence::Count,
            ApplicationQueryObservableInfluence::Aggregate,
            ApplicationQueryObservableInfluence::Explanation,
            ApplicationQueryObservableInfluence::HistoricalMembership,
            ApplicationQueryObservableInfluence::Preview,
            ApplicationQueryObservableInfluence::LiveMembership,
        ])
    }

    pub fn permits(&self, surface: ApplicationQueryObservableInfluence) -> bool {
        self.permitted.contains(&surface)
    }

    pub fn permitted(
        &self,
    ) -> impl ExactSizeIterator<Item = ApplicationQueryObservableInfluence> + '_ {
        self.permitted.iter().copied()
    }
}
