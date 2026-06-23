use std::marker::PhantomData;

use super::super::ForgeQueryAuthorityLane;
use super::delivery::ForgeQueryEffectPayload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectHandle<T = ForgeQueryEffectPayload> {
    name: String,
    authority_lane: ForgeQueryAuthorityLane,
    marker: PhantomData<T>,
}

impl<T> ForgeQueryEffectHandle<T> {
    pub(in crate::runtime) fn new(
        name: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
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

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }
}
