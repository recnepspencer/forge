use worth_relational::facade::runtime::CustomInvariantRegistration;

#[derive(Default)]
pub(crate) struct WorthQueryCompiledDomainSubstrates {
    pub(crate) custom_invariants: Vec<CustomInvariantRegistration>,
}
