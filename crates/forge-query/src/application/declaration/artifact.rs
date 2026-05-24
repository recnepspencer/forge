use forge_foundational::facade::{
    compare_canonical_basis, derive_canonical_digest, prepare_canonical_basis_bundle,
    prepare_canonical_basis_sequence, prepare_canonical_comparison, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact,
    CanonicalBasisValue, CanonicalBundleReadyArtifact, CanonicalComparisonOutcome,
    CanonicalDerivedDigest, CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial,
    CanonicalDigestFrontDoor, CanonicalEquivalenceBasis, CanonicalizationRuleVersion,
};
use forge_proof::TransitionOutcome;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationFamilyTaxonomy,
    ForgeQueryDeclarationPrimaryAuthorityFamily, ForgeQueryDomainEntryMarker,
    ForgeQueryDomainOperatingContext, ForgeQueryGroupedDeclarationPosture,
    ForgeQuerySignalCompatibilityPosture,
};

use super::comparison::ForgeQueryCanonicalDeclarationComparison;
use super::input::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput,
};
use super::raw_input::ForgeQueryRawDeclarationInput;
use super::version::ForgeQueryDeclarationCanonicalizationVersion;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationCanonicalizationError {
    EmptyDeclarationEntries {
        declaration_family_key: &'static str,
    },
    BasisConstructionDenied(String),
    DigestDerivationDenied(CanonicalDigestDerivationDenial),
    ComparisonPreparationFailed,
}

pub struct ForgeQueryCanonicalDeclarationArtifact<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    handle_identity_digest: String,
    declaration_family_key: &'static str,
    declaration_taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    canonical_entries: Vec<CanonicalBasisEntry>,
    canonical_basis_bundle: CanonicalBundleReadyArtifact,
    declaration_digest: CanonicalDerivedDigest,
    version: ForgeQueryDeclarationCanonicalizationVersion,
    _marker: std::marker::PhantomData<(D, I)>,
}

impl<D, I> ForgeQueryCanonicalDeclarationArtifact<D, I>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    fn new(
        handle_identity_digest: String,
        declaration_family_key: &'static str,
        declaration_taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
        canonical_entries: Vec<CanonicalBasisEntry>,
        canonical_basis_bundle: CanonicalBundleReadyArtifact,
        declaration_digest: CanonicalDerivedDigest,
        version: ForgeQueryDeclarationCanonicalizationVersion,
    ) -> Self {
        Self {
            handle_identity_digest,
            declaration_family_key,
            declaration_taxonomy,
            canonical_entries,
            canonical_basis_bundle,
            declaration_digest,
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

    pub fn declaration_taxonomy(&self) -> ForgeQueryDeclarationFamilyTaxonomy {
        self.declaration_taxonomy
    }

    pub fn declaration_primary_authority_family(
        &self,
    ) -> ForgeQueryDeclarationPrimaryAuthorityFamily {
        self.declaration_taxonomy.primary_authority_family()
    }

    pub fn declaration_signal_compatibility(&self) -> ForgeQuerySignalCompatibilityPosture {
        self.declaration_taxonomy.signal_compatibility()
    }

    pub fn declaration_grouped_posture(&self) -> ForgeQueryGroupedDeclarationPosture {
        self.declaration_taxonomy.grouped_posture()
    }

    pub fn canonical_basis_bundle(&self) -> &CanonicalBundleReadyArtifact {
        &self.canonical_basis_bundle
    }

    pub fn declaration_digest(&self) -> &CanonicalDerivedDigest {
        &self.declaration_digest
    }

    pub fn version(&self) -> &ForgeQueryDeclarationCanonicalizationVersion {
        &self.version
    }

    pub fn canonicalization_version(&self) -> &ForgeQueryDeclarationCanonicalizationVersion {
        &self.version
    }

    pub fn compare_under<J>(
        &self,
        right: &ForgeQueryCanonicalDeclarationArtifact<D, J>,
        basis: CanonicalEquivalenceBasis,
    ) -> Result<ForgeQueryCanonicalDeclarationComparison, ForgeQueryDeclarationCanonicalizationError>
    where
        J: ForgeQueryDeclarationInput<D>,
    {
        let left_basis =
            canonical_basis_from_entries(&self.canonical_entries, self.version.foundational());
        let right_basis =
            canonical_basis_from_entries(&right.canonical_entries, right.version.foundational());
        let ready = match prepare_canonical_comparison(basis, left_basis, right_basis) {
            TransitionOutcome::Success(ready) => ready,
            _ => {
                return Err(ForgeQueryDeclarationCanonicalizationError::ComparisonPreparationFailed)
            }
        };
        let outcome: CanonicalComparisonOutcome = compare_canonical_basis(&ready);
        Ok(ForgeQueryCanonicalDeclarationComparison::new(outcome))
    }
}

pub(crate) fn forge_query_canonical_declaration<D, C, I>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    input: I,
    version: ForgeQueryDeclarationCanonicalizationVersion,
) -> Result<ForgeQueryCanonicalDeclarationArtifact<D, I>, ForgeQueryDeclarationCanonicalizationError>
where
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
{
    let raw = ForgeQueryRawDeclarationInput::new(input);
    if raw.canonical_entries().is_empty() {
        return Err(
            ForgeQueryDeclarationCanonicalizationError::EmptyDeclarationEntries {
                declaration_family_key: raw.declaration_family_key(),
            },
        );
    }

    let canonical_entries = declaration_entries(handle, &raw);
    let canonical_basis_bundle =
        canonical_basis_bundle_from_entries(&canonical_entries, version.foundational());

    let algorithm = CanonicalDigestAlgorithmId::test_stable_fixture();
    let digest_ready = match CanonicalDigestFrontDoor
        .for_bundle(canonical_basis_bundle.clone(), algorithm)
    {
        TransitionOutcome::Success(ready) => ready,
        TransitionOutcome::Denied(denial) => {
            return Err(ForgeQueryDeclarationCanonicalizationError::DigestDerivationDenied(denial))
        }
        _ => {
            return Err(
                ForgeQueryDeclarationCanonicalizationError::DigestDerivationDenied(
                    CanonicalDigestDerivationDenial::InputShapeMismatch,
                ),
            )
        }
    };
    let declaration_digest = derive_canonical_digest(digest_ready);

    Ok(ForgeQueryCanonicalDeclarationArtifact::new(
        handle.handle_identity_digest().to_string(),
        raw.declaration_family_key(),
        raw.declaration_taxonomy(),
        canonical_entries,
        canonical_basis_bundle,
        declaration_digest,
        version,
    ))
}

