use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use worth_store_physical_format::{
    CurrentPhysicalRecordPlacement, DurablePhysicalRootManifest, PhysicalRecordFormatDeclaration,
};
use worth_store_recovery_physics::{PhysicalRedoTarget, PhysicalRedoTargetIdentity};

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
    let mut inline_reads = 0_u64;
    let mut extent_reads = 0_u64;
    let mut has_absent = false;
    let mut extent_manifests = BTreeSet::new();
    for target in targets {
        match target.identity() {
            PhysicalRedoTargetIdentity::InlinePage { segment, page, .. }
                if inline_locations.contains(&(segment, page)) =>
            {
                inline_reads = inline_reads
                    .checked_add(1)
                    .ok_or(ArtifactReadCeilingDenial::Overflow)?;
            }
            PhysicalRedoTargetIdentity::ExtentChunk { extent, .. }
                if extent_locations.contains(&extent) =>
            {
                extent_reads = extent_reads
                    .checked_add(1)
                    .ok_or(ArtifactReadCeilingDenial::Overflow)?;
                extent_manifests.insert(extent);
            }
            _ => has_absent = true,
        }
    }
    let routed_blocks = routed_block_ceiling(inline_reads, maximum_manifest_entries)?;
    let absence_reads = if has_absent {
        if maximum_manifest_entries == 0 {
            return Err(ArtifactReadCeilingDenial::ManifestEntriesExhausted);
        }
        maximum_manifest_entries
            .checked_add(1)
            .ok_or(ArtifactReadCeilingDenial::Overflow)?
    } else {
        0
    };
    let publication_inventory_reads = maximum_manifest_entries
        .checked_add(1)
        .ok_or(ArtifactReadCeilingDenial::Overflow)?;
    routed_blocks
        .checked_add(inline_reads)
        .and_then(|value| value.checked_add(extent_reads))
        .and_then(|value| value.checked_add(extent_manifests.len() as u64))
        .and_then(|value| value.checked_add(absence_reads))
        .and_then(|value| value.checked_add(publication_inventory_reads))
        .map(|ceiling| ArtifactReadCeiling {
            addressed_reads: ceiling.max(1),
        })
        .ok_or(ArtifactReadCeilingDenial::Overflow)
}

fn routed_block_ceiling(
    inline_reads: u64,
    maximum_manifest_entries: u64,
) -> Result<u64, ArtifactReadCeilingDenial> {
    match (inline_reads, maximum_manifest_entries) {
        (0, _) => Ok(0),
        (_, 0) => Err(ArtifactReadCeilingDenial::ManifestEntriesExhausted),
        (_, maximum) => Ok(maximum),
    }
}

#[cfg(test)]
mod tests {
    use super::{routed_block_ceiling, ArtifactReadCeilingDenial};

    #[test]
    fn inline_routing_cannot_obtain_a_read_ceiling_after_entry_exhaustion() {
        assert_eq!(
            routed_block_ceiling(1, 0),
            Err(ArtifactReadCeilingDenial::ManifestEntriesExhausted)
        );
        assert_eq!(routed_block_ceiling(0, 0), Ok(0));
        assert_eq!(routed_block_ceiling(1, 1), Ok(1));
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
