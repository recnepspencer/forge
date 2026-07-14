use std::marker::PhantomData;

use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryAdmittedGraphReadAccessPlan, WorthQueryGraphReadAccessAdmission,
    WorthQueryGraphReadAccessShapeExplanationError, WorthQueryReadFamily, WorthQueryRuntime,
};

use super::{
    WorthQueryDomainHandleDenial, WorthQueryDomainHandleDenialKind,
    WorthQueryInstalledDomainAuthorityWitness, WorthQueryInstalledDomainHandle,
};

#[derive(Debug)]
pub enum WorthQueryInstalledDomainReadAdmissionError {
    InstalledAuthority(WorthQueryDomainHandleDenial),
    ReadShape(WorthQueryGraphReadAccessShapeExplanationError),
}

impl From<WorthQueryDomainHandleDenial> for WorthQueryInstalledDomainReadAdmissionError {
    fn from(value: WorthQueryDomainHandleDenial) -> Self {
        Self::InstalledAuthority(value)
    }
}

impl From<WorthQueryGraphReadAccessShapeExplanationError>
    for WorthQueryInstalledDomainReadAdmissionError
{
    fn from(value: WorthQueryGraphReadAccessShapeExplanationError) -> Self {
        Self::ReadShape(value)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledDomainExecutionDriftKind {
    ForeignRuntime,
    StaleInstallation,
    PackageMeaningChanged,
    BasisChanged,
    PolicyChanged,
    LowerBindingChanged,
    ResourceClosed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryInstalledDomainExecutionNextAction {
    RebindInstalledDomain,
    ReadmitBasis,
    ReadmitPolicy,
    ReadmitLowerBinding,
    ReopenResource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainExecutionDrift {
    kind: WorthQueryInstalledDomainExecutionDriftKind,
    next_action: WorthQueryInstalledDomainExecutionNextAction,
}

impl WorthQueryInstalledDomainExecutionDrift {
    fn from_handle_denial(denial: &WorthQueryDomainHandleDenial) -> Self {
        let kind = match denial.kind() {
            WorthQueryDomainHandleDenialKind::DomainNotInstalled
            | WorthQueryDomainHandleDenialKind::ForeignRuntime => {
                WorthQueryInstalledDomainExecutionDriftKind::ForeignRuntime
            }
            WorthQueryDomainHandleDenialKind::StaleInstallationGeneration => {
                WorthQueryInstalledDomainExecutionDriftKind::StaleInstallation
            }
            WorthQueryDomainHandleDenialKind::PackageIdentityChanged => {
                WorthQueryInstalledDomainExecutionDriftKind::PackageMeaningChanged
            }
        };
        Self {
            kind,
            next_action: WorthQueryInstalledDomainExecutionNextAction::RebindInstalledDomain,
        }
    }

    fn basis_changed() -> Self {
        Self {
            kind: WorthQueryInstalledDomainExecutionDriftKind::BasisChanged,
            next_action: WorthQueryInstalledDomainExecutionNextAction::ReadmitBasis,
        }
    }

    pub fn kind(&self) -> WorthQueryInstalledDomainExecutionDriftKind {
        self.kind
    }

    pub fn next_action(&self) -> WorthQueryInstalledDomainExecutionNextAction {
        self.next_action
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainExecutionReceipt {
    installed_authority: WorthQueryInstalledDomainAuthorityWitness,
    basis_identity: WorthQueryEvidenceIdentity,
    capability_authority_identity: WorthQueryEvidenceIdentity,
    admission_identity: WorthQueryEvidenceIdentity,
    plan_identity: Option<WorthQueryEvidenceIdentity>,
    receipt_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInstalledDomainExecutionReceipt {
    fn new(
        installed_authority: WorthQueryInstalledDomainAuthorityWitness,
        basis_identity: WorthQueryEvidenceIdentity,
        admission: &WorthQueryGraphReadAccessAdmission,
        plan: Option<&WorthQueryAdmittedGraphReadAccessPlan>,
    ) -> Self {
        let capability_authority_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecution)
                .field_shape(WorthQueryEvidenceTag::new("capability"), "graph-read")
                .field_value(
                    WorthQueryEvidenceTag::new("authority_receipt"),
                    admission.authority_receipt().digest(),
                )
                .seal();
        let admission_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecution)
                .field_shape(WorthQueryEvidenceTag::new("phase"), "admission")
                .field_value(WorthQueryEvidenceTag::new("admission"), admission.digest())
                .seal();
        let plan_identity = plan.map(|plan| {
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecution)
                .field_shape(WorthQueryEvidenceTag::new("phase"), "plan")
                .field_value(WorthQueryEvidenceTag::new("plan"), plan.digest())
                .seal()
        });
        let mut receipt_identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecution)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("installed_authority"),
                    installed_authority.witness_identity(),
                )
                .field_evidence_identity(WorthQueryEvidenceTag::new("basis"), &basis_identity)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("capability_authority"),
                    &capability_authority_identity,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("admission"),
                    &admission_identity,
                );
        if let Some(plan_identity) = plan_identity.as_ref() {
            receipt_identity = receipt_identity
                .field_evidence_identity(WorthQueryEvidenceTag::new("plan"), plan_identity);
        }
        let receipt_identity = receipt_identity.seal();
        Self {
            installed_authority,
            basis_identity,
            capability_authority_identity,
            admission_identity,
            plan_identity,
            receipt_identity,
        }
    }

    pub fn installed_authority(&self) -> &WorthQueryInstalledDomainAuthorityWitness {
        &self.installed_authority
    }

    pub fn basis_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.basis_identity
    }

    pub fn capability_authority_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.capability_authority_identity
    }

    pub fn admission_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admission_identity
    }

    pub fn plan_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.plan_identity.as_ref()
    }

    pub fn receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.receipt_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainReadAdmission<D> {
    admission: WorthQueryGraphReadAccessAdmission,
    plan: Option<WorthQueryAdmittedGraphReadAccessPlan>,
    receipt: WorthQueryInstalledDomainExecutionReceipt,
    marker: PhantomData<fn() -> D>,
}

