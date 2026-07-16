use sha2::{Digest, Sha256};

use crate::{DivergentReplicaHistoryReport, ReplicaHistoryClassification, ReplicationPeerId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldPrimaryDivergenceDisposition {
    RetainForForensics,
    AuthorizedDiscard,
    RebootstrapAfterForensicRetention,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OldPrimaryRejoinDenial {
    HistoryNotDivergent,
    PeerMismatch,
    MissingDispositionAuthorization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OldPrimaryRejoinPlan {
    old_primary: ReplicationPeerId,
    promoted_primary: ReplicationPeerId,
    divergence: DivergentReplicaHistoryReport,
    disposition: OldPrimaryDivergenceDisposition,
    authorization_fingerprint: Option<[u8; 32]>,
    fingerprint: [u8; 32],
}

impl OldPrimaryRejoinPlan {
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    pub const fn disposition(&self) -> OldPrimaryDivergenceDisposition {
        self.disposition
    }

    pub const fn divergence(&self) -> &DivergentReplicaHistoryReport {
        &self.divergence
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReplicationRejoinOwner;

impl ReplicationRejoinOwner {
    pub fn plan(
        old_primary: ReplicationPeerId,
        promoted_primary: ReplicationPeerId,
        divergence: DivergentReplicaHistoryReport,
        disposition: OldPrimaryDivergenceDisposition,
        authorization_fingerprint: Option<[u8; 32]>,
    ) -> Result<OldPrimaryRejoinPlan, OldPrimaryRejoinDenial> {
        if divergence.observation().peer() != &old_primary {
            return Err(OldPrimaryRejoinDenial::PeerMismatch);
        }
        if divergence.classification() != ReplicaHistoryClassification::Divergent {
            return Err(OldPrimaryRejoinDenial::HistoryNotDivergent);
        }
        if matches!(disposition, OldPrimaryDivergenceDisposition::AuthorizedDiscard)
            && authorization_fingerprint.is_none()
        {
            return Err(OldPrimaryRejoinDenial::MissingDispositionAuthorization);
        }
        let fingerprint = rejoin_fingerprint(
            &old_primary,
            &promoted_primary,
            disposition,
            authorization_fingerprint,
        );
        Ok(OldPrimaryRejoinPlan {
            old_primary,
            promoted_primary,
            divergence,
            disposition,
            authorization_fingerprint,
            fingerprint,
        })
    }
}

fn rejoin_fingerprint(
    old_primary: &ReplicationPeerId,
    promoted_primary: &ReplicationPeerId,
    disposition: OldPrimaryDivergenceDisposition,
    authorization_fingerprint: Option<[u8; 32]>,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-old-primary-rejoin-plan-v1");
    digest.update(old_primary.as_str().as_bytes());
    digest.update(promoted_primary.as_str().as_bytes());
    digest.update([disposition as u8]);
    digest.update(authorization_fingerprint.unwrap_or([0; 32]));
    digest.finalize().into()
}
