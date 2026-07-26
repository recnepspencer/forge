use crate::identity::data::VersionId;
use crate::visibility::execution_basis::{
    RelationalExecutionBasisDenial, RelationalExecutionBasisLease,
};

use super::RuntimeBridgeRelationalSource;

impl RuntimeBridgeRelationalSource {
    /// Asks the owning Relational source to retain one exact version for a
    /// managed execution. The returned move-only lease is the read authority;
    /// a copied version or snapshot identity cannot substitute for it.
    pub fn admit_execution_basis(
        &self,
        version_id: VersionId,
    ) -> Result<RelationalExecutionBasisLease, RelationalExecutionBasisDenial> {
        crate::visibility::execution_basis::admit_execution_basis(&self.runtime, version_id)
    }
}
