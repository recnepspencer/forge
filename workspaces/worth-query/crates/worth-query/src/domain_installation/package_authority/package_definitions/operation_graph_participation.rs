use std::any::TypeId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryDomainOperationGraphParticipationRecord {
    operation_marker: TypeId,
    family_marker: TypeId,
    graph_marker: TypeId,
    role: String,
}

impl WorthQueryDomainOperationGraphParticipationRecord {
    pub(crate) fn typed<O: 'static, F: 'static, G: 'static>(role: impl Into<String>) -> Self {
        Self {
            operation_marker: TypeId::of::<O>(),
            family_marker: TypeId::of::<F>(),
            graph_marker: TypeId::of::<G>(),
            role: role.into(),
        }
    }

    pub(crate) fn operation_marker(&self) -> TypeId {
        self.operation_marker
    }

    pub(crate) fn family_marker(&self) -> TypeId {
        self.family_marker
    }

    pub(crate) fn graph_marker(&self) -> TypeId {
        self.graph_marker
    }

    pub(crate) fn role(&self) -> &str {
        &self.role
    }
}
