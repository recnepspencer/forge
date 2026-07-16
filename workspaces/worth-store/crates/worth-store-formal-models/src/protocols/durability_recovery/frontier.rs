use super::{DurabilityRecoveryAction, DurabilityRecoveryDenial};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WalFrontierState {
    Absent,
    Proposed,
    CompletedInMemory,
    FenceRequested,
    FenceCompleted,
    Acknowledged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageFrontierState {
    Clean,
    FlushRequested,
    Durable,
    DurabilityUncertain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointFrontierState {
    Absent,
    Begun,
    Durable,
    Published,
    Selected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DirectorySyncFrontierState {
    Absent,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayFrontierState {
    Absent,
    Required,
    Applied,
    SkippedIdempotent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveredRootFrontierState {
    Absent,
    Pending,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DurabilityRecoveryFrontier {
    wal: WalFrontierState,
    page: PageFrontierState,
    checkpoint: CheckpointFrontierState,
    directory_sync: DirectorySyncFrontierState,
    replay: ReplayFrontierState,
    recovered_root: RecoveredRootFrontierState,
    crashed: bool,
}

impl DurabilityRecoveryFrontier {
    pub const fn initial() -> Self {
        Self {
            wal: WalFrontierState::Absent,
            page: PageFrontierState::Clean,
            checkpoint: CheckpointFrontierState::Absent,
            directory_sync: DirectorySyncFrontierState::Absent,
            replay: ReplayFrontierState::Absent,
            recovered_root: RecoveredRootFrontierState::Absent,
            crashed: false,
        }
    }

    pub fn apply(
        &mut self,
        action: DurabilityRecoveryAction,
    ) -> Result<(), DurabilityRecoveryDenial> {
        use DurabilityRecoveryAction as Action;
        if self.crashed && action != Action::Reopen {
            return Err(DurabilityRecoveryDenial::IllegalTransition);
        }
        match action {
            Action::WalAppendProposed if self.wal == WalFrontierState::Absent => {
                self.wal = WalFrontierState::Proposed;
            }
            Action::WalAppendCompletedInMemory if self.wal == WalFrontierState::Proposed => {
                self.wal = WalFrontierState::CompletedInMemory;
            }
            Action::WalFenceRequested if self.wal == WalFrontierState::CompletedInMemory => {
                self.wal = WalFrontierState::FenceRequested;
            }
            Action::WalFenceCompleted if self.wal == WalFrontierState::FenceRequested => {
                self.wal = WalFrontierState::FenceCompleted;
            }
            Action::WalAcknowledgmentLegal if self.wal == WalFrontierState::FenceCompleted => {
                self.wal = WalFrontierState::Acknowledged;
            }
            Action::WalAcknowledgmentLegal => {
                return Err(DurabilityRecoveryDenial::AmbiguousWalDurability);
            }
            Action::PageFlushRequested if self.page == PageFrontierState::Clean => {
                self.page = PageFrontierState::FlushRequested;
            }
            Action::PageFlushCompleted if self.wal != WalFrontierState::Acknowledged => {
                return Err(DurabilityRecoveryDenial::PageFlushAheadOfWal);
            }
            Action::PageFlushCompleted if self.page == PageFrontierState::FlushRequested => {
                self.page = PageFrontierState::Durable;
            }
            Action::PageFlushDurabilityUncertain
                if self.page == PageFrontierState::FlushRequested =>
            {
                self.page = PageFrontierState::DurabilityUncertain;
            }
            Action::CheckpointBegun if self.checkpoint == CheckpointFrontierState::Absent => {
                self.checkpoint = CheckpointFrontierState::Begun;
            }
            Action::CheckpointDurable
                if self.wal != WalFrontierState::Acknowledged
                    || self.page != PageFrontierState::Durable =>
            {
                return Err(DurabilityRecoveryDenial::CheckpointFrontierNotDurable);
            }
            Action::CheckpointDurable if self.checkpoint == CheckpointFrontierState::Begun => {
                self.checkpoint = CheckpointFrontierState::Durable;
            }
            Action::DirectorySyncCompleted
                if self.checkpoint == CheckpointFrontierState::Durable =>
            {
                self.directory_sync = DirectorySyncFrontierState::Completed;
            }
            Action::DirectorySyncFailed if self.checkpoint == CheckpointFrontierState::Durable => {
                self.directory_sync = DirectorySyncFrontierState::Failed;
            }
            Action::CheckpointPublished
                if self.checkpoint == CheckpointFrontierState::Durable
                    && self.directory_sync == DirectorySyncFrontierState::Completed =>
            {
                self.checkpoint = CheckpointFrontierState::Published;
            }
            Action::CheckpointPublished => {
                return Err(DurabilityRecoveryDenial::DirectorySyncNotDurable);
            }
            Action::CheckpointSelected if self.checkpoint == CheckpointFrontierState::Published => {
                self.checkpoint = CheckpointFrontierState::Selected;
            }
            Action::CheckpointSelected => {
                return Err(DurabilityRecoveryDenial::CheckpointFrontierNotDurable);
            }
            Action::RecoveryReplayRequired
                if self.checkpoint == CheckpointFrontierState::Selected
                    && self.replay == ReplayFrontierState::Absent =>
            {
                self.replay = ReplayFrontierState::Required;
            }
            Action::RecoveryReplayRejectedGenerationMismatch => {
                return Err(DurabilityRecoveryDenial::RedoGenerationMismatch);
            }
            Action::RecoveryReplayApplied if self.replay == ReplayFrontierState::Required => {
                self.replay = ReplayFrontierState::Applied;
            }
            Action::RecoveryReplaySkippedIdempotent
                if self.replay == ReplayFrontierState::Required =>
            {
                self.replay = ReplayFrontierState::SkippedIdempotent;
            }
            Action::RecoveryReplayRequired
            | Action::RecoveryReplayApplied
            | Action::RecoveryReplaySkippedIdempotent => {
                return Err(DurabilityRecoveryDenial::RecoveryBasisNotSelected);
            }
            Action::RecoveredRootPublicationPending
                if self.checkpoint == CheckpointFrontierState::Selected
                    && self.recovered_root == RecoveredRootFrontierState::Absent =>
            {
                self.recovered_root = RecoveredRootFrontierState::Pending;
            }
            Action::RecoveredRootPublicationPending => {
                return Err(DurabilityRecoveryDenial::RecoveryBasisNotSelected);
            }
            Action::RecoveredRootPublicationCompleted
                if self.recovered_root == RecoveredRootFrontierState::Pending
                    && matches!(
                        self.replay,
                        ReplayFrontierState::Applied | ReplayFrontierState::SkippedIdempotent
                    ) =>
            {
                self.recovered_root = RecoveredRootFrontierState::Completed;
            }
            Action::RecoveredRootPublicationCompleted => {
                return Err(DurabilityRecoveryDenial::ReplayNotResolved);
            }
            Action::Crash => self.crashed = true,
            Action::Reopen if self.crashed => self.reopen(),
            _ => return Err(DurabilityRecoveryDenial::IllegalTransition),
        }
        Ok(())
    }

    fn reopen(&mut self) {
        self.crashed = false;
        self.wal = match self.wal {
            WalFrontierState::FenceCompleted | WalFrontierState::Acknowledged => self.wal,
            _ => WalFrontierState::Absent,
        };
        self.page = match self.page {
            PageFrontierState::Durable => PageFrontierState::Durable,
            _ => PageFrontierState::Clean,
        };
        if self.checkpoint == CheckpointFrontierState::Begun {
            self.checkpoint = CheckpointFrontierState::Absent;
        }
        if self.directory_sync == DirectorySyncFrontierState::Failed {
            self.directory_sync = DirectorySyncFrontierState::Absent;
        }
        if self.recovered_root != RecoveredRootFrontierState::Completed {
            self.recovered_root = RecoveredRootFrontierState::Absent;
            self.replay = ReplayFrontierState::Absent;
        }
    }

    pub const fn wal_acknowledged(self) -> bool {
        matches!(self.wal, WalFrontierState::Acknowledged)
    }

    pub const fn root_publication_pending(self) -> bool {
        matches!(self.recovered_root, RecoveredRootFrontierState::Pending)
    }

    pub const fn wal_state(self) -> WalFrontierState {
        self.wal
    }

    pub const fn checkpoint_state(self) -> CheckpointFrontierState {
        self.checkpoint
    }

    pub const fn recovered_root_state(self) -> RecoveredRootFrontierState {
        self.recovered_root
    }

    pub const fn is_crashed(self) -> bool {
        self.crashed
    }
}
