use crate::identity::data::VersionId;
use crate::visibility::execution_basis::{
    RelationalExecutionBasisDenial, RelationalExecutionBasisLease,
};
use worth_runtime_bridge::facade::BridgeTruthViewEvaluation;

use super::RuntimeBridgeRelationalSource;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RelationalBridgeTruthViewBasisDenial {
    SnapshotAuthority(String),
    ExecutionBasis(RelationalExecutionBasisDenial),
}

impl RuntimeBridgeRelationalSource {
    /// Asks the owning Relational source to retain one exact version for a
    /// managed execution. The returned move-only lease is the read authority;
    /// a copied version or snapshot identity cannot substitute for it.
    pub fn admit_execution_basis(
        &self,
        version_id: VersionId,
    ) -> Result<RelationalExecutionBasisLease, RelationalExecutionBasisDenial> {
        self.runtime.with_runtime(|runtime| {
            crate::visibility::execution_basis::admit_execution_basis(runtime, version_id)
        })
    }

    /// Retains the exact Relational version materialized by one typed Bridge
    /// truth-view evaluation.
    ///
    /// The Bridge observation is required deliberately: copied snapshot,
    /// commit, branch, or version identities cannot call this authority join.
    pub fn admit_truth_view_execution_basis(
        &self,
        evaluation: &BridgeTruthViewEvaluation,
    ) -> Result<RelationalExecutionBasisLease, RelationalBridgeTruthViewBasisDenial> {
        let version_id = self
            .runtime
            .with_runtime(|runtime| {
                super::snapshot_authority::resolve_snapshot_version(
                    runtime,
                    evaluation.snapshot_identity(),
                )
            })
            .map_err(|error| {
                RelationalBridgeTruthViewBasisDenial::SnapshotAuthority(error.to_string())
            })?;
        self.admit_execution_basis(version_id)
            .map_err(RelationalBridgeTruthViewBasisDenial::ExecutionBasis)
    }
}
