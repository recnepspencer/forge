use std::collections::BTreeSet;

use crate::authoring::AspectFieldKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationAspectContract {
    required: Vec<AspectFieldKey>,
    preserved: Vec<AspectFieldKey>,
    published: Vec<AspectFieldKey>,
    masked: Vec<AspectFieldKey>,
    incompatible: Vec<AspectFieldKey>,
}

impl ForgeQueryDeclarationAspectContract {
    pub fn empty() -> Self {
        Self::new([], [], [], [], [])
    }

    pub fn new(
        required: impl IntoIterator<Item = AspectFieldKey>,
        preserved: impl IntoIterator<Item = AspectFieldKey>,
        published: impl IntoIterator<Item = AspectFieldKey>,
        masked: impl IntoIterator<Item = AspectFieldKey>,
        incompatible: impl IntoIterator<Item = AspectFieldKey>,
    ) -> Self {
        Self {
            required: sorted_unique(required),
            preserved: sorted_unique(preserved),
            published: sorted_unique(published),
            masked: sorted_unique(masked),
            incompatible: sorted_unique(incompatible),
        }
    }

    #[cfg(test)]
    pub fn from_slices(
        required: &[&str],
        preserved: &[&str],
        published: &[&str],
        masked: &[&str],
        incompatible: &[&str],
    ) -> Self {
        Self::new(
            crate::application::test_declaration_aspect_keys(required),
            crate::application::test_declaration_aspect_keys(preserved),
            crate::application::test_declaration_aspect_keys(published),
            crate::application::test_declaration_aspect_keys(masked),
            crate::application::test_declaration_aspect_keys(incompatible),
        )
    }

    pub fn required(&self) -> &[AspectFieldKey] {
        &self.required
    }

    pub fn preserved(&self) -> &[AspectFieldKey] {
        &self.preserved
    }

    pub fn published(&self) -> &[AspectFieldKey] {
        &self.published
    }

    pub fn masked(&self) -> &[AspectFieldKey] {
        &self.masked
    }

    pub fn incompatible(&self) -> &[AspectFieldKey] {
        &self.incompatible
    }

    pub fn semantic_interest_keys(&self) -> Vec<AspectFieldKey> {
        sorted_unique(
            self.required
                .iter()
                .chain(self.preserved.iter())
                .chain(self.published.iter())
                .cloned(),
        )
    }

    pub fn default_coverage(&self) -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_present(self.semantic_interest_keys())
    }
}

pub(crate) fn route_scoped_declaration_aspect_contract(
    declaration_contract: &ForgeQueryDeclarationAspectContract,
) -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::new(
        declaration_contract.required().iter().cloned(),
        declaration_contract.preserved().iter().cloned(),
        [],
        declaration_contract.masked().iter().cloned(),
        declaration_contract.incompatible().iter().cloned(),
    )
}

pub(crate) fn authority_scoped_envelope_aspect_contract(
    envelope_contract: &ForgeQueryDeclarationAspectContract,
) -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::new(
        envelope_contract.required().iter().cloned(),
        envelope_contract.preserved().iter().cloned(),
        envelope_contract.published().iter().cloned(),
        envelope_contract.masked().iter().cloned(),
        envelope_contract.incompatible().iter().cloned(),
    )
}

