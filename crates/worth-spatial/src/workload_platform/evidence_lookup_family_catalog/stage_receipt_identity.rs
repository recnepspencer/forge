use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EvidenceLookupStageReceiptFamilyIdentity {
    value: &'static str,
    digest: String,
}

impl EvidenceLookupStageReceiptFamilyIdentity {
    pub fn boolean_common_plane() -> Self {
        Self::declared("boolean-common-plane-stage-receipts")
    }

    pub fn boolean_event_ledger() -> Self {
        Self::declared("boolean-event-ledger-stage-receipts")
    }

    pub fn boolean_operand_projection_consumption() -> Self {
        Self::declared("boolean-operand-projection-consumption-stage-receipts")
    }

    fn declared(value: &'static str) -> Self {
        let digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-stage-receipt-family:v1".to_string(),
                value.to_string(),
            ],
        );
        Self { value, digest }
    }

    pub fn as_str(&self) -> &'static str {
        self.value
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
