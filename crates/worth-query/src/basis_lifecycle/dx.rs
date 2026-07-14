use crate::identity::hash_parts;

use super::admission::{
    admit_basis_capability, evaluate_basis_inspection_advisory_eligibility,
    evaluate_basis_mutation_preparation_eligibility, evaluate_basis_observation_eligibility,
};
use super::intent::{normalize_raw_basis_intent, RawBasisIntent};
use super::lanes::{
    BasisOperationLane, InspectionLaneWitness, MutationPreparationLaneWitness,
    ObservationLaneWitness,
};
use super::lower_runtime::{readmit_lower_runtime_evidence, LowerRuntimeBasisEvidence};
use super::proofs::{
    AdmittedBasisCapability, AdvisoryBasisEligibility, BasisIntentDenial, DeniedBasisCapability,
};
use super::receipts::{
    emit_observation_basis_receipt, envelope_basis_use, BasisUseReceipt,
    SelfDescribingBasisEnvelope,
};
use super::scoping::{scope_basis_for_observation, ScopedObservationBasis};
use super::support::{discover_basis_lifecycle_support, BasisLifecycleSupportDiscovery};
use super::taxonomy::BasisFamily;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct BasisLifecycleIntentBuilder;

impl BasisLifecycleIntentBuilder {
    pub fn current_head(&self) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::CurrentHead)
    }

    pub fn branch_head(
        &self,
        branch_identity: impl Into<String>,
        accessible: bool,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::BranchHead {
            branch_identity: branch_identity.into(),
            accessible,
        })
    }

    pub fn preview_derived(
        &self,
        preview_identity: impl Into<String>,
        source_basis_identity: impl Into<String>,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::PreviewDerived {
            preview_identity: preview_identity.into(),
            source_basis_identity: source_basis_identity.into(),
        })
    }

    pub fn policy_scoped(
        &self,
        policy_digest: impl Into<String>,
        tenant_identity: impl Into<String>,
        branch_identity: impl Into<String>,
        schema_identity: impl Into<String>,
    ) -> BasisLifecyclePolicyIntentDraft {
        BasisLifecyclePolicyIntentDraft {
            policy_digest: policy_digest.into(),
            tenant_identity: tenant_identity.into(),
            branch_identity: branch_identity.into(),
            schema_identity: schema_identity.into(),
            tenant_schema_matches: true,
            policy_masks_operation: false,
            advisory_visibility: false,
        }
    }

    pub fn support(
        &self,
        family: BasisFamily,
        lane: &'static str,
    ) -> BasisLifecycleSupportDiscovery {
        discover_basis_lifecycle_support(family, lane)
    }
}

