use forge_store_physical_backend::{BackendDurabilityProfile, BackendDurabilityProfileId};
use forge_store_physical_format::PageGenerationCell;

use crate::LogSequenceNumber;

use super::{
    PageLsn, PageLsnPublicationCounterSnapshot, RollbackImagePublicationDeclaration,
    RollbackImagePublicationPosture, UnadmittedDirtyPagePublicationDenial,
    WalBeforeDataOrderingProof,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoUndoPublicationProof<P: BackendDurabilityProfile> {
    ordering: WalBeforeDataOrderingProof<P>,
    rollback_posture: RollbackImagePublicationPosture,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NoUndoPublicationAuthority {
    _sealed: (),
}

impl NoUndoPublicationAuthority {
    const fn recovery_redo_only() -> Self {
        Self { _sealed: () }
    }

    fn admit_redo_only_mutation<P: BackendDurabilityProfile>(
        self,
        ordering: WalBeforeDataOrderingProof<P>,
    ) -> NoUndoPublicationProof<P> {
        NoUndoPublicationProof::admitted_redo_only_mutation(ordering)
    }

    fn admit_rollback_image_protected<P: BackendDurabilityProfile>(
        self,
        ordering: WalBeforeDataOrderingProof<P>,
        declaration: RollbackImagePublicationDeclaration,
    ) -> Result<NoUndoPublicationProof<P>, UnadmittedDirtyPagePublicationDenial> {
        NoUndoPublicationProof::rollback_image_protected(ordering, declaration)
    }
}

impl<P: BackendDurabilityProfile> NoUndoPublicationProof<P> {
    pub fn deny_missing_required_rollback_image(
        ordering: WalBeforeDataOrderingProof<P>,
    ) -> Result<Self, UnadmittedDirtyPagePublicationDenial> {
        NoUndoPublicationProof::missing_required_rollback_image(ordering)
    }

    pub(crate) fn admitted_redo_only_mutation(ordering: WalBeforeDataOrderingProof<P>) -> Self {
        Self {
            ordering,
            rollback_posture:
                RollbackImagePublicationPosture::NotRequiredForAdmittedRedoOnlyMutation,
        }
    }

    pub(crate) fn rollback_image_protected(
        ordering: WalBeforeDataOrderingProof<P>,
        declaration: RollbackImagePublicationDeclaration,
    ) -> Result<Self, UnadmittedDirtyPagePublicationDenial> {
        if declaration.dirty_identity() != ordering.evidence().dirty_identity()
            || declaration.page_generation() != ordering.page_generation()
            || declaration.page_lsn() != ordering.page_lsn()
        {
            return Err(
                UnadmittedDirtyPagePublicationDenial::rollback_image_declaration_mismatch(
                    P::ID,
                    ordering.evidence().dirty_identity(),
                    ordering.page_generation(),
                    declaration.page_generation(),
                    ordering.page_lsn(),
                    ordering.wal_frontier(),
                    ordering.counters().with_no_undo_denial(),
                ),
            );
        }
        Ok(Self {
            ordering,
            rollback_posture: RollbackImagePublicationPosture::RollbackImageProtectsUnadmittedBytes,
        })
    }

    pub(crate) fn missing_required_rollback_image(
        ordering: WalBeforeDataOrderingProof<P>,
    ) -> Result<Self, UnadmittedDirtyPagePublicationDenial> {
        Err(
            UnadmittedDirtyPagePublicationDenial::rollback_image_required(
                P::ID,
                ordering.evidence().dirty_identity(),
                ordering.page_generation(),
                ordering.page_lsn(),
                ordering.wal_frontier(),
                ordering.counters().with_no_undo_denial(),
            ),
        )
    }

    const fn ordering(&self) -> &WalBeforeDataOrderingProof<P> {
        &self.ordering
    }

    const fn rollback_posture(&self) -> RollbackImagePublicationPosture {
        self.rollback_posture
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoUndoPublicationEligibility {
    profile_id: BackendDurabilityProfileId,
    page_generation: PageGenerationCell,
    page_lsn: PageLsn,
    wal_frontier: LogSequenceNumber,
    rollback_posture: RollbackImagePublicationPosture,
    counters: PageLsnPublicationCounterSnapshot,
}

impl NoUndoPublicationEligibility {
    pub fn redo_only<P: BackendDurabilityProfile>(proof: NoUndoPublicationProof<P>) -> Self {
        let ordering = proof.ordering();
        Self {
            profile_id: P::ID,
            page_generation: ordering.page_generation(),
            page_lsn: ordering.page_lsn(),
            wal_frontier: ordering.wal_frontier(),
            rollback_posture: proof.rollback_posture(),
            counters: ordering.counters().with_no_undo_eligibility(),
        }
    }

    pub(crate) fn admitted_dirty_publication<P: BackendDurabilityProfile>(
        ordering: WalBeforeDataOrderingProof<P>,
    ) -> Self {
        Self::redo_only(
            NoUndoPublicationAuthority::recovery_redo_only().admit_redo_only_mutation(ordering),
        )
    }

    pub(crate) fn rollback_image_protected<P: BackendDurabilityProfile>(
        ordering: WalBeforeDataOrderingProof<P>,
        declaration: RollbackImagePublicationDeclaration,
    ) -> Result<Self, UnadmittedDirtyPagePublicationDenial> {
        let proof = NoUndoPublicationAuthority::recovery_redo_only()
            .admit_rollback_image_protected(ordering, declaration)?;
        Ok(Self::redo_only(proof))
    }

    pub const fn profile_id(&self) -> BackendDurabilityProfileId {
        self.profile_id
    }

    pub const fn page_generation(&self) -> PageGenerationCell {
        self.page_generation
    }

    pub const fn page_lsn(&self) -> PageLsn {
        self.page_lsn
    }

    pub const fn wal_frontier(&self) -> LogSequenceNumber {
        self.wal_frontier
    }

    pub const fn rollback_posture(&self) -> RollbackImagePublicationPosture {
        self.rollback_posture
    }

    pub const fn counters(&self) -> PageLsnPublicationCounterSnapshot {
        self.counters
    }
}
