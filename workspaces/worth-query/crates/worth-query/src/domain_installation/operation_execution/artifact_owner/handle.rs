use std::sync::Arc;

use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDisposition, WorthQueryArtifactOwnerSnapshot,
    WorthQueryArtifactSemanticProjection, WorthQueryArtifactTraceMeaning,
    WorthQueryArtifactTransferAdmission, WorthQueryBorrowedArtifactView,
    WorthQueryDisposedArtifact, WorthQueryRetainedArtifactLease, WorthQueryRuntimeArtifactOwner,
};

pub struct WorthQueryMoveOnlyArtifactHandle {
    pub(super) core: WorthQueryArtifactHandleCore,
}

pub struct WorthQueryTransferredArtifactHandle {
    pub(super) core: WorthQueryArtifactHandleCore,
}

pub(super) enum WorthQueryArtifactHandleGuard {
    Owner,
    Lease,
}

pub(super) struct WorthQueryArtifactHandleCore {
    pub(super) owner: Arc<WorthQueryRuntimeArtifactOwner>,
    pub(super) handle_identity: String,
    pub(super) holder_stage: String,
    pub(super) lifecycle_generation: u64,
    pub(super) disposition: WorthQueryArtifactDisposition,
    pub(super) guard: WorthQueryArtifactHandleGuard,
    pub(super) active: bool,
}

impl WorthQueryMoveOnlyArtifactHandle {
    pub fn identity(&self) -> &str {
        &self.core.handle_identity
    }

    pub fn occurrence_identity(&self) -> &str {
        &self.core.owner.binding().occurrence_identity
    }

    pub fn semantic_projection(&self) -> &WorthQueryArtifactSemanticProjection {
        self.core.owner.semantic_projection()
    }

    pub fn retained_bytes(&self) -> usize {
        self.core.owner.retained_bytes()
    }

    pub fn owner_snapshot(&self) -> WorthQueryArtifactOwnerSnapshot {
        self.core.owner.snapshot()
    }

    pub fn borrow(
        &self,
        purpose: impl Into<String>,
    ) -> Result<WorthQueryBorrowedArtifactView<'_>, WorthQueryArtifactDenial> {
        WorthQueryBorrowedArtifactView::admit(
            &self.core.owner,
            self.core.lifecycle_generation,
            purpose,
        )
    }

    pub fn retain(
        &self,
        lease_role: impl Into<String>,
    ) -> Result<WorthQueryRetainedArtifactLease, WorthQueryArtifactDenial> {
        WorthQueryRetainedArtifactLease::admit(
            &self.core.owner,
            self.core.lifecycle_generation,
            lease_role,
        )
    }

    pub fn dispose(mut self) -> Result<WorthQueryDisposedArtifact, WorthQueryArtifactDenial> {
        self.core
            .dispose(WorthQueryArtifactDisposition::Disposed, true)
    }

    pub(crate) fn cancel(mut self) -> WorthQueryDisposedArtifact {
        self.core
            .dispose(WorthQueryArtifactDisposition::Cancelled, false)
            .expect("dropping an owned workflow artifact cannot retain an active borrow")
    }

    pub(crate) fn retire_for_trace(mut self) -> WorthQueryDisposedArtifact {
        self.core
            .dispose(WorthQueryArtifactDisposition::Released, false)
            .expect("completed artifact cannot retain an active borrow")
    }

    pub(crate) fn transfer(
        mut self,
        admission: &WorthQueryArtifactTransferAdmission,
    ) -> Result<WorthQueryTransferredArtifactHandle, WorthQueryArtifactDenial> {
        self.core.validate_transfer(admission)?;
        let lifecycle_generation = self
            .core
            .owner
            .admit_transfer(self.core.lifecycle_generation)?;
        self.core.active = false;
        Ok(WorthQueryTransferredArtifactHandle {
            core: WorthQueryArtifactHandleCore::new(
                Arc::clone(&self.core.owner),
                admission.consumer_stage.clone(),
                lifecycle_generation,
                WorthQueryArtifactDisposition::Transferred,
                WorthQueryArtifactHandleGuard::Owner,
            ),
        })
    }

    pub(crate) fn lease_for_transfer(
        &self,
        admission: &WorthQueryArtifactTransferAdmission,
        lease_role: impl Into<String>,
    ) -> Result<WorthQueryTransferredArtifactHandle, WorthQueryArtifactDenial> {
        self.core.validate_transfer(admission)?;
        let lease = self.retain(lease_role)?;
        Ok(WorthQueryTransferredArtifactHandle::from_lease(
            lease,
            &admission.consumer_stage,
        ))
    }

    pub(crate) fn trace_meaning(&self) -> WorthQueryArtifactTraceMeaning {
        self.core.trace_meaning()
    }

    pub(crate) fn contract_matches(
        &self,
        reference: &worth_query_installation::facade::WorthQueryArtifactContractReference,
    ) -> bool {
        self.core.contract_matches(reference)
    }

    pub(crate) fn validate_output(
        &self,
        admission: &WorthQueryArtifactTransferAdmission,
    ) -> Result<(), WorthQueryArtifactDenial> {
        self.core.validate_output(admission)?;
        self.core
            .owner
            .validate_generation(self.core.lifecycle_generation)
    }
}

