use std::marker::PhantomData;

use super::super::WorthQueryAuthorityLane;
use super::delivery::WorthQueryEffectPayload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEffectHandle<T = WorthQueryEffectPayload> {
    name: String,
    authority_lane: WorthQueryAuthorityLane,
    marker: PhantomData<T>,
}

impl<T> WorthQueryEffectHandle<T> {
    pub(in crate::runtime) fn new(
        name: impl Into<String>,
        authority_lane: WorthQueryAuthorityLane,
    ) -> Self {
        Self {
            name: name.into(),
            authority_lane,
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn authority_lane(&self) -> WorthQueryAuthorityLane {
        self.authority_lane
    }
}
