pub(in crate::domain_computation) struct WorthQueryApplicationQueryGraphReadCompletion {
    session_identity: worth_foundational::facade::CanonicalDigestId,
    branch_id: worth_relational::facade::history::BranchId,
}

impl WorthQueryApplicationQueryGraphReadCompletion {
    pub(super) fn mint(session: &super::WorthQueryApplicationQueryGraphWorkSession) -> Self {
        Self {
            session_identity: *session.identity(),
            branch_id: session.branch_affinity().relational_branch().clone(),
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
