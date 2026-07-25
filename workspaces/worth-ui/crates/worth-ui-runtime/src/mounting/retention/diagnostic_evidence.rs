use worth_ui_host_contract::{
    UiMountedDiagnosticProjection, UiMountedFrameIdentity, UiMountedInstanceIdentity,
    UiSurfaceBindingGeneration,
};

pub(crate) type UiRetainedMountedDiagnosticRow = (
    UiSurfaceBindingGeneration,
    UiMountedInstanceIdentity,
    UiMountedDiagnosticProjection,
);

#[derive(Clone)]
pub(crate) struct UiRetainedMountedDiagnostics {
    frame: UiMountedFrameIdentity,
    rows: Box<[UiRetainedMountedDiagnosticRow]>,
    structural_bytes: usize,
}

impl UiRetainedMountedDiagnostics {
    pub(crate) fn prepare(frame: &super::super::UiPreparedMountedFrame) -> Option<Self> {
        let mut rows = frame
            .surfaces()
            .iter()
            .flat_map(|surface| {
                let binding = surface.requirement().binding();
                surface
                    .projection()
                    .nodes()
                    .iter()
                    .map(move |node| (binding, node.mounted_instance(), node.diagnostic()))
            })
            .collect::<Vec<_>>();
        rows.sort_by_key(|(binding, instance, _)| (*binding, *instance));
        rows.dedup_by_key(|(binding, instance, _)| (*binding, *instance));
        let row_bytes = rows
            .len()
            .checked_mul(std::mem::size_of::<UiRetainedMountedDiagnosticRow>())?;
        let structural_bytes = std::mem::size_of::<Self>().checked_add(row_bytes)?;
        Some(Self {
            frame: frame.canonical_core().frame(),
            rows: rows.into_boxed_slice(),
            structural_bytes,
        })
    }

    pub(crate) fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub(crate) fn rows(&self) -> &[UiRetainedMountedDiagnosticRow] {
        &self.rows
    }

    pub(crate) fn structural_bytes(&self) -> usize {
        self.structural_bytes
    }
}
