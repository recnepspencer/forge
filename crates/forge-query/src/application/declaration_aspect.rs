use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationAspectContract {
    required: Vec<String>,
    preserved: Vec<String>,
    published: Vec<String>,
    masked: Vec<String>,
    incompatible: Vec<String>,
}

impl ForgeQueryDeclarationAspectContract {
    pub fn empty() -> Self {
        Self::new(
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
        )
    }

    pub fn new(
        required: impl IntoIterator<Item = impl Into<String>>,
        preserved: impl IntoIterator<Item = impl Into<String>>,
        published: impl IntoIterator<Item = impl Into<String>>,
        masked: impl IntoIterator<Item = impl Into<String>>,
        incompatible: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            required: sorted_unique(required),
            preserved: sorted_unique(preserved),
            published: sorted_unique(published),
            masked: sorted_unique(masked),
            incompatible: sorted_unique(incompatible),
        }
    }

    pub fn from_slices(
        required: &[&str],
        preserved: &[&str],
        published: &[&str],
        masked: &[&str],
        incompatible: &[&str],
    ) -> Self {
        Self::new(
            required.iter().copied(),
            preserved.iter().copied(),
            published.iter().copied(),
            masked.iter().copied(),
            incompatible.iter().copied(),
        )
    }

    pub fn required(&self) -> &[String] {
        &self.required
    }

    pub fn preserved(&self) -> &[String] {
        &self.preserved
    }

    pub fn published(&self) -> &[String] {
        &self.published
    }

    pub fn masked(&self) -> &[String] {
        &self.masked
    }

    pub fn incompatible(&self) -> &[String] {
        &self.incompatible
    }

    pub fn semantic_interest_paths(&self) -> Vec<String> {
        sorted_unique(
            self.required
                .iter()
                .chain(self.preserved.iter())
                .chain(self.published.iter())
                .cloned(),
        )
    }

    pub fn default_coverage(&self) -> ForgeQueryDeclarationAspectCoverage {
        ForgeQueryDeclarationAspectCoverage::from_present(self.semantic_interest_paths())
    }
}

pub(crate) fn route_scoped_declaration_aspect_contract(
    declaration_contract: &ForgeQueryDeclarationAspectContract,
) -> ForgeQueryDeclarationAspectContract {
    ForgeQueryDeclarationAspectContract::new(
        declaration_contract.required().iter().cloned(),
        declaration_contract.preserved().iter().cloned(),
        std::iter::empty::<String>(),
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
    present: Vec<String>,
    masked: Vec<String>,
    conflicting: Vec<String>,
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
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
        )
    }

    pub fn new(
        present: impl IntoIterator<Item = impl Into<String>>,
        masked: impl IntoIterator<Item = impl Into<String>>,
        conflicting: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            present: sorted_unique(present),
            masked: sorted_unique(masked),
            conflicting: sorted_unique(conflicting),
        }
    }

    pub fn from_present(present: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::new(
            present,
            std::iter::empty::<String>(),
            std::iter::empty::<String>(),
        )
    }

    pub fn from_slices(present: &[&str], masked: &[&str], conflicting: &[&str]) -> Self {
        Self::new(
            present.iter().copied(),
            masked.iter().copied(),
            conflicting.iter().copied(),
        )
    }

    pub fn present(&self) -> &[String] {
        &self.present
    }

    pub fn masked(&self) -> &[String] {
        &self.masked
    }

    pub fn conflicting(&self) -> &[String] {
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
            contract.semantic_interest_paths().into_iter().collect();

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
            contract.semantic_interest_paths().into_iter().collect();
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
        std::iter::empty::<String>(),
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

fn sorted_unique(values: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
    let mut values = values.into_iter().map(Into::into).collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests;
