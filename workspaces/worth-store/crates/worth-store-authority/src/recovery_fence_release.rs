use crate::{
    RecoveryCutoverAuthorityOwner, RecoveryWriteFenceDenial, RecoveryWriteFenceDisposition,
    RecoveryWriteFencePort, RecoveryWriteFenceReceipt, RecoveryWriteFenceReleaseReceipt,
    RecoveryWriteFenceReleaseRequest, StoreCurrentAuthorityWitness,
};

impl RecoveryCutoverAuthorityOwner {
    pub fn release_write_fence(
        fence: RecoveryWriteFenceReceipt,
        disposition: RecoveryWriteFenceDisposition,
        port: &impl RecoveryWriteFencePort,
    ) -> Result<RecoveryWriteFenceReleaseReceipt, RecoveryWriteFenceDenial> {
        release(
            fence.fence_identity(),
            fence.plan_fingerprint(),
            disposition,
            port,
        )
    }

    pub fn release_recovered_write_fence(
        current: &StoreCurrentAuthorityWitness,
        fence_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        disposition: RecoveryWriteFenceDisposition,
        port: &impl RecoveryWriteFencePort,
    ) -> Result<RecoveryWriteFenceReleaseReceipt, RecoveryWriteFenceDenial> {
        let _authority_binding = current.authority_identity();
        release(fence_identity, plan_fingerprint, disposition, port)
    }

    pub fn release_terminal_write_fence(
        fence_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        disposition: RecoveryWriteFenceDisposition,
        port: &impl RecoveryWriteFencePort,
    ) -> Result<RecoveryWriteFenceReleaseReceipt, RecoveryWriteFenceDenial> {
        release(fence_identity, plan_fingerprint, disposition, port)
    }
}

fn release(
    fence_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    disposition: RecoveryWriteFenceDisposition,
    port: &impl RecoveryWriteFencePort,
) -> Result<RecoveryWriteFenceReleaseReceipt, RecoveryWriteFenceDenial> {
    if fence_identity == [0; 32] || plan_fingerprint == [0; 32] {
        return Err(RecoveryWriteFenceDenial::InvalidBinding);
    }
    let provider = port.release(RecoveryWriteFenceReleaseRequest {
        fence_identity,
        plan_fingerprint,
        disposition,
    })?;
    if provider.fence_identity != fence_identity || provider.plan_fingerprint != plan_fingerprint {
        return Err(RecoveryWriteFenceDenial::ReleaseReceiptMismatch);
    }
    if !provider.released {
        return Err(RecoveryWriteFenceDenial::ReleaseRejected);
    }
    Ok(RecoveryWriteFenceReleaseReceipt {
        fence_identity,
        plan_fingerprint,
        disposition,
    })
}
