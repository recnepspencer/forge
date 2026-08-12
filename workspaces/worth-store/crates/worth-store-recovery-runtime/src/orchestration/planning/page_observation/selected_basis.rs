use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use worth_store_physical_format::{
    CurrentPhysicalRecordPlacement, DurablePhysicalRootManifest, PhysicalRecordFormatDeclaration,
};
use worth_store_recovery_physics::{PhysicalRedoTarget, PhysicalRedoTargetIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ArtifactReadCeiling {
    pub(crate) addressed_reads: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArtifactReadCeilingDenial {
    ManifestEntriesExhausted,
    Overflow,
}

pub(crate) fn artifact_read_ceiling(
    placements: &[CurrentPhysicalRecordPlacement],
    targets: &[PhysicalRedoTarget],
    maximum_manifest_entries: u64,
    retained_fallback: bool,
) -> Result<ArtifactReadCeiling, ArtifactReadCeilingDenial> {
    let inline_locations = placements
        .iter()
        .filter_map(|placement| match placement {
            CurrentPhysicalRecordPlacement::Inline(inline) => {
                Some((inline.segment().get(), inline.page().get()))
            }
            CurrentPhysicalRecordPlacement::Extent(_) => None,
        })
        .collect::<BTreeSet<_>>();
    let extent_locations = placements
        .iter()
        .filter_map(|placement| match placement {
            CurrentPhysicalRecordPlacement::Extent(extent) => Some(extent.extent().get()),
            CurrentPhysicalRecordPlacement::Inline(_) => None,
        })
        .collect::<BTreeSet<_>>();
    if maximum_manifest_entries == 0 {
        return Err(ArtifactReadCeilingDenial::ManifestEntriesExhausted);
    }
    let mut inline_reads = BTreeSet::new();
    let mut extent_reads = BTreeSet::new();
    let mut extent_manifests = BTreeSet::new();
    for target in targets {
        match target.identity() {
            PhysicalRedoTargetIdentity::InlinePage { segment, page, .. }
                if inline_locations.contains(&(segment, page)) =>
            {
                inline_reads.insert((segment, page));
            }
            PhysicalRedoTargetIdentity::ExtentChunk { extent, chunk, .. }
                if extent_locations.contains(&extent) =>
            {
                extent_reads.insert((extent, chunk));
                extent_manifests.insert(extent);
            }
            _ => {}
        }
    }
    let selected_source_reads = maximum_manifest_entries
        .checked_add(1 + u64::from(retained_fallback))
        .ok_or(ArtifactReadCeilingDenial::Overflow)?;
    selected_source_reads
        .checked_add(inline_reads.len() as u64)
        .and_then(|value| value.checked_add(extent_reads.len() as u64))
        .and_then(|value| value.checked_add(extent_manifests.len() as u64))
        .map(|ceiling| ArtifactReadCeiling {
            addressed_reads: ceiling,
        })
        .ok_or(ArtifactReadCeilingDenial::Overflow)
}

#[cfg(test)]
mod tests {
    use super::{artifact_read_ceiling, ArtifactReadCeilingDenial};

    #[test]
    fn selected_source_inventory_cannot_read_after_entry_exhaustion() {
        assert_eq!(
            artifact_read_ceiling(&[], &[], 0, false),
            Err(ArtifactReadCeilingDenial::ManifestEntriesExhausted)
        );
        assert_eq!(
            artifact_read_ceiling(&[], &[], 1, false)
                .unwrap()
                .addressed_reads,
            2
        );
    }
}

pub(super) fn selected_absence_identity(
    root: &DurablePhysicalRootManifest,
    placements: &[CurrentPhysicalRecordPlacement],
    format: PhysicalRecordFormatDeclaration,
) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.recovery.selected-root-absence.v1");
    digest.update(root.encode(format));
    for placement in placements {
        digest.update(placement.record().allocation_epoch());
        digest.update(placement.record().ordinal().to_le_bytes());
        match placement {
            CurrentPhysicalRecordPlacement::Inline(inline) => {
                digest.update([1]);
                digest.update(inline.segment().get().to_le_bytes());
                digest.update(inline.segment_generation().to_le_bytes());
                digest.update(inline.page().get().to_le_bytes());
                digest.update(inline.page_generation().to_le_bytes());
                digest.update(inline.slot().get().to_le_bytes());
                digest.update(inline.slot_generation().to_le_bytes());
            }
            CurrentPhysicalRecordPlacement::Extent(extent) => {
                digest.update([2]);
                digest.update(extent.extent().get().to_le_bytes());
                digest.update(extent.extent_generation().to_le_bytes());
            }
        }
    }
    digest.finalize().into()
}
