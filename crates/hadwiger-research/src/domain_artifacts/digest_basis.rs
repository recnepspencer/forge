use worth_foundational::facade::{
    derive_canonical_digest, prepare_canonical_basis_sequence, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue,
    CanonicalDigestAlgorithmId, CanonicalDigestFrontDoor, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

use super::core_artifact::{
    HadwigerArtifactAuthorityOwner, HadwigerArtifactCore, HadwigerArtifactDigest,
    HadwigerArtifactKind, HadwigerArtifactReference, HadwigerArtifactShapeError,
    HadwigerArtifactSourceReference,
};

const HADWIGER_ARTIFACT_DIGEST_VERSION: &str = "WORTH.hadwiger.artifact.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum HadwigerArtifactPayloadEntry {
    Text { locus: &'static str, value: String },
    Unsigned { locus: &'static str, value: u128 },
}

impl HadwigerArtifactPayloadEntry {
    pub(crate) fn text(locus: &'static str, value: impl Into<String>) -> Self {
        Self::Text {
            locus,
            value: value.into(),
        }
    }

    pub(crate) fn unsigned(locus: &'static str, value: u128) -> Self {
        Self::Unsigned { locus, value }
    }
}

pub(crate) fn artifact_core(
    artifact_kind: HadwigerArtifactKind,
    authority_owner: HadwigerArtifactAuthorityOwner,
    source_reference: HadwigerArtifactSourceReference,
    mut parent_artifacts: Vec<HadwigerArtifactReference>,
    payload_entries: Vec<HadwigerArtifactPayloadEntry>,
) -> Result<HadwigerArtifactCore, HadwigerArtifactShapeError> {
    parent_artifacts.sort_by_key(HadwigerArtifactReference::stable_token);
    let artifact_digest = artifact_digest(
        artifact_kind,
        authority_owner,
        &source_reference,
        &parent_artifacts,
        payload_entries,
    )?;
    Ok(HadwigerArtifactCore::new(
        artifact_kind,
        artifact_digest,
        authority_owner,
        source_reference,
        parent_artifacts,
    ))
}

fn artifact_digest(
    artifact_kind: HadwigerArtifactKind,
    authority_owner: HadwigerArtifactAuthorityOwner,
    source_reference: &HadwigerArtifactSourceReference,
    parent_artifacts: &[HadwigerArtifactReference],
    payload_entries: Vec<HadwigerArtifactPayloadEntry>,
) -> Result<HadwigerArtifactDigest, HadwigerArtifactShapeError> {
    let domain = CanonicalBasisDomain::Future("WORTH.hadwiger.artifact");
    let mut entries = vec![
        text_entry(
            domain,
            "digest_schema_version",
            HADWIGER_ARTIFACT_DIGEST_VERSION,
        ),
        text_entry(domain, "artifact_kind", artifact_kind.as_str()),
        text_entry(domain, "authority_owner", authority_owner.as_str()),
        text_entry(domain, "source_reference", source_reference.stable_token()),
    ];

    for (index, parent) in parent_artifacts.iter().enumerate() {
        entries.push(text_entry(
            domain,
            format!("parent_artifact.{index:04}"),
            parent.stable_token(),
        ));
    }
    for (index, payload) in payload_entries.into_iter().enumerate() {
        entries.push(payload_entry(domain, index, payload));
    }

    let version = CanonicalizationRuleVersion::new(HADWIGER_ARTIFACT_DIGEST_VERSION)
        .expect("Hadwiger artifact digest version is a stable literal");
    let sequence = match prepare_canonical_basis_sequence(version, domain, entries) {
        TransitionOutcome::Success(sequence) => sequence,
        _ => {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "canonical_basis",
            })
        }
    };
    let ready = match CanonicalDigestFrontDoor
        .for_sequence(sequence, CanonicalDigestAlgorithmId::sha256())
    {
        TransitionOutcome::Success(ready) => ready,
        _ => {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "canonical_digest",
            })
        }
    };
    Ok(HadwigerArtifactDigest::from_canonical(
        derive_canonical_digest(ready),
    ))
}

fn payload_entry(
    domain: CanonicalBasisDomain,
    index: usize,
    payload: HadwigerArtifactPayloadEntry,
) -> CanonicalBasisEntry {
    match payload {
        HadwigerArtifactPayloadEntry::Text { locus, value } => {
            text_entry(domain, format!("payload.{index:04}.{locus}"), value)
        }
        HadwigerArtifactPayloadEntry::Unsigned { locus, value } => CanonicalBasisEntry::new(
            domain,
            CanonicalBasisLocus::Named(format!("payload.{index:04}.{locus}").into()),
            CanonicalBasisEntryKind::Field,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits128,
                value,
            },
        ),
    }
}

fn text_entry(
    domain: CanonicalBasisDomain,
    locus: impl Into<String>,
    value: impl Into<String>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Field,
        CanonicalBasisValue::ExactText(value.into().into()),
    )
}
