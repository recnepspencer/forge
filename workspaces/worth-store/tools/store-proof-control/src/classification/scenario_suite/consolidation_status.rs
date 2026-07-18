use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationEvidenceStatus {
    pub schema_version: u32,
    pub pre_consolidation_execution_evidence: String,
    pub identity_predicate_preservation: String,
    pub failure_localization_evidence: String,
    pub shared_source_codegen_evidence: String,
    pub closeout_eligible: bool,
    pub limitation: String,
}

impl ConsolidationEvidenceStatus {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported consolidation evidence schema: {}",
                self.schema_version
            ));
        }
        let full_parity = self.pre_consolidation_execution_evidence == "captured"
            && self.identity_predicate_preservation == "captured"
            && self.failure_localization_evidence == "captured"
            && self.shared_source_codegen_evidence == "captured";
        if self.closeout_eligible && !full_parity {
            return Err(
                "scenario consolidation cannot claim closeout without all four evidence classes"
                    .to_owned(),
            );
        }
        if !full_parity && self.limitation.trim().is_empty() {
            return Err("incomplete scenario consolidation evidence lacks a limitation".to_owned());
        }
        Ok(())
    }
}
