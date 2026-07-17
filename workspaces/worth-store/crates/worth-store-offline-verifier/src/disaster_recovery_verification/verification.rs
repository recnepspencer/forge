use std::io::Read;

use sha2::{Digest, Sha256};
use worth_store_replication::MaterializedDisasterRecoveryBundle;

use super::{
    closure_verification::verify_cross_component_closure, DisasterRecoveryClosureDenial,
    DisasterRecoveryVerificationPolicy, IndependentlyOpenedDisasterRecoveryBundle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DisasterRecoveryVerificationCounters {
    components_opened: u64,
    backend_bytes_read: u64,
    maximum_resident_buffer_bytes: u64,
    cross_component_closure_checks: u64,
    assumption_checks: u64,
}

impl DisasterRecoveryVerificationCounters {
    pub const fn components_opened(self) -> u64 {
        self.components_opened
    }

    pub const fn backend_bytes_read(self) -> u64 {
        self.backend_bytes_read
    }

    pub const fn maximum_resident_buffer_bytes(self) -> u64 {
        self.maximum_resident_buffer_bytes
    }

    pub const fn cross_component_closure_checks(self) -> u64 {
        self.cross_component_closure_checks
    }

    pub const fn assumption_checks(self) -> u64 {
        self.assumption_checks
    }
}

#[derive(Debug)]
pub enum DisasterRecoveryVerificationDenial {
    InvalidBufferBudget,
    BundleRootUnavailable,
    ComponentEscapesBundle,
    SymbolicLinkComponent,
    MissingComponent,
    ComponentLengthMismatch,
    ComponentDigestMismatch,
    CrossComponentClosure(DisasterRecoveryClosureDenial),
    CounterOverflow,
    UnsupportedFormatAssumption,
    UnsupportedBackendAssumption,
    Io(std::io::Error),
}

impl From<std::io::Error> for DisasterRecoveryVerificationDenial {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug)]
pub struct IndependentlyVerifiedDisasterRecoveryBundle {
    materialized: MaterializedDisasterRecoveryBundle,
    verification_identity: [u8; 32],
    counters: DisasterRecoveryVerificationCounters,
}

impl IndependentlyVerifiedDisasterRecoveryBundle {
    pub const fn materialized(&self) -> &MaterializedDisasterRecoveryBundle {
        &self.materialized
    }

    pub const fn verification_identity(&self) -> [u8; 32] {
        self.verification_identity
    }

    pub const fn counters(&self) -> DisasterRecoveryVerificationCounters {
        self.counters
    }
}

pub fn verify_disaster_recovery_bundle(
    opened: IndependentlyOpenedDisasterRecoveryBundle,
    resident_buffer_bytes: usize,
    policy: &DisasterRecoveryVerificationPolicy,
) -> Result<IndependentlyVerifiedDisasterRecoveryBundle, DisasterRecoveryVerificationDenial> {
    if resident_buffer_bytes == 0 {
        return Err(DisasterRecoveryVerificationDenial::InvalidBufferBudget);
    }
    let materialized = opened.materialized;
    let canonical_root = std::fs::canonicalize(materialized.root())
        .map_err(|_| DisasterRecoveryVerificationDenial::BundleRootUnavailable)?;
    let mut buffer = Vec::new();
    buffer
        .try_reserve_exact(resident_buffer_bytes)
        .map_err(|_| DisasterRecoveryVerificationDenial::InvalidBufferBudget)?;
    buffer.resize(resident_buffer_bytes, 0);
    let mut counters = DisasterRecoveryVerificationCounters {
        components_opened: 0,
        backend_bytes_read: 0,
        maximum_resident_buffer_bytes: u64::try_from(resident_buffer_bytes)
            .map_err(|_| DisasterRecoveryVerificationDenial::CounterOverflow)?,
        cross_component_closure_checks: 0,
        assumption_checks: 0,
    };
    let mut verification_digest = Sha256::new();
    verification_digest.update(b"worth-store-independent-dr-verification-v1");
    verification_digest.update(materialized.manifest_identity());
    for component in materialized.components() {
        verify_assumptions(component, policy, &mut counters)?;
        verify_component(
            &canonical_root,
            component,
            &mut buffer,
            &mut counters,
            &mut verification_digest,
        )?;
    }
    counters.cross_component_closure_checks = verify_cross_component_closure(&materialized)
        .map_err(DisasterRecoveryVerificationDenial::CrossComponentClosure)?;
    Ok(IndependentlyVerifiedDisasterRecoveryBundle {
        materialized,
        verification_identity: verification_digest.finalize().into(),
        counters,
    })
}