pub(crate) fn merged_authority_aspect_contract(
    envelope_contract: &ForgeQueryDeclarationAspectContract,
    authority_contract: &ForgeQueryDeclarationAspectContract,
) -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::new(
        envelope_contract
            .required()
            .iter()
            .chain(authority_contract.required().iter())
            .cloned(),
        envelope_contract
            .preserved()
            .iter()
            .chain(authority_contract.preserved().iter())
            .cloned(),
        envelope_contract
            .published()
            .iter()
            .chain(authority_contract.published().iter())
            .cloned(),
        envelope_contract
            .masked()
            .iter()
            .chain(authority_contract.masked().iter())
            .cloned(),
        envelope_contract
            .incompatible()
            .iter()
            .chain(authority_contract.incompatible().iter())
            .cloned(),
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationAspectCoverage {
    present: Vec<AspectFieldKey>,
    masked: Vec<AspectFieldKey>,
    conflicting: Vec<AspectFieldKey>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationAspectCoverageBasis {
    DeclaredFamilyCoverage,
    ReviewedRetainedCoverage,
    SupportReportedCoverage,
    EnvelopePublishedCoverage,
    BridgeMappedCoverage,
}

impl ForgeQueryDeclarationAspectCoverageBasis {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredFamilyCoverage => "declared_family_coverage",
            Self::ReviewedRetainedCoverage => "reviewed_retained_coverage",
            Self::SupportReportedCoverage => "support_reported_coverage",
            Self::EnvelopePublishedCoverage => "envelope_published_coverage",
            Self::BridgeMappedCoverage => "bridge_mapped_coverage",
        }
    }
}

impl ForgeQueryDeclarationAspectCoverage {
    pub fn empty() -> Self {
        Self::new(
            Vec::<AspectFieldKey>::new(),
            Vec::<AspectFieldKey>::new(),
            Vec::<AspectFieldKey>::new(),
        )
    }

    pub fn new(
        present: impl IntoIterator<Item = AspectFieldKey>,
        masked: impl IntoIterator<Item = AspectFieldKey>,
        conflicting: impl IntoIterator<Item = AspectFieldKey>,
    ) -> Self {
        Self {
            present: sorted_unique(present),
            masked: sorted_unique(masked),
            conflicting: sorted_unique(conflicting),
        }
    }

    pub fn from_present(present: impl IntoIterator<Item = AspectFieldKey>) -> Self {
        Self::new(
            present,
            std::iter::empty::<AspectFieldKey>(),
            std::iter::empty::<AspectFieldKey>(),
        )
    }

    #[cfg(test)]
    pub fn from_slices(present: &[&str], masked: &[&str], conflicting: &[&str]) -> Self {
        Self::new(
            crate::application::test_declaration_aspect_keys(present),
            crate::application::test_declaration_aspect_keys(masked),
            crate::application::test_declaration_aspect_keys(conflicting),
        )
    }

    pub fn present(&self) -> &[AspectFieldKey] {
        &self.present
    }

    pub fn masked(&self) -> &[AspectFieldKey] {
        &self.masked
    }

    pub fn conflicting(&self) -> &[AspectFieldKey] {
        &self.conflicting
    }

    pub fn fit_against(
        &self,
        contract: &ForgeQueryDeclarationAspectContract,
    ) -> ForgeQueryDeclarationAspectFit {
        let masked: BTreeSet<_> = self.masked.iter().cloned().collect();
        let conflicting: BTreeSet<_> = self.conflicting.iter().cloned().collect();
        let visible_present: BTreeSet<_> = self
            .present
            .iter()
            .filter(|path| !masked.contains(*path) && !conflicting.contains(*path))
            .cloned()
            .collect();
        let required: BTreeSet<_> = contract.required.iter().cloned().collect();
        let incompatible: BTreeSet<_> = contract.incompatible.iter().cloned().collect();
        let semantic_interest: BTreeSet<_> =
            contract.semantic_interest_keys().into_iter().collect();

        if !conflicting.is_disjoint(&required) || !visible_present.is_disjoint(&incompatible) {
            return ForgeQueryDeclarationAspectFit::Conflict;
        }

        let missing_required: BTreeSet<_> = required
            .iter()
            .filter(|path| !visible_present.contains(*path) || masked.contains(*path))
            .cloned()
            .collect();
        if !missing_required.is_empty() {
            let present_required_count = required.difference(&missing_required).count();
            if present_required_count > 0 {
                return ForgeQueryDeclarationAspectFit::Partial;
            }
            return ForgeQueryDeclarationAspectFit::MissingRequired;
        }

        if semantic_interest.is_empty() {
            return ForgeQueryDeclarationAspectFit::Exact;
        }
        if visible_present == semantic_interest
            && self.masked.is_empty()
            && self.conflicting.is_empty()
        {
            return ForgeQueryDeclarationAspectFit::Exact;
        }
        if semantic_interest.is_subset(&visible_present) {
            return ForgeQueryDeclarationAspectFit::CompatibleSuperset;
        }
        if !visible_present.is_disjoint(&semantic_interest) {
            return ForgeQueryDeclarationAspectFit::Partial;
        }
        ForgeQueryDeclarationAspectFit::MissingRequired
    }

