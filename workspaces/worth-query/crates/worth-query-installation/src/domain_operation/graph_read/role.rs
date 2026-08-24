use super::{WorthQueryOperationGraphReadScope, WorthQueryOperationNativeProjectionContract};

use crate::domain_operation::{
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryOperationGraphReadRole {
    role: String,
    participation: WorthQueryOperationGraphParticipation,
    access: WorthQueryOperationGraphAccess,
    read_scopes: Vec<WorthQueryOperationGraphReadScope>,
}

impl WorthQueryOperationGraphReadRole {
    pub(crate) fn new(
        role: String,
        participation: WorthQueryOperationGraphParticipation,
        access: WorthQueryOperationGraphAccess,
        read_scopes: Vec<WorthQueryOperationGraphReadScope>,
    ) -> Self {
        Self {
            role,
            participation,
            access,
            read_scopes,
        }
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub const fn participation(&self) -> &WorthQueryOperationGraphParticipation {
        &self.participation
    }

    pub const fn access(&self) -> WorthQueryOperationGraphAccess {
        self.access
    }

    pub fn read_scopes(&self) -> &[WorthQueryOperationGraphReadScope] {
        &self.read_scopes
    }
}

/// Portable domain-operation role. It cannot impersonate a schema-bound
/// installed application read role.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainOperationGraphReadRole {
    pub role: String,
    pub participation: WorthQueryOperationGraphParticipation,
    pub access: WorthQueryOperationGraphAccess,
    pub semantic_reads: Vec<WorthQueryOperationNativeProjectionContract>,
}
