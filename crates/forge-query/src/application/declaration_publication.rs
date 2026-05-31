use std::collections::BTreeSet;

use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationAspectPublication {
    present: Vec<String>,
    widened: Vec<String>,
    elided: Vec<String>,
    masked: Vec<String>,
}

impl ForgeQueryDeclarationAspectPublication {
    pub fn empty() -> Self {
        Self::new(
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
            Vec::<String>::new(),
        )
    }

    pub fn new(
        present: impl IntoIterator<Item = impl Into<String>>,
        widened: impl IntoIterator<Item = impl Into<String>>,
        elided: impl IntoIterator<Item = impl Into<String>>,
        masked: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            present: sorted_unique(present),
            widened: sorted_unique(widened),
            elided: sorted_unique(elided),
            masked: sorted_unique(masked),
        }
    }

    pub fn present(&self) -> &[String] {
        &self.present
    }

    pub fn widened(&self) -> &[String] {
        &self.widened
    }

    pub fn elided(&self) -> &[String] {
        &self.elided
    }

    pub fn masked(&self) -> &[String] {
        &self.masked
    }
}

pub fn foundational_publication_for_profile(
    contract: &ForgeQueryDeclarationAspectContract,
    coverage: &ForgeQueryDeclarationAspectCoverage,
    profile: FoundationalBoundaryEvidenceMaterializationProfile,
) -> ForgeQueryDeclarationAspectPublication {
    let tier = match profile {
        FoundationalBoundaryEvidenceMaterializationProfile::ElideSupportAndDiagnostics => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean
        }
        FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady
        }
        FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness => {
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive
        }
    };
    declaration_publication_for_tier(contract, coverage, tier)
}

pub fn declaration_publication_for_tier(
    contract: &ForgeQueryDeclarationAspectContract,
    coverage: &ForgeQueryDeclarationAspectCoverage,
    tier: ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
) -> ForgeQueryDeclarationAspectPublication {
    let masked = coverage.masked().iter().cloned().collect::<BTreeSet<_>>();
    let conflicting = coverage
        .conflicting()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let visible_present = coverage
        .present()
        .iter()
        .filter(|path| !masked.contains(*path) && !conflicting.contains(*path))
        .cloned()
        .collect::<BTreeSet<_>>();
    let required = contract.required().iter().cloned().collect::<BTreeSet<_>>();
    let preserved = contract
        .preserved()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let published = contract
        .published()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();

    let selected = match tier {
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean => {
            required.clone()
        }
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady => {
            required.union(&preserved).cloned().collect()
        }
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive => required
            .union(&preserved)
            .chain(published.iter())
            .cloned()
            .collect(),
    };
    let semantic_interest = contract
        .semantic_interest_paths()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let present = selected
        .intersection(&visible_present)
        .cloned()
        .collect::<BTreeSet<_>>();
    let widened = present
        .difference(&required)
        .cloned()
        .collect::<BTreeSet<_>>();
    let elided = semantic_interest
        .difference(&selected)
        .cloned()
        .collect::<BTreeSet<_>>();
    let masked = contract
        .masked()
        .iter()
        .chain(masked.iter())
        .chain(conflicting.iter())
        .cloned()
        .collect::<BTreeSet<_>>();

    ForgeQueryDeclarationAspectPublication::new(present, widened, elided, masked)
}

fn sorted_unique(values: impl IntoIterator<Item = impl Into<String>>) -> Vec<String> {
    let mut values = values.into_iter().map(Into::into).collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

#[cfg(test)]
mod tests {
    use super::{
        declaration_publication_for_tier, foundational_publication_for_profile,
        ForgeQueryDeclarationAspectPublication,
    };
    use crate::application::{
        ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
        ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
    };
    use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

    #[test]
    fn lean_support_ready_and_full_descriptive_widen_semantic_publication_explicitly() {
        let contract = ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.local_topology"],
            &["selection.material_edit"],
            &[],
            &[],
        );
        let coverage = ForgeQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_edge",
                "selection.local_topology",
                "selection.material_edit",
            ],
            &[],
            &[],
        );

        let lean = declaration_publication_for_tier(
            &contract,
            &coverage,
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean,
        );
        let support_ready = declaration_publication_for_tier(
            &contract,
            &coverage,
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::SupportReady,
        );
        let full = declaration_publication_for_tier(
            &contract,
            &coverage,
            ForgeQueryDeclarationEntryOrchestrationMaterializationTier::FullDescriptive,
        );

        assert_eq!(lean.present(), &["selection.active_edge".to_string()]);
        assert!(lean.widened().is_empty());
        assert_eq!(
            lean.elided(),
            &[
                "selection.local_topology".to_string(),
                "selection.material_edit".to_string()
            ]
        );

        assert_eq!(
            support_ready.present(),
            &[
                "selection.active_edge".to_string(),
                "selection.local_topology".to_string()
            ]
        );
        assert_eq!(
            support_ready.widened(),
            &["selection.local_topology".to_string()]
        );

        assert_eq!(
            full.present(),
            &[
                "selection.active_edge".to_string(),
                "selection.local_topology".to_string(),
                "selection.material_edit".to_string()
            ]
        );
        assert_eq!(
            full.widened(),
            &[
                "selection.local_topology".to_string(),
                "selection.material_edit".to_string()
            ]
        );
    }

    #[test]
    fn publication_preserves_masked_slices_without_promoting_them_to_present() {
        let contract = ForgeQueryDeclarationAspectContract::from_slices(
            &["selection.active_edge"],
            &["selection.local_topology"],
            &[],
            &["selection.private_authority"],
            &[],
        );
        let coverage = ForgeQueryDeclarationAspectCoverage::from_slices(
            &[
                "selection.active_edge",
                "selection.local_topology",
                "selection.private_authority",
            ],
            &["selection.local_topology"],
            &[],
        );

        let publication = foundational_publication_for_profile(
            &contract,
            &coverage,
            FoundationalBoundaryEvidenceMaterializationProfile::ElideDiagnostics,
        );

        assert_eq!(
            publication.present(),
            &["selection.active_edge".to_string()]
        );
        assert_eq!(
            publication.masked(),
            &[
                "selection.local_topology".to_string(),
                "selection.private_authority".to_string()
            ]
        );
    }

    #[test]
    fn empty_publication_stays_empty() {
        assert_eq!(
            ForgeQueryDeclarationAspectPublication::empty(),
            declaration_publication_for_tier(
                &ForgeQueryDeclarationAspectContract::empty(),
                &ForgeQueryDeclarationAspectCoverage::empty(),
                ForgeQueryDeclarationEntryOrchestrationMaterializationTier::OperationalLean
            )
        );
    }
}
