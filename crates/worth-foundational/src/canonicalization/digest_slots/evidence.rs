use super::super::{
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalizationCost, CanonicalizationRuleVersion,
};
use super::algorithm::{
    CanonicalDigestAlgorithmMetadata, CanonicalDigestInputDomain, CanonicalDigestInputShape,
};
use super::material::domain_material_token;
use super::CanonicalDigestWorkEvidence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDigestDerivationInput {
    algorithm: CanonicalDigestAlgorithmMetadata,
    evidence: CanonicalDigestInputEvidence,
    material: Vec<u8>,
    work: CanonicalDigestWorkEvidence,
}

impl CanonicalDigestDerivationInput {
    pub(super) fn new(
        algorithm: CanonicalDigestAlgorithmMetadata,
        evidence: CanonicalDigestInputEvidence,
        material: Vec<u8>,
        work: CanonicalDigestWorkEvidence,
    ) -> Self {
        Self {
            algorithm,
            evidence,
            material,
            work,
        }
    }

    pub fn algorithm(&self) -> &CanonicalDigestAlgorithmMetadata {
        &self.algorithm
    }

    pub fn evidence(&self) -> &CanonicalDigestInputEvidence {
        &self.evidence
    }

    pub(super) fn material(&self) -> &[u8] {
        &self.material
    }

    pub const fn work(&self) -> CanonicalDigestWorkEvidence {
        self.work
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalDigestInputEvidence {
    SingleSequence(CanonicalDigestBasisSequence),
    DomainBundle(CanonicalDigestBasisBundle),
    ExportBundle(CanonicalDigestBasisBundle),
}

impl CanonicalDigestInputEvidence {
    pub const fn input_shape(&self) -> CanonicalDigestInputShape {
        match self {
            Self::SingleSequence(_) => CanonicalDigestInputShape::SingleSequence,
            Self::DomainBundle(_) => CanonicalDigestInputShape::DomainBundle,
            Self::ExportBundle(_) => CanonicalDigestInputShape::ExportBundle,
        }
    }

    pub fn version(&self) -> &CanonicalizationRuleVersion {
        match self {
            Self::SingleSequence(sequence) => sequence.version(),
            Self::DomainBundle(bundle) | Self::ExportBundle(bundle) => bundle.version(),
        }
    }

    pub const fn input_domain(&self) -> CanonicalDigestInputDomain {
        match self {
            Self::SingleSequence(sequence) => CanonicalDigestInputDomain::Single(sequence.domain()),
            Self::DomainBundle(_) => CanonicalDigestInputDomain::DomainBundle,
            Self::ExportBundle(_) => CanonicalDigestInputDomain::ExportBundle,
        }
    }

    pub fn entry_count(&self) -> u32 {
        match self {
            Self::SingleSequence(sequence) => sequence.cost().entry_count(),
            Self::DomainBundle(bundle) | Self::ExportBundle(bundle) => bundle
                .sequences()
                .iter()
                .map(|sequence| sequence.cost().entry_count())
                .sum(),
        }
    }

    pub fn input_id(&self) -> CanonicalDigestInputId {
        match self {
            Self::SingleSequence(sequence) => CanonicalDigestInputId::new(format!(
                "sequence:{}:{}:{}",
                domain_material_token(sequence.domain()),
                sequence.version().as_str(),
                sequence.cost().entry_count()
            )),
            Self::DomainBundle(bundle) => bundle_input_id("domain-bundle", bundle),
            Self::ExportBundle(bundle) => bundle_input_id("export-bundle", bundle),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDigestBasisBundle {
    version: CanonicalizationRuleVersion,
    sequences: Vec<CanonicalDigestBasisSequence>,
}

impl CanonicalDigestBasisBundle {
    pub(super) fn new(
        version: CanonicalizationRuleVersion,
        sequences: Vec<CanonicalDigestBasisSequence>,
    ) -> Self {
        Self { version, sequences }
    }

    pub fn version(&self) -> &CanonicalizationRuleVersion {
        &self.version
    }

    pub fn sequences(&self) -> &[CanonicalDigestBasisSequence] {
        &self.sequences
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDigestBasisSequence {
    version: CanonicalizationRuleVersion,
    domain: CanonicalBasisDomain,
    entries: Vec<CanonicalBasisEntry>,
    cost: CanonicalizationCost,
}

impl CanonicalDigestBasisSequence {
    pub(super) fn new(
        version: CanonicalizationRuleVersion,
        domain: CanonicalBasisDomain,
        entries: &[CanonicalBasisEntry],
        cost: CanonicalizationCost,
    ) -> Self {
        Self {
            version,
            domain,
            entries: entries.to_vec(),
            cost,
        }
    }

    pub fn version(&self) -> &CanonicalizationRuleVersion {
        &self.version
    }

    pub const fn domain(&self) -> CanonicalBasisDomain {
        self.domain
    }

    pub fn entries(&self) -> &[CanonicalBasisEntry] {
        &self.entries
    }

    pub const fn cost(&self) -> CanonicalizationCost {
        self.cost
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalDigestInputId(String);

impl CanonicalDigestInputId {
    pub(super) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn bundle_input_id(kind: &str, bundle: &CanonicalDigestBasisBundle) -> CanonicalDigestInputId {
    let domains = bundle
        .sequences()
        .iter()
        .map(|sequence| domain_material_token(sequence.domain()))
        .collect::<Vec<_>>()
        .join(",");
    CanonicalDigestInputId::new(format!(
        "{}:{}:{}",
        kind,
        bundle.version().as_str(),
        domains
    ))
}
