use super::{
    CompactionCandidateRangeSet, CompactionCutoverStabilityProof, CompactionDeferredReclaimQueue,
    CompactionProtectedReferenceSet, CompactionReadInterlockCounters,
    CompactionReadInterlockDenial, CompactionReadInterlockPlan, CompactionRewritePublication,
    CompactionSourceIntegrityEvidence,
};
use crate::{LatchAcquisitionDenial, LatchDeniedBeforeWaitEvidence, RootEpoch};
use forge_store_recovery_physics::CompactionCutoverRecoveryPosture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionMutationLaneReceipt {
    kind: CompactionMutationLaneReceiptKind,
    denial: CompactionReadInterlockDenial,
    origin: CompactionMutationLaneOrigin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompactionMutationLaneOrigin {
    protected: CompactionProtectedReferenceSet,
    candidates: CompactionCandidateRangeSet,
    source_epoch: RootEpoch,
    target_epoch: RootEpoch,
}

macro_rules! define_compaction_mutation_outcomes {
    ($( $variant:ident => $fact:ident ),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum CompactionMutationLaneReceiptKind {
            $($variant),+
        }

        impl CompactionMutationLaneReceiptKind {
            const fn production_transition(self) -> super::CompactionCutoverTransition {
                match self {
                    $(Self::$variant => super::CompactionCutoverTransitionKind::$fact.transition()),+
                }
            }
        }
    };
}

define_compaction_mutation_outcomes!(
    InPlaceOverwriteDenied => DenyInPlaceOverwrite,
    EarlyReclaimDenied => DenyEarlyReclaim,
    StaleEpochReuseDenied => DenyStaleEpochReuse,
    BackendResidueCandidateSelectionDenied => DenyBackendResidue,
    LatchHierarchyInversionDenied => DenyLatchHierarchyInversion,
    MixedRootReadDenied => DenyMixedRootRead,
);

impl CompactionMutationLaneReceipt {
    pub fn from_in_place_overwrite_denial(
        plan: CompactionReadInterlockPlan,
    ) -> (Self, CompactionReadInterlockCounters) {
        let origin = CompactionMutationLaneOrigin::from_plan(&plan);
        let (denial, counters) = plan.deny_in_place_overwrite();
        (
            Self {
                kind: CompactionMutationLaneReceiptKind::InPlaceOverwriteDenied,
                denial,
                origin,
            },
            counters,
        )
    }

    pub fn from_early_reclaim_denial(
        queue: &CompactionDeferredReclaimQueue,
    ) -> (Self, CompactionReadInterlockCounters) {
        let (denial, counters) = queue.reject_early_reclaim();
        (
            Self {
                kind: CompactionMutationLaneReceiptKind::EarlyReclaimDenied,
                denial,
                origin: CompactionMutationLaneOrigin::from_plan(queue.publication().delta().plan()),
            },
            counters,
        )
    }

    pub fn from_stale_epoch_admission_denial(
        expected_plan: &CompactionReadInterlockPlan,
        source_epoch: RootEpoch,
        target_epoch: RootEpoch,
        source_evidence: CompactionSourceIntegrityEvidence,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        let origin = CompactionMutationLaneOrigin::from_plan(expected_plan);
        match CompactionReadInterlockPlan::admit(
            expected_plan.protected().clone(),
            expected_plan.candidates().clone(),
            source_epoch,
            target_epoch,
            source_evidence,
        ) {
            Ok(_) => Err(CompactionReadInterlockDenial::ExpectedMutationLaneDenialNotProduced),
            Err(
                denial @ (CompactionReadInterlockDenial::StaleCompactionSourceEpoch { .. }
                | CompactionReadInterlockDenial::StaleEpochReuse { .. }),
            ) => Ok(Self {
                kind: CompactionMutationLaneReceiptKind::StaleEpochReuseDenied,
                denial,
                origin,
            }),
            Err(denial) => Err(denial),
        }
    }

    pub fn from_backend_residue_denial(
        publication: CompactionRewritePublication,
        recovery_posture: CompactionCutoverRecoveryPosture,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        let origin = CompactionMutationLaneOrigin::from_plan(publication.delta().plan());
        match CompactionCutoverStabilityProof::admit(publication, recovery_posture) {
            Ok(_) => Err(CompactionReadInterlockDenial::ExpectedMutationLaneDenialNotProduced),
            Err(denial @ CompactionReadInterlockDenial::BackendResidueCandidateSelection(_)) => {
                Ok(Self {
                    kind: CompactionMutationLaneReceiptKind::BackendResidueCandidateSelectionDenied,
                    denial,
                    origin,
                })
            }
            Err(denial) => Err(denial),
        }
    }

    pub fn from_latch_hierarchy_inversion_denial(
        plan: &CompactionReadInterlockPlan,
        evidence: LatchDeniedBeforeWaitEvidence,
    ) -> Result<Self, CompactionReadInterlockDenial> {
        if evidence.denial() != LatchAcquisitionDenial::HierarchyInversion {
            return Err(CompactionReadInterlockDenial::ExpectedMutationLaneDenialNotProduced);
        }
        Ok(Self {
            kind: CompactionMutationLaneReceiptKind::LatchHierarchyInversionDenied,
            denial: CompactionReadInterlockDenial::LatchAcquisition(evidence.denial()),
            origin: CompactionMutationLaneOrigin::from_plan(plan),
        })
    }

    pub fn from_mixed_root_read_denial(plan: &CompactionReadInterlockPlan) -> Self {
        Self {
            kind: CompactionMutationLaneReceiptKind::MixedRootReadDenied,
            denial: CompactionReadInterlockDenial::MixedRootDuringCompaction,
            origin: CompactionMutationLaneOrigin::from_plan(plan),
        }
    }

    pub const fn kind(&self) -> CompactionMutationLaneReceiptKind {
        self.kind
    }

    pub const fn cutover_transition_kind(&self) -> super::CompactionCutoverTransitionKind {
        match self.kind {
            CompactionMutationLaneReceiptKind::InPlaceOverwriteDenied => {
                super::CompactionCutoverTransitionKind::DenyInPlaceOverwrite
            }
            CompactionMutationLaneReceiptKind::EarlyReclaimDenied => {
                super::CompactionCutoverTransitionKind::DenyEarlyReclaim
            }
            CompactionMutationLaneReceiptKind::StaleEpochReuseDenied => {
                super::CompactionCutoverTransitionKind::DenyStaleEpochReuse
            }
            CompactionMutationLaneReceiptKind::BackendResidueCandidateSelectionDenied => {
                super::CompactionCutoverTransitionKind::DenyBackendResidue
            }
            CompactionMutationLaneReceiptKind::LatchHierarchyInversionDenied => {
                super::CompactionCutoverTransitionKind::DenyLatchHierarchyInversion
            }
            CompactionMutationLaneReceiptKind::MixedRootReadDenied => {
                super::CompactionCutoverTransitionKind::DenyMixedRootRead
            }
        }
    }

    pub const fn cutover_transition(&self) -> super::CompactionCutoverTransition {
        self.kind.production_transition()
    }

    pub const fn denial(&self) -> CompactionReadInterlockDenial {
        self.denial
    }

    pub const fn origin(&self) -> &CompactionMutationLaneOrigin {
        &self.origin
    }
}

impl CompactionMutationLaneOrigin {
    pub fn from_plan(plan: &CompactionReadInterlockPlan) -> Self {
        Self::from_parts(
            plan.protected(),
            plan.candidates(),
            plan.source_epoch(),
            plan.target_epoch(),
        )
    }

    fn from_parts(
        protected: &CompactionProtectedReferenceSet,
        candidates: &CompactionCandidateRangeSet,
        source_epoch: RootEpoch,
        target_epoch: RootEpoch,
    ) -> Self {
        Self {
            protected: protected.clone(),
            candidates: candidates.clone(),
            source_epoch,
            target_epoch,
        }
    }

    pub const fn protected(&self) -> &CompactionProtectedReferenceSet {
        &self.protected
    }

    pub const fn candidates(&self) -> &CompactionCandidateRangeSet {
        &self.candidates
    }

    pub const fn source_epoch(&self) -> RootEpoch {
        self.source_epoch
    }

    pub const fn target_epoch(&self) -> RootEpoch {
        self.target_epoch
    }
}
