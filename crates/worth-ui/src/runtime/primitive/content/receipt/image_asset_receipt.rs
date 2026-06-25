use crate::capability::{ImageAssetDescriptor, ImageAssetSourceKind};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveImageAssetReceipt {
    asset_id: String,
    source_kind: ImageAssetSourceKind,
    source_key: String,
    intrinsic_width_points: f32,
    intrinsic_height_points: f32,
    receipt_digest: u64,
}

impl WorthUiPrimitiveImageAssetReceipt {
    pub(crate) fn from_descriptor(descriptor: &ImageAssetDescriptor) -> Self {
        let source_kind = descriptor.source_kind();
        let intrinsic_width_points = f32::from(descriptor.intrinsic_width_points());
        let intrinsic_height_points = f32::from(descriptor.intrinsic_height_points());
        let asset_id = descriptor.id().as_str().to_owned();
        let source_key = descriptor.source_key().to_owned();
        let receipt_digest = image_asset_receipt_digest(
            &asset_id,
            source_kind,
            &source_key,
            intrinsic_width_points,
            intrinsic_height_points,
        );
        Self {
            asset_id,
            source_kind,
            source_key,
            intrinsic_width_points,
            intrinsic_height_points,
            receipt_digest,
        }
    }

    pub fn asset_id(&self) -> &str {
        &self.asset_id
    }

    pub fn source_kind(&self) -> ImageAssetSourceKind {
        self.source_kind
    }

    pub fn source_key(&self) -> &str {
        &self.source_key
    }

    pub fn intrinsic_width_points(&self) -> f32 {
        self.intrinsic_width_points
    }

    pub fn intrinsic_height_points(&self) -> f32 {
        self.intrinsic_height_points
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

fn image_asset_receipt_digest(
    asset_id: &str,
    source_kind: ImageAssetSourceKind,
    source_key: &str,
    intrinsic_width_points: f32,
    intrinsic_height_points: f32,
) -> u64 {
    format!(
        "image-asset|{asset_id}|{}|{source_key}|{}x{}",
        source_kind.token(),
        intrinsic_width_points,
        intrinsic_height_points
    )
    .bytes()
    .fold(0xcbf2_9ce4_8422_2325, |mut acc, byte| {
        acc ^= u64::from(byte);
        acc.wrapping_mul(0x0000_0100_0000_01b3)
    })
}
