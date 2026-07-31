use core::marker::PhantomData;

use super::{
    UiIntentPayload, UiIntentPayloadProjection, UiIntentPayloadProjectionViolation,
    UiIntentProjectedValue, UiSealedIntentPayload,
};

pub(crate) trait UiRegisteredIntentPayloadProjector: Send + Sync {
    fn project(
        &self,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiSealedIntentPayload, UiIntentPayloadProjectionViolation>;
}

pub(crate) struct UiTypedIntentPayloadProjector<I: super::super::UiIntent> {
    intent: PhantomData<fn() -> I>,
}

impl<I: super::super::UiIntent> UiTypedIntentPayloadProjector<I> {
    pub(crate) const fn new() -> Self {
        Self {
            intent: PhantomData,
        }
    }
}

impl<I: super::super::UiIntent> UiRegisteredIntentPayloadProjector
    for UiTypedIntentPayloadProjector<I>
{
    fn project(
        &self,
        values: Vec<UiIntentProjectedValue>,
    ) -> Result<UiSealedIntentPayload, UiIntentPayloadProjectionViolation> {
        let mut projection = UiIntentPayloadProjection::<I::Payload>::new(values);
        let payload = I::Payload::project(&mut projection)?;
        projection.finish()?;
        Ok(UiSealedIntentPayload::new::<I>(payload))
    }
}
