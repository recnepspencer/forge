#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RelationshipProofCounters {
    relationship_proof_admission_count: usize,
    relationship_proof_denial_count: usize,
    relationship_proof_topology_width: usize,
    relationship_proof_recursive_broadening_denial_count: usize,
    forbidden_host_callback_proof_count: usize,
    truth_touch_count: usize,
}

impl RelationshipProofCounters {
    pub(crate) fn admit(&mut self, topology_width: usize) {
        self.relationship_proof_admission_count += 1;
        self.relationship_proof_topology_width += topology_width;
    }

    pub(crate) fn deny(&mut self) {
        self.relationship_proof_denial_count += 1;
    }

    pub(crate) fn deny_recursive_broadening(&mut self) {
        self.relationship_proof_recursive_broadening_denial_count += 1;
        self.deny();
    }

    pub(crate) fn deny_host_callback(&mut self) {
        self.forbidden_host_callback_proof_count += 1;
        self.deny();
    }

    pub fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    pub fn relationship_proof_denial_count(&self) -> usize {
        self.relationship_proof_denial_count
    }

    pub fn relationship_proof_topology_width(&self) -> usize {
        self.relationship_proof_topology_width
    }

    pub fn relationship_proof_recursive_broadening_denial_count(&self) -> usize {
        self.relationship_proof_recursive_broadening_denial_count
    }

    pub fn forbidden_host_callback_proof_count(&self) -> usize {
        self.forbidden_host_callback_proof_count
    }

    pub fn truth_touch_count(&self) -> usize {
        self.truth_touch_count
    }

    pub fn digest_parts(&self) -> Vec<String> {
        vec![
            format!(
                "proof_admission:{}",
                self.relationship_proof_admission_count
            ),
            format!("proof_denial:{}", self.relationship_proof_denial_count),
            format!("proof_width:{}", self.relationship_proof_topology_width),
            format!(
                "proof_recursive_denial:{}",
                self.relationship_proof_recursive_broadening_denial_count
            ),
            format!(
                "proof_host_callback:{}",
                self.forbidden_host_callback_proof_count
            ),
            format!("truth_touch:{}", self.truth_touch_count),
        ]
    }
}
