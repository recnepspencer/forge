use forge_foundational::facade::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisReadyArtifact, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

mod grammar;
mod value;

use grammar::{NativeDomain, NativeEntry};

#[derive(Serialize, Deserialize)]
struct NativeCanonicalBasis {
    version: String,
    domain: NativeDomain,
    entries: Vec<NativeEntry>,
}

pub(crate) fn serialize<S>(
    basis: &CanonicalBasisReadyArtifact,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    NativeCanonicalBasis::try_from(basis)
        .map_err(serde::ser::Error::custom)?
        .serialize(serializer)
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<CanonicalBasisReadyArtifact, D::Error>
where
    D: Deserializer<'de>,
{
    let basis = NativeCanonicalBasis::deserialize(deserializer)?;
    let version = CanonicalizationRuleVersion::new(basis.version)
        .ok_or_else(|| serde::de::Error::custom("invalid diagnostic canonical basis version"))?;
    let domain = CanonicalBasisDomain::try_from(basis.domain).map_err(serde::de::Error::custom)?;
    let entries = basis
        .entries
        .into_iter()
        .map(CanonicalBasisEntry::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::de::Error::custom)?;

    match prepare_canonical_basis_sequence(version, domain, entries) {
        TransitionOutcome::Success(ready) => Ok(ready),
        other => Err(serde::de::Error::custom(format!(
            "diagnostic canonical basis was not ready: {other:?}"
        ))),
    }
}

impl TryFrom<&CanonicalBasisReadyArtifact> for NativeCanonicalBasis {
    type Error = String;

    fn try_from(basis: &CanonicalBasisReadyArtifact) -> Result<Self, Self::Error> {
        let canonical_basis_terms = basis.payload();
        Ok(Self {
            version: canonical_basis_terms.version().as_str().to_string(),
            domain: canonical_basis_terms.domain().try_into()?,
            entries: canonical_basis_terms
                .entries()
                .iter()
                .map(NativeEntry::try_from)
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
