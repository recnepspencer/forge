//! One bounded physical staging owner for an admitted atlas transaction.

use super::{
    PendingAtlasUpload, UiNativeGpuUploadReceipt, UiNativeTextAtlasGpuBatchUpload,
    UiNativeTextAtlasGpuPages,
};
use crate::native::text_atlas::upload_staging::{
    copy_rows, submit_copies, validate_upload_target, AtlasCopyCommand, CopyLayout,
};
use crate::native::text_atlas::UiNativeTextAtlasDenial;
use crate::native::{UiNativeOwnedResource, UiNativeResourceClass, UiNativeResourceRegistry};

pub(super) fn upload_batch_for_transaction(
    pages: &mut UiNativeTextAtlasGpuPages,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    resources: &mut UiNativeResourceRegistry,
    transaction: u64,
    uploads: &[UiNativeTextAtlasGpuBatchUpload<'_>],
) -> Result<UiNativeGpuUploadReceipt, UiNativeTextAtlasDenial> {
    let mut staged_bytes = Vec::new();
    let mut layouts = Vec::with_capacity(uploads.len());
    let mut logical_bytes = 0_u64;
    for request in uploads {
        let target = pages.page_target(request.kind, request.page)?;
        validate_upload_target(&target, request.kind, request.origin, request.upload)?;
        let layout = CopyLayout::from_upload(request.upload)?;
        let offset = u64::try_from(staged_bytes.len())
            .map_err(|_| UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
        staged_bytes.extend(copy_rows(request.upload, layout)?);
        layouts.push((layout, offset));
        logical_bytes = logical_bytes
            .checked_add(u64::try_from(request.upload.bytes().len()).unwrap_or(u64::MAX))
            .ok_or(UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
    }
    let physical_bytes = u64::try_from(staged_bytes.len())
        .map_err(|_| UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
    if physical_bytes == 0 || physical_bytes > 8 * 1_024 * 1_024 {
        return Err(UiNativeTextAtlasDenial::StagingCapacityExceeded);
    }
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("worth-ui-text-atlas-staging"),
        size: physical_bytes,
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let owner = UiNativeOwnedResource::register(
        buffer,
        UiNativeResourceClass::AtlasStagingBuffer,
        resources,
    )
    .map_err(|_| UiNativeTextAtlasDenial::StagingCapacityExceeded)?;
    queue.write_buffer(&owner, 0, &staged_bytes);
    let copies = uploads
        .iter()
        .zip(layouts)
        .map(|(request, (layout, staging_offset))| AtlasCopyCommand {
            target: pages
                .page_target(request.kind, request.page)
                .expect("validated atlas page remains owned"),
            origin: request.origin,
            upload: request.upload,
            layout,
            staging_offset,
        })
        .collect::<Vec<_>>();
    let submission = submit_copies(device, queue, &owner, &copies);
    pages.pending.push(PendingAtlasUpload {
        device: device.clone(),
        staging: owner,
        submission,
        transaction,
        physical_bytes,
    });
    Ok(UiNativeGpuUploadReceipt {
        logical_bytes,
        physical_bytes,
    })
}