    pub fn scoped_to_contract(
        &self,
        contract: &ForgeQueryDeclarationAspectContract,
    ) -> ForgeQueryDeclarationAspectCoverage {
        let semantic_interest: BTreeSet<_> =
            contract.semantic_interest_keys().into_iter().collect();
        ForgeQueryDeclarationAspectCoverage::new(
            self.present
                .iter()
                .filter(|path| semantic_interest.contains(*path))
                .cloned(),
            self.masked
                .iter()
                .filter(|path| semantic_interest.contains(*path))
                .cloned(),
            self.conflicting
                .iter()
                .filter(|path| semantic_interest.contains(*path))
                .cloned(),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationAspectFit {
    Exact,
    CompatibleSuperset,
    Partial,
    MissingRequired,
    Conflict,
}

impl ForgeQueryDeclarationAspectFit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::CompatibleSuperset => "compatible_superset",
            Self::Partial => "partial",
            Self::MissingRequired => "missing_required",
            Self::Conflict => "conflict",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationAuthorityAspectMismatch {
    MissingRequiredAspect,
    AspectConflict,
    AuthorityAspectGap,
    AuthorityAspectAmbiguity,
    BasisAspectMismatch,
}

impl ForgeQueryDeclarationAuthorityAspectMismatch {
    pub fn reason(self) -> &'static str {
        match self {
            Self::MissingRequiredAspect => {
                "the retained public slice does not expose every required semantic aspect"
            }
            Self::AspectConflict => {
                "the retained public slice conflicts with the required authority-scoped semantic aspect contract"
            }
            Self::AuthorityAspectGap => {
                "the lower-authority routing surface cannot cover the required semantic aspect slice"
            }
            Self::AuthorityAspectAmbiguity => {
                "multiple lower-authority mappings claim the same semantic aspect slice"
            }
            Self::BasisAspectMismatch => {
                "the lower-authority basis posture does not satisfy the required semantic aspect slice"
            }
        }
    }
}

pub(crate) fn aspect_coverage_from_publication(
    publication: &crate::application::ForgeQueryDeclarationAspectPublication,
) -> ForgeQueryDeclarationAspectCoverage {
    ForgeQueryDeclarationAspectCoverage::new(
        publication.present().iter().cloned(),
        publication
            .masked()
            .iter()
            .chain(publication.elided().iter())
            .cloned(),
        std::iter::empty::<AspectFieldKey>(),
    )
}

pub(crate) fn authority_mismatch_from_fit(
    fit: ForgeQueryDeclarationAspectFit,
) -> Option<ForgeQueryDeclarationAuthorityAspectMismatch> {
    match fit {
        ForgeQueryDeclarationAspectFit::Exact
        | ForgeQueryDeclarationAspectFit::CompatibleSuperset => None,
        ForgeQueryDeclarationAspectFit::Partial => {
            Some(ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectGap)
        }
        ForgeQueryDeclarationAspectFit::MissingRequired => {
            Some(ForgeQueryDeclarationAuthorityAspectMismatch::MissingRequiredAspect)
        }
        ForgeQueryDeclarationAspectFit::Conflict => {
            Some(ForgeQueryDeclarationAuthorityAspectMismatch::AspectConflict)
        }
    }
}

pub(in crate::application) fn terminal_declaration_aspect_projection(
    key: &AspectFieldKey,
) -> String {
    format!("{}.{}", key.aspect().as_str(), key.field().as_str())
}

fn sorted_unique(values: impl IntoIterator<Item = AspectFieldKey>) -> Vec<AspectFieldKey> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests;
