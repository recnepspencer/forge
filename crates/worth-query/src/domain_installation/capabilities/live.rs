use std::marker::PhantomData;

use crate::ordinary::live::{
    declare_live, WorthQueryLiveDeclaration, WorthQueryLiveDeclarationStop,
    WorthQueryLiveOpenOutcome, WorthQueryLiveOpenStop, WorthQueryManagedLiveCheckpointOutcome,
    WorthQueryManagedLiveDelivery, WorthQueryManagedLiveHandle,
    WorthQueryManagedLiveLifecycleObservation,
};
use crate::ordinary::read::{WorthQueryDeclaredReadIntent, WorthQueryReadContextDeclaration};
use crate::runtime::{
    WorthQueryLiveReadResult, WorthQueryReadBuilder, WorthQueryReadDenial, WorthQueryRuntimeError,
    WorthQueryWorkspace,
};

use super::super::{
    WorthQueryInstalledDomainAuthorityWitness, WorthQueryInstalledDomainCapabilityKind,
    WorthQueryInstalledDomainExecutionDrift, WorthQueryInstalledDomainExecutionReceipt,
    WorthQueryInstalledDomainHandle,
};
use super::{
    close_outcome, WorthQueryInstalledDomainLiveCheckpointOutcome,
    WorthQueryInstalledDomainLiveCheckpointStop, WorthQueryInstalledDomainLiveCloseOutcome,
    WorthQueryInstalledDomainLiveContinuation,
};

pub struct WorthQueryInstalledDomainLiveDeclaration<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    declaration: WorthQueryLiveDeclaration,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryInstalledDomainLiveDeclaration<D> {
    pub fn using(
        self,
        context: impl Into<WorthQueryReadContextDeclaration>,
    ) -> WorthQueryInstalledDomainLiveRequest<D> {
        WorthQueryInstalledDomainLiveRequest {
            witness: self.witness,
            request: self.declaration.using(context),
            marker: PhantomData,
        }
    }
}

pub struct WorthQueryInstalledDomainLiveRequest<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    request: crate::ordinary::live::WorthQueryLiveRequest,
    marker: PhantomData<fn() -> D>,
}

impl<D: 'static> WorthQueryInstalledDomainLiveRequest<D> {
    pub fn open(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryInstalledDomainLiveOpenOutcome<D>, WorthQueryInstalledDomainExecutionDrift>
    {
        WorthQueryInstalledDomainExecutionDrift::validate::<D>(&self.witness, workspace)?;
        let basis_identity = workspace.snapshot_identity().evidence_identity();
        let declaration_identity = WorthQueryInstalledDomainExecutionReceipt::label_identity(
            "live-declaration",
            self.request.declaration_identity().as_str(),
        );
        Ok(match self.request.open(workspace) {
            WorthQueryLiveOpenOutcome::Opened(completion) => {
                let receipt = WorthQueryInstalledDomainExecutionReceipt::new(
                    self.witness,
                    WorthQueryInstalledDomainCapabilityKind::LiveOpen,
                    declaration_identity,
                    basis_identity,
                    WorthQueryInstalledDomainExecutionReceipt::label_identity(
                        "live-context-admission",
                        completion.context_receipt().digest(),
                    ),
                );
                WorthQueryInstalledDomainLiveOpenOutcome::Opened(
                    WorthQueryInstalledDomainLiveHandle {
                        handle: completion.into_handle(),
                        receipt,
                        marker: PhantomData,
                    },
                )
            }
            WorthQueryLiveOpenOutcome::Stopped(stop) => {
                WorthQueryInstalledDomainLiveOpenOutcome::Stopped(stop)
            }
        })
    }
}

pub enum WorthQueryInstalledDomainLiveOpenOutcome<D> {
    Opened(WorthQueryInstalledDomainLiveHandle<D>),
    Stopped(WorthQueryLiveOpenStop),
}

#[must_use = "installed live handles own a Query resource until closed"]
pub struct WorthQueryInstalledDomainLiveHandle<D> {
    pub(super) handle: WorthQueryManagedLiveHandle,
    pub(super) receipt: WorthQueryInstalledDomainExecutionReceipt,
    pub(super) marker: PhantomData<fn() -> D>,
}

