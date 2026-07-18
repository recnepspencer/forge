use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::classification::ConsolidationEvidenceStatus;
use crate::discovery::BaselineCaptureStatus;
use crate::evidence::sha256_file;

use super::C2QuarantinedClaim;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalEvidencePolicy {
    pub schema_version: u32,
    pub decision: HistoricalEvidenceDecision,
    pub missing_evidence: BTreeSet<String>,
    pub replacement_predicates: BTreeSet<String>,
    pub prohibited_claims: BTreeSet<String>,
    pub quarantines: Vec<C2QuarantinedClaim>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum HistoricalEvidenceDecision {
    RetrospectiveQuarantine,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoricalEvidenceDisposition {
    pub policy_sha256: String,
    pub baseline_status_sha256: String,
    pub consolidation_status_sha256: String,
    pub posture: String,
    pub prohibited_claims: BTreeSet<String>,
    pub quarantines: Vec<C2QuarantinedClaim>,
}

impl HistoricalEvidencePolicy {
    pub fn read_and_assess(
        policy_path: &Path,
        baseline_path: &Path,
        consolidation_path: &Path,
    ) -> Result<HistoricalEvidenceDisposition, String> {
        let policy: Self = crate::evidence::read_json(policy_path)?;
        let baseline: BaselineCaptureStatus = crate::evidence::read_json(baseline_path)?;
        let consolidation: ConsolidationEvidenceStatus =
            crate::evidence::read_json(consolidation_path)?;
        baseline.validate()?;
        consolidation.validate()?;
        if baseline.closeout_eligible || consolidation.closeout_eligible {
            return Err(
                "retrospective policy may not replace a partially claimed historical baseline"
                    .to_owned(),
            );
        }
        policy.validate()?;
        Ok(HistoricalEvidenceDisposition {
            policy_sha256: sha256_file(policy_path)?,
            baseline_status_sha256: sha256_file(baseline_path)?,
            consolidation_status_sha256: sha256_file(consolidation_path)?,
            posture: "known historical surface preserved; unknown historical execution facts permanently quarantined and never asserted"
                .to_owned(),
            prohibited_claims: policy.prohibited_claims,
            quarantines: policy.quarantines,
        })
    }

    fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported historical evidence policy schema: {}",
                self.schema_version
            ));
        }
        if self.decision != HistoricalEvidenceDecision::RetrospectiveQuarantine {
            return Err("historical evidence policy has an unsupported decision".to_owned());
        }
        require_exact("missing evidence", &self.missing_evidence, MISSING_EVIDENCE)?;
        require_exact(
            "replacement predicates",
            &self.replacement_predicates,
            REPLACEMENT_PREDICATES,
        )?;
        require_exact(
            "prohibited claims",
            &self.prohibited_claims,
            PROHIBITED_CLAIMS,
        )?;
        let quarantine_ids: BTreeSet<_> = self
            .quarantines
            .iter()
            .map(|claim| claim.claim_identity.as_str())
            .collect();
        let expected: BTreeSet<_> = REQUIRED_QUARANTINES.iter().copied().collect();
        if quarantine_ids != expected || quarantine_ids.len() != self.quarantines.len() {
            return Err(format!(
                "historical policy quarantine set differs: expected {expected:?}, observed {quarantine_ids:?}"
            ));
        }
        for quarantine in &self.quarantines {
            quarantine.validate()?;
        }
        Ok(())
    }
}

impl HistoricalEvidenceDisposition {
    pub(super) fn validate(&self) -> Result<(), String> {
        for (name, digest) in [
            ("policy", &self.policy_sha256),
            ("baseline status", &self.baseline_status_sha256),
            ("consolidation status", &self.consolidation_status_sha256),
        ] {
            if digest.len() != 64 || !digest.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                return Err(format!("historical {name} identity is not SHA-256"));
            }
        }
        if self.posture
            != "known historical surface preserved; unknown historical execution facts permanently quarantined and never asserted"
        {
            return Err("historical disposition overclaims its preservation posture".to_owned());
        }
        require_exact(
            "prohibited claims",
            &self.prohibited_claims,
            PROHIBITED_CLAIMS,
        )?;
        let identities: BTreeSet<_> = self
            .quarantines
            .iter()
            .map(|claim| claim.claim_identity.as_str())
            .collect();
        let expected: BTreeSet<_> = REQUIRED_QUARANTINES.iter().copied().collect();
        if identities != expected || identities.len() != self.quarantines.len() {
            return Err("historical disposition lost a required quarantine".to_owned());
        }
        for quarantine in &self.quarantines {
            quarantine.validate()?;
        }
        Ok(())
    }
}

fn require_exact(name: &str, observed: &BTreeSet<String>, expected: &[&str]) -> Result<(), String> {
    let expected: BTreeSet<_> = expected.iter().map(|item| (*item).to_owned()).collect();
    if observed == &expected {
        Ok(())
    } else {
        Err(format!(
            "historical policy {name} differs: expected {expected:?}, observed {observed:?}"
        ))
    }
}

const MISSING_EVIDENCE: &[&str] = &[
    "pre-c1-exact-executable-universe",
    "pre-c1-cold-warm-external-process-cost",
    "pre-consolidation-same-seed-behavioral-parity",
];

const REPLACEMENT_PREDICATES: &[&str] = &[
    "current-clean-warm-five-case-iteration-envelope",
    "current-executable-reverse-parity",
    "current-proof-full-body-seal",
    "known-baseline-ledger-reachability",
    "required-controlled-defect-localization",
];

const PROHIBITED_CLAIMS: &[&str] = &[
    "exact-pre-c1-proof-cardinality",
    "measured-pre-c1-to-post-c1-speedup",
    "same-seed-pre-consolidation-behavioral-parity",
];

const REQUIRED_QUARANTINES: &[&str] = &[
    "historical-undiscovered-proof-universe",
    "pre-consolidation-behavioral-parity",
];
