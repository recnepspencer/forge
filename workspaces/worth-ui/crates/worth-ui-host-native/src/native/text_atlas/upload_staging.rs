//! Row-aligned staging construction and one exact WGPU copy submission.

use super::capacity::source_channels;
use super::recovery::UiNativeTextAtlasDenial;
use super::transaction::{upload_shape_is_valid, UiNativeTextAtlasUpload};
use super::upload::{AtlasPageTarget, UiNativeGpuAtlasKind};

pub(super) struct AtlasCopySubmission<'copy> {
    pub(super) device: &'copy wgpu::Device,
    pub(super) queue: &'copy wgpu::Queue,
    pub(super) target: AtlasPageTarget<'copy>,
    pub(super) origin: [u32; 2],
    pub(super) upload: &'copy UiNativeTextAtlasUpload,
    pub(super) layout: CopyLayout,
    pub(super) staging: &'copy wgpu::Buffer,
}

#[derive(Clone, Copy)]
pub(super) struct CopyLayout {
    pub(super) row_bytes: u64,
    pub(super) padded_row: u64,
    pub(super) staging_size: u64,
}

impl CopyLayout {
    pub(super) fn from_upload(
        upload: &UiNativeTextAtlasUpload,
    ) -> Result<Self, UiNativeTextAtlasDenial> {
        let row_bytes = u64::from(upload.width())
            .checked_mul(source_channels(upload.key().source()))
            .ok_or(UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
        let padded_row = align_to_copy_row(row_bytes)?;
        let staging_size = padded_row
            .checked_mul(u64::from(upload.height()))
            .ok_or(UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
        if staging_size > 8 * 1_024 * 1_024 {
            return Err(UiNativeTextAtlasDenial::StagingCapacityExceeded);
        }
        Ok(Self {
            row_bytes,
            padded_row,
            staging_size,
        })
    }
}

pub(super) fn validate_upload_target(
    target: &AtlasPageTarget<'_>,
    kind: UiNativeGpuAtlasKind,
    origin: [u32; 2],
    upload: &UiNativeTextAtlasUpload,
) -> Result<(), UiNativeTextAtlasDenial> {
    let invalid = !source_matches_kind(upload.key().source(), kind)
        || !upload_shape_is_valid(upload)
        || origin[0]
            .checked_add(upload.width())
            .is_none_or(|right| right > target.width)
        || origin[1]
            .checked_add(upload.height())
            .is_none_or(|bottom| bottom > target.height);
    (!invalid)
        .then_some(())
        .ok_or(UiNativeTextAtlasDenial::RasterBatchMismatch)
}

pub(super) fn copy_rows(
    upload: &UiNativeTextAtlasUpload,
    layout: CopyLayout,
) -> Result<Vec<u8>, UiNativeTextAtlasDenial> {
    let staging_length = usize::try_from(layout.staging_size)
        .map_err(|_| UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
    let row_bytes_length = usize::try_from(layout.row_bytes)
        .map_err(|_| UiNativeTextAtlasDenial::RasterBatchMismatch)?;
    let mut staged_bytes = vec![0_u8; staging_length];
    for row in 0..upload.height() {
        let source_start = usize::try_from(u64::from(row) * layout.row_bytes)
            .map_err(|_| UiNativeTextAtlasDenial::RasterBatchMismatch)?;
        let source_end = source_start
            .checked_add(row_bytes_length)
            .ok_or(UiNativeTextAtlasDenial::RasterBatchMismatch)?;
        let bytes = upload
            .bytes()
            .get(source_start..source_end)
            .ok_or(UiNativeTextAtlasDenial::RasterBatchMismatch)?;
        let destination_start = usize::try_from(u64::from(row) * layout.padded_row)
            .map_err(|_| UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
        let destination_end = destination_start
            .checked_add(bytes.len())
            .ok_or(UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
        staged_bytes[destination_start..destination_end].copy_from_slice(bytes);
    }
    Ok(staged_bytes)
}

pub(super) fn submit_copy(input: AtlasCopySubmission<'_>) -> wgpu::SubmissionIndex {
    let mut encoder = input
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("worth-ui-text-atlas-upload"),
        });
    encoder.copy_buffer_to_texture(
        wgpu::TexelCopyBufferInfo {
            buffer: input.staging,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(u32::try_from(input.layout.padded_row).unwrap_or(u32::MAX)),
                rows_per_image: Some(input.upload.height()),
            },
        },
        wgpu::TexelCopyTextureInfo {
            texture: input.target.texture,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: input.origin[0],
                y: input.origin[1],
                z: 0,
            },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::Extent3d {
            width: input.upload.width(),
            height: input.upload.height(),
            depth_or_array_layers: 1,
        },
    );
    input.queue.submit([encoder.finish()])
}

pub(super) fn align_to_copy_row(bytes: u64) -> Result<u64, UiNativeTextAtlasDenial> {
    bytes
        .checked_add(255)
        .map(|value| value / 256 * 256)
        .ok_or(UiNativeTextAtlasDenial::StagingCapacityExceeded)
}

pub(super) fn source_matches_kind(
    source: worth_ui_host_contract::UiGlyphRasterSource,
    kind: UiNativeGpuAtlasKind,
) -> bool {
    match kind {
        UiNativeGpuAtlasKind::Alpha => matches!(
            source,
            worth_ui_host_contract::UiGlyphRasterSource::AlphaOutline
                | worth_ui_host_contract::UiGlyphRasterSource::LastResort
        ),
        UiNativeGpuAtlasKind::Color => matches!(
            source,
            worth_ui_host_contract::UiGlyphRasterSource::ColorOutline
                | worth_ui_host_contract::UiGlyphRasterSource::ColorBitmap
        ),
    }
}
