#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthQueryInstalledGraphObligationOwner {
    Relational,
    RuntimeBridge,
    Signal,
    QueryAdmission,
    QueryExecution,
}

pub(super) const RELATIONAL_READ_ROUTE: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::Relational,
    WorthQueryInstalledGraphObligationOwner::QueryExecution,
];

pub(super) const PRINCIPAL_AUTHORIZATION_ROUTE: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::Relational,
    WorthQueryInstalledGraphObligationOwner::QueryAdmission,
];

pub(super) const POLICY_AUTHORIZATION_ROUTE: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::Relational,
    WorthQueryInstalledGraphObligationOwner::RuntimeBridge,
    WorthQueryInstalledGraphObligationOwner::Signal,
    WorthQueryInstalledGraphObligationOwner::QueryAdmission,
];

pub(super) const MUTATION_TOUCH_ROUTE: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::Relational,
    WorthQueryInstalledGraphObligationOwner::QueryAdmission,
];

pub(super) const EFFECT_APPLICATION_ROUTE: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::QueryExecution,
    WorthQueryInstalledGraphObligationOwner::Relational,
];

pub(super) const INVARIANT_EXECUTION_ROUTE: &[WorthQueryInstalledGraphObligationOwner] = &[
    WorthQueryInstalledGraphObligationOwner::QueryExecution,
    WorthQueryInstalledGraphObligationOwner::Relational,
];
