pub(in crate::domain_computation) struct WorthQueryOperationGraphReadCompletion {
    session_identity: worth_foundational::facade::CanonicalDigestId,
    branch_id: worth_relational::facade::history::BranchId,
}

impl WorthQueryOperationGraphReadCompletion {
    pub(in crate::domain_computation::primary_graph) const fn mint(
        session_identity: worth_foundational::facade::CanonicalDigestId,
        branch_id: worth_relational::facade::history::BranchId,
    ) -> Self {
        Self {
            session_identity,
            branch_id,
        }
    }

    pub(in crate::domain_computation) const fn session_identity(
        &self,
    ) -> &worth_foundational::facade::CanonicalDigestId {
        &self.session_identity
    }

    pub(in crate::domain_computation) const fn branch_id(
        &self,
    ) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }
}

pub(in crate::domain_computation) struct WorthQueryOperationMutationTouchCompletion {
    session_identity: worth_foundational::facade::CanonicalDigestId,
    branch_id: worth_relational::facade::history::BranchId,
    realized_effect_count: usize,
}

impl WorthQueryOperationMutationTouchCompletion {
    pub(super) const fn mint(
        session_identity: worth_foundational::facade::CanonicalDigestId,
        branch_id: worth_relational::facade::history::BranchId,
        realized_effect_count: usize,
    ) -> Self {
        Self {
            session_identity,
            branch_id,
            realized_effect_count,
        }
    }

    pub(in crate::domain_computation) const fn session_identity(
        &self,
    ) -> &worth_foundational::facade::CanonicalDigestId {
        &self.session_identity
    }

    pub(in crate::domain_computation) const fn branch_id(
        &self,
    ) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }

    pub(in crate::domain_computation) const fn realized_effect_count(&self) -> usize {
        self.realized_effect_count
    }
}

pub(in crate::domain_computation) struct WorthQueryOperationInvariantExecutionCompletion {
    session_identity: worth_foundational::facade::CanonicalDigestId,
    branch_id: worth_relational::facade::history::BranchId,
    receipt_count: usize,
}

impl WorthQueryOperationInvariantExecutionCompletion {
    pub(super) fn mint(
        session_identity: worth_foundational::facade::CanonicalDigestId,
        branch_id: worth_relational::facade::history::BranchId,
        receipt_count: usize,
    ) -> Result<Self, ()> {
        (receipt_count > 0)
            .then_some(Self {
                session_identity,
                branch_id,
                receipt_count,
            })
            .ok_or(())
    }

    pub(in crate::domain_computation) const fn session_identity(
        &self,
    ) -> &worth_foundational::facade::CanonicalDigestId {
        &self.session_identity
    }

    pub(in crate::domain_computation) const fn branch_id(
        &self,
    ) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }

    pub(in crate::domain_computation) const fn receipt_count(&self) -> usize {
        self.receipt_count
    }
}

pub(in crate::domain_computation) struct WorthQueryOperationEffectApplicationCompletion {
    session_identity: worth_foundational::facade::CanonicalDigestId,
    branch_id: worth_relational::facade::history::BranchId,
    provider_runtime_instance_id: u64,
    commit_id: worth_relational::facade::history::CommitId,
}

impl WorthQueryOperationEffectApplicationCompletion {
    pub(super) fn mint(
        session_identity: worth_foundational::facade::CanonicalDigestId,
        receipt: &super::WorthQueryApplicationCommitReceipt,
    ) -> Self {
        Self {
            session_identity,
            branch_id: receipt.branch_id().clone(),
            provider_runtime_instance_id: receipt.provider_runtime_instance_id(),
            commit_id: receipt.commit_id(),
        }
    }

    pub(in crate::domain_computation) const fn session_identity(
        &self,
    ) -> &worth_foundational::facade::CanonicalDigestId {
        &self.session_identity
    }

    pub(in crate::domain_computation) const fn branch_id(
        &self,
    ) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }

    pub(in crate::domain_computation) const fn provider_runtime_instance_id(&self) -> u64 {
        self.provider_runtime_instance_id
    }

    pub(in crate::domain_computation) const fn commit_id(
        &self,
    ) -> worth_relational::facade::history::CommitId {
        self.commit_id
    }
}
