use sha2::{Digest, Sha256};

use crate::{
    RecoveryAuthorityAdmissionPolicy, RecoveryAuthorityAdmissionPolicyDenial,
    RecoveryAuthorityAdmissionPosture, StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryWriteFenceDenial {
    InvalidBinding,
    StaleCurrentAuthority,
    CutoverPlanMismatch,
    ProviderUnavailable,
    ProviderRejected,
    QuiescenceNotEstablished,
    ReleaseRejected,
    ReleaseReceiptMismatch,
}

#[derive(Debug, Clone, Copy)]
pub struct RecoveryWriteFenceRequest {
    plan_fingerprint: [u8; 32],
    expected_current_authority: StoreCurrentAuthorityIdentity,
    cutover_plan_fingerprint: [u8; 32],
    candidate_media_identity: [u8; 32],
    authority_delta_identity: [u8; 32],
}

impl RecoveryWriteFenceRequest {
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn expected_current_authority(self) -> StoreCurrentAuthorityIdentity {
        self.expected_current_authority
    }
    pub const fn cutover_plan_fingerprint(self) -> [u8; 32] {
        self.cutover_plan_fingerprint
    }
    pub const fn candidate_media_identity(self) -> [u8; 32] {
        self.candidate_media_identity
    }
    pub const fn authority_delta_identity(self) -> [u8; 32] {
        self.authority_delta_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWriteFenceProviderReceipt {
    fence_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    observed_current_authority: StoreCurrentAuthorityIdentity,
    quiescence_established: bool,
}

impl RecoveryWriteFenceProviderReceipt {
    pub const fn observed(
        fence_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        observed_current_authority: StoreCurrentAuthorityIdentity,
        quiescence_established: bool,
    ) -> Self {
        Self {
            fence_identity,
            plan_fingerprint,
            observed_current_authority,
            quiescence_established,
        }
    }
}

pub trait RecoveryWriteFencePort {
    fn establish(
        &self,
        request: RecoveryWriteFenceRequest,
    ) -> Result<RecoveryWriteFenceProviderReceipt, RecoveryWriteFenceDenial>;
    fn release(
        &self,
        request: RecoveryWriteFenceReleaseRequest,
    ) -> Result<RecoveryWriteFenceReleaseProviderReceipt, RecoveryWriteFenceDenial>;
    fn recover_active(
        &self,
        request: RecoveryWriteFenceRecoveryRequest,
    ) -> Result<RecoveryWriteFenceProviderReceipt, RecoveryWriteFenceDenial>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWriteFenceRecoveryRequest {
    fence_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    expected_current_authority: StoreCurrentAuthorityIdentity,
    cutover_plan_fingerprint: [u8; 32],
    candidate_media_identity: [u8; 32],
}

impl RecoveryWriteFenceRecoveryRequest {
    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn expected_current_authority(self) -> StoreCurrentAuthorityIdentity {
        self.expected_current_authority
    }
    pub const fn cutover_plan_fingerprint(self) -> [u8; 32] {
        self.cutover_plan_fingerprint
    }
    pub const fn candidate_media_identity(self) -> [u8; 32] {
        self.candidate_media_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryWriteFenceDisposition {
    Readmitted,
    RejectedByAuthority,
    Abandoned,
    RetainedForForensics,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWriteFenceReleaseRequest {
    pub(crate) fence_identity: [u8; 32],
    pub(crate) plan_fingerprint: [u8; 32],
    pub(crate) disposition: RecoveryWriteFenceDisposition,
}

impl RecoveryWriteFenceReleaseRequest {
    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn disposition(self) -> RecoveryWriteFenceDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWriteFenceReleaseProviderReceipt {
    pub(crate) fence_identity: [u8; 32],
    pub(crate) plan_fingerprint: [u8; 32],
    pub(crate) released: bool,
}

impl RecoveryWriteFenceReleaseProviderReceipt {
    pub const fn observed(
        fence_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        released: bool,
    ) -> Self {
        Self {
            fence_identity,
            plan_fingerprint,
            released,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWriteFenceReleaseReceipt {
    pub(crate) fence_identity: [u8; 32],
    pub(crate) plan_fingerprint: [u8; 32],
    pub(crate) disposition: RecoveryWriteFenceDisposition,
}

impl RecoveryWriteFenceReleaseReceipt {
    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn disposition(self) -> RecoveryWriteFenceDisposition {
        self.disposition
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWriteFencePlan {
    fingerprint: [u8; 32],
    expected_current_authority: StoreCurrentAuthorityIdentity,
    cutover_plan_fingerprint: [u8; 32],
    candidate_media_identity: [u8; 32],
    authority_delta_identity: [u8; 32],
}

impl RecoveryWriteFencePlan {
    pub const fn fingerprint(self) -> [u8; 32] {
        self.fingerprint
    }
    pub const fn cutover_plan_fingerprint(self) -> [u8; 32] {
        self.cutover_plan_fingerprint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryWriteFenceReceipt {
    fence_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    cutover_plan_fingerprint: [u8; 32],
    fenced_authority: StoreCurrentAuthorityIdentity,
    candidate_media_identity: [u8; 32],
}

impl RecoveryWriteFenceReceipt {
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }
    pub const fn cutover_plan_fingerprint(self) -> [u8; 32] {
        self.cutover_plan_fingerprint
    }
    pub const fn fenced_authority(self) -> StoreCurrentAuthorityIdentity {
        self.fenced_authority
    }
    pub const fn candidate_media_identity(self) -> [u8; 32] {
        self.candidate_media_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentAuthorityReadmissionReceipt {
    authority_identity: StoreCurrentAuthorityIdentity,
    publication_identity: [u8; 32],
    write_fence_plan_fingerprint: [u8; 32],
    authority_posture: RecoveryAuthorityAdmissionPosture,
    admission_policy: RecoveryAuthorityAdmissionPolicy,
}

impl CurrentAuthorityReadmissionReceipt {
    pub const fn authority_identity(self) -> StoreCurrentAuthorityIdentity {
        self.authority_identity
    }
    pub const fn publication_identity(self) -> [u8; 32] {
        self.publication_identity
    }
    pub const fn authority_posture(self) -> RecoveryAuthorityAdmissionPosture {
        self.authority_posture
    }
    pub const fn admission_policy(self) -> RecoveryAuthorityAdmissionPolicy {
        self.admission_policy
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryAuthorityReadmissionDenial {
    StaleCurrentAuthority,
    PublicationMismatch,
    AdmissionPolicy(RecoveryAuthorityAdmissionPolicyDenial),
}

#[derive(Debug, Clone, Copy)]
pub struct RecoveryCutoverAuthorityOwner;

impl RecoveryCutoverAuthorityOwner {
    pub fn lower_write_fence(
        current: &StoreCurrentAuthorityWitness,
        cutover_plan_fingerprint: [u8; 32],
        candidate_media_identity: [u8; 32],
        authority_delta_identity: [u8; 32],
    ) -> Result<RecoveryWriteFencePlan, RecoveryWriteFenceDenial> {
        if cutover_plan_fingerprint == [0; 32]
            || candidate_media_identity == [0; 32]
            || authority_delta_identity == [0; 32]
        {
            return Err(RecoveryWriteFenceDenial::InvalidBinding);
        }
        let expected_current_authority = current.authority_identity();
        let mut digest = Sha256::new();
        digest.update(b"worth-store-recovery-write-fence-plan-v1");
        digest.update(expected_current_authority.fingerprint());
        digest.update(cutover_plan_fingerprint);
        digest.update(candidate_media_identity);
        digest.update(authority_delta_identity);
        Ok(RecoveryWriteFencePlan {
            fingerprint: digest.finalize().into(),
            expected_current_authority,
            cutover_plan_fingerprint,
            candidate_media_identity,
            authority_delta_identity,
        })
    }

    pub fn establish_write_fence(
        plan: RecoveryWriteFencePlan,
        current: &StoreCurrentAuthorityWitness,
        port: &impl RecoveryWriteFencePort,
    ) -> Result<RecoveryWriteFenceReceipt, RecoveryWriteFenceDenial> {
        if current.authority_identity() != plan.expected_current_authority {
            return Err(RecoveryWriteFenceDenial::StaleCurrentAuthority);
        }
        let provider = port.establish(RecoveryWriteFenceRequest {
            plan_fingerprint: plan.fingerprint,
            expected_current_authority: plan.expected_current_authority,
            cutover_plan_fingerprint: plan.cutover_plan_fingerprint,
            candidate_media_identity: plan.candidate_media_identity,
            authority_delta_identity: plan.authority_delta_identity,
        })?;
        if provider.plan_fingerprint != plan.fingerprint
            || provider.observed_current_authority != plan.expected_current_authority
        {
            return Err(RecoveryWriteFenceDenial::CutoverPlanMismatch);
        }
        if provider.fence_identity == [0; 32] || !provider.quiescence_established {
            return Err(RecoveryWriteFenceDenial::QuiescenceNotEstablished);
        }
        Ok(RecoveryWriteFenceReceipt {
            fence_identity: provider.fence_identity,
            plan_fingerprint: plan.fingerprint,
            cutover_plan_fingerprint: plan.cutover_plan_fingerprint,
            fenced_authority: plan.expected_current_authority,
            candidate_media_identity: plan.candidate_media_identity,
        })
    }

    pub fn readmit_published_recovery(
        current: &StoreCurrentAuthorityWitness,
        fence: RecoveryWriteFenceReceipt,
        publication_identity: [u8; 32],
        published_candidate_media_identity: [u8; 32],
        authority_posture: RecoveryAuthorityAdmissionPosture,
        admission_policy: RecoveryAuthorityAdmissionPolicy,
    ) -> Result<CurrentAuthorityReadmissionReceipt, RecoveryAuthorityReadmissionDenial> {
        if current.authority_identity() != fence.fenced_authority {
            return Err(RecoveryAuthorityReadmissionDenial::StaleCurrentAuthority);
        }
        if publication_identity == [0; 32]
            || published_candidate_media_identity != fence.candidate_media_identity
        {
            return Err(RecoveryAuthorityReadmissionDenial::PublicationMismatch);
        }
        admission_policy
            .validate(authority_posture)
            .map_err(RecoveryAuthorityReadmissionDenial::AdmissionPolicy)?;
        Ok(CurrentAuthorityReadmissionReceipt {
            authority_identity: current.authority_identity(),
            publication_identity,
            write_fence_plan_fingerprint: fence.plan_fingerprint,
            authority_posture,
            admission_policy,
        })
    }

    pub fn recover_active_write_fence(
        current: &StoreCurrentAuthorityWitness,
        fence_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        cutover_plan_fingerprint: [u8; 32],
        candidate_media_identity: [u8; 32],
        port: &impl RecoveryWriteFencePort,
    ) -> Result<RecoveryWriteFenceReceipt, RecoveryWriteFenceDenial> {
        if fence_identity == [0; 32]
            || plan_fingerprint == [0; 32]
            || cutover_plan_fingerprint == [0; 32]
            || candidate_media_identity == [0; 32]
        {
            return Err(RecoveryWriteFenceDenial::InvalidBinding);
        }
        let expected_current_authority = current.authority_identity();
        let provider = port.recover_active(RecoveryWriteFenceRecoveryRequest {
            fence_identity,
            plan_fingerprint,
            expected_current_authority,
            cutover_plan_fingerprint,
            candidate_media_identity,
        })?;
        if provider.fence_identity != fence_identity
            || provider.plan_fingerprint != plan_fingerprint
            || provider.observed_current_authority != expected_current_authority
        {
            return Err(RecoveryWriteFenceDenial::CutoverPlanMismatch);
        }
        if !provider.quiescence_established {
            return Err(RecoveryWriteFenceDenial::QuiescenceNotEstablished);
        }
        Ok(RecoveryWriteFenceReceipt {
            fence_identity,
            plan_fingerprint,
            cutover_plan_fingerprint,
            fenced_authority: expected_current_authority,
            candidate_media_identity,
        })
    }
}

#[cfg(test)]
#[path = "recovery_cutover_tests.rs"]
mod tests;
