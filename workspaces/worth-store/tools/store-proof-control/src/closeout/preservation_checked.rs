use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::classification::{ClassifiedInventory, ProofDisposition};
use crate::evidence::sha256_serialized;
use crate::preservation::{
    validate_current_reachability, validate_ledger, ProofPreservationLedger,
};
use crate::ValidatedProofInventory;

use super::subject_map::production_subjects;
use super::{C2QuarantinedClaim, HistoricalEvidenceDisposition, ProductionSubject};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreservationCheckedProofRun {
    schema_version: u32,
    evidence_identity: String,
    known_baseline_inventory_sha256: String,
    preservation_ledger_sha256: String,
    current_inventory_sha256: String,
    current_executable_listing_sha256: String,
    current_behavior_authority_sha256: String,
    post_baseline_authority_sha256: String,
    known_baseline_cases: usize,
    current_cases: usize,
    known_baseline_assertion_predicates: usize,
    current_assertion_predicates: usize,
    disposition_counts: BTreeMap<String, usize>,
    historical: HistoricalEvidenceDisposition,
    quarantines: Vec<C2QuarantinedClaim>,
    production_subjects: Vec<ProductionSubject>,
}

#[derive(Debug, Clone)]
pub struct PreservationAuthorityDigests {
    pub current_executable_listing_sha256: String,
    pub current_behavior_authority_sha256: String,
    pub post_baseline_authority_sha256: String,
}

impl PreservationCheckedProofRun {
    pub fn assess(
        baseline: &ClassifiedInventory,
        baseline_validated: &ValidatedProofInventory,
        ledger: &ProofPreservationLedger,
        current: &ValidatedProofInventory,
        historical_non_case_aggregates: &BTreeSet<String>,
        historical: HistoricalEvidenceDisposition,
        authority: PreservationAuthorityDigests,
    ) -> Result<Self, String> {
        validate_ledger(baseline_validated, ledger).map_err(join_denials)?;
        validate_current_reachability(ledger, current, historical_non_case_aggregates)
            .map_err(join_denials)?;
        require_sha256(
            "current executable listing",
            &authority.current_executable_listing_sha256,
        )?;
        require_sha256(
            "current behavior authority",
            &authority.current_behavior_authority_sha256,
        )?;
        require_sha256(
            "post-baseline authority",
            &authority.post_baseline_authority_sha256,
        )?;
        let mut quarantines = historical.quarantines.clone();
        quarantines.extend(ledger_quarantines(ledger));
        quarantines.sort();
        ensure_unique_quarantines(&quarantines)?;
        let mut report = Self {
            schema_version: 1,
            evidence_identity: String::new(),
            known_baseline_inventory_sha256: sha256_serialized(baseline)?,
            preservation_ledger_sha256: sha256_serialized(ledger)?,
            current_inventory_sha256: sha256_serialized(current.inventory())?,
            current_executable_listing_sha256: authority.current_executable_listing_sha256,
            current_behavior_authority_sha256: authority.current_behavior_authority_sha256,
            post_baseline_authority_sha256: authority.post_baseline_authority_sha256,
            known_baseline_cases: ledger.rows.len(),
            current_cases: current.inventory().proofs.len(),
            known_baseline_assertion_predicates: ledger
                .rows
                .iter()
                .map(|row| row.assertion_predicates.len())
                .sum(),
            current_assertion_predicates: current
                .inventory()
                .proofs
                .iter()
                .map(|proof| proof.case.assertion_predicates.len())
                .sum(),
            disposition_counts: disposition_counts(ledger),
            historical,
            quarantines,
            production_subjects: production_subjects(current),
        };
        report.evidence_identity = report.expected_identity()?;
        report.validate()?;
        Ok(report)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err(format!(
                "unsupported preservation checked run schema: {}",
                self.schema_version
            ));
        }
        for (name, digest) in [
            ("baseline inventory", &self.known_baseline_inventory_sha256),
            ("preservation ledger", &self.preservation_ledger_sha256),
            ("current inventory", &self.current_inventory_sha256),
            (
                "current executable listing",
                &self.current_executable_listing_sha256,
            ),
            (
                "current behavior authority",
                &self.current_behavior_authority_sha256,
            ),
            (
                "post-baseline authority",
                &self.post_baseline_authority_sha256,
            ),
            ("historical policy", &self.historical.policy_sha256),
        ] {
            require_sha256(name, digest)?;
        }
        self.historical.validate()?;
        if self.known_baseline_cases == 0
            || self.current_cases == 0
            || self.known_baseline_assertion_predicates == 0
            || self.current_assertion_predicates == 0
            || self.disposition_counts.values().sum::<usize>() != self.known_baseline_cases
            || self.production_subjects.is_empty()
        {
            return Err(
                "preservation checked run has an empty or inconsistent proof surface".to_owned(),
            );
        }
        ensure_unique_quarantines(&self.quarantines)?;
        if self.expected_identity()? != self.evidence_identity {
            return Err("preservation checked run identity does not match its contents".to_owned());
        }
        Ok(())
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn quarantines(&self) -> &[C2QuarantinedClaim] {
        &self.quarantines
    }

