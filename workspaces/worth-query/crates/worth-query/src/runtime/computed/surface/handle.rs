use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDerivedViewHandle<T = crate::runtime::WorthQueryUnrefinedLiveShape> {
    name: String,
    authority_lane: WorthQueryAuthorityLane,
    marker: PhantomData<T>,
}

impl<T> WorthQueryDerivedViewHandle<T> {
    pub(in crate::runtime) fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            authority_lane: WorthQueryAuthorityLane::DerivedRuntimeState,
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
