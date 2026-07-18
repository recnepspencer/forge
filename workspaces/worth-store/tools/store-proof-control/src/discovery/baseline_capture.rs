use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineCaptureStatus {
    pub schema_version: u32,
    pub topology_inventory: String,
    pub historical_artifact_observation: String,
    pub cold_execution_observation: String,
    pub warm_execution_observation: String,
    pub external_process_observation: String,
    pub closeout_eligible: bool,
    pub limitation: String,
}

impl BaselineCaptureStatus {
    pub fn topology_only(historical_artifact_observation: String) -> Self {
        Self {
            schema_version: 1,
            topology_inventory: "captured-before-consolidation-with-known-discovery-gaps".to_owned(),
            historical_artifact_observation,
            cold_execution_observation: "not-captured".to_owned(),
            warm_execution_observation: "not-captured".to_owned(),
            external_process_observation: "not-captured".to_owned(),
            closeout_eligible: false,
            limitation: "Topology was frozen from source approximation rather than libtest and rustdoc executable listings, and without cold, warm, or external-process run observations. Historical compile-fail and ignored doctests were therefore omitted and cannot be reconstructed as contemporaneous observations; this baseline cannot qualify closeout.".to_owned(),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported baseline capture schema: {}",
                self.schema_version
            ));
        }
        if self.topology_inventory != "captured-before-consolidation-with-known-discovery-gaps" {
            return Err("pre-cleanup topology inventory is not frozen".to_owned());
        }
        let run_evidence_complete = self.cold_execution_observation == "captured"
            && self.warm_execution_observation == "captured"
            && self.external_process_observation == "captured";
        if self.closeout_eligible && !run_evidence_complete {
            return Err(
                "baseline cannot claim closeout eligibility without cold, warm, and external-process observations"
                    .to_owned(),
            );
        }
        if !run_evidence_complete && self.limitation.trim().is_empty() {
            return Err("incomplete baseline capture lacks an explicit limitation".to_owned());
        }
        Ok(())
    }
}
