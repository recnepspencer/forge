use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput,
};

use super::family_markers::{
    ConflictGraphExtractionDeclarationFamily, CoreExtractionDeclarationFamily,
    GeneratedPatternClosureDeclarationFamily, MotifSeedDeclarationFamily,
    PeriodicQuotientCellDeclarationFamily, TerminalForcingStudyDeclarationFamily,
    TileContactWitnessDeclarationFamily, TilingEquivalenceClassificationDeclarationFamily,
    TilingReactivationDeclarationFamily, TilingSuppressionDeclarationFamily,
};
use super::tiling_request_types::{
    ConflictGraphExtractionDeclaration, CoreExtractionDeclaration,
    GeneratedPatternClosureDeclaration, MotifSeedDeclaration, PeriodicQuotientCellDeclaration,
    TerminalForcingStudyDeclaration, TileContactWitnessDeclaration,
    TilingEquivalenceClassificationDeclaration, TilingReactivationDeclaration,
    TilingSuppressionDeclaration,
};
use crate::query_entry::HadwigerResearchDomainEntry;

fn kind_entry(kind: &'static str) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::text("declaration_kind", kind)
}

fn optional_text(
    entries: &mut Vec<ForgeQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(locus, value));
    }
}

fn optional_unsigned(
    entries: &mut Vec<ForgeQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    value: Option<u32>,
) {
    if let Some(value) = value {
        entries.push(ForgeQueryDeclarationCanonicalEntry::new(
            locus,
            ForgeQueryDeclarationCanonicalEntryKind::Field,
            ForgeQueryDeclarationCanonicalValue::UnsignedInteger(value as u128),
        ));
    }
}

fn repeated_text(
    entries: &mut Vec<ForgeQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    values: &[String],
) {
    for (index, value) in values.iter().enumerate() {
        entries.push(ForgeQueryDeclarationCanonicalEntry::text(
            format!("{locus}.{index:04}"),
            value,
        ));
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for MotifSeedDeclaration {
    type Family = MotifSeedDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("motif_seed"),
            ForgeQueryDeclarationCanonicalEntry::text("motif_id", self.motif_id()),
        ];
        optional_text(&mut entries, "source_family", self.source_family());
        optional_text(&mut entries, "novelty_signature", self.novelty_signature());
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for TerminalForcingStudyDeclaration {
    type Family = TerminalForcingStudyDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("terminal_forcing_study"),
            ForgeQueryDeclarationCanonicalEntry::text("study_id", self.study_id()),
            ForgeQueryDeclarationCanonicalEntry::text("motif_ref", self.motif_ref()),
        ];
        repeated_text(&mut entries, "terminal", self.terminals());
        optional_text(&mut entries, "relation_goal", self.relation_goal());
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for PeriodicQuotientCellDeclaration {
    type Family = PeriodicQuotientCellDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("periodic_quotient_cell"),
            ForgeQueryDeclarationCanonicalEntry::text("cell_id", self.cell_id()),
        ];
        optional_text(&mut entries, "lattice_basis_ref", self.lattice_basis_ref());
        optional_text(
            &mut entries,
            "boundary_ownership_ref",
            self.boundary_ownership_ref(),
        );
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for GeneratedPatternClosureDeclaration
{
    type Family = GeneratedPatternClosureDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("generated_pattern_closure"),
            ForgeQueryDeclarationCanonicalEntry::text("closure_id", self.closure_id()),
            ForgeQueryDeclarationCanonicalEntry::text("pattern_ref", self.pattern_ref()),
        ];
        repeated_text(&mut entries, "generator", self.generators());
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for TileContactWitnessDeclaration {
    type Family = TileContactWitnessDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("tile_contact_witness"),
            ForgeQueryDeclarationCanonicalEntry::text("contact_id", self.contact_id()),
        ];
        optional_text(&mut entries, "left_tile_ref", self.left_tile_ref());
        optional_text(&mut entries, "right_tile_ref", self.right_tile_ref());
        optional_text(&mut entries, "contact_signature", self.contact_signature());
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for ConflictGraphExtractionDeclaration
{
    type Family = ConflictGraphExtractionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("conflict_graph_extraction"),
            ForgeQueryDeclarationCanonicalEntry::text("extraction_id", self.extraction_id()),
            ForgeQueryDeclarationCanonicalEntry::text("subject_ref", self.subject_ref()),
        ];
        optional_text(
            &mut entries,
            "distance_certificate_family",
            self.distance_certificate_family(),
        );
        optional_unsigned(
            &mut entries,
            "required_color_count",
            self.required_color_count(),
        );
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for CoreExtractionDeclaration {
    type Family = CoreExtractionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("core_extraction"),
            ForgeQueryDeclarationCanonicalEntry::text(
                "core_extraction_id",
                self.core_extraction_id(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "conflict_graph_ref",
                self.conflict_graph_ref(),
            ),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for TilingEquivalenceClassificationDeclaration
{
    type Family = TilingEquivalenceClassificationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_equivalence_classification"),
            ForgeQueryDeclarationCanonicalEntry::text("equivalence_id", self.equivalence_id()),
            ForgeQueryDeclarationCanonicalEntry::text("scope", self.scope()),
            ForgeQueryDeclarationCanonicalEntry::text("left_ref", self.left_ref()),
            ForgeQueryDeclarationCanonicalEntry::text("right_ref", self.right_ref()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for TilingSuppressionDeclaration {
    type Family = TilingSuppressionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_suppression"),
            ForgeQueryDeclarationCanonicalEntry::text("suppression_id", self.suppression_id()),
            ForgeQueryDeclarationCanonicalEntry::text("equivalence_ref", self.equivalence_ref()),
            ForgeQueryDeclarationCanonicalEntry::text("suppression_ref", self.suppression_ref()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for TilingReactivationDeclaration {
    type Family = TilingReactivationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_reactivation"),
            ForgeQueryDeclarationCanonicalEntry::text("reactivation_id", self.reactivation_id()),
            ForgeQueryDeclarationCanonicalEntry::text("suppression_ref", self.suppression_ref()),
            ForgeQueryDeclarationCanonicalEntry::text(
                "qualifying_evidence_ref",
                self.qualifying_evidence_ref(),
            ),
        ]
    }
}