impl WorthQueryTransferredArtifactHandle {
    fn from_lease(mut lease: WorthQueryRetainedArtifactLease, stage: &str) -> Self {
        lease.active = false;
        let owner = Arc::clone(&lease.owner);
        Self {
            core: WorthQueryArtifactHandleCore::new(
                owner,
                stage.to_owned(),
                lease.lifecycle_generation,
                WorthQueryArtifactDisposition::Leased,
                WorthQueryArtifactHandleGuard::Lease,
            ),
        }
    }

    pub fn identity(&self) -> &str {
        &self.core.handle_identity
    }

    pub fn occurrence_identity(&self) -> &str {
        &self.core.owner.binding().occurrence_identity
    }

    pub fn semantic_projection(&self) -> &WorthQueryArtifactSemanticProjection {
        self.core.owner.semantic_projection()
    }

    pub fn borrow(
        &self,
        purpose: impl Into<String>,
    ) -> Result<WorthQueryBorrowedArtifactView<'_>, WorthQueryArtifactDenial> {
        WorthQueryBorrowedArtifactView::admit(
            &self.core.owner,
            self.core.lifecycle_generation,
            purpose,
        )
    }

    pub fn into_output(mut self) -> WorthQueryMoveOnlyArtifactHandle {
        self.core.active = false;
        WorthQueryMoveOnlyArtifactHandle {
            core: WorthQueryArtifactHandleCore::new(
                Arc::clone(&self.core.owner),
                self.core.holder_stage.clone(),
                self.core.lifecycle_generation,
                self.core.disposition,
                std::mem::replace(&mut self.core.guard, WorthQueryArtifactHandleGuard::Lease),
            ),
        }
    }

    pub(crate) fn trace_meaning(&self) -> WorthQueryArtifactTraceMeaning {
        self.core.trace_meaning()
    }

    pub(crate) fn validate_input(
        &self,
        admission: &WorthQueryArtifactTransferAdmission,
    ) -> Result<(), WorthQueryArtifactDenial> {
        self.core.validate_transfer(admission)?;
        match self.core.guard {
            WorthQueryArtifactHandleGuard::Owner => self
                .core
                .owner
                .validate_generation(self.core.lifecycle_generation),
            WorthQueryArtifactHandleGuard::Lease => self.core.owner.validate_live(),
        }
    }

    pub(crate) fn contract_matches(
        &self,
        reference: &worth_query_installation::facade::WorthQueryArtifactContractReference,
    ) -> bool {
        self.core.contract_matches(reference)
    }
}

impl WorthQueryArtifactHandleCore {
    pub(super) fn new_owner(
        owner: Arc<WorthQueryRuntimeArtifactOwner>,
        holder_stage: String,
        disposition: WorthQueryArtifactDisposition,
    ) -> Self {
        Self::new(
            owner,
            holder_stage,
            1,
            disposition,
            WorthQueryArtifactHandleGuard::Owner,
        )
    }

    pub(super) fn new(
        owner: Arc<WorthQueryRuntimeArtifactOwner>,
        holder_stage: String,
        lifecycle_generation: u64,
        disposition: WorthQueryArtifactDisposition,
        guard: WorthQueryArtifactHandleGuard,
    ) -> Self {
        let handle_identity = crate::identity::hash_parts(&[
            "worth_query_move_only_artifact_handle_v1".into(),
            format!("owner:{}", owner.binding().owner_identity),
            format!("stage:{holder_stage}"),
            format!("generation:{lifecycle_generation}"),
            format!("disposition:{}", disposition.canonical_name()),
        ]);
        Self {
            owner,
            handle_identity,
            holder_stage,
            lifecycle_generation,
            disposition,
            guard,
            active: true,
        }
    }

    fn dispose(
        &mut self,
        disposition: WorthQueryArtifactDisposition,
        require_no_lease: bool,
    ) -> Result<WorthQueryDisposedArtifact, WorthQueryArtifactDenial> {
        self.owner.validate_generation(self.lifecycle_generation)?;
        let provider_disposed = match self.guard {
            WorthQueryArtifactHandleGuard::Owner => {
                self.owner.release_owner(disposition, require_no_lease)?
            }
            WorthQueryArtifactHandleGuard::Lease => self.owner.release_lease(disposition),
        };
        self.active = false;
        Ok(WorthQueryDisposedArtifact::new(
            self.owner.binding().owner_identity.clone(),
            self.owner.binding().occurrence_identity.clone(),
            provider_disposed,
        ))
    }
}

impl Drop for WorthQueryArtifactHandleCore {
    fn drop(&mut self) {
        if !self.active {
            return;
        }
        match self.guard {
            WorthQueryArtifactHandleGuard::Owner => {
                self.owner
                    .release_owner(WorthQueryArtifactDisposition::Released, false)
                    .expect("owned artifact handle cannot outlive an active borrow");
            }
            WorthQueryArtifactHandleGuard::Lease => {
                self.owner
                    .release_lease(WorthQueryArtifactDisposition::Released);
            }
        }
    }
}

impl std::fmt::Debug for WorthQueryMoveOnlyArtifactHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryMoveOnlyArtifactHandle")
            .field("identity", &self.identity())
            .field("occurrence_identity", &self.occurrence_identity())
            .field("retained_bytes", &self.retained_bytes())
            .finish_non_exhaustive()
    }
}

impl std::fmt::Debug for WorthQueryTransferredArtifactHandle {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryTransferredArtifactHandle")
            .field("identity", &self.identity())
            .field("occurrence_identity", &self.occurrence_identity())
            .finish_non_exhaustive()
    }
}
