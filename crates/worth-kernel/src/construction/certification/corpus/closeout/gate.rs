use crate::construction::digest::digest_owned_parts;

use super::required_inventory::PrimitiveConstructionCorpusRequiredScenarioInventory;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionCorpusCloseoutGateStatus {
    required_rows_present: bool,
    supporting_reports_verified: bool,
    gate_verified: bool,
    gate_digest: String,
}

impl PrimitiveConstructionCorpusCloseoutGateStatus {
    pub(crate) fn new(
        requirements: &PrimitiveConstructionCorpusRequiredScenarioInventory,
        required_rows_present: bool,
        supporting_reports_verified: bool,
        supporting_digests: impl IntoIterator<Item = String>,
    ) -> Self {
        let gate_verified = required_rows_present && supporting_reports_verified;
        let gate_digest = digest_owned_parts(
            &std::iter::once(requirements.inventory_digest().to_string())
                .chain(supporting_digests)
                .chain([
                    required_rows_present.to_string(),
                    supporting_reports_verified.to_string(),
                    gate_verified.to_string(),
                ])
                .collect::<Vec<_>>(),
        );
        Self {
            required_rows_present,
            supporting_reports_verified,
            gate_verified,
            gate_digest,
        }
    }

    pub(crate) fn required_rows_present(&self) -> bool {
        self.required_rows_present
    }

    pub(crate) fn gate_verified(&self) -> bool {
        self.gate_verified
    }

    pub(crate) fn gate_digest(&self) -> &str {
        &self.gate_digest
    }
}
