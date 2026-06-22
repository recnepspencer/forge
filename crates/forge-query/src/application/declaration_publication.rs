use std::collections::BTreeSet;

use forge_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationEntryOrchestrationMaterializationTier,
};
use crate::authoring::AspectFieldKey;

use super::declaration_aspect::terminal_declaration_aspect_projection;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationAspectPublication {
    present: Vec<AspectFieldKey>,
    widened: Vec<AspectFieldKey>,
    elided: Vec<AspectFieldKey>,
    masked: Vec<AspectFieldKey>,
}

impl ForgeQueryDeclarationAspectPublication {
    pub fn empty() -> Self {
        Self::new(
            Vec::<AspectFieldKey>::new(),
            Vec::<AspectFieldKey>::new(),
            Vec::<AspectFieldKey>::new(),
            Vec::<AspectFieldKey>::new(),
        )
    }

    pub fn new(
        present: impl IntoIterator<Item = AspectFieldKey>,
        widened: impl IntoIterator<Item = AspectFieldKey>,
        elided: impl IntoIterator<Item = AspectFieldKey>,
        masked: impl IntoIterator<Item = AspectFieldKey>,
    ) -> Self {
        Self {
            present: sorted_unique(present),
            widened: sorted_unique(widened),
            elided: sorted_unique(elided),
            masked: sorted_unique(masked),
        }
    }

    pub fn present(&self) -> &[AspectFieldKey] {
        &self.present
    }

    pub fn widened(&self) -> &[AspectFieldKey] {
        &self.widened
    }

    pub fn elided(&self) -> &[AspectFieldKey] {
        &self.elided
    }

    pub fn masked(&self) -> &[AspectFieldKey] {
        &self.masked
    }

    pub(crate) fn terminal_present_projections_for_boundary(&self) -> Vec<String> {
        terminal_declaration_aspect_projections(&self.present)
    }

    pub(crate) fn terminal_widened_projections_for_boundary(&self) -> Vec<String> {
        terminal_declaration_aspect_projections(&self.widened)
    }

    pub(crate) fn terminal_elided_projections_for_boundary(&self) -> Vec<String> {
        terminal_declaration_aspect_projections(&self.elided)
    }

    pub(crate) fn terminal_masked_projections_for_boundary(&self) -> Vec<String> {
        terminal_declaration_aspect_projections(&self.masked)
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
        .semantic_interest_keys()
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

pub(crate) fn terminal_declaration_aspect_projections(fields: &[AspectFieldKey]) -> Vec<String> {
    fields
        .iter()
        .map(terminal_declaration_aspect_projection)
        .collect()
}

fn sorted_unique(values: impl IntoIterator<Item = AspectFieldKey>) -> Vec<AspectFieldKey> {
    let mut values = values.into_iter().collect::<Vec<_>>();
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
        assert_declaration_aspect_projections, ForgeQueryDeclarationAspectContract,
        ForgeQueryDeclarationAspectCoverage,
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

        assert_declaration_aspect_projections(lean.present(), &["selection.active_edge"]);
        assert!(lean.widened().is_empty());
        assert_declaration_aspect_projections(
            lean.elided(),
            &["selection.local_topology", "selection.material_edit"],
        );

        assert_declaration_aspect_projections(
            support_ready.present(),
            &["selection.active_edge", "selection.local_topology"],
        );
        assert_declaration_aspect_projections(
            support_ready.widened(),
            &["selection.local_topology"],
        );

        assert_declaration_aspect_projections(
            full.present(),
            &[
                "selection.active_edge",
                "selection.local_topology",
                "selection.material_edit",
            ],
        );
        assert_declaration_aspect_projections(
            full.widened(),
            &["selection.local_topology", "selection.material_edit"],
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

        assert_declaration_aspect_projections(publication.present(), &["selection.active_edge"]);
        assert_declaration_aspect_projections(
            publication.masked(),
            &["selection.local_topology", "selection.private_authority"],
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
