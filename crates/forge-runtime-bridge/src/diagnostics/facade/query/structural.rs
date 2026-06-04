use super::*;

impl BridgeDiagnosticsFacade {
    pub fn structural_remap_records(&self) -> Vec<BridgeCanonicalStructuralRemapRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .structural_remap_records()
    }

    pub fn structural_branch_comparison_records(
        &self,
    ) -> Vec<BridgeCanonicalStructuralBranchComparisonRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .structural_branch_comparison_records()
    }

    pub fn last_structural_remap_record(&self) -> Option<BridgeCanonicalStructuralRemapRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_structural_remap_record()
    }

    pub fn last_structural_branch_comparison_record(
        &self,
    ) -> Option<BridgeCanonicalStructuralBranchComparisonRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .last_structural_branch_comparison_record()
    }

    pub fn structural_remap_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeCanonicalStructuralRemapRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .structural_remap_record_for_identity(record_identity)
    }

    pub fn structural_branch_comparison_record_for_identity(
        &self,
        record_identity: &str,
    ) -> Option<BridgeCanonicalStructuralBranchComparisonRecord> {
        self.state
            .read()
            .expect("bridge diagnostics lock poisoned")
            .structural_branch_comparison_record_for_identity(record_identity)
    }
}
