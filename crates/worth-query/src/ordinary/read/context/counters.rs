#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryReadContextAdmissionCounters {
    canonical_query_identity_read_count: usize,
    policy_tenant_admission_attempt_count: usize,
    policy_tenant_admitted_count: usize,
    relationship_proof_admission_attempt_count: usize,
    relationship_proof_admitted_count: usize,
    graph_authority_admission_attempt_count: usize,
    graph_authority_admitted_count: usize,
}

impl WorthQueryReadContextAdmissionCounters {
    pub fn canonical_query_identity_read_count(&self) -> usize {
        self.canonical_query_identity_read_count
    }

    pub fn policy_tenant_admission_attempt_count(&self) -> usize {
        self.policy_tenant_admission_attempt_count
    }

    pub fn policy_tenant_admitted_count(&self) -> usize {
        self.policy_tenant_admitted_count
    }

    pub fn relationship_proof_admission_attempt_count(&self) -> usize {
        self.relationship_proof_admission_attempt_count
    }

    pub fn relationship_proof_admitted_count(&self) -> usize {
        self.relationship_proof_admitted_count
    }

    pub fn graph_authority_admission_attempt_count(&self) -> usize {
        self.graph_authority_admission_attempt_count
    }

    pub fn graph_authority_admitted_count(&self) -> usize {
        self.graph_authority_admitted_count
    }

    pub(crate) fn begin() -> Self {
        Self {
            canonical_query_identity_read_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn record_policy_tenant_admission_attempt(&mut self) {
        self.policy_tenant_admission_attempt_count += 1;
    }

    pub(crate) fn record_policy_tenant_admitted(&mut self) {
        self.policy_tenant_admitted_count += 1;
    }

    pub(crate) fn record_relationship_proof_admission_attempt(&mut self) {
        self.relationship_proof_admission_attempt_count += 1;
    }

    pub(crate) fn record_relationship_proof_admitted(&mut self) {
        self.relationship_proof_admitted_count += 1;
    }

    pub(crate) fn record_graph_authority_admission_attempt(&mut self) {
        self.graph_authority_admission_attempt_count += 1;
    }

    pub(crate) fn record_graph_authority_admitted(&mut self) {
        self.graph_authority_admitted_count += 1;
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "context-counters:{}:{}:{}:{}:{}:{}:{}",
            self.canonical_query_identity_read_count,
            self.policy_tenant_admission_attempt_count,
            self.policy_tenant_admitted_count,
            self.relationship_proof_admission_attempt_count,
            self.relationship_proof_admitted_count,
            self.graph_authority_admission_attempt_count,
            self.graph_authority_admitted_count,
        )
    }
}
