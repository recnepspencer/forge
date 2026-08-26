mod canonical_inventory;

use worth_query_installation::facade::WorthQueryPortablePackageManifest;

use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::limits::WorthQueryPackageArchiveLimits;
use crate::manifest::{decode_manifest_frame, MANIFEST_FRAME_BYTES};
use crate::record::{
    WorthQueryPackageArchiveDecodeWork, WorthQueryPackageArchiveRecordDecoder,
    WorthQueryUntrustedPortablePackageRecordFrame,
};

use canonical_inventory::CanonicalArchiveInventory;

/// One complete, structurally decoded package archive carrying no Query authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryUntrustedPortablePackageArchive {
    manifest: WorthQueryPortablePackageManifest,
    frames: Vec<WorthQueryUntrustedPortablePackageRecordFrame>,
    decode_work: WorthQueryPackageArchiveDecodeWork,
}

impl WorthQueryUntrustedPortablePackageArchive {
    pub const fn manifest(&self) -> &WorthQueryPortablePackageManifest {
        &self.manifest
    }

    pub fn frames(&self) -> &[WorthQueryUntrustedPortablePackageRecordFrame] {
        &self.frames
    }

    pub const fn decode_work(&self) -> WorthQueryPackageArchiveDecodeWork {
        self.decode_work
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryPortablePackageManifest,
        Vec<WorthQueryUntrustedPortablePackageRecordFrame>,
    ) {
        (self.manifest, self.frames)
    }
}

/// Decodes one complete archive into descriptive, still-untrusted package records.
pub fn decode_package_archive(
    bytes: &[u8],
    limits: WorthQueryPackageArchiveLimits,
) -> Result<WorthQueryUntrustedPortablePackageArchive, Denial> {
    let limits = limits.narrowed();
    require_archive_byte_budget(bytes.len(), limits)?;
    let mut input = BinaryInput::new(bytes);
    let manifest_bytes = input.take(
        usize::try_from(MANIFEST_FRAME_BYTES)
            .map_err(|_| Denial::new(Kind::InvalidManifestLength))?,
    )?;
    let manifest = decode_manifest_frame(manifest_bytes, limits)?;
    let capacity = usize::try_from(manifest.record_count())
        .map_err(|_| Denial::new(Kind::RecordBudgetExceeded))?;
    let mut frames = Vec::with_capacity(capacity);
    let mut decoder = WorthQueryPackageArchiveRecordDecoder::new(limits);
    let mut inventory = CanonicalArchiveInventory::new(&manifest);
    for _ in 0..manifest.record_count() {
        let frame = decoder.decode_next_frame(&mut input)?;
        inventory.admit(&frame)?;
        frames.push(frame);
    }
    inventory.finish()?;
    if !input.is_finished() {
        return Err(Denial::new(Kind::TrailingBytes));
    }
    Ok(WorthQueryUntrustedPortablePackageArchive {
        manifest,
        frames,
        decode_work: decoder.work(),
    })
}

fn require_archive_byte_budget(
    observed: usize,
    limits: WorthQueryPackageArchiveLimits,
) -> Result<(), Denial> {
    if u64::try_from(observed).unwrap_or(u64::MAX) > limits.maximum_archive_bytes() {
        return Err(Denial::new(Kind::ArchiveByteBudgetExceeded));
    }
    Ok(())
}
