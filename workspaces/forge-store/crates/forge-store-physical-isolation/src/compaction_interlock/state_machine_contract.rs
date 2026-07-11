//! Sealed compaction-cutover cases emitted by ordinary physical operations.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompactionCutoverState {
    PlanAdmitted,
    RewriteLowered,
    LsmTombstoneRetentionAdmitted,
    PublicationCommitted,
    RecoveryVisibilityAdmitted,
    ReclaimDeferred,
    Reclaimed,
    Denied,
}

macro_rules! compaction_outcome_cases {
    ($( $case:ident: $from:ident => $to:ident ),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum CompactionCutoverTransitionKind { $($case),+ }

        impl CompactionCutoverTransitionKind {
            pub(super) fn all() -> impl ExactSizeIterator<Item = Self> {
                [$(Self::$case),+].into_iter()
            }

            pub(super) const fn transition(self) -> CompactionCutoverTransition {
                match self {
                    $(Self::$case => CompactionCutoverTransition::new(
                        CompactionCutoverState::$from,
                        Self::$case,
                        CompactionCutoverState::$to,
                    )),+
                }
            }
        }
    };
}

compaction_outcome_cases!(
    LowerRewrite: PlanAdmitted => RewriteLowered,
    AdmitLsmTombstoneRetention: RewriteLowered => LsmTombstoneRetentionAdmitted,
    PublishRewrite: LsmTombstoneRetentionAdmitted => PublicationCommitted,
    DenyLsmPhysicalTarget: RewriteLowered => Denied,
    AdmitRecoveryVisibility: PublicationCommitted => RecoveryVisibilityAdmitted,
    DeferReclaim: PublicationCommitted => ReclaimDeferred,
    DrainReclaimAfterReadRelease: ReclaimDeferred => Reclaimed,
    DenyInPlaceOverwrite: PlanAdmitted => Denied,
    DenyEarlyReclaim: ReclaimDeferred => Denied,
    DenyStaleEpochReuse: PlanAdmitted => Denied,
    DenyBackendResidue: PublicationCommitted => Denied,
    DenyLatchHierarchyInversion: PlanAdmitted => Denied,
    DenyMixedRootRead: PublicationCommitted => Denied,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompactionCutoverTransition {
    from: CompactionCutoverState,
    kind: CompactionCutoverTransitionKind,
    to: CompactionCutoverState,
}

impl CompactionCutoverTransition {
    const fn new(
        from: CompactionCutoverState,
        kind: CompactionCutoverTransitionKind,
        to: CompactionCutoverState,
    ) -> Self {
        Self { from, kind, to }
    }

    pub const fn from(self) -> CompactionCutoverState {
        self.from
    }
    pub const fn kind(self) -> CompactionCutoverTransitionKind {
        self.kind
    }
    pub const fn to(self) -> CompactionCutoverState {
        self.to
    }
}

/// Complete owner case inventory generated from the same declaration used by
/// every ordinary compaction outcome.
pub fn compaction_cutover_outcome_facts(
) -> impl ExactSizeIterator<Item = CompactionCutoverTransition> {
    CompactionCutoverTransitionKind::all().map(CompactionCutoverTransitionKind::transition)
}
