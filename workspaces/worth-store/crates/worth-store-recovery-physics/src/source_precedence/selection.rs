use super::{
    PhysicalCheckpointBase, PhysicalRecoveryResidue, SelectedCompactionProduct,
    SelectedPhysicalPageFacts, SelectedPhysicalRoot, SelectedPhysicalWalTail,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSourceSelection {
    root: SelectedPhysicalRoot,
    page_facts: SelectedPhysicalPageFacts,
    checkpoint: Option<PhysicalCheckpointBase>,
    wal_tail: SelectedPhysicalWalTail,
    compaction: Option<SelectedCompactionProduct>,
    residue: Vec<PhysicalRecoveryResidue>,
    trace: PhysicalSourceSelectionTrace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalSourceSelectionTrace {
    root_role: super::SelectedPhysicalRootRole,
    current_rejected: bool,
    previous_rejected: bool,
    retained_previous: bool,
    checkpoint_selected: bool,
    wal_segments: u64,
    interrupted_wal_tail: bool,
    compaction_selected: bool,
    residue_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSourceSelectionDenial {
    WalRequiresCheckpoint,
    CompactionRequiresCheckpoint,
}

pub fn select_physical_recovery_sources(
    root: SelectedPhysicalRoot,
    page_facts: SelectedPhysicalPageFacts,
    checkpoint: Option<PhysicalCheckpointBase>,
    wal_tail: SelectedPhysicalWalTail,
    compaction: Option<SelectedCompactionProduct>,
    residue: Vec<PhysicalRecoveryResidue>,
) -> Result<PhysicalSourceSelection, PhysicalSourceSelectionDenial> {
    if checkpoint.is_none() && !wal_tail.segments().is_empty() {
        return Err(PhysicalSourceSelectionDenial::WalRequiresCheckpoint);
    }
    if checkpoint.is_none() && compaction.is_some() {
        return Err(PhysicalSourceSelectionDenial::CompactionRequiresCheckpoint);
    }
    let trace = PhysicalSourceSelectionTrace {
        root_role: root.role(),
        current_rejected: root.current_rejected(),
        previous_rejected: root.previous_rejected(),
        retained_previous: root.retained_previous().is_some(),
        checkpoint_selected: checkpoint.is_some(),
        wal_segments: wal_tail.segments().len() as u64,
        interrupted_wal_tail: wal_tail
            .segments()
            .last()
            .is_some_and(|segment| segment.interrupted_tail().is_some()),
        compaction_selected: compaction.is_some(),
        residue_count: residue.len() as u64,
    };
    Ok(PhysicalSourceSelection {
        root,
        page_facts,
        checkpoint,
        wal_tail,
        compaction,
        residue,
        trace,
    })
}

impl PhysicalSourceSelection {
    pub const fn root(&self) -> &SelectedPhysicalRoot {
        &self.root
    }

    pub const fn page_facts(&self) -> &SelectedPhysicalPageFacts {
        &self.page_facts
    }

    pub const fn checkpoint(&self) -> Option<&PhysicalCheckpointBase> {
        self.checkpoint.as_ref()
    }

    pub const fn wal_tail(&self) -> &SelectedPhysicalWalTail {
        &self.wal_tail
    }

    pub const fn compaction(&self) -> Option<SelectedCompactionProduct> {
        self.compaction
    }

    pub fn residue(&self) -> &[PhysicalRecoveryResidue] {
        &self.residue
    }

    pub const fn trace(&self) -> PhysicalSourceSelectionTrace {
        self.trace
    }
}

impl PhysicalSourceSelectionTrace {
    pub const fn root_role(self) -> super::SelectedPhysicalRootRole {
        self.root_role
    }
    pub const fn previous_rejected(self) -> bool {
        self.previous_rejected
    }
    pub const fn current_rejected(self) -> bool {
        self.current_rejected
    }
    pub const fn retained_previous(self) -> bool {
        self.retained_previous
    }
    pub const fn checkpoint_selected(self) -> bool {
        self.checkpoint_selected
    }
    pub const fn wal_segments(self) -> u64 {
        self.wal_segments
    }
    pub const fn interrupted_wal_tail(self) -> bool {
        self.interrupted_wal_tail
    }
    pub const fn compaction_selected(self) -> bool {
        self.compaction_selected
    }
    pub const fn residue_count(self) -> u64 {
        self.residue_count
    }
}
