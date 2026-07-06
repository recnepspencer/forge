use crate::{
    AdmittedBlobPlacement, BlobChunkReachabilityProofSet, BlobChunkRegisteredDedupeReference,
    BlobChunkRootPublication, BlobCorruptionGuard, LifecycleReceipt,
};
use forge_store_contracts::{S6BackgroundPressureDeclaration, S6BackgroundPressureKind};
use forge_store_physical_isolation::{
    CompactionReadInterlockDenial, CompactionReadInterlockPlan, StablePhysicalReadReceipt,
};
use forge_store_tiering::S7ColdPlacementState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionReadHold {
    Released(StablePhysicalReadReceipt),
    Active(StablePhysicalReadReceipt),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionS6Pacing {
    Admitted {
        declaration: S6BackgroundPressureDeclaration,
        foreground_yields: u64,
    },
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobCompactionColdReadiness {
    Available(S7ColdPlacementState),
    Unavailable(S7ColdPlacementState),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobCompactionPhysicalInterlock {
    Admitted(CompactionReadInterlockPlan),
    Denied(CompactionReadInterlockDenial),
}

#[derive(Debug)]
pub struct BlobCompactionIntent {
    lifecycle: LifecycleReceipt,
    uncompacted_publication: BlobChunkRootPublication,
    reachability: Option<BlobChunkReachabilityProofSet>,
    placement: AdmittedBlobPlacement,
    dedupe_references: Vec<BlobChunkRegisteredDedupeReference>,
    quarantine_holds: Vec<BlobCorruptionGuard>,
    read_hold: BlobCompactionReadHold,
    pacing: BlobCompactionS6Pacing,
    cold: BlobCompactionColdReadiness,
    physical: BlobCompactionPhysicalInterlock,
}

impl BlobCompactionIntent {
    pub fn for_published_generation(
        lifecycle: LifecycleReceipt,
        uncompacted_publication: BlobChunkRootPublication,
        reachability: BlobChunkReachabilityProofSet,
        placement: AdmittedBlobPlacement,
        read_hold: BlobCompactionReadHold,
        physical: CompactionReadInterlockPlan,
    ) -> Self {
        Self {
            lifecycle,
            uncompacted_publication,
            reachability: Some(reachability),
            placement,
            dedupe_references: Vec::new(),
            quarantine_holds: Vec::new(),
            read_hold,
            pacing: BlobCompactionS6Pacing::Admitted {
                declaration: S6BackgroundPressureDeclaration::compaction_rewrite(),
                foreground_yields: 0,
            },
            cold: BlobCompactionColdReadiness::Available(S7ColdPlacementState::HotAvailable),
            physical: BlobCompactionPhysicalInterlock::Admitted(physical),
        }
    }

    pub fn without_reachability(
        lifecycle: LifecycleReceipt,
        uncompacted_publication: BlobChunkRootPublication,
        placement: AdmittedBlobPlacement,
        read_hold: BlobCompactionReadHold,
        physical: CompactionReadInterlockPlan,
    ) -> Self {
        Self {
            lifecycle,
            uncompacted_publication,
            reachability: None,
            placement,
            dedupe_references: Vec::new(),
            quarantine_holds: Vec::new(),
            read_hold,
            pacing: BlobCompactionS6Pacing::Admitted {
                declaration: S6BackgroundPressureDeclaration::compaction_rewrite(),
                foreground_yields: 0,
            },
            cold: BlobCompactionColdReadiness::Available(S7ColdPlacementState::HotAvailable),
            physical: BlobCompactionPhysicalInterlock::Admitted(physical),
        }
    }

    pub fn with_read_hold(mut self, read_hold: BlobCompactionReadHold) -> Self {
        self.read_hold = read_hold;
        self
    }

    pub fn with_s6_pacing(mut self, pacing: BlobCompactionS6Pacing) -> Self {
        self.pacing = pacing;
        self
    }

    pub fn with_cold_readiness(mut self, cold: BlobCompactionColdReadiness) -> Self {
        self.cold = cold;
        self
    }

    pub fn with_dedupe_references(
        mut self,
        references: impl IntoIterator<Item = BlobChunkRegisteredDedupeReference>,
    ) -> Self {
        self.dedupe_references = references.into_iter().collect();
        self
    }

    pub fn with_quarantine_holds(
        mut self,
        holds: impl IntoIterator<Item = BlobCorruptionGuard>,
    ) -> Self {
        self.quarantine_holds = holds.into_iter().collect();
        self
    }

    pub fn with_physical_interlock_denial(mut self, denial: CompactionReadInterlockDenial) -> Self {
        self.physical = BlobCompactionPhysicalInterlock::Denied(denial);
        self
    }

    pub(crate) fn lifecycle(&self) -> &LifecycleReceipt {
        &self.lifecycle
    }

    pub(crate) const fn uncompacted_publication(&self) -> &BlobChunkRootPublication {
        &self.uncompacted_publication
    }

    pub(crate) fn reachability(&self) -> Option<&BlobChunkReachabilityProofSet> {
        self.reachability.as_ref()
    }

    pub(crate) const fn placement(&self) -> &AdmittedBlobPlacement {
        &self.placement
    }

    pub(crate) fn dedupe_references(&self) -> &[BlobChunkRegisteredDedupeReference] {
        &self.dedupe_references
    }

    pub(crate) fn quarantine_holds(&self) -> &[BlobCorruptionGuard] {
        &self.quarantine_holds
    }

    pub(crate) const fn read_hold(&self) -> BlobCompactionReadHold {
        self.read_hold
    }

    pub(crate) const fn pacing(&self) -> BlobCompactionS6Pacing {
        self.pacing
    }

    pub(crate) const fn cold(&self) -> BlobCompactionColdReadiness {
        self.cold
    }

    pub(crate) const fn physical(&self) -> &BlobCompactionPhysicalInterlock {
        &self.physical
    }
}

impl BlobCompactionPhysicalInterlock {
    pub(crate) const fn admitted(&self) -> Option<&CompactionReadInterlockPlan> {
        match self {
            Self::Admitted(plan) => Some(plan),
            Self::Denied(_) => None,
        }
    }

    pub(crate) const fn denial(&self) -> Option<CompactionReadInterlockDenial> {
        match self {
            Self::Admitted(_) => None,
            Self::Denied(denial) => Some(*denial),
        }
    }
}

impl BlobCompactionReadHold {
    pub const fn released(receipt: StablePhysicalReadReceipt) -> Self {
        Self::Released(receipt)
    }

    pub const fn active(receipt: StablePhysicalReadReceipt) -> Self {
        Self::Active(receipt)
    }

    pub const fn is_active(self) -> bool {
        matches!(self, Self::Active(_))
    }

    pub(crate) const fn released_receipt(self) -> Option<StablePhysicalReadReceipt> {
        match self {
            Self::Released(receipt) => Some(receipt),
            Self::Active(_) => None,
        }
    }
}

impl BlobCompactionS6Pacing {
    pub const fn admitted_compaction(foreground_yields: u64) -> Self {
        Self::Admitted {
            declaration: S6BackgroundPressureDeclaration::compaction_rewrite(),
            foreground_yields,
        }
    }

    pub const fn supports_compaction(self) -> bool {
        match self {
            Self::Admitted { declaration, .. } => {
                matches!(
                    declaration.kind(),
                    S6BackgroundPressureKind::CompactionRewrite
                )
            }
            Self::Unsupported => false,
        }
    }

    pub const fn foreground_yields(self) -> u64 {
        match self {
            Self::Admitted {
                foreground_yields, ..
            } => foreground_yields,
            Self::Unsupported => 0,
        }
    }
}

impl BlobCompactionColdReadiness {
    pub const fn from_state(state: S7ColdPlacementState) -> Self {
        if matches!(
            state,
            S7ColdPlacementState::HotAvailable | S7ColdPlacementState::ColdAvailable
        ) {
            Self::Available(state)
        } else {
            Self::Unavailable(state)
        }
    }

    pub const fn permits_compaction(self) -> bool {
        matches!(self, Self::Available(_))
    }

    pub const fn state(self) -> S7ColdPlacementState {
        match self {
            Self::Available(state) | Self::Unavailable(state) => state,
        }
    }
}
