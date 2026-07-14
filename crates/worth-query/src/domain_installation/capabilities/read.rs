use std::marker::PhantomData;

use crate::ordinary::read::{
    declare, WorthQueryDeclaredReadIntent, WorthQueryProjectionDeclaration,
    WorthQueryProjectionOutcome, WorthQueryReadCompletion, WorthQueryReadContextDeclaration,
    WorthQueryReadDeclaration, WorthQueryReadDeclarationStop, WorthQueryReadOutcome,
    WorthQueryReadRequest, WorthQueryReadStop,
};
use crate::runtime::{WorthQueryReadBuilder, WorthQueryReadDenial, WorthQueryWorkspace};

use super::super::{
    WorthQueryInstalledDomainAuthorityWitness, WorthQueryInstalledDomainCapabilityKind,
    WorthQueryInstalledDomainExecutionDrift, WorthQueryInstalledDomainExecutionReceipt,
    WorthQueryInstalledDomainHandle,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainReadDeclaration<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    declaration: WorthQueryReadDeclaration,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryInstalledDomainReadDeclaration<D> {
    pub fn using(
        self,
        context: impl Into<WorthQueryReadContextDeclaration>,
    ) -> WorthQueryInstalledDomainReadRequest<D> {
        WorthQueryInstalledDomainReadRequest {
            witness: self.witness,
            request: self.declaration.using(context),
            marker: PhantomData,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainReadRequest<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    request: WorthQueryReadRequest,
    marker: PhantomData<fn() -> D>,
}

impl<D: 'static> WorthQueryInstalledDomainReadRequest<D> {
    pub fn run(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> Result<WorthQueryInstalledDomainReadOutcome<D>, WorthQueryInstalledDomainExecutionDrift>
    {
        WorthQueryInstalledDomainExecutionDrift::validate::<D>(&self.witness, workspace)?;
        let declaration_identity = WorthQueryInstalledDomainExecutionReceipt::label_identity(
            "read-declaration",
            self.request.declaration_identity().as_str(),
        );
        let outcome = self.request.run(workspace);
        Ok(WorthQueryInstalledDomainReadOutcome {
            witness: self.witness,
            declaration_identity,
            outcome,
            marker: PhantomData,
        })
    }
}

#[derive(Debug)]
pub struct WorthQueryInstalledDomainReadOutcome<D> {
    witness: WorthQueryInstalledDomainAuthorityWitness,
    declaration_identity: crate::WorthQueryEvidenceIdentity,
    outcome: WorthQueryReadOutcome,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryInstalledDomainReadOutcome<D> {
    pub fn completed(&self) -> Option<&WorthQueryReadCompletion> {
        self.outcome.completed()
    }

    pub fn stop(&self) -> Option<&WorthQueryReadStop> {
        self.outcome.stop()
    }

    pub fn into_result(
        self,
    ) -> Result<WorthQueryInstalledDomainReadCompletion<D>, WorthQueryReadStop> {
        let completion = self.outcome.into_result()?;
        let lower_receipt = completion.result().receipt();
        let receipt = WorthQueryInstalledDomainExecutionReceipt::new(
            self.witness,
            WorthQueryInstalledDomainCapabilityKind::Read,
            self.declaration_identity,
            lower_receipt.snapshot_evidence_identity(),
            WorthQueryInstalledDomainExecutionReceipt::label_identity(
                "read-result",
                lower_receipt.result_digest(),
            ),
        );
        Ok(WorthQueryInstalledDomainReadCompletion {
            completion,
            receipt,
            marker: PhantomData,
        })
    }
}

pub struct WorthQueryInstalledDomainReadCompletion<D> {
    completion: WorthQueryReadCompletion,
    receipt: WorthQueryInstalledDomainExecutionReceipt,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryInstalledDomainReadCompletion<D> {
    pub fn completion(&self) -> &WorthQueryReadCompletion {
        &self.completion
    }

    pub fn receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }

    pub fn project(
        &self,
        declaration: WorthQueryProjectionDeclaration,
    ) -> WorthQueryInstalledDomainProjectionOutcome<D> {
        let outcome = self.completion.consume_projection(declaration);
        let receipt = WorthQueryInstalledDomainExecutionReceipt::new(
            self.receipt.installed_authority().clone(),
            WorthQueryInstalledDomainCapabilityKind::Projection,
            self.receipt.declaration_identity().clone(),
            self.receipt.basis_identity().clone(),
            self.receipt.operational_identity().clone(),
        );
        WorthQueryInstalledDomainProjectionOutcome {
            outcome,
            receipt,
            marker: PhantomData,
        }
    }

    pub(crate) fn witness(&self) -> &WorthQueryInstalledDomainAuthorityWitness {
        self.receipt.installed_authority()
    }

    pub(crate) fn declaration_identity(&self) -> &crate::WorthQueryEvidenceIdentity {
        self.receipt.declaration_identity()
    }
}

pub struct WorthQueryInstalledDomainProjectionOutcome<D> {
    outcome: WorthQueryProjectionOutcome,
    receipt: WorthQueryInstalledDomainExecutionReceipt,
    marker: PhantomData<fn() -> D>,
}

impl<D> WorthQueryInstalledDomainProjectionOutcome<D> {
    pub fn outcome(&self) -> &WorthQueryProjectionOutcome {
        &self.outcome
    }

    pub fn receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }

    pub fn into_outcome(self) -> WorthQueryProjectionOutcome {
        self.outcome
    }
}

impl<D> WorthQueryInstalledDomainHandle<D> {
    pub fn read(
        &self,
        author: impl FnOnce(
            WorthQueryReadBuilder<WorthQueryDeclaredReadIntent>,
        ) -> Result<WorthQueryDeclaredReadIntent, WorthQueryReadDenial>,
    ) -> Result<WorthQueryInstalledDomainReadDeclaration<D>, WorthQueryReadDeclarationStop> {
        declare(author).map(|declaration| WorthQueryInstalledDomainReadDeclaration {
            witness: self.authority_witness(),
            declaration,
            marker: PhantomData,
        })
    }
}