pub fn basis_lifecycle() -> BasisLifecycleIntentBuilder {
    BasisLifecycleIntentBuilder
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecycleIntentDraft {
    raw: RawBasisIntent,
}

impl BasisLifecycleIntentDraft {
    pub(crate) fn new(raw: RawBasisIntent) -> Self {
        Self { raw }
    }

    pub(crate) fn into_raw(self) -> RawBasisIntent {
        self.raw
    }

    pub fn for_observation(self) -> Result<ObservationBasisAdmissionPath, BasisIntentDenial> {
        ObservationBasisAdmissionPath::new(self.raw)
    }

    pub fn for_mutation_preparation(
        self,
    ) -> Result<MutationPreparationBasisAdmissionPath, BasisIntentDenial> {
        MutationPreparationBasisAdmissionPath::new(self.raw)
    }

    pub fn for_inspection_advisory(self) -> Result<InspectionAdvisoryBasisPath, BasisIntentDenial> {
        InspectionAdvisoryBasisPath::new(self.raw)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLifecyclePolicyIntentDraft {
    policy_digest: String,
    tenant_identity: String,
    branch_identity: String,
    schema_identity: String,
    tenant_schema_matches: bool,
    policy_masks_operation: bool,
    advisory_visibility: bool,
}

impl BasisLifecyclePolicyIntentDraft {
    pub fn tenant_schema_mismatch(mut self) -> Self {
        self.tenant_schema_matches = false;
        self
    }

    pub fn policy_masks_operation(mut self) -> Self {
        self.policy_masks_operation = true;
        self
    }

    pub fn advisory_visibility(mut self) -> Self {
        self.advisory_visibility = true;
        self
    }

    pub fn for_observation(self) -> Result<ObservationBasisAdmissionPath, BasisIntentDenial> {
        self.into_draft().for_observation()
    }

    pub(super) fn into_draft(self) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::PolicyScoped {
            policy_digest: self.policy_digest,
            tenant_identity: self.tenant_identity,
            branch_identity: self.branch_identity,
            schema_identity: self.schema_identity,
            tenant_schema_matches: self.tenant_schema_matches,
            policy_masks_operation: self.policy_masks_operation,
            advisory_visibility: self.advisory_visibility,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationBasisAdmissionPath {
    normalized: super::proofs::NormalizedBasisIntent,
}

impl ObservationBasisAdmissionPath {
    fn new(raw: RawBasisIntent) -> Result<Self, BasisIntentDenial> {
        let normalized = normalize_raw_basis_intent(raw, ObservationLaneWitness::lane_name())?;
        Ok(Self { normalized })
    }

    pub fn admit(self) -> Result<ObservationBasisUsePath, DeniedBasisCapability> {
        let eligibility = evaluate_basis_observation_eligibility(self.normalized)?;
        Ok(ObservationBasisUsePath {
            capability: admit_basis_capability(eligibility),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationBasisUsePath {
    capability: AdmittedBasisCapability<ObservationLaneWitness>,
}

impl ObservationBasisUsePath {
    pub fn capability(&self) -> &AdmittedBasisCapability<ObservationLaneWitness> {
        &self.capability
    }

    pub fn scope(self) -> ScopedObservationBasis {
        scope_basis_for_observation(self.capability)
    }

    pub fn bind_lower_runtime(
        self,
        evidence: LowerRuntimeBasisEvidence,
    ) -> Result<ObservationBasisReceiptPath, DeniedBasisCapability> {
        let bound = readmit_lower_runtime_evidence(self.scope(), evidence)?;
        Ok(ObservationBasisReceiptPath {
            receipt: emit_observation_basis_receipt(bound),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationBasisReceiptPath {
    receipt: BasisUseReceipt,
}

impl ObservationBasisReceiptPath {
    pub fn receipt(&self) -> &BasisUseReceipt {
        &self.receipt
    }

    pub fn envelope(self) -> SelfDescribingBasisEnvelope {
        envelope_basis_use(self.receipt)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MutationPreparationBasisAdmissionPath {
    normalized: super::proofs::NormalizedBasisIntent,
}

impl MutationPreparationBasisAdmissionPath {
    fn new(raw: RawBasisIntent) -> Result<Self, BasisIntentDenial> {
        let normalized =
            normalize_raw_basis_intent(raw, MutationPreparationLaneWitness::lane_name())?;
        Ok(Self { normalized })
    }

    pub fn admit(
        self,
    ) -> Result<AdmittedBasisCapability<MutationPreparationLaneWitness>, DeniedBasisCapability>
    {
        evaluate_basis_mutation_preparation_eligibility(self.normalized).map(admit_basis_capability)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InspectionAdvisoryBasisPath {
    normalized: super::proofs::NormalizedBasisIntent,
}

impl InspectionAdvisoryBasisPath {
    fn new(raw: RawBasisIntent) -> Result<Self, BasisIntentDenial> {
        let normalized = normalize_raw_basis_intent(raw, InspectionLaneWitness::lane_name())?;
        Ok(Self { normalized })
    }

    pub fn inspect_advisory(
        self,
    ) -> Result<AdvisoryBasisEligibility<InspectionLaneWitness>, DeniedBasisCapability> {
        evaluate_basis_inspection_advisory_eligibility(self.normalized)
    }
}

pub fn basis_lifecycle_dx_certification_digest() -> String {
    hash_parts(&[
        "basis_lifecycle_dx_certification_v1".to_string(),
        "current_head_observation_admit_bind_receipt_envelope".to_string(),
        "branch_head_mutation_preparation_admit".to_string(),
        "preview_derived_inspection_advisory".to_string(),
        "support_discovery".to_string(),
        "typed_denial_handling".to_string(),
    ])
}

#[cfg(test)]
mod tests;