    pub fn production_subjects(&self) -> &[ProductionSubject] {
        &self.production_subjects
    }

    pub fn known_baseline_cases(&self) -> usize {
        self.known_baseline_cases
    }

    pub fn current_cases(&self) -> usize {
        self.current_cases
    }

    fn expected_identity(&self) -> Result<String, String> {
        let mut basis = self.clone();
        basis.evidence_identity.clear();
        sha256_serialized(&basis)
    }
}

fn disposition_counts(ledger: &ProofPreservationLedger) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for row in &ledger.rows {
        *counts
            .entry(disposition_name(row.disposition).to_owned())
            .or_insert(0) += 1;
    }
    counts
}

fn disposition_name(disposition: ProofDisposition) -> &'static str {
    match disposition {
        ProofDisposition::PreserveUnchanged => "preserve-unchanged",
        ProofDisposition::PreserveAndMove => "preserve-and-move",
        ProofDisposition::PreserveAndReclassify => "preserve-and-reclassify",
        ProofDisposition::PreserveAndConsolidate => "preserve-and-consolidate",
        ProofDisposition::ReplaceWithStrongerProof => "replace-with-stronger-proof",
        ProofDisposition::DuplicateProofRemoveAfterParity => "duplicate-remove-after-parity",
        ProofDisposition::InvalidClaimQuarantine => "invalid-claim-quarantine",
    }
}

fn ledger_quarantines(ledger: &ProofPreservationLedger) -> Vec<C2QuarantinedClaim> {
    ledger
        .rows
        .iter()
        .filter_map(|row| {
            row.quarantine
                .as_ref()
                .map(|quarantine| C2QuarantinedClaim {
                    claim_identity: row.stable_case_id.clone(),
                    rationale: quarantine.rationale.clone(),
                    missing_evidence: vec!["executable-physical-reality".to_owned()],
                    c2_owner: "executable-reality-ledger".to_owned(),
                })
        })
        .collect()
}

fn ensure_unique_quarantines(quarantines: &[C2QuarantinedClaim]) -> Result<(), String> {
    let mut identities = BTreeSet::new();
    for quarantine in quarantines {
        quarantine.validate()?;
        if !identities.insert(quarantine.claim_identity.as_str()) {
            return Err(format!(
                "duplicate C2 quarantine identity: {}",
                quarantine.claim_identity
            ));
        }
    }
    Ok(())
}

fn require_sha256(name: &str, value: &str) -> Result<(), String> {
    if value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err(format!("{name} is not a SHA-256 identity"))
    }
}

fn join_denials(denials: Vec<String>) -> String {
    denials.join("\n  - ")
}
