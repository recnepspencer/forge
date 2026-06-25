mod registration;

use crate::capability::{
    CapabilitySupportKind, ImageAssetId, RegistrationCandidate, IMAGE_ASSET_FAMILY_NAME,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImageAssetSourceKind {
    LocalStatic,
    RemoteUnsupported,
    AsyncUnsupported,
}

impl ImageAssetSourceKind {
    pub fn local_static() -> Self {
        Self::LocalStatic
    }

    pub fn remote_unsupported() -> Self {
        Self::RemoteUnsupported
    }

    pub fn async_unsupported() -> Self {
        Self::AsyncUnsupported
    }

    pub fn token(self) -> &'static str {
        match self {
            Self::LocalStatic => "local_static",
            Self::RemoteUnsupported => "remote_unsupported",
            Self::AsyncUnsupported => "async_unsupported",
        }
    }

    fn is_admitted(self) -> bool {
        matches!(self, Self::LocalStatic)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageAssetDescriptor {
    id: ImageAssetId,
    source_kind: ImageAssetSourceKind,
    source_key: String,
    intrinsic_width_points: u16,
    intrinsic_height_points: u16,
}

impl ImageAssetDescriptor {
    pub fn local_static(
        id: ImageAssetId,
        source_key: impl Into<String>,
        intrinsic_width_points: u16,
        intrinsic_height_points: u16,
    ) -> Self {
        Self {
            id,
            source_kind: ImageAssetSourceKind::LocalStatic,
            source_key: source_key.into(),
            intrinsic_width_points,
            intrinsic_height_points,
        }
    }

    pub fn unsupported(
        id: ImageAssetId,
        source_kind: ImageAssetSourceKind,
        source_key: impl Into<String>,
    ) -> Self {
        Self {
            id,
            source_kind,
            source_key: source_key.into(),
            intrinsic_width_points: 0,
            intrinsic_height_points: 0,
        }
    }

    pub(crate) fn registration_candidate(&self) -> RegistrationCandidate {
        RegistrationCandidate::new(
            IMAGE_ASSET_FAMILY_NAME,
            self.id.as_str(),
            if self.is_admitted_local_static() {
                CapabilitySupportKind::Admitted
            } else {
                CapabilitySupportKind::Unsupported
            },
        )
    }

    pub fn id(&self) -> &ImageAssetId {
        &self.id
    }

    pub fn source_kind(&self) -> ImageAssetSourceKind {
        self.source_kind
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn intrinsic_width_points(&self) -> u16 {
        self.intrinsic_width_points
    }

    pub fn intrinsic_height_points(&self) -> u16 {
        self.intrinsic_height_points
    }

    fn is_admitted_local_static(&self) -> bool {
        self.source_kind.is_admitted()
            && !self.source_key.trim().is_empty()
            && self.intrinsic_width_points > 0
            && self.intrinsic_height_points > 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenImageAssetEntry {
    descriptor: ImageAssetDescriptor,
    projection_key: String,
}

impl FrozenImageAssetEntry {
    fn new(descriptor: ImageAssetDescriptor) -> Self {
        let projection_key = format!(
            "{}|{}|{}|{}x{}",
            descriptor.id().as_str(),
            descriptor.source_kind().token(),
            descriptor.source_key(),
            descriptor.intrinsic_width_points(),
            descriptor.intrinsic_height_points()
        );
        Self {
            descriptor,
            projection_key,
        }
    }

    pub fn descriptor(&self) -> &ImageAssetDescriptor {
        &self.descriptor
    }

    pub fn projection_key(&self) -> &str {
        &self.projection_key
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenImageAssetCapabilities {
    entries: Vec<FrozenImageAssetEntry>,
}

impl FrozenImageAssetCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<ImageAssetDescriptor>,
        accepted_assets: &ImageAssetAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_assets.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let entries = descriptors
            .into_iter()
            .map(FrozenImageAssetEntry::new)
            .collect();
        Self { entries }
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenImageAssetEntry] {
        &self.entries
    }

    pub fn get(&self, id: &ImageAssetId) -> Option<&ImageAssetDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x45f2_51ad_9f31_8bc7, |basis, entry| {
                fold_bytes(basis, entry.projection_key().as_bytes())
            })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ImageAssetRegistry {
    descriptors: Vec<ImageAssetDescriptor>,
}

impl ImageAssetRegistry {
    pub(crate) fn empty() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub(crate) fn push(&mut self, descriptor: ImageAssetDescriptor) {
        self.descriptors.push(descriptor);
    }

    pub(crate) fn freeze(
        self,
        accepted_assets: &ImageAssetAcceptedRegistrationProof,
    ) -> FrozenImageAssetCapabilities {
        FrozenImageAssetCapabilities::from_accepted_descriptors(self.descriptors, accepted_assets)
    }
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}

pub(crate) use registration::ImageAssetAcceptedRegistrationProof;