impl<D: 'static> WorthQueryInstalledDomainLiveHandle<D> {
    pub fn name(&self) -> &str {
        self.handle.name()
    }

    pub fn installation_receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }

    pub fn read(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryInstalledDomainLiveRead, WorthQueryInstalledDomainLiveOperationError>
    {
        self.validate(workspace)?;
        let result = self
            .handle
            .read(workspace)
            .map_err(WorthQueryInstalledDomainLiveOperationError::Runtime)?;
        let receipt = self.receipt.derive(
            WorthQueryInstalledDomainCapabilityKind::LiveRead,
            WorthQueryInstalledDomainExecutionReceipt::label_identity(
                "live-read-result",
                result.receipt().result_digest(),
            ),
        );
        Ok(WorthQueryInstalledDomainLiveRead { result, receipt })
    }

    pub fn drain(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryInstalledDomainLiveDelivery, WorthQueryInstalledDomainLiveOperationError>
    {
        self.validate(workspace)?;
        let delivery = self
            .handle
            .drain(workspace)
            .map_err(WorthQueryInstalledDomainLiveOperationError::Runtime)?;
        let operational = delivery
            .batches()
            .last()
            .map(|batch| batch.delivery_batch_identity().clone())
            .unwrap_or_else(|| self.receipt.operational_identity().clone());
        Ok(WorthQueryInstalledDomainLiveDelivery {
            delivery,
            receipt: self.receipt.derive(
                WorthQueryInstalledDomainCapabilityKind::LiveDelivery,
                operational,
            ),
        })
    }

    pub fn observe(
        &self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<
        WorthQueryManagedLiveLifecycleObservation,
        WorthQueryInstalledDomainLiveOperationError,
    > {
        self.validate(workspace)?;
        self.handle
            .observe(workspace)
            .map_err(WorthQueryInstalledDomainLiveOperationError::Runtime)
    }

    pub fn checkpoint(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryInstalledDomainLiveCheckpointOutcome<D> {
        if let Err(drift) = WorthQueryInstalledDomainExecutionDrift::validate::<D>(
            self.receipt.installed_authority(),
            workspace,
        ) {
            return WorthQueryInstalledDomainLiveCheckpointOutcome::AuthorityStopped(self, drift);
        }
        let receipt = self.receipt;
        match self.handle.checkpoint(workspace) {
            WorthQueryManagedLiveCheckpointOutcome::Checkpointed(completion) => {
                let operational = completion.checkpoint().continuation_identity().clone();
                WorthQueryInstalledDomainLiveCheckpointOutcome::Checkpointed(
                    WorthQueryInstalledDomainLiveContinuation {
                        continuation: completion.into_continuation(),
                        receipt: receipt.derive(
                            WorthQueryInstalledDomainCapabilityKind::LiveCheckpoint,
                            operational,
                        ),
                        marker: PhantomData,
                    },
                )
            }
            WorthQueryManagedLiveCheckpointOutcome::Stopped(stop) => {
                WorthQueryInstalledDomainLiveCheckpointOutcome::RuntimeStopped(
                    WorthQueryInstalledDomainLiveCheckpointStop {
                        stop,
                        receipt,
                        marker: PhantomData,
                    },
                )
            }
        }
    }

    pub fn close(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryInstalledDomainLiveCloseOutcome<D> {
        if let Err(drift) = WorthQueryInstalledDomainExecutionDrift::validate::<D>(
            self.receipt.installed_authority(),
            workspace,
        ) {
            return WorthQueryInstalledDomainLiveCloseOutcome::AuthorityStopped(self, drift);
        }
        close_outcome(self.handle.close(workspace), self.receipt)
    }

    fn validate(
        &self,
        workspace: &WorthQueryWorkspace,
    ) -> Result<(), WorthQueryInstalledDomainLiveOperationError> {
        WorthQueryInstalledDomainExecutionDrift::validate::<D>(
            self.receipt.installed_authority(),
            workspace,
        )
        .map_err(WorthQueryInstalledDomainLiveOperationError::Authority)
    }
}

pub enum WorthQueryInstalledDomainLiveOperationError {
    Authority(WorthQueryInstalledDomainExecutionDrift),
    Runtime(WorthQueryRuntimeError),
}

pub struct WorthQueryInstalledDomainLiveRead {
    result: WorthQueryLiveReadResult,
    receipt: WorthQueryInstalledDomainExecutionReceipt,
}

impl WorthQueryInstalledDomainLiveRead {
    pub fn result(&self) -> &WorthQueryLiveReadResult {
        &self.result
    }
    pub fn receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }
}

pub struct WorthQueryInstalledDomainLiveDelivery {
    delivery: WorthQueryManagedLiveDelivery,
    receipt: WorthQueryInstalledDomainExecutionReceipt,
}

impl WorthQueryInstalledDomainLiveDelivery {
    pub fn delivery(&self) -> &WorthQueryManagedLiveDelivery {
        &self.delivery
    }
    pub fn receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }
}

impl<D> WorthQueryInstalledDomainHandle<D> {
    pub fn live(
        &self,
        name: impl Into<String>,
        author: impl FnOnce(
            WorthQueryReadBuilder<WorthQueryDeclaredReadIntent>,
        ) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial>,
    ) -> Result<WorthQueryInstalledDomainLiveDeclaration<D>, WorthQueryLiveDeclarationStop> {
        declare_live(name, author).map(|declaration| WorthQueryInstalledDomainLiveDeclaration {
            witness: self.authority_witness(),
            declaration,
            marker: PhantomData,
        })
    }
}
