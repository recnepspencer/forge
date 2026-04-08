use std::sync::Arc;

use crate::routing::BridgePlannedRoute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoweredConsumedChangeSet {
    Routing {
        planned_routes: Arc<[BridgePlannedRoute]>,
    },
    ReplayAudit {
        canonical_member_identities: Arc<[Arc<str>]>,
    },
}

impl LoweredConsumedChangeSet {
    pub fn planned_routes(&self) -> Option<&[BridgePlannedRoute]> {
        match self {
            Self::Routing { planned_routes } => Some(planned_routes),
            Self::ReplayAudit { .. } => None,
        }
    }

    pub fn replay_audit_member_identities(&self) -> Option<&[Arc<str>]> {
        match self {
            Self::Routing { .. } => None,
            Self::ReplayAudit {
                canonical_member_identities,
            } => Some(canonical_member_identities),
        }
    }
}
