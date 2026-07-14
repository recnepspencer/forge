use std::marker::PhantomData;

use crate::ordinary::mutation::{
    declare as declare_mutation, WorthQueryMutationContext, WorthQueryMutationDeclaration,
    WorthQueryMutationDeclarationStop, WorthQueryMutationOutcome, WorthQueryMutationStop,
};
use crate::ordinary::workflow::{
    declare as declare_workflow, WorthQueryWorkflowContext, WorthQueryWorkflowOutcome,
    WorthQueryWorkflowStop,
};
use crate::runtime::{
    WorthQueryAspectMutationBuilder, WorthQueryRuntimeError, WorthQueryWorkspace,
    WorthQueryWriteCommand,
};
use crate::session_label::WorthQuerySessionLabel;

use super::super::{
    WorthQueryInstalledDomainAuthorityWitness, WorthQueryInstalledDomainCapabilityKind,
    WorthQueryInstalledDomainCapabilityStop, WorthQueryInstalledDomainExecutionDrift,
    WorthQueryInstalledDomainExecutionReceipt, WorthQueryInstalledDomainHandle,
};

pub struct WorthQueryInstalledDomainMutationDeclaration<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    declaration: WorthQueryMutationDeclaration,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryInstalledDomainMutationDeclaration<D> {
    pub fn with_rich_inspection(mut self) -> Self {
        self.declaration = self.declaration.with_rich_inspection();
        self
    }

    pub fn using(
        self,
        context: WorthQueryMutationContext,
    ) -> WorthQueryInstalledDomainMutationRequest<D> {
        let declaration_identity = self.declaration.identity().evidence_identity().clone();
        WorthQueryInstalledDomainMutationRequest {
            witness: self.witness,
            declaration_identity,
            request: self.declaration.using(context),
            marker: PhantomData,
        }
    }

    pub fn workflow(
        self,
        label: WorthQuerySessionLabel,
    ) -> WorthQueryInstalledDomainWorkflowDeclaration<D> {
        let declaration = declare_workflow(label, self.declaration);
        WorthQueryInstalledDomainWorkflowDeclaration {
            witness: self.witness,
            declaration,
            marker: PhantomData,
        }
    }
}

pub struct WorthQueryInstalledDomainMutationRequest<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    declaration_identity: crate::WorthQueryEvidenceIdentity,
    request: crate::ordinary::mutation::WorthQueryMutationRequest,
    marker: PhantomData<fn() -> D>,
}

impl<D: 'static> WorthQueryInstalledDomainMutationRequest<D> {
    pub fn run(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<
        WorthQueryInstalledDomainMutationOutcome,
        WorthQueryInstalledDomainCapabilityStop<WorthQueryInstalledDomainExecutionDrift>,
    > {
        WorthQueryInstalledDomainExecutionDrift::validate::<D>(&self.witness, workspace).map_err(
            |drift| {
                WorthQueryInstalledDomainCapabilityStop::new(
                    self.witness.clone(),
                    WorthQueryInstalledDomainCapabilityKind::Mutation,
                    self.declaration_identity.clone(),
                    drift,
                )
            },
        )?;
        Ok(match self.request.run(workspace) {
            WorthQueryMutationOutcome::Completed(completion) => {
                let receipt = WorthQueryInstalledDomainExecutionReceipt::new(
                    self.witness,
                    WorthQueryInstalledDomainCapabilityKind::Mutation,
                    self.declaration_identity,
                    completion.receipt().snapshot_evidence_identity().clone(),
                    completion.receipt().commit_evidence_identity().clone(),
                );
                WorthQueryInstalledDomainMutationOutcome::Completed(
                    WorthQueryInstalledDomainMutationCompletion {
                        completion,
                        receipt,
                    },
                )
            }
            WorthQueryMutationOutcome::Stopped(stop) => {
                WorthQueryInstalledDomainMutationOutcome::Stopped(
                    WorthQueryInstalledDomainCapabilityStop::new(
                        self.witness,
                        WorthQueryInstalledDomainCapabilityKind::Mutation,
                        self.declaration_identity,
                        stop,
                    ),
                )
            }
        })
    }
}

pub enum WorthQueryInstalledDomainMutationOutcome {
    Completed(WorthQueryInstalledDomainMutationCompletion),
    Stopped(WorthQueryInstalledDomainCapabilityStop<WorthQueryMutationStop>),
}

