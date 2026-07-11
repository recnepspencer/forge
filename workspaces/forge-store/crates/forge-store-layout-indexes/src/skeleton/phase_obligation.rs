#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PhaseSkeletonObligationRow {
    phase_number: u8,
    owning_crate: &'static str,
    owning_module_path: &'static str,
    public_facade_path: &'static str,
    consumed_authority: &'static str,
    minted_authority: &'static str,
    courtroom_boundary: &'static str,
    shortcut_proof: &'static str,
}

const OBLIGATIONS: &[S8PhaseSkeletonObligationRow] = &[
    S8PhaseSkeletonObligationRow::new(
        0,
        "forge-store-layout-indexes",
        "forge_store_layout_indexes",
        "forge_store_layout_indexes::{layout_families,layout_strategy_admission,access_planning,access_lowering,access_execution,layout_rebuild,layout_migration,layout_counters,layout_readmission,layout_customization,layout_closeout,layout_certification}",
        "existing Store family authority",
        "S.8 topology, responsibility, facade, and witness contract",
        "certification/test-support/offline/foundational/terminal surfaces cannot mint production authority",
        "external-crate UI compile-fail proof",
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8PhaseSkeletonObligation;

impl S8PhaseSkeletonObligationRow {
    pub const fn new(
        phase_number: u8,
        owning_crate: &'static str,
        owning_module_path: &'static str,
        public_facade_path: &'static str,
        consumed_authority: &'static str,
        minted_authority: &'static str,
        courtroom_boundary: &'static str,
        shortcut_proof: &'static str,
    ) -> Self {
        Self {
            phase_number,
            owning_crate,
            owning_module_path,
            public_facade_path,
            consumed_authority,
            minted_authority,
            courtroom_boundary,
            shortcut_proof,
        }
    }

    pub const fn phase_number(&self) -> u8 {
        self.phase_number
    }

    pub const fn owning_crate(&self) -> &'static str {
        self.owning_crate
    }

    pub const fn owning_module_path(&self) -> &'static str {
        self.owning_module_path
    }

    pub const fn public_facade_path(&self) -> &'static str {
        self.public_facade_path
    }

    pub const fn consumed_authority(&self) -> &'static str {
        self.consumed_authority
    }

    pub const fn minted_authority(&self) -> &'static str {
        self.minted_authority
    }

    pub const fn courtroom_boundary(&self) -> &'static str {
        self.courtroom_boundary
    }

    pub const fn shortcut_proof(&self) -> &'static str {
        self.shortcut_proof
    }
}

impl S8PhaseSkeletonObligation {
    pub const fn for_phase(phase: u8) -> &'static [S8PhaseSkeletonObligationRow] {
        if phase == 0 {
            OBLIGATIONS
        } else {
            &[]
        }
    }
}
