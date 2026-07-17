use sha2::{Digest, Sha256};

use super::{OldPrimaryDivergenceDisposition, OldPrimaryRejoinPlan};
use crate::ReplicationPeerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OldPrimaryRejoinExecutionRequest {
    plan_fingerprint: [u8; 32],
    old_primary: ReplicationPeerId,
    promoted_primary: ReplicationPeerId,
    disposition: OldPrimaryDivergenceDisposition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldPrimaryRejoinExecutionDenial {
    OwnerRejected,
    BindingMismatch,
    MissingForensicRetention,
    UnexpectedRebootstrapTarget,
    MissingRebootstrapTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OldPrimaryRejoinReceipt {
    plan_fingerprint: [u8; 32],
    disposition: OldPrimaryDivergenceDisposition,
    forensic_retention_identity: Option<[u8; 32]>,
    rebootstrap_target_identity: Option<[u8; 32]>,
    receipt_identity: [u8; 32],
}

pub trait OldPrimaryRejoinExecutionPort {
    fn resolve_old_primary_divergence(
        &mut self,
        request: OldPrimaryRejoinExecutionRequest,
    ) -> Result<OldPrimaryRejoinReceipt, OldPrimaryRejoinExecutionDenial>;
}

impl OldPrimaryRejoinExecutionRequest {
    pub(super) fn from_plan(plan: &OldPrimaryRejoinPlan) -> Self {
        Self {
            plan_fingerprint: plan.fingerprint(),
            old_primary: plan.old_primary().clone(),
            promoted_primary: plan.promoted_primary().clone(),
            disposition: plan.disposition(),
        }
    }

    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn old_primary(&self) -> &ReplicationPeerId {
        &self.old_primary
    }
    pub const fn promoted_primary(&self) -> &ReplicationPeerId {
        &self.promoted_primary
    }
    pub const fn disposition(&self) -> OldPrimaryDivergenceDisposition {
        self.disposition
    }
}

impl OldPrimaryRejoinReceipt {
    pub fn from_rejoin_owner(
        request: &OldPrimaryRejoinExecutionRequest,
        forensic_retention_identity: Option<[u8; 32]>,
        rebootstrap_target_identity: Option<[u8; 32]>,
    ) -> Result<Self, OldPrimaryRejoinExecutionDenial> {
        validate_result(
            request.disposition,
            forensic_retention_identity,
            rebootstrap_target_identity,
        )?;
        let mut digest = Sha256::new();
        digest.update(b"worth-store-old-primary-rejoin-receipt-v1");
        digest.update(request.plan_fingerprint);
        digest.update([request.disposition as u8]);
        digest.update(forensic_retention_identity.unwrap_or([0; 32]));
        digest.update(rebootstrap_target_identity.unwrap_or([0; 32]));
        Ok(Self {
            plan_fingerprint: request.plan_fingerprint,
            disposition: request.disposition,
            forensic_retention_identity,
            rebootstrap_target_identity,
            receipt_identity: digest.finalize().into(),
        })
    }

    pub(super) fn validate_for_plan(
        &self,
        plan: &OldPrimaryRejoinPlan,
    ) -> Result<(), OldPrimaryRejoinExecutionDenial> {
        if self.plan_fingerprint != plan.fingerprint() || self.disposition != plan.disposition() {
            return Err(OldPrimaryRejoinExecutionDenial::BindingMismatch);
        }
        validate_result(
            self.disposition,
            self.forensic_retention_identity,
            self.rebootstrap_target_identity,
        )
    }

    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn disposition(&self) -> OldPrimaryDivergenceDisposition {
        self.disposition
    }
    pub const fn forensic_retention_identity(&self) -> Option<[u8; 32]> {
        self.forensic_retention_identity
    }
    pub const fn rebootstrap_target_identity(&self) -> Option<[u8; 32]> {
        self.rebootstrap_target_identity
    }
    pub const fn receipt_identity(&self) -> [u8; 32] {
        self.receipt_identity
    }
}

impl OldPrimaryRejoinPlan {
    pub fn execute(
        &self,
        port: &mut impl OldPrimaryRejoinExecutionPort,
    ) -> Result<OldPrimaryRejoinReceipt, OldPrimaryRejoinExecutionDenial> {
        execute_old_primary_rejoin(self, port)
    }
}

pub(super) fn execute_old_primary_rejoin(
    plan: &OldPrimaryRejoinPlan,
    port: &mut impl OldPrimaryRejoinExecutionPort,
) -> Result<OldPrimaryRejoinReceipt, OldPrimaryRejoinExecutionDenial> {
    let request = OldPrimaryRejoinExecutionRequest::from_plan(plan);
    let receipt = port.resolve_old_primary_divergence(request)?;
    receipt.validate_for_plan(plan)?;
    Ok(receipt)
}

fn validate_result(
    disposition: OldPrimaryDivergenceDisposition,
    forensic: Option<[u8; 32]>,
    target: Option<[u8; 32]>,
) -> Result<(), OldPrimaryRejoinExecutionDenial> {
    if forensic == Some([0; 32]) {
        return Err(OldPrimaryRejoinExecutionDenial::MissingForensicRetention);
    }
    if target == Some([0; 32]) {
        return Err(OldPrimaryRejoinExecutionDenial::MissingRebootstrapTarget);
    }
    match disposition {
        OldPrimaryDivergenceDisposition::RetainForForensics => {
            if forensic.is_none() {
                return Err(OldPrimaryRejoinExecutionDenial::MissingForensicRetention);
            }
            if target.is_some() {
                return Err(OldPrimaryRejoinExecutionDenial::UnexpectedRebootstrapTarget);
            }
        }
        OldPrimaryDivergenceDisposition::AuthorizedDiscard => {
            if target.is_some() {
                return Err(OldPrimaryRejoinExecutionDenial::UnexpectedRebootstrapTarget);
            }
        }
        OldPrimaryDivergenceDisposition::RebootstrapAfterForensicRetention => {
            if forensic.is_none() {
                return Err(OldPrimaryRejoinExecutionDenial::MissingForensicRetention);
            }
            if target.is_none() {
                return Err(OldPrimaryRejoinExecutionDenial::MissingRebootstrapTarget);
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        DivergentReplicaHistoryReport, ReplicaHistoryObservation, ReplicaRecoveryFrontier,
        ReplicationLineageIdentity, ReplicationRejoinOwner,
    };

    #[test]
    fn rebootstrap_completion_requires_forensic_retention_and_a_new_target() {
        let plan = plan(OldPrimaryDivergenceDisposition::RebootstrapAfterForensicRetention);
        let mut incomplete = RejoinPort {
            forensic: Some([7; 32]),
            target: None,
        };
        assert_eq!(
            plan.execute(&mut incomplete),
            Err(OldPrimaryRejoinExecutionDenial::MissingRebootstrapTarget)
        );

        let mut complete = RejoinPort {
            forensic: Some([7; 32]),
            target: Some([8; 32]),
        };
        let receipt = plan.execute(&mut complete).unwrap();
        assert_eq!(receipt.forensic_retention_identity(), Some([7; 32]));
        assert_eq!(receipt.rebootstrap_target_identity(), Some([8; 32]));
    }

    #[test]
    fn forensic_retention_cannot_smuggle_an_unverified_rebootstrap_target() {
        let plan = plan(OldPrimaryDivergenceDisposition::RetainForForensics);
        let mut port = RejoinPort {
            forensic: Some([7; 32]),
            target: Some([8; 32]),
        };
        assert_eq!(
            plan.execute(&mut port),
            Err(OldPrimaryRejoinExecutionDenial::UnexpectedRebootstrapTarget)
        );
    }

    #[test]
    fn plan_identity_changes_when_the_divergence_basis_changes() {
        let first = plan_with_media(OldPrimaryDivergenceDisposition::RetainForForensics, [3; 32]);
        let second = plan_with_media(OldPrimaryDivergenceDisposition::RetainForForensics, [9; 32]);
        assert_ne!(first.fingerprint(), second.fingerprint());
    }

    struct RejoinPort {
        forensic: Option<[u8; 32]>,
        target: Option<[u8; 32]>,
    }

    impl OldPrimaryRejoinExecutionPort for RejoinPort {
        fn resolve_old_primary_divergence(
            &mut self,
            request: OldPrimaryRejoinExecutionRequest,
        ) -> Result<OldPrimaryRejoinReceipt, OldPrimaryRejoinExecutionDenial> {
            OldPrimaryRejoinReceipt::from_rejoin_owner(&request, self.forensic, self.target)
        }
    }

    fn plan(disposition: OldPrimaryDivergenceDisposition) -> OldPrimaryRejoinPlan {
        plan_with_media(disposition, [3; 32])
    }

    fn plan_with_media(
        disposition: OldPrimaryDivergenceDisposition,
        durable_media_identity: [u8; 32],
    ) -> OldPrimaryRejoinPlan {
        let old_primary = ReplicationPeerId::from_declared_peer("old-primary").unwrap();
        let promoted = ReplicationPeerId::from_declared_peer("promoted-primary").unwrap();
        let old_lineage = ReplicationLineageIdentity::from_declared_lineage("old").unwrap();
        let current_lineage = ReplicationLineageIdentity::from_declared_lineage("current").unwrap();
        let divergence = DivergentReplicaHistoryReport::classify(
            ReplicaHistoryObservation {
                peer: old_primary.clone(),
                lineage: old_lineage,
                frontier: ReplicaRecoveryFrontier::admit(9, 8, 7, 6, 1).unwrap(),
                blob_closure_complete: true,
                authoritative_media_admissible: true,
                durable_media_identity,
            },
            current_lineage,
        );
        ReplicationRejoinOwner::plan(
            old_primary,
            promoted,
            divergence,
            disposition,
            Some([4; 32]),
        )
        .unwrap()
    }
}
