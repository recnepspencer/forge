#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OwnerCaseObservation<C> {
    case_id: C,
}

impl<C: Copy> OwnerCaseObservation<C> {
    pub const fn case_id(self) -> C {
        self.case_id
    }

    pub(super) const fn issued(case_id: C) -> Self {
        Self { case_id }
    }
}

/// Projects the case reached by an ordinary owner operation.
///
/// The trait is sealed so only production outcome owners in this crate can
/// issue observations. Case declarations and copied case identifiers cannot
/// satisfy an executed-outcome boundary.
pub trait ObserveOwnerCase: sealed::Sealed {
    type CaseId: Copy;

    fn owner_case_observation(&self) -> OwnerCaseObservation<Self::CaseId>;
}

pub(crate) mod sealed {
    pub trait Sealed {}
}
