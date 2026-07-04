use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphCrossFamilyCloseoutMatrixRow {
    family_kind: TouchedGraphParityFamilyKind,
    covered_surface_count: usize,
    representative_path_covered: bool,
    declare_once_parity_passed: bool,
    public_proof_parity_passed: bool,
    diagnostic_parity_passed: bool,
    readiness_handoff_passed: bool,
    deleted_count: usize,
    capped_residue_count: usize,
    query_gap_count: usize,
    blocked_outside_roadmap_count: usize,
}

impl WorthTouchedGraphCrossFamilyCloseoutMatrixRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        family_kind: TouchedGraphParityFamilyKind,
        covered_surface_count: usize,
        representative_path_covered: bool,
        declare_once_parity_passed: bool,
        public_proof_parity_passed: bool,
        diagnostic_parity_passed: bool,
        readiness_handoff_passed: bool,
        deleted_count: usize,
        capped_residue_count: usize,
        query_gap_count: usize,
        blocked_outside_roadmap_count: usize,
    ) -> Self {
        Self {
            family_kind,
            covered_surface_count,
            representative_path_covered,
            declare_once_parity_passed,
            public_proof_parity_passed,
            diagnostic_parity_passed,
            readiness_handoff_passed,
            deleted_count,
            capped_residue_count,
            query_gap_count,
            blocked_outside_roadmap_count,
        }
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn covered_surface_count(&self) -> usize {
        self.covered_surface_count
    }

    pub const fn representative_path_covered(&self) -> bool {
        self.representative_path_covered
    }

    pub const fn declare_once_parity_passed(&self) -> bool {
        self.declare_once_parity_passed
    }

    pub const fn public_proof_parity_passed(&self) -> bool {
        self.public_proof_parity_passed
    }

    pub const fn diagnostic_parity_passed(&self) -> bool {
        self.diagnostic_parity_passed
    }

    pub const fn readiness_handoff_passed(&self) -> bool {
        self.readiness_handoff_passed
    }

    pub const fn deleted_count(&self) -> usize {
        self.deleted_count
    }

    pub const fn capped_residue_count(&self) -> usize {
        self.capped_residue_count
    }

    pub const fn query_gap_count(&self) -> usize {
        self.query_gap_count
    }

    pub const fn blocked_outside_roadmap_count(&self) -> usize {
        self.blocked_outside_roadmap_count
    }

    pub const fn total_certified_rows(&self) -> usize {
        self.covered_surface_count
            + self.deleted_count
            + self.capped_residue_count
            + self.query_gap_count
            + self.blocked_outside_roadmap_count
    }

    pub const fn is_covered_family(&self) -> bool {
        self.covered_surface_count > 0
    }
}
