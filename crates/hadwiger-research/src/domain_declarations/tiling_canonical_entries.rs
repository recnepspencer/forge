use worth_query::facade::domain::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue, WorthQueryDeclarationInput,
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

fn kind_entry(kind: &'static str) -> WorthQueryDeclarationCanonicalEntry {
    WorthQueryDeclarationCanonicalEntry::text("declaration_kind", kind)
}

fn optional_text(
    entries: &mut Vec<WorthQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    value: Option<&str>,
) {
    if let Some(value) = value {
        entries.push(WorthQueryDeclarationCanonicalEntry::text(locus, value));
    }
}

fn optional_unsigned(
    entries: &mut Vec<WorthQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    value: Option<u32>,
) {
    if let Some(value) = value {
        entries.push(WorthQueryDeclarationCanonicalEntry::new(
            locus,
            WorthQueryDeclarationCanonicalEntryKind::Field,
            WorthQueryDeclarationCanonicalValue::UnsignedInteger(value as u128),
        ));
    }
}

fn repeated_text(
    entries: &mut Vec<WorthQueryDeclarationCanonicalEntry>,
    locus: &'static str,
    values: &[String],
) {
    for (index, value) in values.iter().enumerate() {
        entries.push(WorthQueryDeclarationCanonicalEntry::text(
            format!("{locus}.{index:04}"),
            value,
        ));
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for MotifSeedDeclaration {
    type Family = MotifSeedDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("motif_seed"),
            WorthQueryDeclarationCanonicalEntry::text("motif_id", self.motif_id()),
        ];
        optional_text(&mut entries, "source_family", self.source_family());
        optional_text(&mut entries, "novelty_signature", self.novelty_signature());
        entries
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for TerminalForcingStudyDeclaration {
    type Family = TerminalForcingStudyDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("terminal_forcing_study"),
            WorthQueryDeclarationCanonicalEntry::text("study_id", self.study_id()),
            WorthQueryDeclarationCanonicalEntry::text("motif_ref", self.motif_ref()),
        ];
        repeated_text(&mut entries, "terminal", self.terminals());
        optional_text(&mut entries, "relation_goal", self.relation_goal());
        entries
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for PeriodicQuotientCellDeclaration {
    type Family = PeriodicQuotientCellDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("periodic_quotient_cell"),
            WorthQueryDeclarationCanonicalEntry::text("cell_id", self.cell_id()),
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

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for GeneratedPatternClosureDeclaration
{
    type Family = GeneratedPatternClosureDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("generated_pattern_closure"),
            WorthQueryDeclarationCanonicalEntry::text("closure_id", self.closure_id()),
            WorthQueryDeclarationCanonicalEntry::text("pattern_ref", self.pattern_ref()),
        ];
        repeated_text(&mut entries, "generator", self.generators());
        entries
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for TileContactWitnessDeclaration {
    type Family = TileContactWitnessDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("tile_contact_witness"),
            WorthQueryDeclarationCanonicalEntry::text("contact_id", self.contact_id()),
        ];
        optional_text(&mut entries, "left_tile_ref", self.left_tile_ref());
        optional_text(&mut entries, "right_tile_ref", self.right_tile_ref());
        optional_text(&mut entries, "contact_signature", self.contact_signature());
        entries
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for ConflictGraphExtractionDeclaration
{
    type Family = ConflictGraphExtractionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("conflict_graph_extraction"),
            WorthQueryDeclarationCanonicalEntry::text("extraction_id", self.extraction_id()),
            WorthQueryDeclarationCanonicalEntry::text("subject_ref", self.subject_ref()),
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

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for CoreExtractionDeclaration {
    type Family = CoreExtractionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("core_extraction"),
            WorthQueryDeclarationCanonicalEntry::text(
                "core_extraction_id",
                self.core_extraction_id(),
            ),
            WorthQueryDeclarationCanonicalEntry::text(
                "conflict_graph_ref",
                self.conflict_graph_ref(),
            ),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for TilingEquivalenceClassificationDeclaration
{
    type Family = TilingEquivalenceClassificationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_equivalence_classification"),
            WorthQueryDeclarationCanonicalEntry::text("equivalence_id", self.equivalence_id()),
            WorthQueryDeclarationCanonicalEntry::text("scope", self.scope()),
            WorthQueryDeclarationCanonicalEntry::text("left_ref", self.left_ref()),
            WorthQueryDeclarationCanonicalEntry::text("right_ref", self.right_ref()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for TilingSuppressionDeclaration {
    type Family = TilingSuppressionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_suppression"),
            WorthQueryDeclarationCanonicalEntry::text("suppression_id", self.suppression_id()),
            WorthQueryDeclarationCanonicalEntry::text("equivalence_ref", self.equivalence_ref()),
            WorthQueryDeclarationCanonicalEntry::text("suppression_ref", self.suppression_ref()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for TilingReactivationDeclaration {
    type Family = TilingReactivationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("tiling_reactivation"),
            WorthQueryDeclarationCanonicalEntry::text("reactivation_id", self.reactivation_id()),
            WorthQueryDeclarationCanonicalEntry::text("suppression_ref", self.suppression_ref()),
            WorthQueryDeclarationCanonicalEntry::text(
                "qualifying_evidence_ref",
                self.qualifying_evidence_ref(),
            ),
        ]
    }
}
