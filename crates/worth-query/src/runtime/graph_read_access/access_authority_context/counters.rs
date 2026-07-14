#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGraphReadAccessAuthorityCounters {
    authority_admission_count: usize,
    policy_tenant_admission_count: usize,
    relationship_proof_admission_count: usize,
    authority_denial_count: usize,
    adjacency_buffer_build_count: usize,
    frontier_buffer_build_count: usize,
    visited_buffer_build_count: usize,
    result_buffer_build_count: usize,
}

impl WorthQueryGraphReadAccessAuthorityCounters {
    pub fn authority_admission_count(&self) -> usize {
        self.authority_admission_count
    }

    pub fn policy_tenant_admission_count(&self) -> usize {
        self.policy_tenant_admission_count
    }

    pub fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    pub fn authority_denial_count(&self) -> usize {
        self.authority_denial_count
    }

    pub fn adjacency_buffer_build_count(&self) -> usize {
        self.adjacency_buffer_build_count
    }

    pub fn frontier_buffer_build_count(&self) -> usize {
        self.frontier_buffer_build_count
    }

    pub fn visited_buffer_build_count(&self) -> usize {
        self.visited_buffer_build_count
    }

    pub fn result_buffer_build_count(&self) -> usize {
        self.result_buffer_build_count
    }

    pub(crate) fn admitted(policy_tenant: bool, relationship_proof: bool) -> Self {
        Self {
            authority_admission_count: 1,
            policy_tenant_admission_count: usize::from(policy_tenant),
            relationship_proof_admission_count: usize::from(relationship_proof),
            authority_denial_count: 0,
            adjacency_buffer_build_count: 0,
            frontier_buffer_build_count: 0,
            visited_buffer_build_count: 0,
            result_buffer_build_count: 0,
        }
    }

    pub(crate) fn denied(policy_tenant: bool, relationship_proof: bool) -> Self {
        Self {
            authority_admission_count: 0,
            policy_tenant_admission_count: usize::from(policy_tenant),
            relationship_proof_admission_count: usize::from(relationship_proof),
            authority_denial_count: 1,
            adjacency_buffer_build_count: 0,
            frontier_buffer_build_count: 0,
            visited_buffer_build_count: 0,
            result_buffer_build_count: 0,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "authority_counters:admit:{}:policy:{}:proof:{}:deny:{}:adjacency:{}:frontier:{}:visited:{}:result:{}",
            self.authority_admission_count,
            self.policy_tenant_admission_count,
            self.relationship_proof_admission_count,
            self.authority_denial_count,
            self.adjacency_buffer_build_count,
            self.frontier_buffer_build_count,
            self.visited_buffer_build_count,
            self.result_buffer_build_count
        )
    }
}
