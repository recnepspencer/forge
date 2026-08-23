use std::collections::BTreeMap;

use worth_ui_host_contract::{UiHostObservationSequence, UiSurfaceBindingGeneration};

use super::presentation_basis::UiEguiPresentedInputBasis;

#[derive(Default)]
pub(crate) struct UiEguiInputObservationState {
    presentations: BTreeMap<UiSurfaceBindingGeneration, UiEguiPresentedInputBasis>,
    partitions: BTreeMap<UiSurfaceBindingGeneration, UiEguiInputPartition>,
    input_recipients: BTreeMap<u64, worth_ui_host_contract::UiHostInputRecipientBindingReceipt>,
}

#[derive(Clone)]
pub(super) struct UiEguiInputTranslationState {
    next_sequence: Option<u64>,
    pub(super) pointer: super::pointer::UiEguiPointerTranslationState,
    pub(super) text_ime: super::text_ime::UiEguiTextImeTranslationState,
}

struct UiEguiInputPartition {
    host_session: u64,
    translation: UiEguiInputTranslationState,
}

pub(super) enum UiEguiPresentedInputSelection {
    Missing,
    Unique(UiEguiPresentedInputBasis),
    Ambiguous(usize),
}

impl UiEguiInputObservationState {
    pub(super) fn record_presentation(&mut self, basis: UiEguiPresentedInputBasis) {
        let binding = basis.presentation().binding();
        if self
            .presentations
            .get(&binding)
            .is_some_and(|current| current.host_session() != basis.host_session())
        {
            self.partitions.remove(&binding);
        }
        self.presentations.insert(binding, basis);
    }

    pub(super) fn select_presented_basis(&self) -> UiEguiPresentedInputSelection {
        match self.presentations.len() {
            0 => UiEguiPresentedInputSelection::Missing,
            1 => UiEguiPresentedInputSelection::Unique(
                *self
                    .presentations
                    .values()
                    .next()
                    .expect("one presentation basis exists"),
            ),
            count => UiEguiPresentedInputSelection::Ambiguous(count),
        }
    }

    pub(super) fn transaction_state(
        &self,
        basis: UiEguiPresentedInputBasis,
    ) -> UiEguiInputTranslationState {
        self.partitions
            .get(&basis.presentation().binding())
            .filter(|partition| partition.host_session == basis.host_session())
            .map(|partition| partition.translation.clone())
            .unwrap_or_default()
    }

    pub(super) fn input_recipient(
        &self,
        basis: UiEguiPresentedInputBasis,
    ) -> Option<worth_ui_host_contract::UiHostInputRecipientBindingReceipt> {
        self.input_recipients
            .get(&basis.host_session())
            .copied()
            .filter(|recipient| recipient.binding() == basis.presentation().binding())
    }

    pub(super) fn install_input_recipient(
        &mut self,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        let presented = self
            .presentations
            .get(&binding.binding())
            .is_some_and(|basis| basis.host_session() == binding.host_session());
        if !presented {
            return false;
        }
        self.input_recipients
            .insert(binding.host_session(), binding);
        true
    }

    pub(super) fn clear_input_recipient(
        &mut self,
        binding: worth_ui_host_contract::UiHostInputRecipientBindingReceipt,
    ) -> bool {
        if self.input_recipients.get(&binding.host_session()) != Some(&binding) {
            return false;
        }
        self.input_recipients.remove(&binding.host_session());
        true
    }

    pub(super) fn commit(
        &mut self,
        basis: UiEguiPresentedInputBasis,
        translation: UiEguiInputTranslationState,
    ) {
        self.partitions.insert(
            basis.presentation().binding(),
            UiEguiInputPartition {
                host_session: basis.host_session(),
                translation,
            },
        );
    }

    pub(super) fn remove_binding(&mut self, binding: UiSurfaceBindingGeneration) {
        self.presentations.remove(&binding);
        self.partitions.remove(&binding);
        self.input_recipients
            .retain(|_, recipient| recipient.binding() != binding);
    }

    pub(super) fn release_session(&mut self, host_session: u64) {
        self.presentations
            .retain(|_, basis| basis.host_session() != host_session);
        self.partitions
            .retain(|_, partition| partition.host_session != host_session);
        self.input_recipients.remove(&host_session);
    }
}

impl UiEguiInputTranslationState {
    pub(super) fn take_sequence(&mut self) -> Option<UiHostObservationSequence> {
        let value = self.next_sequence?;
        self.next_sequence = value.checked_add(1);
        Some(UiHostObservationSequence::new(value))
    }
}

impl Default for UiEguiInputTranslationState {
    fn default() -> Self {
        Self {
            next_sequence: Some(1),
            pointer: Default::default(),
            text_ime: Default::default(),
        }
    }
}
