use worth_ui_host_contract::{
    UiHostImeCompositionPhase, UiHostImePreedit, UiHostImePreeditConstructionDenial,
    UiHostObservationPayload,
};

#[derive(Clone, Copy, Default)]
pub(super) struct UiEguiTextTranslator;

#[derive(Clone, Copy, Default)]
pub(super) struct UiEguiImeTranslator;

#[derive(Clone)]
pub(super) struct UiEguiTextImeTranslationState {
    next_revision: Option<u64>,
}

pub(super) enum UiEguiTextImeTranslationDenial {
    RevisionExhausted,
    Preedit(UiHostImePreeditConstructionDenial),
}

impl UiEguiTextTranslator {
    pub(super) const fn capability(self) -> worth_ui_host_contract::WorthUiHostCapability {
        worth_ui_host_contract::WorthUiHostCapability::TextInput
    }

    pub(super) fn translate(
        self,
        state: &mut UiEguiTextImeTranslationState,
        text: &str,
    ) -> Result<UiHostObservationPayload, UiEguiTextImeTranslationDenial> {
        state.text(text)
    }
}

impl UiEguiImeTranslator {
    pub(super) const fn capability(self) -> worth_ui_host_contract::WorthUiHostCapability {
        worth_ui_host_contract::WorthUiHostCapability::Ime
    }

    pub(super) fn preedit(
        self,
        state: &mut UiEguiTextImeTranslationState,
        text: &str,
        active_range_chars: Option<std::ops::Range<usize>>,
    ) -> Result<UiHostObservationPayload, UiEguiTextImeTranslationDenial> {
        state.preedit(text, active_range_chars)
    }

    pub(super) fn commit(
        self,
        state: &mut UiEguiTextImeTranslationState,
        text: &str,
    ) -> Result<UiHostObservationPayload, UiEguiTextImeTranslationDenial> {
        state.commit(text)
    }
}

impl UiEguiTextImeTranslationState {
    pub(super) fn text(
        &mut self,
        text: &str,
    ) -> Result<UiHostObservationPayload, UiEguiTextImeTranslationDenial> {
        Ok(UiHostObservationPayload::TextInput {
            revision: self.take_revision()?,
            text: text.into(),
        })
    }

    pub(super) fn preedit(
        &mut self,
        text: &str,
        active_range_chars: Option<std::ops::Range<usize>>,
    ) -> Result<UiHostObservationPayload, UiEguiTextImeTranslationDenial> {
        let revision = self.take_revision()?;
        let phase = if text.is_empty() {
            UiHostImeCompositionPhase::Cancel
        } else {
            UiHostImeCompositionPhase::Preedit(
                UiHostImePreedit::from_unicode_scalar_range(text, active_range_chars)
                    .map_err(UiEguiTextImeTranslationDenial::Preedit)?,
            )
        };
        Ok(UiHostObservationPayload::ImeComposition { revision, phase })
    }

    pub(super) fn commit(
        &mut self,
        text: &str,
    ) -> Result<UiHostObservationPayload, UiEguiTextImeTranslationDenial> {
        Ok(UiHostObservationPayload::ImeComposition {
            revision: self.take_revision()?,
            phase: UiHostImeCompositionPhase::Commit(text.into()),
        })
    }

    fn take_revision(&mut self) -> Result<u64, UiEguiTextImeTranslationDenial> {
        let revision = self
            .next_revision
            .ok_or(UiEguiTextImeTranslationDenial::RevisionExhausted)?;
        self.next_revision = revision.checked_add(1);
        Ok(revision)
    }
}

impl Default for UiEguiTextImeTranslationState {
    fn default() -> Self {
        Self {
            next_revision: Some(1),
        }
    }
}
