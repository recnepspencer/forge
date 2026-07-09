use worth_query::facade::{
    WORTHQueryDeclarationCanonicalEntry, WORTHQueryDeclarationCanonicalEntryKind,
    WORTHQueryDeclarationCanonicalValue, WORTHQueryDeclarationInput,
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

fn kind_entry(kind: &'static str) -> WORTHQueryDeclarationCanonicalEntry {
    WORTHQueryDeclarationCanonicalEntry::text("declaration_kind", kind)
}

fn optional_text(
    entries: &mut Vec<WORTHQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        entries.push(WORTHQueryDeclarationCanonicalEntry::text(locus, value));
    }
}

fn optional_unsigned(
    entries: &mut Vec<WORTHQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    value: Option<u32>,
) {
    if let Some(value) = value {
        entries.push(WORTHQueryDeclarationCanonicalEntry::new(
            locus,
            WORTHQueryDeclarationCanonicalEntryKind::Field,
            WORTHQueryDeclarationCanonicalValue::UnsignedInteger(value as u128),
        ));
    }
}

fn repeated_text(
    entries: &mut Vec<WORTHQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    values: &[String],
) {
    for (index, value) in values.iter().enumerate() {
        entries.push(WORTHQueryDeclarationCanonicalEntry::text(
            format!("{locus}.{index:04}"),
            value,
        ));
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry> for MotifSeedDeclaration {
    type Family = MotifSeedDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("motif_seed"),
            WORTHQueryDeclarationCanonicalEntry::text("motif_id", self.motif_id()),
        ];
        optional_text(&mut entries, "source_family", self.source_family());
        optional_text(&mut entries, "novelty_signature", self.novelty_signature());
        entries
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry> for TerminalForcingStudyDeclaration {
    type Family = TerminalForcingStudyDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("terminal_forcing_study"),
            WORTHQueryDeclarationCanonicalEntry::text("study_id", self.study_id()),
            WORTHQueryDeclarationCanonicalEntry::text("motif_ref", self.motif_ref()),
        ];
        repeated_text(&mut entries, "terminal", self.terminals());
        optional_text(&mut entries, "relation_goal", self.relation_goal());
        entries
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry> for PeriodicQuotientCellDeclaration {
    type Family = PeriodicQuotientCellDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("periodic_quotient_cell"),
            WORTHQueryDeclarationCanonicalEntry::text("cell_id", self.cell_id()),
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

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry>
    for GeneratedPatternClosureDeclaration
{
    type Family = GeneratedPatternClosureDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("generated_pattern_closure"),
            WORTHQueryDeclarationCanonicalEntry::text("closure_id", self.closure_id()),
            WORTHQueryDeclarationCanonicalEntry::text("pattern_ref", self.pattern_ref()),
        ];
        repeated_text(&mut entries, "generator", self.generators());
        entries
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry> for TileContactWitnessDeclaration {
    type Family = TileContactWitnessDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("tile_contact_witness"),
            WORTHQueryDeclarationCanonicalEntry::text("contact_id", self.contact_id()),
        ];
        optional_text(&mut entries, "left_tile_ref", self.left_tile_ref());
        optional_text(&mut entries, "right_tile_ref", self.right_tile_ref());
        optional_text(&mut entries, "contact_signature", self.contact_signature());
        entries
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry>
    for ConflictGraphExtractionDeclaration
{
    type Family = ConflictGraphExtractionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("conflict_graph_extraction"),
            WORTHQueryDeclarationCanonicalEntry::text("extraction_id", self.extraction_id()),
            WORTHQueryDeclarationCanonicalEntry::text("subject_ref", self.subject_ref()),
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

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry> for CoreExtractionDeclaration {
    type Family = CoreExtractionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("core_extraction"),
            WORTHQueryDeclarationCanonicalEntry::text(
                "core_extraction_id",
                self.core_extraction_id(),
            ),
            WORTHQueryDeclarationCanonicalEntry::text(
                "conflict_graph_ref",
                self.conflict_graph_ref(),
            ),
        ]
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry>
    for TilingEquivalenceClassificationDeclaration
{
    type Family = TilingEquivalenceClassificationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_equivalence_classification"),
            WORTHQueryDeclarationCanonicalEntry::text("equivalence_id", self.equivalence_id()),
            WORTHQueryDeclarationCanonicalEntry::text("scope", self.scope()),
            WORTHQueryDeclarationCanonicalEntry::text("left_ref", self.left_ref()),
            WORTHQueryDeclarationCanonicalEntry::text("right_ref", self.right_ref()),
        ]
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry> for TilingSuppressionDeclaration {
    type Family = TilingSuppressionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_suppression"),
            WORTHQueryDeclarationCanonicalEntry::text("suppression_id", self.suppression_id()),
            WORTHQueryDeclarationCanonicalEntry::text("equivalence_ref", self.equivalence_ref()),
            WORTHQueryDeclarationCanonicalEntry::text("suppression_ref", self.suppression_ref()),
        ]
    }
}

impl WORTHQueryDeclarationInput<HadwigerResearchDomainEntry> for TilingReactivationDeclaration {
    type Family = TilingReactivationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WORTHQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_reactivation"),
            WORTHQueryDeclarationCanonicalEntry::text("reactivation_id", self.reactivation_id()),
            WORTHQueryDeclarationCanonicalEntry::text("suppression_ref", self.suppression_ref()),
            WORTHQueryDeclarationCanonicalEntry::text(
                "qualifying_evidence_ref",
                self.qualifying_evidence_ref(),
            ),
        ]
    }
}
