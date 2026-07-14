/// Move-only preview input; committed execution cannot consume it.
///
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::{UiHostPreviewPaintInput, WorthUiExecutionLaneInput};
/// fn forbidden(input: &UiHostPreviewPaintInput) { let _ = WorthUiExecutionLaneInput::new(input); }
/// ```
/// ```compile_fail
/// use worth_ui_runtime::facade::runtime_handoff::UiHostPreviewPaintInput;
/// fn move_only(input: UiHostPreviewPaintInput) { let _ = input.clone(); }
/// ```
#[derive(Debug)]
#[must_use = "preview paint input must be consumed or explicitly discarded"]
pub struct UiHostPreviewPaintInput {
    preview: crate::runtime::UiResizePreviewOutcome,
}

#[derive(Clone, Copy, Debug)]
pub struct UiHostPreviewPaintGeometry<'a> {
    preview: &'a crate::runtime::UiResizePreviewOutcome,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostPreviewPaintDenial {
    HostUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostPreviewDiscardReason {
    Superseded,
    HostChoseNotToPaint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPreviewPaintContext {
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    target: crate::graph::UiGraphNodeIdentity,
    extent: crate::runtime::UiResizeLogicalExtent,
    stream_counters: crate::runtime::UiDragResizeCounters,
    strategy: crate::evidence::UiDragResizeStrategy,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPreviewPaintReceipt {
    context: UiHostPreviewPaintContext,
    painted_candidates: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPreviewPaintDenialReport {
    context: UiHostPreviewPaintContext,
    denial: UiHostPreviewPaintDenial,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHostPreviewPaintDiscardReport {
    context: UiHostPreviewPaintContext,
    reason: UiHostPreviewDiscardReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiHostPreviewPaintDisposition {
    Painted(UiHostPreviewPaintReceipt),
    Denied(UiHostPreviewPaintDenialReport),
    Discarded(UiHostPreviewPaintDiscardReport),
}

pub trait WorthUiPreviewPaintHost {
    fn paint_preview(
        &mut self,
        geometry: UiHostPreviewPaintGeometry<'_>,
    ) -> Result<(), UiHostPreviewPaintDenial>;
}

impl<F> WorthUiPreviewPaintHost for F
where
    F: for<'preview> FnMut(
        UiHostPreviewPaintGeometry<'preview>,
    ) -> Result<(), UiHostPreviewPaintDenial>,
{
    fn paint_preview(
        &mut self,
        geometry: UiHostPreviewPaintGeometry<'_>,
    ) -> Result<(), UiHostPreviewPaintDenial> {
        self(geometry)
    }
}

pub(crate) fn seal_preview_paint_input(
    preview: crate::runtime::UiResizePreviewOutcome,
) -> UiHostPreviewPaintInput {
    UiHostPreviewPaintInput { preview }
}

impl UiHostPreviewPaintInput {
    pub(crate) fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.preview.preview_candidate().frame_epoch()
    }
    pub fn consume(self, host: &mut impl WorthUiPreviewPaintHost) -> UiHostPreviewPaintDisposition {
        let context = self.context();
        match host.paint_preview(UiHostPreviewPaintGeometry {
            preview: &self.preview,
        }) {
            Ok(()) => UiHostPreviewPaintDisposition::Painted(UiHostPreviewPaintReceipt {
                context,
                painted_candidates: self
                    .preview
                    .preview_candidate()
                    .allocation_candidates()
                    .len() as u16,
            }),
            Err(denial) => UiHostPreviewPaintDisposition::Denied(UiHostPreviewPaintDenialReport {
                context,
                denial,
            }),
        }
    }

    pub fn discard(self, reason: UiHostPreviewDiscardReason) -> UiHostPreviewPaintDisposition {
        UiHostPreviewPaintDisposition::Discarded(UiHostPreviewPaintDiscardReport {
            context: self.context(),
            reason,
        })
    }

    fn context(&self) -> UiHostPreviewPaintContext {
        let candidate = self.preview.preview_candidate();
        UiHostPreviewPaintContext {
            frame_epoch: candidate.frame_epoch(),
            target: candidate.target(),
            extent: candidate.extent(),
            stream_counters: self.preview.counters(),
            strategy: self.preview.evidence().strategy(),
        }
    }
}

impl UiHostPreviewPaintGeometry<'_> {
    pub fn frame_epoch(self) -> crate::runtime::UiAllocationFrameEpoch {
        self.preview.preview_candidate().frame_epoch()
    }
    pub fn target(self) -> crate::graph::UiGraphNodeIdentity {
        self.preview.preview_candidate().target()
    }
    pub fn extent(self) -> crate::runtime::UiResizeLogicalExtent {
        self.preview.preview_candidate().extent()
    }
    pub fn candidate_count(self) -> u16 {
        self.preview
            .preview_candidate()
            .allocation_candidates()
            .len() as u16
    }
    pub fn all_candidates_admitted(self) -> bool {
        self.preview
            .preview_candidate()
            .allocation_candidates()
            .iter()
            .all(crate::runtime::UiAllocationPreviewCandidate::candidate_is_admitted)
    }
}

impl UiHostPreviewPaintContext {
    pub fn frame_epoch(self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
    pub fn target(self) -> crate::graph::UiGraphNodeIdentity {
        self.target
    }
    pub fn extent(self) -> crate::runtime::UiResizeLogicalExtent {
        self.extent
    }
    pub fn stream_counters(self) -> crate::runtime::UiDragResizeCounters {
        self.stream_counters
    }
    pub fn strategy(self) -> crate::evidence::UiDragResizeStrategy {
        self.strategy
    }
}
impl UiHostPreviewPaintReceipt {
    pub fn context(self) -> UiHostPreviewPaintContext {
        self.context
    }
    pub fn painted_candidates(self) -> u16 {
        self.painted_candidates
    }
}
impl UiHostPreviewPaintDenialReport {
    pub fn context(self) -> UiHostPreviewPaintContext {
        self.context
    }
    pub fn denial(self) -> UiHostPreviewPaintDenial {
        self.denial
    }
}
impl UiHostPreviewPaintDiscardReport {
    pub fn context(self) -> UiHostPreviewPaintContext {
        self.context
    }
    pub fn reason(self) -> UiHostPreviewDiscardReason {
        self.reason
    }
}
