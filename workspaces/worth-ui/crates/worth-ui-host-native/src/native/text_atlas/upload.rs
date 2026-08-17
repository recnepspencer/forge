//! Real WGPU atlas pages and aligned staging uploads.

#[path = "upload_batch.rs"]
mod batch;
#[path = "upload/correlation.rs"]
mod correlation;

use super::recovery::UiNativeTextAtlasDenial;
use super::UiNativeTextAtlasUpload;
use crate::native::{UiNativeOwnedResource, UiNativeResourceClass, UiNativeResourceRegistry};
use correlation::PendingAtlasTransactionCorrelation;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum UiNativeGpuAtlasKind {
    Alpha,
    Color,
}

pub(crate) struct UiNativeTextAtlasGpuPages {
    alpha: Vec<UiNativeOwnedResource<wgpu::Texture>>,
    color: Vec<UiNativeOwnedResource<wgpu::Texture>>,
    pending: Vec<PendingAtlasUpload>,
    correlations: Vec<PendingAtlasTransactionCorrelation>,
}

pub(crate) struct UiNativeTextAtlasGpuUploadRequest<'upload> {
    pub(crate) device: &'upload wgpu::Device,
    pub(crate) queue: &'upload wgpu::Queue,
    pub(crate) resources: &'upload mut UiNativeResourceRegistry,
    pub(crate) kind: UiNativeGpuAtlasKind,
    pub(crate) page: u32,
    pub(crate) origin: [u32; 2],
    pub(crate) upload: &'upload UiNativeTextAtlasUpload,
}

#[derive(Clone, Copy)]
pub(crate) struct UiNativeTextAtlasGpuBatchUpload<'upload> {
    pub(crate) kind: UiNativeGpuAtlasKind,
    pub(crate) page: u32,
    pub(crate) origin: [u32; 2],
    pub(crate) upload: &'upload UiNativeTextAtlasUpload,
}

pub(super) struct AtlasPageTarget<'page> {
    pub(super) texture: &'page wgpu::Texture,
    pub(super) width: u32,
    pub(super) height: u32,
}

struct PendingAtlasUpload {
    staging: UiNativeOwnedResource<wgpu::Buffer>,
    submission: wgpu::SubmissionIndex,
    transaction: u64,
    physical_bytes: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiNativeTextAtlasPhysicalPoll {
    Pending,
    Completed,
    Indeterminate,
}

impl UiNativeTextAtlasGpuPages {
    pub(crate) fn new() -> Self {
        Self {
            alpha: Vec::new(),
            color: Vec::new(),
            pending: Vec::new(),
            correlations: Vec::new(),
        }
    }

    pub(crate) fn page_count(&self, kind: UiNativeGpuAtlasKind) -> usize {
        match kind {
            UiNativeGpuAtlasKind::Alpha => self.alpha.len(),
            UiNativeGpuAtlasKind::Color => self.color.len(),
        }
    }

    pub(crate) fn page_view(
        &self,
        kind: UiNativeGpuAtlasKind,
        page: u32,
    ) -> Option<(wgpu::TextureView, [u32; 2])> {
        let page = usize::try_from(page).ok()?;
        let (texture, extent) = match kind {
            UiNativeGpuAtlasKind::Alpha => (self.alpha.get(page)?, [1_024, 1_024]),
            UiNativeGpuAtlasKind::Color => (self.color.get(page)?, [2_048, 2_048]),
        };
        Some((
            texture.create_view(&wgpu::TextureViewDescriptor::default()),
            extent,
        ))
    }

