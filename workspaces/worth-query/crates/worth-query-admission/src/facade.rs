//! Public admission-authority contract.

pub mod authenticated_principal {
    pub use crate::authenticated_principal::*;
}

pub mod domain_computation {
    pub use crate::domain_computation::*;
}

pub mod basis {
    pub use crate::domain_computation::basis_lifecycle::*;
}

pub mod resource_admission {
    pub use crate::domain_computation::execution_resource_admission::*;
}

pub mod convergence_epoch {
    pub use crate::domain_computation::convergence_epoch_admission::*;
}

pub mod policy {
    pub use crate::domain_computation::policy_basis::*;
}

pub mod relationship {
    pub use crate::domain_computation::relationship_proof::*;
}

pub mod tenant {
    pub use crate::domain_computation::tenant_basis::*;
}
