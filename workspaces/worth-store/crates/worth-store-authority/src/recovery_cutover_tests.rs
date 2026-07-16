use super::*;

struct ReleasePort {
    returned_plan: [u8; 32],
    released: bool,
}

impl RecoveryWriteFencePort for ReleasePort {
    fn establish(
        &self,
        _request: RecoveryWriteFenceRequest,
    ) -> Result<RecoveryWriteFenceProviderReceipt, RecoveryWriteFenceDenial> {
        Err(RecoveryWriteFenceDenial::ProviderUnavailable)
    }

    fn release(
        &self,
        request: RecoveryWriteFenceReleaseRequest,
    ) -> Result<RecoveryWriteFenceReleaseProviderReceipt, RecoveryWriteFenceDenial> {
        Ok(RecoveryWriteFenceReleaseProviderReceipt::observed(
            request.fence_identity(),
            self.returned_plan,
            self.released,
        ))
    }

    fn recover_active(
        &self,
        _request: RecoveryWriteFenceRecoveryRequest,
    ) -> Result<RecoveryWriteFenceProviderReceipt, RecoveryWriteFenceDenial> {
        Err(RecoveryWriteFenceDenial::ProviderUnavailable)
    }
}

#[test]
fn fence_release_is_bound_to_the_established_plan() {
    let fence = fence();
    let port = ReleasePort {
        returned_plan: [9; 32],
        released: true,
    };
    assert_eq!(
        RecoveryCutoverAuthorityOwner::release_write_fence(
            fence,
            RecoveryWriteFenceDisposition::Readmitted,
            &port
        ),
        Err(RecoveryWriteFenceDenial::ReleaseReceiptMismatch)
    );
}

#[test]
fn provider_must_confirm_quiescence_was_released() {
    let fence = fence();
    let port = ReleasePort {
        returned_plan: fence.plan_fingerprint(),
        released: false,
    };
    assert_eq!(
        RecoveryCutoverAuthorityOwner::release_write_fence(
            fence,
            RecoveryWriteFenceDisposition::Readmitted,
            &port
        ),
        Err(RecoveryWriteFenceDenial::ReleaseRejected)
    );
}

fn fence() -> RecoveryWriteFenceReceipt {
    RecoveryWriteFenceReceipt {
        fence_identity: [4; 32],
        plan_fingerprint: [5; 32],
        cutover_plan_fingerprint: [6; 32],
        fenced_authority: StoreCurrentAuthorityIdentity::from_persisted_fingerprint([7; 32]),
        candidate_media_identity: [8; 32],
    }
}
