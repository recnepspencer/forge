use worth_store_contracts::StableDigest;

use crate::BlobReachabilityEdgeKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlobRetentionHoldKind {
    Generation,
    TimeWindow,
    Export,
    Capsule,
    Quarantine,
    ReadPlan,
    Checkpoint,
    TenantCustody,
    ResumeSession,
    PlacementMove,
    Backup,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobRetentionHold {
    kind: BlobRetentionHoldKind,
    identity: StableDigest,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BlobRetentionHoldSet {
    holds: Vec<BlobRetentionHold>,
}

impl BlobRetentionHold {
    pub fn generation(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::Generation, identity_basis)
    }

    pub fn time_window(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::TimeWindow, identity_basis)
    }

    pub fn export(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::Export, identity_basis)
    }

    pub fn capsule(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::Capsule, identity_basis)
    }

    pub fn quarantine(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::Quarantine, identity_basis)
    }

    pub fn read_plan(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::ReadPlan, identity_basis)
    }

    pub fn checkpoint(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::Checkpoint, identity_basis)
    }

    pub fn tenant_custody(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::TenantCustody, identity_basis)
    }

    pub fn resume_session(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::ResumeSession, identity_basis)
    }

    pub fn placement_move(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::PlacementMove, identity_basis)
    }

    pub fn backup(identity_basis: impl AsRef<str>) -> Self {
        Self::new(BlobRetentionHoldKind::Backup, identity_basis)
    }

    pub fn from_reachability_hold_kind(
        kind: BlobReachabilityEdgeKind,
        identity_basis: impl AsRef<str>,
    ) -> Self {
        let kind = match kind {
            BlobReachabilityEdgeKind::GenerationHoldReference => BlobRetentionHoldKind::Generation,
            BlobReachabilityEdgeKind::TimeWindowHoldReference => BlobRetentionHoldKind::TimeWindow,
            BlobReachabilityEdgeKind::CheckpointHoldReference => BlobRetentionHoldKind::Checkpoint,
            BlobReachabilityEdgeKind::BackupHoldReference => BlobRetentionHoldKind::Backup,
            BlobReachabilityEdgeKind::ExportHoldReference => BlobRetentionHoldKind::Export,
            BlobReachabilityEdgeKind::TenantCustodyHoldReference => {
                BlobRetentionHoldKind::TenantCustody
            }
            BlobReachabilityEdgeKind::ReplicationCapsuleReference => BlobRetentionHoldKind::Capsule,
            BlobReachabilityEdgeKind::ReadPlanHoldReference => BlobRetentionHoldKind::ReadPlan,
            BlobReachabilityEdgeKind::QuarantineHoldReference => BlobRetentionHoldKind::Quarantine,
            BlobReachabilityEdgeKind::PlacementMoveReference => {
                BlobRetentionHoldKind::PlacementMove
            }
            BlobReachabilityEdgeKind::ResumeSessionReference => {
                BlobRetentionHoldKind::ResumeSession
            }
            _ => BlobRetentionHoldKind::Generation,
        };
        Self::new(kind, identity_basis)
    }

    fn new(kind: BlobRetentionHoldKind, identity_basis: impl AsRef<str>) -> Self {
        let identity = StableDigest::new(format!(
            "s7.retention.hold:{:?}:{}",
            kind,
            identity_basis.as_ref()
        ))
        .expect("retention hold identity is nonempty");
        Self { kind, identity }
    }

    pub const fn kind(&self) -> BlobRetentionHoldKind {
        self.kind
    }

    pub const fn identity(&self) -> &StableDigest {
        &self.identity
    }
}

impl BlobRetentionHoldSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_holds(holds: impl IntoIterator<Item = BlobRetentionHold>) -> Self {
        let mut set = Self::new();
        for hold in holds {
            set.insert(hold);
        }
        set
    }

    pub fn insert(&mut self, hold: BlobRetentionHold) {
        if self
            .holds
            .iter()
            .any(|existing| existing.identity() == hold.identity())
        {
            return;
        }
        self.holds.push(hold);
        self.holds.sort_by(|left, right| {
            left.kind()
                .cmp(&right.kind())
                .then(left.identity().as_str().cmp(right.identity().as_str()))
        });
    }

    pub fn first_blocking_hold(&self) -> Option<&BlobRetentionHold> {
        self.holds.first()
    }

    pub fn holds(&self) -> &[BlobRetentionHold] {
        &self.holds
    }

    pub fn is_empty(&self) -> bool {
        self.holds.is_empty()
    }
}
