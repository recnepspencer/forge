use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use sha2::{Digest, Sha256};
use worth_store_authority::{
    ControlStoreFencingPort, ControlStoreFencingProviderDenial, ControlStoreSelectionCoordinates,
    ExternalFenceGrant, ExternalServeLeaseGrant, OperationalFencingAuthorityPort,
    OperationalFencingProviderDenial, PrimaryServeLeaseRequest, PromotionFenceOperationIdentity,
    StoreCurrentAuthorityIdentity,
};
use worth_store_replication::{
    durable_replica_target_identity, LoweredReplicaBootstrapPlan, OldPrimaryRejoinExecutionDenial,
    OldPrimaryRejoinExecutionPort, OldPrimaryRejoinExecutionRequest, OldPrimaryRejoinReceipt,
    ReplicaBootstrapDenial, ReplicaBootstrapExecutionCounters, ReplicaBootstrapExecutionPort,
    ReplicaBootstrapExecutionReport, ReplicaRecoveryFrontier,
};

pub struct ScenarioBootstrapOwner {
    source: PathBuf,
    target: PathBuf,
    frontier: ReplicaRecoveryFrontier,
}

impl ScenarioBootstrapOwner {
    pub fn new(source: &Path, target: &Path, frontier: ReplicaRecoveryFrontier) -> Self {
        Self {
            source: source.to_path_buf(),
            target: target.to_path_buf(),
            frontier,
        }
    }
}

impl ReplicaBootstrapExecutionPort for ScenarioBootstrapOwner {
    fn execute_replica_bootstrap(
        &mut self,
        plan: &LoweredReplicaBootstrapPlan,
    ) -> Result<ReplicaBootstrapExecutionReport, ReplicaBootstrapDenial> {
        let (bytes, requests, maximum_resident) = copy_tree_bounded(&self.source, &self.target)?;
        let target_identity = durable_replica_target_identity(&self.target)?;
        let counters =
            ReplicaBootstrapExecutionCounters::measured(bytes, bytes, requests, maximum_resident)
                .ok_or(ReplicaBootstrapDenial::ExecutionFailed)?;
        Ok(ReplicaBootstrapExecutionReport::from_replication_owner(
            plan.source_lease_identity(),
            self.frontier,
            target_identity,
            counters,
        ))
    }
}

fn copy_tree_bounded(
    source: &Path,
    target: &Path,
) -> Result<(u64, u64, u64), ReplicaBootstrapDenial> {
    const BUFFER_BYTES: usize = 64 * 1024;
    std::fs::create_dir_all(target).map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
    let mut pending = vec![source.to_path_buf()];
    let mut total = 0_u64;
    let mut requests = 0_u64;
    let mut buffer = vec![0; BUFFER_BYTES];
    while let Some(directory) = pending.pop() {
        for entry in
            std::fs::read_dir(&directory).map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?
        {
            let path = entry
                .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?
                .path();
            let metadata = std::fs::symlink_metadata(&path)
                .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
            if metadata.file_type().is_symlink() {
                return Err(ReplicaBootstrapDenial::ExecutionFailed);
            }
            let relative = path
                .strip_prefix(source)
                .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
            let output = target.join(relative);
            if metadata.is_dir() {
                std::fs::create_dir_all(&output)
                    .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
                pending.push(path);
            } else if metadata.is_file() {
                let mut input = std::fs::File::open(path)
                    .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
                let mut output = std::fs::File::create(output)
                    .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
                loop {
                    let read = std::io::Read::read(&mut input, &mut buffer)
                        .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
                    if read == 0 {
                        break;
                    }
                    std::io::Write::write_all(&mut output, &buffer[..read])
                        .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
                    total = total
                        .checked_add(read as u64)
                        .ok_or(ReplicaBootstrapDenial::ExecutionFailed)?;
                    requests = requests
                        .checked_add(1)
                        .ok_or(ReplicaBootstrapDenial::ExecutionFailed)?;
                }
                std::io::Write::flush(&mut output)
                    .map_err(|_| ReplicaBootstrapDenial::ExecutionFailed)?;
            } else {
                return Err(ReplicaBootstrapDenial::ExecutionFailed);
            }
        }
    }
    Ok((total, requests.max(1), BUFFER_BYTES as u64))
}

#[derive(Debug)]
pub struct ScenarioFencingProvider {
    coordinates: ControlStoreSelectionCoordinates,
    provider_identity: [u8; 32],
    fences: Mutex<HashMap<PromotionFenceOperationIdentity, ExternalFenceGrant>>,
}

