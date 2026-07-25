//! Public contract for the internal execution authority.

pub mod domain_computation {
    pub use crate::domain_computation::*;
}

pub mod runtime {
    pub use crate::domain_computation::execution_runtime::*;
    pub use crate::domain_computation::{
        WorthQueryExecutionBoundOperationAuthority, WorthQueryExecutionOperationBindingDenial,
        WorthQueryInstalledDomainExecutionAuthority,
    };
}

pub mod provider_session {
    pub use crate::domain_computation::provider_session::*;
}

pub mod installed {
    pub use super::{domain_computation, provider_session, runtime};
}

#[doc(hidden)]
pub mod integration {
    pub use crate::domain_computation::artifact_owner::{
        WorthQueryArtifactAccessAuthority, WorthQueryArtifactProductionAuthority,
        WorthQueryArtifactTransferAdmission, WorthQueryWorkflowArtifactAuthority,
        WorthQueryWorkflowArtifactRegistry,
    };
}
