use crate::domain_computation::application_outcome_identity::WorthQueryApplicationOutcomeIdentity;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorthQueryApplicationCommitOutcomeIdentity(WorthQueryApplicationOutcomeIdentity);

impl WorthQueryApplicationCommitOutcomeIdentity {
    pub(in crate::domain_computation::primary_graph) fn mint() -> Option<Self> {
        WorthQueryApplicationOutcomeIdentity::mint().map(Self)
    }

    pub(in crate::domain_computation::primary_graph) fn restore(value: u64) -> Option<Self> {
        WorthQueryApplicationOutcomeIdentity::restore(value).map(Self)
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}