impl WorthQueryInstalledDomainMutationOutcome {
    pub fn completed(&self) -> Option<&WorthQueryInstalledDomainMutationCompletion> {
        match self {
            Self::Completed(completion) => Some(completion),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryInstalledDomainCapabilityStop<WorthQueryMutationStop>> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }
}

pub struct WorthQueryInstalledDomainMutationCompletion {
    completion: crate::ordinary::mutation::WorthQueryMutationCompletion,
    receipt: WorthQueryInstalledDomainExecutionReceipt,
}

impl WorthQueryInstalledDomainMutationCompletion {
    pub fn completion(&self) -> &crate::ordinary::mutation::WorthQueryMutationCompletion {
        &self.completion
    }
    pub fn receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }
}

pub struct WorthQueryInstalledDomainWorkflowDeclaration<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    declaration: crate::ordinary::workflow::WorthQueryWorkflowDeclaration,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryInstalledDomainWorkflowDeclaration<D> {
    pub fn with_rich_inspection(mut self) -> Self {
        self.declaration = self.declaration.with_rich_inspection();
        self
    }

    pub fn using(
        self,
        context: WorthQueryWorkflowContext,
    ) -> WorthQueryInstalledDomainWorkflowRequest<D> {
        let declaration_identity = self.declaration.identity().evidence_identity().clone();
        WorthQueryInstalledDomainWorkflowRequest {
            witness: self.witness,
            declaration_identity,
            request: self.declaration.using(context),
            marker: PhantomData,
        }
    }
}

pub struct WorthQueryInstalledDomainWorkflowRequest<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    declaration_identity: crate::WorthQueryEvidenceIdentity,
    request: crate::ordinary::workflow::WorthQueryWorkflowRequest,
    marker: PhantomData<fn() -> D>,
}

impl<D: 'static> WorthQueryInstalledDomainWorkflowRequest<D> {
    pub fn run(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<
        WorthQueryInstalledDomainWorkflowOutcome,
        WorthQueryInstalledDomainCapabilityStop<WorthQueryInstalledDomainExecutionDrift>,
    > {
        WorthQueryInstalledDomainExecutionDrift::validate::<D>(&self.witness, workspace).map_err(
            |drift| {
                WorthQueryInstalledDomainCapabilityStop::new(
                    self.witness.clone(),
                    WorthQueryInstalledDomainCapabilityKind::Workflow,
                    self.declaration_identity.clone(),
                    drift,
                )
            },
        )?;
        Ok(match self.request.run(workspace) {
            WorthQueryWorkflowOutcome::Completed(completion) => {
                let receipt = WorthQueryInstalledDomainExecutionReceipt::new(
                    self.witness,
                    WorthQueryInstalledDomainCapabilityKind::Workflow,
                    self.declaration_identity,
                    completion
                        .promotion_eligibility()
                        .snapshot_identity()
                        .evidence_identity(),
                    completion.aftermath().receipt_identity().clone(),
                );
                WorthQueryInstalledDomainWorkflowOutcome::Completed(
                    WorthQueryInstalledDomainWorkflowCompletion {
                        completion,
                        receipt,
                    },
                )
            }
            WorthQueryWorkflowOutcome::Stopped(stop) => {
                WorthQueryInstalledDomainWorkflowOutcome::Stopped(
                    WorthQueryInstalledDomainCapabilityStop::new(
                        self.witness,
                        WorthQueryInstalledDomainCapabilityKind::Workflow,
                        self.declaration_identity,
                        stop,
                    ),
                )
            }
        })
    }
}

pub enum WorthQueryInstalledDomainWorkflowOutcome {
    Completed(WorthQueryInstalledDomainWorkflowCompletion),
    Stopped(WorthQueryInstalledDomainCapabilityStop<WorthQueryWorkflowStop>),
}

impl WorthQueryInstalledDomainWorkflowOutcome {
    pub fn completed(&self) -> Option<&WorthQueryInstalledDomainWorkflowCompletion> {
        match self {
            Self::Completed(completion) => Some(completion),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryInstalledDomainCapabilityStop<WorthQueryWorkflowStop>> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }
}

pub struct WorthQueryInstalledDomainWorkflowCompletion {
    completion: crate::ordinary::workflow::WorthQueryWorkflowCompletion,
    receipt: WorthQueryInstalledDomainExecutionReceipt,
}

impl WorthQueryInstalledDomainWorkflowCompletion {
    pub fn completion(&self) -> &crate::ordinary::workflow::WorthQueryWorkflowCompletion {
        &self.completion
    }
    pub fn receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }
}

impl<D> WorthQueryInstalledDomainHandle<D> {
    pub fn mutation(
        &self,
        author: impl FnOnce(
            WorthQueryAspectMutationBuilder,
        ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError>,
    ) -> Result<WorthQueryInstalledDomainMutationDeclaration<D>, WorthQueryMutationDeclarationStop>
    {
        declare_mutation(author).map(|declaration| WorthQueryInstalledDomainMutationDeclaration {
            witness: self.authority_witness(),
            declaration,
            marker: PhantomData,
        })
    }
}
