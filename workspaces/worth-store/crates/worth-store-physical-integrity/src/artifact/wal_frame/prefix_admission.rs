use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::wal_frame::{
    decode_wal_frame_v1_header, WalSegmentIdentity, WAL_FRAME_V1_FOOTER_BYTES,
    WAL_FRAME_V1_HEADER_BYTES,
};

use crate::{
    PhysicalArtifactScope, PhysicalByteRange, PhysicalIntegrityObservationCounters,
    UntrustedPhysicalArtifact,
};

use super::{validate_wal_frame, WalFrameIntegrityValidation};

/// Admits the first complete WAL frame in one bounded segment suffix.
///
/// The length probe remains inside the integrity owner. It reveals no WAL
/// fields to recovery before the exact frame has passed full validation.
pub fn validate_wal_frame_prefix<'media>(
    suffix: UntrustedPhysicalArtifact<'media>,
    store: StableStoreIdentity,
    identity: WalSegmentIdentity,
    artifact_offset: u64,
) -> (
    WalFrameIntegrityValidation<'media>,
    PhysicalIntegrityObservationCounters,
) {
    let bytes = suffix.bytes();
    let inspected_length = inspected_frame_length(bytes);
    let input_length = inspected_length.min(bytes.len());
    let input = UntrustedPhysicalArtifact::from_bounded_bytes(&bytes[..input_length]);
    let range = PhysicalByteRange::new(artifact_offset, inspected_length as u64)
        .expect("a nonempty bounded WAL suffix yields a nonempty exact frame scope");
    validate_wal_frame(
        input,
        PhysicalArtifactScope::wal_frame(store, identity, range),
    )
}

fn inspected_frame_length(bytes: &[u8]) -> usize {
    if bytes.len() < WAL_FRAME_V1_HEADER_BYTES {
        return WAL_FRAME_V1_HEADER_BYTES;
    }
    let header: &[u8; WAL_FRAME_V1_HEADER_BYTES] = bytes[..WAL_FRAME_V1_HEADER_BYTES]
        .try_into()
        .expect("the guarded WAL prefix contains one complete header");
    let Ok(header) = decode_wal_frame_v1_header(header) else {
        return WAL_FRAME_V1_HEADER_BYTES;
    };
    usize::try_from(header.payload_bytes())
        .ok()
        .and_then(|payload| WAL_FRAME_V1_HEADER_BYTES.checked_add(payload))
        .and_then(|framed| framed.checked_add(WAL_FRAME_V1_FOOTER_BYTES))
        .unwrap_or(bytes.len())
}

#[cfg(test)]
mod tests {
    use worth_store_physical_format::store_namespace::{
        ProposedStoreIdentity, StoreNamespaceIdentityRecord, StoreNamespaceVersion,
    };

    use super::*;
    use crate::{PhysicalDamageCause, PhysicalIntegrityRejection};

    #[test]
    fn short_header_localizes_only_the_known_missing_header_range() {
        let store = StoreNamespaceIdentityRecord::new(
            StoreNamespaceVersion::CURRENT,
            ProposedStoreIdentity::from_nonzero_bytes([9; 16]).unwrap(),
        )
        .published_identity();
        let identity = WalSegmentIdentity::new(3, 4).unwrap();
        let bytes = [0_u8; 17];
        let (validation, _) = validate_wal_frame_prefix(
            UntrustedPhysicalArtifact::from_bounded_bytes(&bytes),
            store,
            identity,
            23,
        );
        let WalFrameIntegrityValidation::Rejected(PhysicalIntegrityRejection::Damaged(damage)) =
            validation
        else {
            panic!("short WAL header must be a typed truncation")
        };
        assert_eq!(damage.cause(), PhysicalDamageCause::Truncated);
        assert_eq!(
            damage.scope().byte_range(),
            PhysicalByteRange::new(23, 116).unwrap()
        );
        assert_eq!(
            damage.damaged_range(),
            PhysicalByteRange::new(40, 99).unwrap()
        );
    }
}