    pub(crate) fn ensure_page(
        &mut self,
        device: &wgpu::Device,
        resources: &mut UiNativeResourceRegistry,
        kind: UiNativeGpuAtlasKind,
    ) -> Result<(), UiNativeTextAtlasDenial> {
        let page_limit = match kind {
            UiNativeGpuAtlasKind::Alpha => 4,
            UiNativeGpuAtlasKind::Color => 2,
        };
        if self.page_count(kind) >= page_limit {
            return Err(UiNativeTextAtlasDenial::PageCapacityExceeded);
        }
        let (width, height, format, class) = match kind {
            UiNativeGpuAtlasKind::Alpha => (
                1_024,
                1_024,
                wgpu::TextureFormat::R8Unorm,
                UiNativeResourceClass::AlphaAtlasPage,
            ),
            UiNativeGpuAtlasKind::Color => (
                2_048,
                2_048,
                wgpu::TextureFormat::Rgba8UnormSrgb,
                UiNativeResourceClass::ColorAtlasPage,
            ),
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("worth-ui-text-atlas-page"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let owner = UiNativeOwnedResource::register(texture, class, resources)
            .map_err(|_| UiNativeTextAtlasDenial::PageCapacityExceeded)?;
        match kind {
            UiNativeGpuAtlasKind::Alpha => self.alpha.push(owner),
            UiNativeGpuAtlasKind::Color => self.color.push(owner),
        }
        Ok(())
    }

    pub(crate) fn upload(
        &mut self,
        request: UiNativeTextAtlasGpuUploadRequest<'_>,
    ) -> Result<UiNativeGpuUploadReceipt, UiNativeTextAtlasDenial> {
        self.upload_for_transaction(request, 0)
    }

    pub(crate) fn upload_for_transaction(
        &mut self,
        request: UiNativeTextAtlasGpuUploadRequest<'_>,
        transaction: u64,
    ) -> Result<UiNativeGpuUploadReceipt, UiNativeTextAtlasDenial> {
        let UiNativeTextAtlasGpuUploadRequest {
            device,
            queue,
            resources,
            kind,
            page,
            origin,
            upload,
        } = request;
        self.upload_batch_for_transaction(
            device,
            queue,
            resources,
            transaction,
            &[UiNativeTextAtlasGpuBatchUpload {
                kind,
                page,
                origin,
                upload,
            }],
        )
    }

    pub(crate) fn upload_batch_for_transaction(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        resources: &mut UiNativeResourceRegistry,
        transaction: u64,
        uploads: &[UiNativeTextAtlasGpuBatchUpload<'_>],
    ) -> Result<UiNativeGpuUploadReceipt, UiNativeTextAtlasDenial> {
        batch::upload_batch_for_transaction(self, device, queue, resources, transaction, uploads)
    }

    fn page_target(
        &self,
        kind: UiNativeGpuAtlasKind,
        page: u32,
    ) -> Result<AtlasPageTarget<'_>, UiNativeTextAtlasDenial> {
        let page =
            usize::try_from(page).map_err(|_| UiNativeTextAtlasDenial::PageCapacityExceeded)?;
        let (texture, width, height) = match kind {
            UiNativeGpuAtlasKind::Alpha => (self.alpha.get(page), 1_024, 1_024),
            UiNativeGpuAtlasKind::Color => (self.color.get(page), 2_048, 2_048),
        };
        texture
            .map(|texture| AtlasPageTarget {
                texture,
                width,
                height,
            })
            .ok_or(UiNativeTextAtlasDenial::PageCapacityExceeded)
    }

    pub(crate) fn settle_pending(
        &mut self,
        device: &wgpu::Device,
        resources: &mut UiNativeResourceRegistry,
    ) {
        let pending = std::mem::take(&mut self.pending);
        let mut remaining = Vec::with_capacity(pending.len());
        for upload in pending {
            let settled = device
                .poll(wgpu::PollType::Wait {
                    submission_index: Some(upload.submission.clone()),
                    timeout: Some(crate::native::presentation::GPU_WAIT_DEADLINE),
                })
                .is_ok();
            if settled {
                upload.staging.close(resources);
            } else {
                remaining.push(upload);
            }
        }
        self.pending = remaining;
        self.retain_live_correlations();
    }

    /// Observe one native transaction without committing any logical atlas
    /// state.  A transaction is complete only after every queued submission
    /// in its group has physically settled and its staging owner is released.
    pub(crate) fn poll_transaction_observation(
        &mut self,
        device: &wgpu::Device,
        resources: &mut UiNativeResourceRegistry,
        transaction: u64,
    ) -> Option<crate::native::physical_work_signal::UiNativePhysicalSignalExternalObservation>
    {
        let basis = self
            .correlations
            .iter()
            .find(|correlation| correlation.transaction == transaction)?
            .basis;
        let pending = std::mem::take(&mut self.pending);
        let mut remaining = Vec::with_capacity(pending.len());
        let mut observed_transaction = false;
        let mut unresolved = false;
        for upload in pending {
            if upload.transaction != transaction {
                remaining.push(upload);
                continue;
            }
            observed_transaction = true;
            match device.poll(wgpu::PollType::Wait {
                submission_index: Some(upload.submission.clone()),
                timeout: Some(std::time::Duration::ZERO),
            }) {
                Ok(_) => upload.staging.close(resources),
                Err(wgpu::PollError::Timeout) => remaining.push(upload),
                Err(wgpu::PollError::WrongSubmissionIndex(_, _)) => {
                    unresolved = true;
                    remaining.push(upload);
                }
            }
        }
        let still_pending = remaining
            .iter()
            .any(|upload| upload.transaction == transaction);
        self.pending = remaining;
        let posture = if unresolved {
            UiNativeTextAtlasPhysicalPoll::Indeterminate
        } else if observed_transaction && !still_pending {
            UiNativeTextAtlasPhysicalPoll::Completed
        } else {
            UiNativeTextAtlasPhysicalPoll::Pending
        };
        // Effects-indeterminate is not terminal physical ownership.  The
        // retained submission and its exact Signal basis remain correlated
        // until recovery observes completion or governed cleanup succeeds.
        if matches!(posture, UiNativeTextAtlasPhysicalPoll::Completed) {
            self.release_transaction_correlation(transaction);
        }
        Some(basis.observe(match posture {
            UiNativeTextAtlasPhysicalPoll::Pending => {
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Pending
            }
            UiNativeTextAtlasPhysicalPoll::Completed => {
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::Completed
            }
            UiNativeTextAtlasPhysicalPoll::Indeterminate => {
                crate::native::physical_work_signal::UiNativePhysicalSignalStatus::EffectsIndeterminate
            }
        }))
    }

    pub(crate) fn pending_count(&self) -> usize {
        self.pending.len()
    }

    pub(crate) fn transaction_pending(&self, transaction: u64) -> bool {
        self.pending
            .iter()
            .any(|upload| upload.transaction == transaction)
    }

    pub(crate) fn pending_physical_bytes(&self) -> u64 {
        self.pending.iter().fold(0_u64, |total, upload| {
            total.saturating_add(upload.physical_bytes)
        })
    }

    pub(crate) fn try_close(self, resources: &mut UiNativeResourceRegistry) -> Result<(), Self> {
        if !self.pending.is_empty() || !self.correlations.is_empty() {
            return Err(self);
        }
        for page in self.alpha {
            page.close(resources);
        }
        for page in self.color {
            page.close(resources);
        }
        Ok(())
    }

    fn retain_live_correlations(&mut self) {
        self.correlations.retain(|correlation| {
            self.pending
                .iter()
                .any(|upload| upload.transaction == correlation.transaction)
        });
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiNativeGpuUploadReceipt {
    pub(crate) logical_bytes: u64,
    pub(crate) physical_bytes: u64,
}

#[cfg(test)]
#[path = "upload_tests.rs"]
pub(super) mod tests;
