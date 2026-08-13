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
    source: crate::mounting::projection::UiMountedDiagnosticSource,
    rows: std::cell::OnceCell<Box<[UiRetainedMountedDiagnosticRow]>>,
    structural_bytes: usize,
}

impl UiRetainedMountedDiagnostics {
    pub(crate) fn prepare(frame: &super::super::UiPreparedMountedFrame) -> Option<Self> {
        let source = frame.diagnostic_source();
        let row_bytes = source
            .len()
            .checked_mul(std::mem::size_of::<UiRetainedMountedDiagnosticRow>())?;
        let structural_bytes = std::mem::size_of::<Self>().checked_add(row_bytes)?;
        Some(Self {
            frame: frame.canonical_core().frame(),
            source,
            rows: std::cell::OnceCell::new(),
            structural_bytes,
        })
    }

    pub(crate) fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub(crate) fn rows(&self) -> &[UiRetainedMountedDiagnosticRow] {
        self.rows.get_or_init(|| {
            let mut rows = self.source.rows().collect::<Vec<_>>();
            rows.sort_by_key(|(binding, instance, _)| (*binding, *instance));
            rows.into_boxed_slice()
        })
    }

    pub(crate) fn structural_bytes(&self) -> usize {
        self.structural_bytes
    }
}