fn verify_assumptions(
    component: &worth_store_replication::DisasterRecoveryComponent,
    policy: &DisasterRecoveryVerificationPolicy,
    counters: &mut DisasterRecoveryVerificationCounters,
) -> Result<(), DisasterRecoveryVerificationDenial> {
    if !policy.supports_format(component) {
        return Err(DisasterRecoveryVerificationDenial::UnsupportedFormatAssumption);
    }
    if !policy.supports_backend(component) {
        return Err(DisasterRecoveryVerificationDenial::UnsupportedBackendAssumption);
    }
    counters.assumption_checks = counters
        .assumption_checks
        .checked_add(2)
        .ok_or(DisasterRecoveryVerificationDenial::CounterOverflow)?;
    Ok(())
}

fn verify_component(
    canonical_root: &std::path::Path,
    component: &worth_store_replication::DisasterRecoveryComponent,
    buffer: &mut [u8],
    counters: &mut DisasterRecoveryVerificationCounters,
    verification_digest: &mut Sha256,
) -> Result<(), DisasterRecoveryVerificationDenial> {
    let declared_path = canonical_root.join(component.relative_path());
    let metadata = std::fs::symlink_metadata(&declared_path)
        .map_err(|_| DisasterRecoveryVerificationDenial::MissingComponent)?;
    if metadata.file_type().is_symlink() {
        return Err(DisasterRecoveryVerificationDenial::SymbolicLinkComponent);
    }
    let canonical_path = std::fs::canonicalize(&declared_path)
        .map_err(|_| DisasterRecoveryVerificationDenial::MissingComponent)?;
    if !canonical_path.starts_with(canonical_root) {
        return Err(DisasterRecoveryVerificationDenial::ComponentEscapesBundle);
    }
    if metadata.len() != component.byte_length() {
        return Err(DisasterRecoveryVerificationDenial::ComponentLengthMismatch);
    }
    let mut file = std::fs::File::open(canonical_path)?;
    let mut component_digest = Sha256::new();
    let mut bytes_read = 0_u64;
    loop {
        let read = file.read(buffer)?;
        if read == 0 {
            break;
        }
        component_digest.update(&buffer[..read]);
        bytes_read = bytes_read
            .checked_add(
                u64::try_from(read)
                    .map_err(|_| DisasterRecoveryVerificationDenial::CounterOverflow)?,
            )
            .ok_or(DisasterRecoveryVerificationDenial::CounterOverflow)?;
    }
    let actual_digest: [u8; 32] = component_digest.finalize().into();
    if bytes_read != component.byte_length() {
        return Err(DisasterRecoveryVerificationDenial::ComponentLengthMismatch);
    }
    if actual_digest != component.expected_digest() {
        return Err(DisasterRecoveryVerificationDenial::ComponentDigestMismatch);
    }
    counters.components_opened = counters
        .components_opened
        .checked_add(1)
        .ok_or(DisasterRecoveryVerificationDenial::CounterOverflow)?;
    counters.backend_bytes_read = counters
        .backend_bytes_read
        .checked_add(bytes_read)
        .ok_or(DisasterRecoveryVerificationDenial::CounterOverflow)?;
    verification_digest.update([component.family() as u8]);
    verification_digest.update(component.relative_path().as_os_str().as_encoded_bytes());
    verification_digest.update(actual_digest);
    verification_digest.update(bytes_read.to_be_bytes());
    Ok(())
}
