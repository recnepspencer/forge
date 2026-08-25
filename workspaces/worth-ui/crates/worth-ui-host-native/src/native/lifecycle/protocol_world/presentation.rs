use crate::native::presentation::port::orchestrator::{
    UiNativePresentationStageControl, UiNativePresentationStagePort,
};
use crate::native::presentation::UiNativePresentationFault;
use crate::native::UiNativePresentationEffectPhase;

use super::schema::{UiNativeProtocolClosePoint, UiNativeProtocolReadback};

#[derive(Clone, Copy)]
pub(crate) struct UiPrepared;
#[derive(Clone, Copy)]
pub(crate) struct UiAcquired;
#[derive(Clone, Copy)]
pub(crate) struct UiEncoded;
#[derive(Clone, Copy)]
pub(crate) struct UiSubmitted;
#[derive(Clone, Copy)]
pub(crate) struct UiHandoff;

pub(in crate::native::lifecycle) struct UiProtocolPresentationPort {
    fault: Option<UiNativePresentationFault>,
    readback: UiNativeProtocolReadback,
}

pub(in crate::native::lifecycle) struct UiProtocolCloseControl<'trace> {
    close_at: Option<UiNativeProtocolClosePoint>,
    completed: &'trace mut Vec<UiNativePresentationEffectPhase>,
}

impl UiProtocolPresentationPort {
    pub(in crate::native::lifecycle) const fn new(
        fault: Option<UiNativePresentationFault>,
        readback: UiNativeProtocolReadback,
    ) -> Self {
        Self { fault, readback }
    }
}

impl<'trace> UiProtocolCloseControl<'trace> {
    pub(in crate::native::lifecycle) fn new(
        close_at: Option<UiNativeProtocolClosePoint>,
        completed: &'trace mut Vec<UiNativePresentationEffectPhase>,
    ) -> Self {
        Self {
            close_at,
            completed,
        }
    }
}

impl UiNativePresentationStagePort for UiProtocolPresentationPort {
    type Prepared = UiPrepared;
    type Acquired = UiAcquired;
    type Encoded = UiEncoded;
    type Submitted = UiSubmitted;
    type PresentHandoff = UiHandoff;
    type Observation = UiNativeProtocolReadback;
    type Failure = UiNativePresentationFault;

    fn prepare(&mut self) -> Result<Self::Prepared, Self::Failure> {
        Ok(UiPrepared)
    }

    fn acquire(&mut self, _: Self::Prepared) -> Result<Self::Acquired, Self::Failure> {
        if let Some(fault) = self.fault.take() {
            return Err(fault);
        }
        Ok(UiAcquired)
    }

    fn encode(&mut self, _: Self::Acquired) -> Result<Self::Encoded, Self::Failure> {
        Ok(UiEncoded)
    }

    fn submit(&mut self, _: Self::Encoded) -> Result<Self::Submitted, Self::Failure> {
        Ok(UiSubmitted)
    }

    fn hand_off(&mut self, _: Self::Submitted) -> Result<Self::PresentHandoff, Self::Failure> {
        Ok(UiHandoff)
    }

    fn observe(&mut self, _: Self::PresentHandoff) -> Result<Self::Observation, Self::Failure> {
        Ok(self.readback)
    }
}

impl UiNativePresentationStageControl for UiProtocolCloseControl<'_> {
    type Stop = UiNativeProtocolClosePoint;

    fn stage_completed(
        &mut self,
        stage: UiNativePresentationEffectPhase,
    ) -> Result<(), Self::Stop> {
        self.completed.push(stage);
        let point = close_point(stage);
        if self.close_at == Some(point) {
            Err(point)
        } else {
            Ok(())
        }
    }
}

const fn close_point(stage: UiNativePresentationEffectPhase) -> UiNativeProtocolClosePoint {
    match stage {
        UiNativePresentationEffectPhase::Prepared => UiNativeProtocolClosePoint::Prepared,
        UiNativePresentationEffectPhase::SurfaceAcquired => {
            UiNativeProtocolClosePoint::SurfaceAcquired
        }
        UiNativePresentationEffectPhase::Encoded => UiNativeProtocolClosePoint::Encoded,
        UiNativePresentationEffectPhase::Submitted => UiNativeProtocolClosePoint::Submitted,
        UiNativePresentationEffectPhase::PresentHandoff => {
            UiNativeProtocolClosePoint::PresentHandoff
        }
    }
}
