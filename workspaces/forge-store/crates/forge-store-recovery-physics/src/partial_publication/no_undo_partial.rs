use crate::{
    PageFlushRecoveryReceipt, RollbackImagePublicationPosture, UnadmittedDirtyPagePublicationDenial,
};

use super::UnadmittedDurablePageMutationDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoUndoPartialPublicationClassification {
    posture: RollbackImageRequiredPosture,
    denial: Option<UnadmittedDurablePageMutationDenial>,
}

impl NoUndoPartialPublicationClassification {
    pub fn from_unadmitted_durable_page_mutation(
        denial: UnadmittedDirtyPagePublicationDenial,
    ) -> Self {
        Self {
            posture: RollbackImageRequiredPosture::RequiredButMissing,
            denial: Some(UnadmittedDurablePageMutationDenial::from_page_publication_denial(denial)),
        }
    }

    pub const fn from_page_flush_recovery_receipt(receipt: &PageFlushRecoveryReceipt) -> Self {
        Self {
            posture: RollbackImageRequiredPosture::from_publication_posture(
                receipt.rollback_posture(),
            ),
            denial: None,
        }
    }

    pub const fn posture(&self) -> RollbackImageRequiredPosture {
        self.posture
    }

    pub const fn denial(&self) -> Option<&UnadmittedDurablePageMutationDenial> {
        self.denial.as_ref()
    }

    pub const fn requires_rejection(&self) -> bool {
        matches!(
            self.posture,
            RollbackImageRequiredPosture::RequiredButMissing
        )
    }

    pub const fn is_deferred_to_undo_capable_recovery(&self) -> bool {
        matches!(
            self.posture,
            RollbackImageRequiredPosture::DeferredToUndoCapableRecovery
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackImageRequiredPosture {
    NotRequiredForAdmittedRedoOnlyMutation,
    ProtectedByRollbackImage,
    RequiredButMissing,
    DeferredToUndoCapableRecovery,
}

impl RollbackImageRequiredPosture {
    pub const fn from_publication_posture(posture: RollbackImagePublicationPosture) -> Self {
        match posture {
            RollbackImagePublicationPosture::NotRequiredForAdmittedRedoOnlyMutation => {
                Self::NotRequiredForAdmittedRedoOnlyMutation
            }
            RollbackImagePublicationPosture::RollbackImageProtectsUnadmittedBytes => {
                Self::ProtectedByRollbackImage
            }
            RollbackImagePublicationPosture::RollbackImageRequiredButMissing => {
                Self::RequiredButMissing
            }
        }
    }
}
