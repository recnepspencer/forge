use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

use super::identity::FoundationalBranchId;

/// The stable, descriptive encoding contract for an owner's immutable target.
///
/// The owner supplies the domain tag, schema version, and canonical bytes. The
/// value deliberately carries no runtime identity, liveness, or authority.
pub trait FoundationalBranchTargetBasis: Clone + Eq + Serialize {
    fn canonical_encoding(&self) -> FoundationalBranchTargetEncoding;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FoundationalBranchTargetEncodingConstructionDenial {
    EmptyDomain,
    ZeroSchemaVersion,
}

/// A versioned, domain-separated canonical representation of an owner target.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct FoundationalBranchTargetEncoding {
    domain: String,
    schema_version: u16,
    bytes: Vec<u8>,
}

impl FoundationalBranchTargetEncoding {
    pub fn new(
        domain: impl Into<String>,
        schema_version: u16,
        bytes: impl Into<Vec<u8>>,
    ) -> Result<Self, FoundationalBranchTargetEncodingConstructionDenial> {
        let domain = domain.into();
        if domain.trim().is_empty() {
            return Err(FoundationalBranchTargetEncodingConstructionDenial::EmptyDomain);
        }
        if schema_version == 0 {
            return Err(FoundationalBranchTargetEncodingConstructionDenial::ZeroSchemaVersion);
        }

        Ok(Self {
            domain,
            schema_version,
            bytes: bytes.into(),
        })
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl<'de> Deserialize<'de> for FoundationalBranchTargetEncoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TargetEncodingFields {
            domain: String,
            schema_version: u16,
            bytes: Vec<u8>,
        }

        let fields = TargetEncodingFields::deserialize(deserializer)?;
        Self::new(fields.domain, fields.schema_version, fields.bytes)
            .map_err(|denial| D::Error::custom(format!("invalid target encoding: {denial:?}")))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub enum FoundationalBranchTarget<T: FoundationalBranchTargetBasis> {
    Empty,
    Basis(T),
}

impl<T: FoundationalBranchTargetBasis> FoundationalBranchTarget<T> {
    pub const fn empty() -> Self {
        Self::Empty
    }

    pub fn basis(value: T) -> Self {
        Self::Basis(value)
    }

    pub fn as_basis(&self) -> Option<&T> {
        match self {
            Self::Empty => None,
            Self::Basis(value) => Some(value),
        }
    }

    pub const fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct FoundationalBranchReferenceGeneration(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FoundationalBranchReferenceGenerationAdvanceDenial {
    Overflow,
}

impl FoundationalBranchReferenceGeneration {
    pub const fn initial() -> Self {
        Self(0)
    }

    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    pub fn checked_advance(
        self,
    ) -> Result<Self, FoundationalBranchReferenceGenerationAdvanceDenial> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(FoundationalBranchReferenceGenerationAdvanceDenial::Overflow)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum FoundationalBranchReferenceMismatchAxis {
    BranchIdentity,
    TargetBasis,
    ReferenceGeneration,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FoundationalBranchReferenceObservation<T: FoundationalBranchTargetBasis> {
    branch_id: FoundationalBranchId,
    target: FoundationalBranchTarget<T>,
    generation: FoundationalBranchReferenceGeneration,
}

impl<T: FoundationalBranchTargetBasis> FoundationalBranchReferenceObservation<T> {
    pub fn new(
        branch_id: FoundationalBranchId,
        target: FoundationalBranchTarget<T>,
        generation: FoundationalBranchReferenceGeneration,
    ) -> Self {
        Self {
            branch_id,
            target,
            generation,
        }
    }

    pub fn branch_id(&self) -> &FoundationalBranchId {
        &self.branch_id
    }

    pub fn target(&self) -> &FoundationalBranchTarget<T> {
        &self.target
    }

    pub const fn generation(&self) -> FoundationalBranchReferenceGeneration {
        self.generation
    }

    /// Stable bytes for structural comparison, logging, and transport tests.
    /// This is descriptive encoding only; it is not an authority or digest.
    pub fn canonical_encoding(&self) -> Vec<u8> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(b"WORTH-BRANCH-REFERENCE");
        encoded.push(0);
        encoded.push(1);
        append_length_prefixed(&mut encoded, self.branch_id.as_str().as_bytes());
        match &self.target {
            FoundationalBranchTarget::Empty => encoded.push(0),
            FoundationalBranchTarget::Basis(value) => {
                encoded.push(1);
                let basis = value.canonical_encoding();
                append_length_prefixed(&mut encoded, basis.domain().as_bytes());
                encoded.extend_from_slice(&basis.schema_version().to_be_bytes());
                append_length_prefixed(&mut encoded, basis.bytes());
            }
        }
        encoded.extend_from_slice(&self.generation.get().to_be_bytes());
        encoded
    }

    pub fn compare(&self, observed: &Self) -> Result<(), FoundationalBranchReferenceMismatch<T>> {
        let mut axes = Vec::new();
        if self.branch_id != observed.branch_id {
            axes.push(FoundationalBranchReferenceMismatchAxis::BranchIdentity);
        }
        if self.target != observed.target {
            axes.push(FoundationalBranchReferenceMismatchAxis::TargetBasis);
        }
        if self.generation != observed.generation {
            axes.push(FoundationalBranchReferenceMismatchAxis::ReferenceGeneration);
        }
        if axes.is_empty() {
            Ok(())
        } else {
            Err(FoundationalBranchReferenceMismatch {
                expected: self.clone(),
                observed: observed.clone(),
                axes,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(deserialize = "T: Deserialize<'de>"))]
pub struct FoundationalBranchReferenceMismatch<T: FoundationalBranchTargetBasis> {
    expected: FoundationalBranchReferenceObservation<T>,
    observed: FoundationalBranchReferenceObservation<T>,
    axes: Vec<FoundationalBranchReferenceMismatchAxis>,
}

impl<T: FoundationalBranchTargetBasis> FoundationalBranchReferenceMismatch<T> {
    pub fn expected(&self) -> &FoundationalBranchReferenceObservation<T> {
        &self.expected
    }

    pub fn observed(&self) -> &FoundationalBranchReferenceObservation<T> {
        &self.observed
    }

    pub fn axes(&self) -> &[FoundationalBranchReferenceMismatchAxis] {
        &self.axes
    }
}

fn append_length_prefixed(encoded: &mut Vec<u8>, bytes: &[u8]) {
    let length = u64::try_from(bytes.len()).expect("usize must fit in u64");
    encoded.extend_from_slice(&length.to_be_bytes());
    encoded.extend_from_slice(bytes);
}
