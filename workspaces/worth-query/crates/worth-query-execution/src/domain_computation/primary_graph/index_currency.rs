use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::history::{BranchId, RelationalCommitReceipt};
use worth_relational::facade::indexes::DerivedIndexBuildRequest;
use worth_relational::facade::runtime::RelationalRuntime;

use super::WorthQueryPrimaryGraphIntegrationHandle;

impl WorthQueryPrimaryGraphIntegrationHandle {
    #[cfg(test)]
    pub(crate) fn ensure_primary_indexes_current(
        &self,
        runtime: &mut RelationalRuntime,
    ) -> Result<(), &'static str> {
        let Some(head) = runtime.history().historical_latest_commit().cloned() else {
            return Ok(());
        };
        self.ensure_primary_indexes_for_commit(runtime, head)
    }

    pub(crate) fn ensure_primary_indexes_current_for_branch(
        &self,
        runtime: &mut RelationalRuntime,
        branch: &BranchId,
    ) -> Result<(), &'static str> {
        let head = super::exact_basis_access::current_branch_head(runtime, branch)
            .ok_or("primary graph branch has no authoritative head")?;
        self.ensure_primary_indexes_for_commit(runtime, head)
    }

    pub(crate) fn ensure_primary_indexes_for_basis(
        &self,
        runtime: &mut RelationalRuntime,
        basis: &AdmittedRelationalBranchBasis,
    ) -> Result<(), &'static str> {
        let observation = basis.observation();
        let Some(head) = runtime
            .history()
            .branch_head_for_observation(&observation)
            .map_err(|_| "application-query basis is not owned by this Relational runtime")?
            .cloned()
        else {
            return Ok(());
        };
        self.ensure_primary_indexes_for_commit(runtime, head)
    }

    fn ensure_primary_indexes_for_commit(
        &self,
        runtime: &mut RelationalRuntime,
        head: RelationalCommitReceipt,
    ) -> Result<(), &'static str> {
        let branch = head.branch_id.clone();
        let current = self.primary_index_ids.iter().all(|index_id| {
            runtime
                .index_access()
                .published_generation_for_commit(*index_id, &head)
                .is_some()
        });
        if current {
            return Ok(());
        }
        let build = runtime
            .index_authority()
            .build_for_commit(DerivedIndexBuildRequest {
                source_commit_id: head.commit_id,
                branch_id: branch,
                index_ids: self.primary_index_ids.to_vec(),
            });
        if build.failed_indexes.is_empty()
            && build.generations.len() == self.primary_index_ids.len()
        {
            Ok(())
        } else {
            Err("primary graph indexes could not recover to the authoritative head")
        }
    }
}
