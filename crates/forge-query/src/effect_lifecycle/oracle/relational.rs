use forge_relational::facade::history::BranchId;
use forge_relational::facade::runtime::RelationalRuntime;

use crate::identity::hash_parts;

use super::{EffectExecutionOracleError, EffectExecutionOracleErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationalExecutionOracle {
    branch_identity: String,
    observed_commit_id: u64,
    observed_version_id: u64,
    observed_parent_commit_ids: Vec<u64>,
    relational_oracle_digest: String,
}

impl RelationalExecutionOracle {
    pub fn new(
        branch_identity: impl Into<String>,
        observed_commit_id: u64,
        observed_version_id: u64,
        observed_parent_commit_ids: Vec<u64>,
    ) -> Self {
        let branch_identity = branch_identity.into();
        let relational_oracle_digest = hash_parts(&[
            "relational_execution_oracle_v1".to_string(),
            format!("branch:{branch_identity}"),
            format!("commit:{observed_commit_id}"),
            format!("version:{observed_version_id}"),
            format!(
                "parents:{}",
                observed_parent_commit_ids
                    .iter()
                    .map(u64::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ]);
        Self {
            branch_identity,
            observed_commit_id,
            observed_version_id,
            observed_parent_commit_ids,
            relational_oracle_digest,
        }
    }

    pub fn observe_branch_head(
        runtime: &RelationalRuntime,
        branch: &BranchId,
    ) -> Result<Self, EffectExecutionOracleError> {
        let history = runtime.history();
        let observed = history.branch_head(branch).ok_or_else(|| {
            EffectExecutionOracleError::new(
                EffectExecutionOracleErrorKind::RelationalObservationMissingBranchHead,
                format!(
                    "independent relational oracle inspection could not resolve branch head for `{}`",
                    branch.0
                ),
                format!("branch:{}", branch.0),
                None,
            )
        })?;
        Ok(Self::new(
            branch.0.clone(),
            observed.commit_id.0,
            observed.version_id.0,
            observed.parents.iter().map(|parent| parent.0).collect(),
        ))
    }

    pub fn branch_identity(&self) -> &str {
        &self.branch_identity
    }

    pub fn observed_commit_id(&self) -> u64 {
        self.observed_commit_id
    }

    pub fn observed_version_id(&self) -> u64 {
        self.observed_version_id
    }

    pub fn observed_parent_commit_ids(&self) -> &[u64] {
        &self.observed_parent_commit_ids
    }

    pub fn relational_oracle_digest(&self) -> &str {
        &self.relational_oracle_digest
    }
}
