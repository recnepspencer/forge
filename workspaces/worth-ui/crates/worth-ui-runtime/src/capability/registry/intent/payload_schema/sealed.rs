use core::marker::PhantomData;

use super::super::UiIntent;

pub(crate) struct UiSealedIntentPayload {
    value: Box<dyn UiSealedIntentPayloadValue>,
}

trait UiSealedIntentPayloadValue: Send {}

struct UiTypedSealedIntentPayload<I: UiIntent> {
    _value: I::Payload,
    intent: PhantomData<fn() -> I>,
}

impl<I: UiIntent> UiSealedIntentPayloadValue for UiTypedSealedIntentPayload<I> {}

impl UiSealedIntentPayload {
    pub(super) fn new<I: UiIntent>(value: I::Payload) -> Self {
        Self {
            value: Box::new(UiTypedSealedIntentPayload::<I> {
                _value: value,
                intent: PhantomData,
            }),
        }
    }

    pub(crate) fn retained_payload_count(&self) -> usize {
        let _ = &self.value;
        1
    }
}
