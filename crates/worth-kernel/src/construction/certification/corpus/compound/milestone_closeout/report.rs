use crate::construction::certification::corpus::closeout::{
    PrimitiveConstructionCorpusCloseoutGateStatus,
    PrimitiveConstructionCorpusRequiredScenarioInventory,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

use super::super::parity::PrimitiveConstructionCompoundParityReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionCompoundMilestoneCloseoutReport {
    parity: PrimitiveConstructionCompoundParityReport,
    requirements: PrimitiveConstructionCorpusRequiredScenarioInventory,
    gate: PrimitiveConstructionCorpusCloseoutGateStatus,
    report_digest: String,
}

impl PrimitiveConstructionCompoundMilestoneCloseoutReport {
    pub(crate) fn new(
        parity: PrimitiveConstructionCompoundParityReport,
        requirements: PrimitiveConstructionCorpusRequiredScenarioInventory,
    ) -> Self {
        let required_rows_present =
            requirements.all_present(|scenario_id| parity.siege().row_for(scenario_id));
        let gate = PrimitiveConstructionCorpusCloseoutGateStatus::from_verified_support(
            &requirements,
            required_rows_present,
            [
                parity.siege().report_digest().to_string(),
                parity.report_digest().to_string(),
            ],
        );
        let report_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ArtifactIdentity,
            &[
                parity.siege().report_digest().to_string(),
                parity.report_digest().to_string(),
                requirements.inventory_digest().to_string(),
                gate.gate_digest().to_string(),
            ],
        );
        Self {
            parity,
            requirements,
            gate,
            report_digest,
        }
    }

    pub fn siege(
        &self,
    ) -> &super::super::report::PrimitiveConstructionCompoundAdversarialSiegeReport {
        self.parity.siege()
    }

    pub fn motion(&self) -> &super::super::report::PrimitiveConstructionCompoundMotionParityReport {
        self.parity.motion()
    }

    pub fn parity(&self) -> &PrimitiveConstructionCompoundParityReport {
        &self.parity
    }

    pub fn grazing(
        &self,
    ) -> &super::super::report::PrimitiveConstructionCompoundGrazingBoundaryReport {
        self.parity.grazing()
    }

    pub fn required_scenarios(&self) -> &[String] {
        self.requirements.scenario_ids()
    }

    pub fn required_row_for(
        &self,
        scenario_id: &str,
    ) -> Option<&super::super::rows::PrimitiveConstructionCompoundRow> {
        self.requirements.row_for(scenario_id, |required| {
            self.parity.siege().row_for(required)
        })
    }

    pub fn required_rows_present(&self) -> bool {
        self.gate.required_rows_present()
    }

    pub fn closeout_gate_verified(&self) -> bool {
        self.gate.gate_verified()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
