use super::row::CrossFamilyCoverageFamilyKind;
use crate::workload_composition::planner_owned_routing::WorthTouchedGraphConflictQueryGapKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchitectureClaimLedgerRowKind {
    Covered,
    CappedResidue,
    QueryGap,
    BlockedOutsideRoadmap,
}

impl ArchitectureClaimLedgerRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::CappedResidue => "capped_residue",
            Self::QueryGap => "query_gap",
            Self::BlockedOutsideRoadmap => "blocked_outside_roadmap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchitectureClaimLedgerRow {
    family_kind: CrossFamilyCoverageFamilyKind,
    owner: String,
    surface_name: String,
    surface_path: String,
    selected_route_packet_digest: String,
    seed_digest: String,
    architecture_claim_digest: String,
    residue_or_firewall_digest: Option<String>,
    claim_kind: ArchitectureClaimLedgerRowKind,
    query_gap_kind: Option<WorthTouchedGraphConflictQueryGapKind>,
    mechanically_unreachable_from_ordinary_path: bool,
}

impl ArchitectureClaimLedgerRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        family_kind: CrossFamilyCoverageFamilyKind,
        owner: impl Into<String>,
        surface_name: impl Into<String>,
        surface_path: impl Into<String>,
        selected_route_packet_digest: impl Into<String>,
        seed_digest: impl Into<String>,
        architecture_claim_digest: impl Into<String>,
        residue_or_firewall_digest: Option<String>,
        claim_kind: ArchitectureClaimLedgerRowKind,
        query_gap_kind: Option<WorthTouchedGraphConflictQueryGapKind>,
        mechanically_unreachable_from_ordinary_path: bool,
    ) -> Self {
        Self {
            family_kind,
            owner: owner.into(),
            surface_name: surface_name.into(),
            surface_path: surface_path.into(),
            selected_route_packet_digest: selected_route_packet_digest.into(),
            seed_digest: seed_digest.into(),
            architecture_claim_digest: architecture_claim_digest.into(),
            residue_or_firewall_digest,
            claim_kind,
            query_gap_kind,
            mechanically_unreachable_from_ordinary_path,
        }
    }

    pub const fn family_kind(&self) -> CrossFamilyCoverageFamilyKind {
        self.family_kind
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn surface_path(&self) -> &str {
        &self.surface_path
    }

    pub fn selected_route_packet_digest(&self) -> &str {
        &self.selected_route_packet_digest
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub fn architecture_claim_digest(&self) -> &str {
        &self.architecture_claim_digest
    }

    pub fn residue_or_firewall_digest(&self) -> Option<&str> {
        self.residue_or_firewall_digest.as_deref()
    }

    pub const fn claim_kind(&self) -> ArchitectureClaimLedgerRowKind {
        self.claim_kind
    }

    pub const fn query_gap_kind(&self) -> Option<WorthTouchedGraphConflictQueryGapKind> {
        self.query_gap_kind
    }

    pub const fn mechanically_unreachable_from_ordinary_path(&self) -> bool {
        self.mechanically_unreachable_from_ordinary_path
    }
}
