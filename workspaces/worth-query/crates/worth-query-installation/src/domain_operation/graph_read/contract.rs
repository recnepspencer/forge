use super::{WorthQueryDomainOperationGraphReadRole, WorthQueryOperationGraphReadRole};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationGraphReadContract {
    NotRequired,
    Declared {
        roles: Vec<WorthQueryOperationGraphReadRole>,
    },
    DeclaredDomain {
        roles: Vec<WorthQueryDomainOperationGraphReadRole>,
    },
}

impl WorthQueryOperationGraphReadContract {
    pub fn roles(&self) -> &[WorthQueryOperationGraphReadRole] {
        match self {
            Self::Declared { roles } => roles,
            Self::NotRequired | Self::DeclaredDomain { .. } => &[],
        }
    }

    pub fn domain_roles(&self) -> &[WorthQueryDomainOperationGraphReadRole] {
        match self {
            Self::DeclaredDomain { roles } => roles,
            Self::NotRequired | Self::Declared { .. } => &[],
        }
    }
}
