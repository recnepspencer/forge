use crate::graph::UiGraphNodeIdentity;

use super::WorthUiTransientInteractionState;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiAdmittedTransientInteraction {
    target: UiGraphNodeIdentity,
    state: WorthUiTransientInteractionState,
    resize_preview: Option<crate::runtime::UiResizePreviewSample>,
    source_identity: u64,
    source_generation: u64,
    source_order: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiTransientInteractionAdmissionDenial {
    SourceOrderExhausted,
    ResizePreviewRequiresTypedSample,
}

pub struct WorthUiTransientInteractionAdmission<'a> {
    authority: &'a mut WorthUiTransientInteractionAdmissionAuthority,
}

#[derive(Debug)]
pub(crate) struct WorthUiTransientInteractionAdmissionAuthority {
    source_generation: u64,
    next_source_order: u64,
}

impl Default for WorthUiTransientInteractionAdmissionAuthority {
    fn default() -> Self {
        Self {
            source_generation: 1,
            next_source_order: 0,
        }
    }
}

impl WorthUiTransientInteractionAdmissionAuthority {
    pub(crate) fn admit(
        &mut self,
        target: UiGraphNodeIdentity,
        state: WorthUiTransientInteractionState,
    ) -> Result<WorthUiAdmittedTransientInteraction, WorthUiTransientInteractionAdmissionDenial>
    {
        if state == WorthUiTransientInteractionState::ResizePreview {
            return Err(
                WorthUiTransientInteractionAdmissionDenial::ResizePreviewRequiresTypedSample,
            );
        }
        self.admit_inner(target, state, None)
    }

    pub(crate) fn admit_resize_preview(
        &mut self,
        sample: crate::runtime::UiResizePreviewSample,
    ) -> Result<WorthUiAdmittedTransientInteraction, WorthUiTransientInteractionAdmissionDenial>
    {
        self.admit_inner(
            sample.target(),
            WorthUiTransientInteractionState::ResizePreview,
            Some(sample),
        )
    }

    fn admit_inner(
        &mut self,
        target: UiGraphNodeIdentity,
        state: WorthUiTransientInteractionState,
        resize_preview: Option<crate::runtime::UiResizePreviewSample>,
    ) -> Result<WorthUiAdmittedTransientInteraction, WorthUiTransientInteractionAdmissionDenial>
    {
        self.next_source_order = self
            .next_source_order
            .checked_add(1)
            .ok_or(WorthUiTransientInteractionAdmissionDenial::SourceOrderExhausted)?;
        Ok(WorthUiAdmittedTransientInteraction {
            target,
            state,
            resize_preview,
            source_identity: target.digest(),
            source_generation: self.source_generation,
            source_order: self.next_source_order,
        })
    }
}

impl<'a> WorthUiTransientInteractionAdmission<'a> {
    pub(crate) fn new(authority: &'a mut WorthUiTransientInteractionAdmissionAuthority) -> Self {
        Self { authority }
    }

    pub fn admit(
        &mut self,
        target: UiGraphNodeIdentity,
        state: WorthUiTransientInteractionState,
    ) -> Result<WorthUiAdmittedTransientInteraction, WorthUiTransientInteractionAdmissionDenial>
    {
        self.authority.admit(target, state)
    }
}

impl WorthUiAdmittedTransientInteraction {
    #[cfg(test)]
    pub(crate) fn for_dispatcher_test(
        target: UiGraphNodeIdentity,
        source_generation: u64,
        source_order: u64,
    ) -> Self {
        Self {
            target,
            state: WorthUiTransientInteractionState::TextInput,
            resize_preview: None,
            source_identity: target.digest(),
            source_generation,
            source_order,
        }
    }

    pub fn target(self) -> UiGraphNodeIdentity {
        self.target
    }
    pub fn state(self) -> WorthUiTransientInteractionState {
        self.state
    }
    pub fn resize_preview(self) -> Option<crate::runtime::UiResizePreviewSample> {
        self.resize_preview
    }
    pub fn source_identity(self) -> u64 {
        self.source_identity
    }
    pub fn source_generation(self) -> u64 {
        self.source_generation
    }
    pub fn source_order(self) -> u64 {
        self.source_order
    }
}
