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

pub mod convergence_epoch {
    pub use crate::domain_computation::convergence_epoch::*;
}

pub mod installed {
    pub use super::{convergence_epoch, domain_computation, provider_session, runtime};
}

#[doc(hidden)]
pub mod integration {
    pub use crate::domain_computation::artifact_owner::{
        WorthQueryArtifactAccessAuthority, WorthQueryArtifactProductionAuthority,
        WorthQueryArtifactTransferAdmission, WorthQueryWorkflowArtifactAuthority,
        WorthQueryWorkflowArtifactRegistry,
    };
    pub use crate::domain_computation::provider_session::graph_provider::bounded_step::provider_anchor::WorthQueryGraphProviderAnchor;

    #[doc(hidden)]
    pub mod legacy_provider_execution {
        pub use crate::domain_computation::provider_session::graph_provider::bounded_step::legacy_one_shot::execute_legacy_one_shot;
    }
}