impl ScenarioFencingProvider {
    pub fn for_current_prefix(control: &crate::OperationalControlStore) -> Self {
        let coordinates = control
            .observe_selection_coordinates()
            .expect("control selection coordinates")
            .expect("durable control prefix");
        let mut identity = Sha256::new();
        identity.update(b"worth-store-s10-scenario-fencing-provider-v1");
        identity.update(coordinates.media_identity_fingerprint());
        identity.update(coordinates.generation().get().to_be_bytes());
        identity.update(coordinates.prefix_digest());
        Self {
            coordinates,
            provider_identity: identity.finalize().into(),
            fences: Mutex::new(HashMap::new()),
        }
    }
}

impl ControlStoreFencingPort for ScenarioFencingProvider {
    fn selected_control_store(
        &self,
        _current_authority: StoreCurrentAuthorityIdentity,
    ) -> Result<ControlStoreSelectionCoordinates, ControlStoreFencingProviderDenial> {
        Ok(self.coordinates)
    }
}

impl OperationalFencingAuthorityPort for ScenarioFencingProvider {
    fn acquire_primary_serve_lease(
        &self,
        request: PrimaryServeLeaseRequest,
    ) -> Result<ExternalServeLeaseGrant, OperationalFencingProviderDenial> {
        Ok(ExternalServeLeaseGrant::from_provider(
            digest(b"s10-serve-lease", request.minimum_epoch_exclusive()),
            request.minimum_epoch_exclusive() + 1,
            request.requested_until_tick(),
            self.provider_identity,
        ))
    }

    fn renew_primary_serve_lease(
        &self,
        _current_token: [u8; 32],
        request: PrimaryServeLeaseRequest,
    ) -> Result<ExternalServeLeaseGrant, OperationalFencingProviderDenial> {
        self.acquire_primary_serve_lease(request)
    }

    fn revoke_and_advance_epoch(
        &self,
        old_lease_token: [u8; 32],
        minimum_epoch_exclusive: u64,
        operation_identity: PromotionFenceOperationIdentity,
    ) -> Result<ExternalFenceGrant, OperationalFencingProviderDenial> {
        let mut fences = self.fences.lock().expect("scenario fence registry");
        Ok(*fences.entry(operation_identity).or_insert_with(|| {
            ExternalFenceGrant::from_provider(
                old_lease_token,
                minimum_epoch_exclusive + 1,
                self.provider_identity,
                digest(b"s10-fence", minimum_epoch_exclusive),
                operation_identity,
            )
        }))
    }

    fn recover_fence(
        &self,
        operation_identity: PromotionFenceOperationIdentity,
    ) -> Result<Option<ExternalFenceGrant>, OperationalFencingProviderDenial> {
        Ok(self
            .fences
            .lock()
            .expect("scenario fence registry")
            .get(&operation_identity)
            .copied())
    }
}

pub struct ScenarioPromotionPublication;

impl crate::ReplicaPromotionPublicationPort for ScenarioPromotionPublication {
    fn publish_promoted_replica(
        &mut self,
        request: crate::ReplicaPromotionPublicationRequest,
    ) -> Result<crate::ReplicaPromotionPublicationReceipt, crate::ReplicaPromotionPublicationDenial>
    {
        let mut identity = Sha256::new();
        identity.update(b"worth-store-s10-scenario-promotion-publication-v1");
        identity.update(request.receipt_identity());
        identity.update(request.target_identity());
        identity.update(request.verification_identity());
        identity.update(request.fence_identity());
        identity.update(request.promoted_epoch().to_be_bytes());
        Ok(
            crate::ReplicaPromotionPublicationReceipt::from_publication_owner(
                identity.finalize().into(),
                request.target_identity(),
                request.promoted_epoch(),
            ),
        )
    }
}

pub struct ScenarioOldPrimaryRejoinOwner;

impl OldPrimaryRejoinExecutionPort for ScenarioOldPrimaryRejoinOwner {
    fn resolve_old_primary_divergence(
        &mut self,
        request: OldPrimaryRejoinExecutionRequest,
    ) -> Result<OldPrimaryRejoinReceipt, OldPrimaryRejoinExecutionDenial> {
        let forensic = bound_identity(b"s10-old-primary-forensics", &request);
        let target = bound_identity(b"s10-old-primary-rebootstrap", &request);
        OldPrimaryRejoinReceipt::from_rejoin_owner(&request, Some(forensic), Some(target))
    }
}

fn bound_identity(domain: &[u8], request: &OldPrimaryRejoinExecutionRequest) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update(request.plan_fingerprint());
    digest.update(request.old_primary().as_str().as_bytes());
    digest.update(request.promoted_primary().as_str().as_bytes());
    digest.finalize().into()
}

fn digest(domain: &[u8], value: u64) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update(value.to_be_bytes());
    digest.finalize().into()
}