impl<D: 'static> WorthQueryInstalledDomainReadAdmission<D> {
    pub(crate) fn admit(
        handle: &WorthQueryInstalledDomainHandle<D>,
        runtime: &WorthQueryRuntime,
        family: &WorthQueryReadFamily,
    ) -> Result<Self, WorthQueryInstalledDomainReadAdmissionError> {
        runtime.validate_installed_domain_handle(handle)?;
        let basis_identity = runtime.current_snapshot_identity().evidence_identity();
        let admission = runtime.admit_graph_read_access_for_family(family)?;
        let plan = WorthQueryAdmittedGraphReadAccessPlan::from_admission(admission.clone());
        let receipt = WorthQueryInstalledDomainExecutionReceipt::new(
            handle.authority_witness(),
            basis_identity,
            &admission,
            plan.as_ref(),
        );
        Ok(Self {
            admission,
            plan,
            receipt,
            marker: PhantomData,
        })
    }

    pub fn admission(&self) -> &WorthQueryGraphReadAccessAdmission {
        &self.admission
    }

    pub fn plan(&self) -> Option<&WorthQueryAdmittedGraphReadAccessPlan> {
        self.plan.as_ref()
    }

    pub fn receipt(&self) -> &WorthQueryInstalledDomainExecutionReceipt {
        &self.receipt
    }

    pub fn validate_for_execution(
        &self,
        handle: &WorthQueryInstalledDomainHandle<D>,
        runtime: &WorthQueryRuntime,
    ) -> Result<(), WorthQueryInstalledDomainExecutionDrift> {
        runtime
            .validate_installed_domain_handle(handle)
            .map_err(|denial| {
                WorthQueryInstalledDomainExecutionDrift::from_handle_denial(&denial)
            })?;
        if handle.authority().authority_identity()
            != self
                .receipt
                .installed_authority()
                .authority()
                .authority_identity()
        {
            let denial =
                WorthQueryDomainHandleDenial::new(WorthQueryDomainHandleDenialKind::ForeignRuntime);
            return Err(WorthQueryInstalledDomainExecutionDrift::from_handle_denial(
                &denial,
            ));
        }
        if runtime.current_snapshot_identity().evidence_identity() != *self.receipt.basis_identity()
        {
            return Err(WorthQueryInstalledDomainExecutionDrift::basis_changed());
        }
        Ok(())
    }
}
