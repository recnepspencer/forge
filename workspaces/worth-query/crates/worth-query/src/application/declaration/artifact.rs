use worth_foundational::facade::{
    compare_canonical_basis, derive_canonical_digest, prepare_canonical_basis_bundle,
    prepare_canonical_basis_sequence, prepare_canonical_comparison, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisValue, CanonicalBundleReadyArtifact, CanonicalComparisonOutcome,
    CanonicalDerivedDigest, CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalDigestFrontDoor, CanonicalEquivalenceBasis, CanonicalizationRuleVersion,
};
use worth_proof::TransitionOutcome;

use crate::application::{
    WorthQueryDeclarationFamilyTaxonomy, WorthQueryDeclarationPrimaryAuthorityFamily,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
    WorthQueryGroupedDeclarationPosture, WorthQueryInstalledDomainDeclarationContext,
    WorthQuerySignalCompatibilityPosture,
};

use super::async_resource::WorthQueryAsyncDeclarationClause;
use super::comparison::WorthQueryCanonicalDeclarationComparison;
use super::input::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue, WorthQueryDeclarationInput,
};
use super::raw_input::WorthQueryRawDeclarationInput;
use super::temporal::WorthQueryTemporalDeclarationClause;
use super::version::WorthQueryDeclarationCanonicalizationVersion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryDeclarationCanonicalizationError {
    EmptyDeclarationEntries {
        declaration_family_key: &'static str,
    },
    BasisConstructionDenied(String),
    DigestDerivationDenied(CanonicalDigestDerivationDenial),
    ComparisonPreparationFailed,
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryCanonicalDeclarationArtifact<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    handle_identity_digest: String,
    declaration_family_key: &'static str,
    declaration_taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    async_resource_clauses: Vec<WorthQueryAsyncDeclarationClause>,
    temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
    declaration_entry_loci: Vec<String>,
    canonical_entries: Vec<CanonicalBasisEntry>,
    canonical_basis_bundle: CanonicalBundleReadyArtifact,
    declaration_digest: CanonicalDerivedDigest,
    declaration_meaning_digest: CanonicalDerivedDigest,
    version: WorthQueryDeclarationCanonicalizationVersion,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D, I> Clone for WorthQueryCanonicalDeclarationArtifact<D, I>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    fn clone(&self) -> Self {
        Self {
            handle_identity_digest: self.handle_identity_digest.clone(),
            declaration_family_key: self.declaration_family_key,
            declaration_taxonomy: self.declaration_taxonomy,
            async_resource_clauses: self.async_resource_clauses.clone(),
            temporal_clauses: self.temporal_clauses.clone(),
            declaration_entry_loci: self.declaration_entry_loci.clone(),
            canonical_entries: self.canonical_entries.clone(),
            canonical_basis_bundle: self.canonical_basis_bundle.clone(),
            declaration_digest: self.declaration_digest.clone(),
            declaration_meaning_digest: self.declaration_meaning_digest.clone(),
            version: self.version.clone(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<D, I> WorthQueryCanonicalDeclarationArtifact<D, I>
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    fn new(
        handle_identity_digest: String,
        declaration_family_key: &'static str,
        declaration_taxonomy: WorthQueryDeclarationFamilyTaxonomy,
        async_resource_clauses: Vec<WorthQueryAsyncDeclarationClause>,
        temporal_clauses: Vec<WorthQueryTemporalDeclarationClause>,
        declaration_entry_loci: Vec<String>,
        canonical_entries: Vec<CanonicalBasisEntry>,
        canonical_basis_bundle: CanonicalBundleReadyArtifact,
        declaration_digest: CanonicalDerivedDigest,
        declaration_meaning_digest: CanonicalDerivedDigest,
        version: WorthQueryDeclarationCanonicalizationVersion,
    ) -> Self {
        Self {
            handle_identity_digest,
            declaration_family_key,
            declaration_taxonomy,
            async_resource_clauses,
            temporal_clauses,
            declaration_entry_loci,
            canonical_entries,
            canonical_basis_bundle,
            declaration_digest,
            declaration_meaning_digest,
            version,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn handle_identity_digest(&self) -> &str {
        &self.handle_identity_digest
    }

    pub fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub fn declaration_taxonomy(&self) -> WorthQueryDeclarationFamilyTaxonomy {
        self.declaration_taxonomy
    }

    pub fn async_resource_clauses(&self) -> &[WorthQueryAsyncDeclarationClause] {
        &self.async_resource_clauses
    }

    pub fn temporal_clauses(&self) -> &[WorthQueryTemporalDeclarationClause] {
        &self.temporal_clauses
    }

    pub fn declaration_primary_authority_family(
        &self,
    ) -> WorthQueryDeclarationPrimaryAuthorityFamily {
        self.declaration_taxonomy.primary_authority_family()
    }

    pub fn declaration_signal_compatibility(&self) -> WorthQuerySignalCompatibilityPosture {
        self.declaration_taxonomy.signal_compatibility()
    }

    pub fn declaration_grouped_posture(&self) -> WorthQueryGroupedDeclarationPosture {
        self.declaration_taxonomy.grouped_posture()
    }

    pub fn canonical_basis_bundle(&self) -> &CanonicalBundleReadyArtifact {
        &self.canonical_basis_bundle
    }

    pub fn declaration_entry_loci(&self) -> &[String] {
        &self.declaration_entry_loci
    }

    pub fn declaration_digest(&self) -> &CanonicalDerivedDigest {
        &self.declaration_digest
    }

    pub fn declaration_meaning_digest(&self) -> &CanonicalDerivedDigest {
        &self.declaration_meaning_digest
    }
    pub fn version(&self) -> &WorthQueryDeclarationCanonicalizationVersion {
        &self.version
    }

    pub fn canonicalization_version(&self) -> &WorthQueryDeclarationCanonicalizationVersion {
        &self.version
    }

    pub fn compare_under<J>(
        &self,
        right: &WorthQueryCanonicalDeclarationArtifact<D, J>,
        basis: CanonicalEquivalenceBasis,
    ) -> Result<WorthQueryCanonicalDeclarationComparison, WorthQueryDeclarationCanonicalizationError>
    where
        J: WorthQueryDeclarationInput<D>,
    {
        let left_basis =
            canonical_basis_from_entries(&self.canonical_entries, self.version.foundational());
        let right_basis =
            canonical_basis_from_entries(&right.canonical_entries, right.version.foundational());
        let ready = match prepare_canonical_comparison(basis, left_basis, right_basis) {
            TransitionOutcome::Success(ready) => ready,
            _ => {
                return Err(WorthQueryDeclarationCanonicalizationError::ComparisonPreparationFailed)
            }
        };
        let outcome: CanonicalComparisonOutcome = compare_canonical_basis(&ready);
        Ok(WorthQueryCanonicalDeclarationComparison::new(outcome))
    }
}

pub(crate) fn worth_query_canonical_declaration<D, C, I>(
    handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
    input: I,
    version: WorthQueryDeclarationCanonicalizationVersion,
) -> Result<WorthQueryCanonicalDeclarationArtifact<D, I>, WorthQueryDeclarationCanonicalizationError>
where
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
{
    let raw = WorthQueryRawDeclarationInput::new(input);
    if raw.canonical_entries().is_empty() {
        return Err(
            WorthQueryDeclarationCanonicalizationError::EmptyDeclarationEntries {
                declaration_family_key: raw.declaration_family_key(),
            },
        );
    }

    let (canonical_entries, declaration_meaning_entries) = declaration_entries(handle, &raw);
    let declaration_entry_loci = raw
        .canonical_entries()
        .iter()
        .map(|entry| entry.locus().to_string())
        .collect::<Vec<_>>();
    let canonical_basis_bundle =
        canonical_basis_bundle_from_entries(&canonical_entries, version.foundational());
    let declaration_digest = derive_declaration_digest(canonical_basis_bundle.clone())?;
    let declaration_meaning_digest = derive_declaration_digest(
        canonical_basis_bundle_from_entries(&declaration_meaning_entries, version.foundational()),
    )?;

    Ok(WorthQueryCanonicalDeclarationArtifact::new(
        handle.handle_identity_digest().to_string(),
        raw.declaration_family_key(),
        raw.declaration_taxonomy(),
        raw.async_resource_clauses().to_vec(),
        raw.temporal_clauses().to_vec(),
        declaration_entry_loci,
        canonical_entries,
        canonical_basis_bundle,
        declaration_digest,
        declaration_meaning_digest,
        version,
    ))
}

fn derive_declaration_digest(
    basis_bundle: CanonicalBundleReadyArtifact,
) -> Result<CanonicalDerivedDigest, WorthQueryDeclarationCanonicalizationError> {
    let algorithm = CanonicalDigestAlgorithmId::sha256();
    match CanonicalDigestFrontDoor.for_bundle(basis_bundle, algorithm) {
        TransitionOutcome::Success(ready) => Ok(derive_canonical_digest(ready)),
        TransitionOutcome::Denied(denial) => {
            Err(WorthQueryDeclarationCanonicalizationError::DigestDerivationDenied(denial))
        }
        _ => Err(
            WorthQueryDeclarationCanonicalizationError::DigestDerivationDenied(
                CanonicalDigestDerivationDenial::InputShapeMismatch,
            ),
        ),
    }
}

fn canonical_basis_from_entries(
    entries: &[CanonicalBasisEntry],
    version: &CanonicalizationRuleVersion,
) -> CanonicalBasisReadyArtifact {
    match prepare_canonical_basis_sequence(
        version.clone(),
        CanonicalBasisDomain::Future("worth_query.declaration"),
        entries.iter().cloned(),
    ) {
        TransitionOutcome::Success(ready) => ready,
        _ => panic!("canonical declaration basis should rebuild cleanly"),
    }
}

fn canonical_basis_bundle_from_entries(
    entries: &[CanonicalBasisEntry],
    version: &CanonicalizationRuleVersion,
) -> CanonicalBundleReadyArtifact {
    let basis = canonical_basis_from_entries(entries, version);
    match prepare_canonical_basis_bundle(version.clone(), [basis]) {
        TransitionOutcome::Success(bundle) => bundle,
        _ => panic!("canonical declaration bundle should rebuild cleanly"),
    }
}

fn declaration_entries<D, I>(
    handle: &WorthQueryInstalledDomainDeclarationContext<
        D,
        impl WorthQueryDomainOperatingContext<D>,
    >,
    raw: &WorthQueryRawDeclarationInput<D, I>,
) -> (Vec<CanonicalBasisEntry>, Vec<CanonicalBasisEntry>)
where
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
{
    let domain = CanonicalBasisDomain::Future("worth_query.declaration");
    let mut meaning_entries = vec![
        text_entry(
            domain,
            "declaration.domain_key",
            CanonicalBasisEntryKind::Header,
            handle.domain_key(),
        ),
        text_entry(
            domain,
            "declaration.family_key",
            CanonicalBasisEntryKind::Shape,
            raw.declaration_family_key(),
        ),
        text_entry(
            domain,
            "declaration.family.primary_authority",
            CanonicalBasisEntryKind::Shape,
            raw.declaration_taxonomy()
                .primary_authority_family()
                .as_str(),
        ),
        text_entry(
            domain,
            "declaration.family.signal_compatibility",
            CanonicalBasisEntryKind::Shape,
            raw.declaration_taxonomy().signal_compatibility().as_str(),
        ),
        text_entry(
            domain,
            "declaration.family.grouped_posture",
            CanonicalBasisEntryKind::Shape,
            raw.declaration_taxonomy().grouped_posture().as_str(),
        ),
    ];
    meaning_entries.extend(
        raw.canonical_entries()
            .iter()
            .map(|entry| convert_entry(domain, entry)),
    );
    let mut authority_entries = meaning_entries.clone();
    authority_entries.insert(
        1,
        text_entry(
            domain,
            "declaration.handle_identity_digest",
            CanonicalBasisEntryKind::Identity,
            handle.handle_identity_digest(),
        ),
    );
    (authority_entries, meaning_entries)
}

fn convert_entry(
    domain: CanonicalBasisDomain,
    entry: &WorthQueryDeclarationCanonicalEntry,
) -> CanonicalBasisEntry {
    let kind = match entry.kind() {
        WorthQueryDeclarationCanonicalEntryKind::Header => CanonicalBasisEntryKind::Header,
        WorthQueryDeclarationCanonicalEntryKind::Shape => CanonicalBasisEntryKind::Shape,
        WorthQueryDeclarationCanonicalEntryKind::Value => CanonicalBasisEntryKind::Value,
        WorthQueryDeclarationCanonicalEntryKind::Field => CanonicalBasisEntryKind::Field,
        WorthQueryDeclarationCanonicalEntryKind::Identity => CanonicalBasisEntryKind::Identity,
    };
    let value = match entry.value() {
        WorthQueryDeclarationCanonicalValue::Null => CanonicalBasisValue::Null,
        WorthQueryDeclarationCanonicalValue::Bool(value) => CanonicalBasisValue::Bool(*value),
        WorthQueryDeclarationCanonicalValue::SignedInteger(value) => {
            CanonicalBasisValue::SignedInteger {
                width: worth_foundational::facade::CanonicalIntegerWidth::Bits64,
                value: *value,
            }
        }
        WorthQueryDeclarationCanonicalValue::UnsignedInteger(value) => {
            CanonicalBasisValue::UnsignedInteger {
                width: worth_foundational::facade::CanonicalIntegerWidth::Bits64,
                value: *value,
            }
        }
        WorthQueryDeclarationCanonicalValue::ExactText(value) => {
            CanonicalBasisValue::ExactText(value.clone().into())
        }
        WorthQueryDeclarationCanonicalValue::DecimalText(value) => {
            CanonicalBasisValue::DecimalText(value.clone().into())
        }
    };
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(entry.locus().to_string().into()),
        kind,
        value,
    )
}

fn text_entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    kind: CanonicalBasisEntryKind,
    value: &str,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        kind,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}
