use crate::validator_invariant_catalog::operator_certification_cutover::{
    WorthTopologyOperatorCertificationCutoverCounters,
    WorthTopologyOperatorSelectedObligationCloseoutRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyOperatorCertificationCutoverPhaseEightSeed {
    phase_seven_enforcement_seed_digest: String,
    closeout_digest: String,
    counters_digest: String,
    selected_obligation_row_digests: Vec<String>,
    seed_digest: String,
}

impl WorthTopologyOperatorCertificationCutoverPhaseEightSeed {
    pub(in crate::validator_invariant_catalog) fn from_closeout(
        phase_seven_enforcement_seed_digest: &str,
        closeout_digest: &str,
        counters: &WorthTopologyOperatorCertificationCutoverCounters,
        rows: &[WorthTopologyOperatorSelectedObligationCloseoutRow],
    ) -> Self {
        let selected_obligation_row_digests = rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .collect();
        let mut seed_parts = vec![
            "worth-topo-operator-certification-cutover-phase-eight-seed-v1".to_string(),
            format!("phase-seven-enforcement-seed:{phase_seven_enforcement_seed_digest}"),
            format!("closeout:{closeout_digest}"),
            format!("counters:{}", counters.counters_digest()),
        ];
        seed_parts.extend(
            rows.iter()
                .map(|row| format!("selected-obligation-row:{}", row.row_digest())),
        );
        Self {
            phase_seven_enforcement_seed_digest: phase_seven_enforcement_seed_digest.to_string(),
            closeout_digest: closeout_digest.to_string(),
            counters_digest: counters.counters_digest().to_string(),
            selected_obligation_row_digests,
            seed_digest: seed_parts.join("|"),
        }
    }

    pub fn phase_seven_enforcement_seed_digest(&self) -> &str {
        &self.phase_seven_enforcement_seed_digest
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn selected_obligation_row_digests(&self) -> &[String] {
        &self.selected_obligation_row_digests
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
