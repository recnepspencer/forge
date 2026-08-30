use worth_store::physical_runtime::{
    recovery_wal::WalSegmentArtifactIdentity, ObservedWalArtifact,
};
use worth_store_physical_format::store_namespace::{NamespaceEntryType, StableStoreIdentity};
use worth_store_recovery_physics::{
    PhysicalRecoveryResidue, PhysicalRecoveryResidueKind, PhysicalWalSegmentCandidate,
};

use crate::entry::{PhysicalRecoveryWalIntegrityDenial, PhysicalRecoveryWalIntegrityObservation};
use crate::integrity_ingress::RecoveryIntegrityIngressCounters;

mod admission;
mod admitted_inventory;
mod conclusion;
pub(super) mod observation_projection;

pub(crate) use admitted_inventory::AdmittedWalInventory;

pub(super) struct WalDiscoveryInventory {
    pub candidates: Vec<PhysicalWalSegmentCandidate>,
    pub admitted: AdmittedWalInventory,
    pub residue: Vec<PhysicalRecoveryResidue>,
    pub corruptions: Vec<PhysicalRecoveryWalIntegrityDenial>,
    pub observations: Vec<PhysicalRecoveryWalIntegrityObservation>,
    pub ingress: RecoveryIntegrityIngressCounters,
    pub canonical_segments: u64,
    pub frames_scanned: u64,
    pub valid_frames: u64,
    pub valid_bytes: u64,
    pub observed_bytes: u64,
    pub torn_suffix_frames: u64,
    pub torn_suffix_bytes: u64,
}

pub(super) enum WalDiscoveryInventoryDenial {
    CounterOverflow,
    FrameLimitExceeded { observed: u64, admitted: u64 },
    SourceBinding,
}

pub(super) fn discover_wal_inventory(
    owner: &worth_store::physical_runtime::PhysicalRecoveryCoordination,
    observed: Vec<ObservedWalArtifact>,
    store: StableStoreIdentity,
    maximum_frames: u64,
) -> Result<WalDiscoveryInventory, WalDiscoveryInventoryDenial> {
    let observed_bytes = observed
        .iter()
        .try_fold(0_u64, |total, artifact| {
            total.checked_add(artifact.bytes().map_or(0, |bytes| bytes.len() as u64))
        })
        .ok_or(WalDiscoveryInventoryDenial::CounterOverflow)?;
    let (mut canonical, mut residue) = partition_entries(observed);
    canonical.sort_unstable_by_key(|(identity, _)| *identity);
    let canonical_count = canonical.len();
    let mut inventory = WalDiscoveryInventory {
        candidates: Vec::new(),
        admitted: AdmittedWalInventory::default(),
        residue: Vec::new(),
        corruptions: Vec::new(),
        observations: Vec::new(),
        ingress: RecoveryIntegrityIngressCounters::default(),
        canonical_segments: canonical_count as u64,
        frames_scanned: 0,
        valid_frames: 0,
        valid_bytes: 0,
        observed_bytes,
        torn_suffix_frames: 0,
        torn_suffix_bytes: 0,
    };
    for (index, (identity, artifact)) in canonical.into_iter().enumerate() {
        let remaining = maximum_frames
            .checked_sub(inventory.frames_scanned)
            .ok_or(WalDiscoveryInventoryDenial::CounterOverflow)?;
        let transcript = admission::admit_segment(owner, identity, artifact, store, remaining)
            .map_err(|denial| match denial {
                admission::WalSegmentAdmissionDenial::CounterOverflow => {
                    WalDiscoveryInventoryDenial::CounterOverflow
                }
                admission::WalSegmentAdmissionDenial::FrameLimitExceeded { observed, admitted } => {
                    WalDiscoveryInventoryDenial::FrameLimitExceeded {
                        observed: inventory.frames_scanned.saturating_add(observed),
                        admitted: inventory.frames_scanned.saturating_add(admitted),
                    }
                }
                admission::WalSegmentAdmissionDenial::SourceBinding => {
                    WalDiscoveryInventoryDenial::SourceBinding
                }
            })?;
        let policy_attempts = if transcript.observed_bytes == 0 {
            0
        } else {
            transcript.counters.attempted
        };
        let attempted = checked_add(inventory.frames_scanned, policy_attempts)?;
        if attempted > maximum_frames {
            return Err(WalDiscoveryInventoryDenial::FrameLimitExceeded {
                observed: attempted,
                admitted: maximum_frames,
            });
        }
        inventory.ingress = inventory
            .ingress
            .checked_add(transcript.counters)
            .ok_or(WalDiscoveryInventoryDenial::CounterOverflow)?;
        let terminal = index + 1 == canonical_count;
        let conclusion = conclusion::conclude_segment(owner, transcript, terminal)
            .ok_or(WalDiscoveryInventoryDenial::CounterOverflow)?;
        inventory.frames_scanned = attempted;
        inventory.valid_frames = checked_add(inventory.valid_frames, conclusion.valid_frames)?;
        inventory.valid_bytes = checked_add(inventory.valid_bytes, conclusion.valid_bytes)?;
        inventory.torn_suffix_frames = checked_add(
            inventory.torn_suffix_frames,
            u64::from(conclusion.torn_bytes != 0),
        )?;
        inventory.torn_suffix_bytes =
            checked_add(inventory.torn_suffix_bytes, conclusion.torn_bytes)?;
        residue.extend(conclusion.residue);
        inventory.corruptions.extend(conclusion.corruptions);
        inventory.observations.extend(conclusion.observations);
        if let Some(candidate) = conclusion.candidate {
            inventory.candidates.push(candidate);
        }
        if let Some(admitted) = conclusion.admitted {
            inventory.admitted.push(admitted);
        }
    }
    inventory.residue = residue;
    Ok(inventory)
}

fn partition_entries(
    observed: Vec<ObservedWalArtifact>,
) -> (
    Vec<(WalSegmentArtifactIdentity, ObservedWalArtifact)>,
    Vec<PhysicalRecoveryResidue>,
) {
    let mut canonical = Vec::new();
    let mut residue = Vec::new();
    for artifact in observed {
        let name = artifact.name().to_string_lossy().into_owned();
        if artifact.entry_type() != NamespaceEntryType::RegularFile {
            residue.push(PhysicalRecoveryResidue::new(
                name,
                PhysicalRecoveryResidueKind::NonRegularWalEntry,
            ));
        } else if let Some(identity) = WalSegmentArtifactIdentity::parse(&name) {
            canonical.push((identity, artifact));
        } else {
            residue.push(PhysicalRecoveryResidue::new(
                name,
                PhysicalRecoveryResidueKind::NonCanonicalWalArtifact,
            ));
        }
    }
    (canonical, residue)
}

fn checked_add(left: u64, right: u64) -> Result<u64, WalDiscoveryInventoryDenial> {
    left.checked_add(right)
        .ok_or(WalDiscoveryInventoryDenial::CounterOverflow)
}
