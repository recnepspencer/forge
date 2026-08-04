#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryInstalledGraphObligationOwner {
    RelationalGraph,
    RuntimeBridgeCorrespondence,
    SignalPolicy,
    QueryApplicationProgram,
    QueryInstalledInvariantProvider,
}

pub(super) const RELATIONAL_GRAPH: &[WorthQueryInstalledGraphObligationOwner] =
    &[WorthQueryInstalledGraphObligationOwner::RelationalGraph];

pub(super) const POLICY_EVALUATION: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::RelationalGraph,
    WorthQueryInstalledGraphObligationOwner::RuntimeBridgeCorrespondence,
    WorthQueryInstalledGraphObligationOwner::SignalPolicy,
];

pub(super) const APPLICATION_TOUCH: &[WorthQueryInstalledGraphObligationOwner] =
    &[WorthQueryInstalledGraphObligationOwner::QueryApplicationProgram];

pub(super) const INSTALLED_INVARIANT: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::RelationalGraph,
    WorthQueryInstalledGraphObligationOwner::QueryInstalledInvariantProvider,
];
