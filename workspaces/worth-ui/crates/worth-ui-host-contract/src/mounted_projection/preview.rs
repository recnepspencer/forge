#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMountedPreviewProjection {
    Resize {
        frame_epoch: u64,
        extent_subpixels: u32,
        candidate_count: u16,
        all_candidates_admitted: bool,
    },
    Omitted(super::UiMountedOmissionReason),
}

impl UiMountedPreviewProjection {
    pub fn resize(
        frame_epoch: u64,
        extent_subpixels: u32,
        candidate_count: u16,
        all_candidates_admitted: bool,
    ) -> Self {
        Self::Resize {
            frame_epoch,
            extent_subpixels,
            candidate_count,
            all_candidates_admitted,
        }
    }
}
