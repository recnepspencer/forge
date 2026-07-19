use worth_relational::facade::runtime::CustomInvariantRegistration;

use crate::runtime::WorthQueryGraphObligationRegistration;

#[derive(Default)]
pub(crate) struct WorthQueryCompiledDomainSubstrates {
    pub(crate) custom_invariants: Vec<CustomInvariantRegistration>,
    pub(crate) graph_obligations: Vec<WorthQueryGraphObligationRegistration>,
}
