use super::{
    SupportActionBreadthBudget, SupportAllocationScope, SupportPathClass,
    SupportProgramDensityClass,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportProgramPathPolicy {
    pub path_class: SupportPathClass,
    pub density_class: SupportProgramDensityClass,
    pub allocation_scope: SupportAllocationScope,
    pub budget: SupportActionBreadthBudget,
    pub payload_header_bytes: u64,
}

impl SupportProgramPathPolicy {
    pub fn admission_request(self, affected_entries: u64) -> SupportProgramPathAdmissionRequest {
        SupportProgramPathAdmissionRequest {
            policy: self,
            affected_entries,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportProgramPathAdmissionRequest {
    pub policy: SupportProgramPathPolicy,
    pub affected_entries: u64,
}
