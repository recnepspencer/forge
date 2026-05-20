use std::marker::PhantomData;

use serde_json::Value;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryDerivedPatch, ForgeQueryRuntimeDeliveryBatch,
    ForgeQueryRuntimeLiveSubscriptionInstallation,
};
use crate::memory_workspace::{ForgeQueryLivePatch, ForgeQueryLiveViewHandle};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryPatchBatch {
    pub view_name: String,
    pub live_patches: Vec<ForgeQueryLivePatch>,
    pub query_delivery_batches: Vec<ForgeQueryRuntimeDeliveryBatch>,
    pub derived_patch_notes: Vec<String>,
    pub derived_patches: Vec<ForgeQueryDerivedPatch>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveView<T = Value> {
    pub(super) handle: ForgeQueryLiveViewHandle,
    pub(super) authority_lane: ForgeQueryAuthorityLane,
    pub(super) subscription_installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    pub(super) marker: PhantomData<T>,
}

impl<T> Clone for ForgeQueryLiveView<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
            authority_lane: self.authority_lane,
            subscription_installation: self.subscription_installation.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> ForgeQueryLiveView<T> {
    pub(in crate::runtime) fn new(
        handle: ForgeQueryLiveViewHandle,
        subscription_installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Self {
        Self {
            handle,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            subscription_installation,
            marker: PhantomData,
        }
    }

    pub fn name(&self) -> &str {
        self.handle.name()
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn subscription_installation(&self) -> &ForgeQueryRuntimeLiveSubscriptionInstallation {
        &self.subscription_installation
    }
}
