use worth_query::facade::domain::{
    WorthQueryDeclarationCanonicalEntry, WorthQueryDeclarationCanonicalEntryKind,
    WorthQueryDeclarationCanonicalValue, WorthQueryDeclarationInput,
};

use super::family_markers::{
    AdvisoryNoteDeclarationFamily, BackgroundTheoremDeclarationFamily,
    CandidateGraphDeclarationFamily, ColorabilityDeclarationFamily, EmbeddingDeclarationFamily,
    LowerBoundTilingIterationDeclarationFamily, LowerBoundWitnessDeclarationFamily,
    PartialAdmissionExplanationDeclarationFamily, PlaneExactValueClaimDeclarationFamily,
    PlaneLowerBoundClaimDeclarationFamily, PlaneUpperBoundClaimDeclarationFamily,
    RejectionExplanationDeclarationFamily, UnitDistanceVerificationDeclarationFamily,
    UpperBoundTilingIterationDeclarationFamily, WholePlaneColoringConstructionDeclarationFamily,
};
use super::proof_claim_request_types::{
    BackgroundTheoremDeclaration, PlaneExactValueClaimDeclaration, PlaneLowerBoundClaimDeclaration,
    PlaneUpperBoundClaimDeclaration,
};
use super::request_types::{
    AdvisoryNoteDeclaration, CandidateGraphDeclaration, ColorabilityDeclaration,
    EmbeddingDeclaration, LowerBoundWitnessDeclaration, PartialAdmissionExplanationDeclaration,
    RejectionExplanationDeclaration, UnitDistanceVerificationDeclaration,
    WholePlaneColoringConstructionDeclaration,
};
use super::tiling_request_types::{
    LowerBoundTilingIterationDeclaration, UpperBoundTilingIterationDeclaration,
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

fn unsigned_entry(locus: &'static str, value: u32) -> WorthQueryDeclarationCanonicalEntry {
    WorthQueryDeclarationCanonicalEntry::new(
        locus,
        WorthQueryDeclarationCanonicalEntryKind::Field,
        WorthQueryDeclarationCanonicalValue::UnsignedInteger(value as u128),
    )
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for CandidateGraphDeclaration {
    type Family = CandidateGraphDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("candidate_graph"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
        ];
        optional_text(&mut entries, "graph_version", self.graph_version());
        optional_text(&mut entries, "source_note", self.source_note());
        entries
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for EmbeddingDeclaration {
    type Family = EmbeddingDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("embedding"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            WorthQueryDeclarationCanonicalEntry::text("embedding_id", self.embedding_id()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for ColorabilityDeclaration {
    type Family = ColorabilityDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("colorability"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            unsigned_entry("color_count", self.color_count()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for LowerBoundWitnessDeclaration {
    type Family = LowerBoundWitnessDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("lower_bound_witness"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            WorthQueryDeclarationCanonicalEntry::text("embedding_id", self.embedding_id()),
            unsigned_entry("color_count", self.color_count()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for AdvisoryNoteDeclaration {
    type Family = AdvisoryNoteDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("advisory_note"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            WorthQueryDeclarationCanonicalEntry::text("note", self.note()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for RejectionExplanationDeclaration {
    type Family = RejectionExplanationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("rejection_explanation"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            WorthQueryDeclarationCanonicalEntry::text("rejection_basis", self.rejection_basis()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for PartialAdmissionExplanationDeclaration
{
    type Family = PartialAdmissionExplanationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("partial_admission_explanation"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for UnitDistanceVerificationDeclaration
{
    type Family = UnitDistanceVerificationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("unit_distance_verification"),
            WorthQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            WorthQueryDeclarationCanonicalEntry::text("embedding_id", self.embedding_id()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for WholePlaneColoringConstructionDeclaration
{
    type Family = WholePlaneColoringConstructionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("whole_plane_coloring_construction"),
            WorthQueryDeclarationCanonicalEntry::text("construction_id", self.construction_id()),
            unsigned_entry("color_count", self.color_count()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for PlaneLowerBoundClaimDeclaration {
    type Family = PlaneLowerBoundClaimDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("plane_lower_bound_claim"),
            WorthQueryDeclarationCanonicalEntry::text("claim_id", self.claim_id()),
            WorthQueryDeclarationCanonicalEntry::text("graph_version_id", self.graph_version_id()),
            unsigned_entry("forbidden_color_count", self.forbidden_color_count()),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for PlaneUpperBoundClaimDeclaration {
    type Family = PlaneUpperBoundClaimDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("plane_upper_bound_claim"),
            WorthQueryDeclarationCanonicalEntry::text("claim_id", self.claim_id()),
            unsigned_entry("color_count", self.color_count()),
            WorthQueryDeclarationCanonicalEntry::text(
                "upper_bound_source",
                self.upper_bound_source(),
            ),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for PlaneExactValueClaimDeclaration {
    type Family = PlaneExactValueClaimDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("plane_exact_value_claim"),
            WorthQueryDeclarationCanonicalEntry::text("claim_id", self.claim_id()),
            unsigned_entry("color_count", self.color_count()),
            WorthQueryDeclarationCanonicalEntry::text(
                "lower_bound_claim_digest",
                self.lower_bound_claim_digest(),
            ),
            WorthQueryDeclarationCanonicalEntry::text(
                "upper_bound_source",
                self.upper_bound_source(),
            ),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry> for BackgroundTheoremDeclaration {
    type Family = BackgroundTheoremDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("background_theorem"),
            WorthQueryDeclarationCanonicalEntry::text("theorem_id", self.theorem_id()),
            WorthQueryDeclarationCanonicalEntry::text(
                "theorem_statement",
                self.theorem_statement(),
            ),
            WorthQueryDeclarationCanonicalEntry::text("source", self.source()),
            WorthQueryDeclarationCanonicalEntry::text(
                "provenance_digest",
                self.provenance_digest(),
            ),
        ]
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for LowerBoundTilingIterationDeclaration
{
    type Family = LowerBoundTilingIterationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("lower_bound_tiling_iteration"),
            WorthQueryDeclarationCanonicalEntry::text("packet_id", self.packet_id()),
            WorthQueryDeclarationCanonicalEntry::text("session_digest", self.session_digest()),
        ];
        for (index, basis) in self.evidence_basis().iter().enumerate() {
            entries.push(WorthQueryDeclarationCanonicalEntry::text(
                format!("evidence_basis_{index:04}"),
                basis,
            ));
        }
        for (index, lane) in self.required_checker_lanes().iter().enumerate() {
            entries.push(WorthQueryDeclarationCanonicalEntry::text(
                format!("required_checker_lane_{index:04}"),
                lane,
            ));
        }
        for (index, obligation) in self.reactivation_obligations().iter().enumerate() {
            entries.push(WorthQueryDeclarationCanonicalEntry::text(
                format!("reactivation_obligation_{index:04}"),
                obligation,
            ));
        }
        entries
    }
}

impl WorthQueryDeclarationInput<HadwigerResearchDomainEntry>
    for UpperBoundTilingIterationDeclaration
{
    type Family = UpperBoundTilingIterationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<WorthQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("upper_bound_tiling_iteration"),
            WorthQueryDeclarationCanonicalEntry::text("packet_id", self.packet_id()),
            WorthQueryDeclarationCanonicalEntry::text("session_digest", self.session_digest()),
        ];
        for (index, basis) in self.evidence_basis().iter().enumerate() {
            entries.push(WorthQueryDeclarationCanonicalEntry::text(
                format!("evidence_basis_{index:04}"),
                basis,
            ));
        }
        for (index, lane) in self.required_checker_lanes().iter().enumerate() {
            entries.push(WorthQueryDeclarationCanonicalEntry::text(
                format!("required_checker_lane_{index:04}"),
                lane,
            ));
        }
        for (index, obligation) in self.reactivation_obligations().iter().enumerate() {
            entries.push(WorthQueryDeclarationCanonicalEntry::text(
                format!("reactivation_obligation_{index:04}"),
                obligation,
            ));
        }
        entries
    }
}
