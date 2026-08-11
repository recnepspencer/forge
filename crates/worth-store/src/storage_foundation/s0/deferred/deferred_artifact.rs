use super::super::artifacts::{
    S0ArtifactEnvelopeMetadata, S0ArtifactValidationCostSurface, S0_ARTIFACT_SCHEMA_VERSION,
};
use super::super::evidence::{S0ArtifactKind, S0StableDigest};
use super::deferred_guarantee_map::DeferredPhysicalGuaranteeMap;
use super::deferred_raw_schema::RawDeferredPhysicalGuaranteeMap;
use super::deferred_validation::S0DeferredGuaranteeParseRejection;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct S0ValidatedDeferredPhysicalGuaranteeMapArtifact {
    map: DeferredPhysicalGuaranteeMap,
    validation_cost: S0ArtifactValidationCostSurface,
}

impl S0ValidatedDeferredPhysicalGuaranteeMapArtifact {
    pub fn map(&self) -> &DeferredPhysicalGuaranteeMap {
        &self.map
    }

    pub fn validation_cost(&self) -> &S0ArtifactValidationCostSurface {
        &self.validation_cost
    }
}

impl DeferredPhysicalGuaranteeMap {
    pub fn envelope(&self) -> &S0ArtifactEnvelopeMetadata {
        &self.envelope
    }

    pub fn rows(&self) -> &[super::deferred_guarantee_row::DeferredPhysicalGuaranteeRow] {
        &self.rows
    }

    pub fn to_canonical_json_bytes(&self) -> Result<Vec<u8>, S0DeferredGuaranteeParseRejection> {
        serde_json::to_vec_pretty(self)
            .map_err(|_| S0DeferredGuaranteeParseRejection::SerializationFailed)
    }

    pub fn validate_canonical_json_bytes(
        bytes: &[u8],
    ) -> Result<S0ValidatedDeferredPhysicalGuaranteeMapArtifact, S0DeferredGuaranteeParseRejection>
    {
        let raw = serde_json::from_slice::<RawDeferredPhysicalGuaranteeMap>(bytes)
            .map_err(|_| S0DeferredGuaranteeParseRejection::NonParseable)?;
        if raw.envelope.schema_version != S0_ARTIFACT_SCHEMA_VERSION {
            return Err(S0DeferredGuaranteeParseRejection::SchemaVersionMismatch);
        }
        if raw.envelope.artifact_kind != S0ArtifactKind::DeferredPhysicalGuaranteeMap {
            return Err(S0DeferredGuaranteeParseRejection::ArtifactKindMismatch);
        }
        let roadmap_parent_digest = S0StableDigest::new(raw.envelope.roadmap_parent_digest)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDigest)?;
        let expected_digest = S0StableDigest::new(raw.envelope.deterministic_digest)
            .map_err(|_| S0DeferredGuaranteeParseRejection::InvalidDigest)?;
        let rows = raw
            .rows
            .into_iter()
            .map(super::deferred_raw_schema::RawDeferredPhysicalGuaranteeRow::into_validated)
            .collect::<Result<Vec<_>, _>>()?;
        let map = Self::new(
            raw.envelope.source_revision,
            roadmap_parent_digest,
            raw.envelope.generated_by,
            raw.envelope.nondeterministic_metadata.into_validated()?,
            rows,
        )?;
        let row_count = map.rows().len() as u64;
        if map.envelope().deterministic_digest() != &expected_digest {
            return Err(S0DeferredGuaranteeParseRejection::DeterministicDigestMismatch);
        }
        let canonicalized_row_byte_count = serde_json::to_vec(map.rows())
            .map_err(|_| S0DeferredGuaranteeParseRejection::SerializationFailed)?
            .len() as u64;
        Ok(S0ValidatedDeferredPhysicalGuaranteeMapArtifact {
            map,
            validation_cost: S0ArtifactValidationCostSurface::new(
                bytes.len() as u64,
                row_count,
                canonicalized_row_byte_count,
                row_count,
            ),
        })
    }
}