fn canonical_basis_from_entries(
    entries: &[CanonicalBasisEntry],
    version: &CanonicalizationRuleVersion,
) -> CanonicalBasisReadyArtifact {
    match prepare_canonical_basis_sequence(
        version.clone(),
        CanonicalBasisDomain::Future("forge_query.declaration"),
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
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, impl ForgeQueryDomainOperatingContext<D>>,
    raw: &ForgeQueryRawDeclarationInput<D, I>,
) -> Vec<CanonicalBasisEntry>
where
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
{
    let domain = CanonicalBasisDomain::Future("forge_query.declaration");
    let mut entries = vec![
        text_entry(
            domain,
            "declaration.domain_key",
            CanonicalBasisEntryKind::Header,
            handle.domain_key(),
        ),
        text_entry(
            domain,
            "declaration.handle_identity_digest",
            CanonicalBasisEntryKind::Identity,
            handle.handle_identity_digest(),
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
    entries.extend(
        raw.canonical_entries()
            .iter()
            .map(|entry| convert_entry(domain, entry)),
    );
    entries
}

fn convert_entry(
    domain: CanonicalBasisDomain,
    entry: &ForgeQueryDeclarationCanonicalEntry,
) -> CanonicalBasisEntry {
    let kind = match entry.kind() {
        ForgeQueryDeclarationCanonicalEntryKind::Header => CanonicalBasisEntryKind::Header,
        ForgeQueryDeclarationCanonicalEntryKind::Shape => CanonicalBasisEntryKind::Shape,
        ForgeQueryDeclarationCanonicalEntryKind::Value => CanonicalBasisEntryKind::Value,
        ForgeQueryDeclarationCanonicalEntryKind::Field => CanonicalBasisEntryKind::Field,
        ForgeQueryDeclarationCanonicalEntryKind::Identity => CanonicalBasisEntryKind::Identity,
    };
    let value = match entry.value() {
        ForgeQueryDeclarationCanonicalValue::Null => CanonicalBasisValue::Null,
        ForgeQueryDeclarationCanonicalValue::Bool(value) => CanonicalBasisValue::Bool(*value),
        ForgeQueryDeclarationCanonicalValue::SignedInteger(value) => {
            CanonicalBasisValue::SignedInteger {
                width: forge_foundational::facade::CanonicalIntegerWidth::Bits64,
                value: *value,
            }
        }
        ForgeQueryDeclarationCanonicalValue::UnsignedInteger(value) => {
            CanonicalBasisValue::UnsignedInteger {
                width: forge_foundational::facade::CanonicalIntegerWidth::Bits64,
                value: *value,
            }
        }
        ForgeQueryDeclarationCanonicalValue::ExactText(value) => {
            CanonicalBasisValue::ExactText(value.clone().into())
        }
        ForgeQueryDeclarationCanonicalValue::DecimalText(value) => {
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
