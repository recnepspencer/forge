use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublicBridgeProjectionConsumptionEvidence {
    consumed_title: String,
    receipt_digest: String,
    fact_set_digest: String,
    source_identity: String,
    extracted_fact_count: usize,
    requested_field: String,
    digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryPublicBridgeProjectionConsumptionEvidence {
    pub fn new(
        consumed_title: impl Into<String>,
        receipt_digest: impl Into<String>,
        fact_set_digest: impl Into<String>,
        source_identity: impl Into<String>,
        extracted_fact_count: usize,
        requested_field: impl Into<String>,
    ) -> Self {
        let consumed_title = consumed_title.into();
        let receipt_digest = receipt_digest.into();
        let fact_set_digest = fact_set_digest.into();
        let source_identity = source_identity.into();
        let requested_field = requested_field.into();
        let digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("consumed_title"),
            &consumed_title,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("receipt_digest"),
            &receipt_digest,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("fact_set_digest"),
            &fact_set_digest,
        )
        .field_value(
            ForgeQueryEvidenceTag::new("source_identity"),
            &source_identity,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("extracted_fact_count"),
            extracted_fact_count,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("requested_field"),
            &requested_field,
        )
        .seal();
        Self {
            consumed_title,
            receipt_digest,
            fact_set_digest,
            source_identity,
            extracted_fact_count,
            requested_field,
            digest,
        }
    }

    pub fn consumed_title(&self) -> &str {
        &self.consumed_title
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }

    pub fn fact_set_digest(&self) -> &str {
        &self.fact_set_digest
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn extracted_fact_count(&self) -> usize {
        self.extracted_fact_count
    }

    pub fn requested_field(&self) -> &str {
        &self.requested_field
    }

    pub fn digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.digest
    }
}
