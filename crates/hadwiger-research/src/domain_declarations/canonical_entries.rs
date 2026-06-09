use forge_query::facade::{
    ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationCanonicalEntryKind,
    ForgeQueryDeclarationCanonicalValue, ForgeQueryDeclarationInput,
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

fn unsigned_entry(locus: &'static str, value: u32) -> ForgeQueryDeclarationCanonicalEntry {
    ForgeQueryDeclarationCanonicalEntry::new(
        locus,
        ForgeQueryDeclarationCanonicalEntryKind::Field,
        ForgeQueryDeclarationCanonicalValue::UnsignedInteger(value as u128),
    )
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for CandidateGraphDeclaration {
    type Family = CandidateGraphDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("candidate_graph"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
        ];
        optional_text(&mut entries, "graph_version", self.graph_version());
        optional_text(&mut entries, "source_note", self.source_note());
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for EmbeddingDeclaration {
    type Family = EmbeddingDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("embedding"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            ForgeQueryDeclarationCanonicalEntry::text("embedding_id", self.embedding_id()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for ColorabilityDeclaration {
    type Family = ColorabilityDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("colorability"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            unsigned_entry("color_count", self.color_count()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for LowerBoundWitnessDeclaration {
    type Family = LowerBoundWitnessDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("lower_bound_witness"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            ForgeQueryDeclarationCanonicalEntry::text("embedding_id", self.embedding_id()),
            unsigned_entry("color_count", self.color_count()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for AdvisoryNoteDeclaration {
    type Family = AdvisoryNoteDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("advisory_note"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            ForgeQueryDeclarationCanonicalEntry::text("note", self.note()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for RejectionExplanationDeclaration {
    type Family = RejectionExplanationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("rejection_explanation"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            ForgeQueryDeclarationCanonicalEntry::text("rejection_basis", self.rejection_basis()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for PartialAdmissionExplanationDeclaration
{
    type Family = PartialAdmissionExplanationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("partial_admission_explanation"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for UnitDistanceVerificationDeclaration
{
    type Family = UnitDistanceVerificationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("unit_distance_verification"),
            ForgeQueryDeclarationCanonicalEntry::text("graph_id", self.graph_id()),
            ForgeQueryDeclarationCanonicalEntry::text("embedding_id", self.embedding_id()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for WholePlaneColoringConstructionDeclaration
{
    type Family = WholePlaneColoringConstructionDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("whole_plane_coloring_construction"),
            ForgeQueryDeclarationCanonicalEntry::text("construction_id", self.construction_id()),
            unsigned_entry("color_count", self.color_count()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for PlaneLowerBoundClaimDeclaration {
    type Family = PlaneLowerBoundClaimDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("plane_lower_bound_claim"),
            ForgeQueryDeclarationCanonicalEntry::text("claim_id", self.claim_id()),
            ForgeQueryDeclarationCanonicalEntry::text("graph_version_id", self.graph_version_id()),
            unsigned_entry("forbidden_color_count", self.forbidden_color_count()),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for PlaneUpperBoundClaimDeclaration {
    type Family = PlaneUpperBoundClaimDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("plane_upper_bound_claim"),
            ForgeQueryDeclarationCanonicalEntry::text("claim_id", self.claim_id()),
            unsigned_entry("color_count", self.color_count()),
            ForgeQueryDeclarationCanonicalEntry::text(
                "upper_bound_source",
                self.upper_bound_source(),
            ),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for PlaneExactValueClaimDeclaration {
    type Family = PlaneExactValueClaimDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("plane_exact_value_claim"),
            ForgeQueryDeclarationCanonicalEntry::text("claim_id", self.claim_id()),
            unsigned_entry("color_count", self.color_count()),
            ForgeQueryDeclarationCanonicalEntry::text(
                "lower_bound_claim_digest",
                self.lower_bound_claim_digest(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text(
                "upper_bound_source",
                self.upper_bound_source(),
            ),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry> for BackgroundTheoremDeclaration {
    type Family = BackgroundTheoremDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        vec![
            kind_entry("background_theorem"),
            ForgeQueryDeclarationCanonicalEntry::text("theorem_id", self.theorem_id()),
            ForgeQueryDeclarationCanonicalEntry::text(
                "theorem_statement",
                self.theorem_statement(),
            ),
            ForgeQueryDeclarationCanonicalEntry::text("source", self.source()),
            ForgeQueryDeclarationCanonicalEntry::text(
                "provenance_digest",
                self.provenance_digest(),
            ),
        ]
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for LowerBoundTilingIterationDeclaration
{
    type Family = LowerBoundTilingIterationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("lower_bound_tiling_iteration"),
            ForgeQueryDeclarationCanonicalEntry::text("packet_id", self.packet_id()),
            ForgeQueryDeclarationCanonicalEntry::text("session_digest", self.session_digest()),
        ];
        for (index, basis) in self.evidence_basis().iter().enumerate() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                format!("evidence_basis_{index:04}"),
                basis,
            ));
        }
        for (index, lane) in self.required_checker_lanes().iter().enumerate() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                format!("required_checker_lane_{index:04}"),
                lane,
            ));
        }
        for (index, obligation) in self.reactivation_obligations().iter().enumerate() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                format!("reactivation_obligation_{index:04}"),
                obligation,
            ));
        }
        entries
    }
}

impl ForgeQueryDeclarationInput<HadwigerResearchDomainEntry>
    for UpperBoundTilingIterationDeclaration
{
    type Family = UpperBoundTilingIterationDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        let mut entries = vec![
            kind_entry("upper_bound_tiling_iteration"),
            ForgeQueryDeclarationCanonicalEntry::text("packet_id", self.packet_id()),
            ForgeQueryDeclarationCanonicalEntry::text("session_digest", self.session_digest()),
        ];
        for (index, basis) in self.evidence_basis().iter().enumerate() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                format!("evidence_basis_{index:04}"),
                basis,
            ));
        }
        for (index, lane) in self.required_checker_lanes().iter().enumerate() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                format!("required_checker_lane_{index:04}"),
                lane,
            ));
        }
        for (index, obligation) in self.reactivation_obligations().iter().enumerate() {
            entries.push(ForgeQueryDeclarationCanonicalEntry::text(
                format!("reactivation_obligation_{index:04}"),
                obligation,
            ));
        }
        entries
    }
}
