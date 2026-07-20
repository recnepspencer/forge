use std::any::TypeId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryDomainOperationRequiredDomainRecord {
    operation_marker: TypeId,
    family_marker: TypeId,
    domain_marker: TypeId,
    role: String,
}

impl WorthQueryDomainOperationRequiredDomainRecord {
    pub(crate) fn typed<O: 'static, F: 'static, R: 'static>(role: impl Into<String>) -> Self {
        Self {
            operation_marker: TypeId::of::<O>(),
            family_marker: TypeId::of::<F>(),
            domain_marker: TypeId::of::<R>(),
            role: role.into(),
        }
    }
    pub(crate) fn operation_marker(&self) -> TypeId {
        self.operation_marker
    }
    pub(crate) fn family_marker(&self) -> TypeId {
        self.family_marker
    }
    pub(crate) fn domain_marker(&self) -> TypeId {
        self.domain_marker
    }
    pub(crate) fn role(&self) -> &str {
        &self.role
    }
}
