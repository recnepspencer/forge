use crate::branch::RelationalBranchIdentity;
use crate::history::data::BranchId;
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
    /// Returns the owner-issued identity for one registered Relational branch.
    /// The branch name is descriptive input; the returned identity is the
    /// runtime-affine value required by execution-basis admission.
    pub fn branch_identity(
        &self,
        branch_id: &BranchId,
    ) -> Result<RelationalBranchIdentity, RelationalExecutionBasisDenial> {
        self.runtime.with_runtime(|runtime| {
            runtime.branch_identity(branch_id).map_err(|_denial| {
                RelationalExecutionBasisDenial::new(
                    crate::visibility::execution_basis::RelationalExecutionBasisDenialKind::BranchMismatch,
                    "Relational branch identity was not owner-admissible",
                    Default::default(),
                )
            })
        })
    }

    /// Resolves the Relational-owned branch for an existing version without
    /// granting execution authority for that version.
    pub fn resolve_execution_basis_branch(&self, version_id: VersionId) -> Option<BranchId> {
        self.runtime.with_runtime(|runtime| {
            crate::visibility::branch_scope::branch_for_version(runtime, version_id)
        })
    }

    /// Asks the owning Relational source to retain one exact version for a
    /// managed execution. The returned move-only lease is the read authority;
    /// a copied branch name, version, or snapshot identity cannot substitute
    /// for the owner-issued identity.
    pub fn admit_execution_basis_for_identity(
        &self,
        identity: &RelationalBranchIdentity,
        version_id: VersionId,
    ) -> Result<RelationalExecutionBasisLease, RelationalExecutionBasisDenial> {
        self.runtime.with_runtime(|runtime| {
            if identity.runtime_instance_id() != runtime.runtime_instance_id() {
                return Err(RelationalExecutionBasisDenial::new(
                    crate::visibility::execution_basis::RelationalExecutionBasisDenialKind::BranchMismatch,
                    "owner branch identity belongs to another runtime",
                    Default::default(),
                ));
            }
            crate::visibility::execution_basis::admit_execution_basis(
                runtime,
                identity.branch_id(),
                version_id,
            )
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
        let branch_id = evaluation
            .record()
            .decision_log()
            .branch_identity()
            .relational_branch_id()
            .map(|branch| BranchId(branch.to_owned()))
            .ok_or_else(|| {
                RelationalBridgeTruthViewBasisDenial::SnapshotAuthority(
                    "Bridge truth-view branch is not a Relational branch identity".to_owned(),
                )
            })?;
        self.runtime.with_runtime(|runtime| {
            let identity = runtime.branch_identity(&branch_id).map_err(|denial| {
                RelationalBridgeTruthViewBasisDenial::SnapshotAuthority(format!(
                    "Bridge truth-view branch was not owner-admissible: {denial:?}"
                ))
            })?;
            crate::visibility::execution_basis::admit_execution_basis(
                runtime,
                identity.branch_id(),
                version_id,
            )
            .map_err(RelationalBridgeTruthViewBasisDenial::ExecutionBasis)
        })
    }
}
