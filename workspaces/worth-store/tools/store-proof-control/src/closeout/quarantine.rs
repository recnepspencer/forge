use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct C2QuarantinedClaim {
    pub claim_identity: String,
    pub rationale: String,
    pub missing_evidence: Vec<String>,
    pub c2_owner: String,
}

impl C2QuarantinedClaim {
    pub(super) fn validate(&self) -> Result<(), String> {
        if self.claim_identity.trim().is_empty()
            || self.rationale.trim().is_empty()
            || self.missing_evidence.is_empty()
            || self
                .missing_evidence
                .iter()
                .any(|evidence| evidence.trim().is_empty())
            || self.c2_owner != "executable-reality-ledger"
        {
            return Err(format!(
                "C2 quarantine has incomplete authority: {:?}",
                self.claim_identity
            ));
        }
        Ok(())
    }
}
