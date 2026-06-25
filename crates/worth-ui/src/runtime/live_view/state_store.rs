use std::collections::BTreeMap;

use super::{WorthUiLiveViewStateFactId, WorthUiLiveViewStateValue};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiLiveViewStateStore {
    values: BTreeMap<WorthUiLiveViewStateFactId, WorthUiLiveViewStateValue>,
}

impl WorthUiLiveViewStateStore {
    pub(crate) fn get(
        &self,
        fact: &WorthUiLiveViewStateFactId,
    ) -> Option<&WorthUiLiveViewStateValue> {
        self.values.get(fact)
    }

    pub(crate) fn record(
        &mut self,
        fact: WorthUiLiveViewStateFactId,
        value: WorthUiLiveViewStateValue,
    ) -> Option<WorthUiLiveViewStateValue> {
        self.values.insert(fact, value)
    }
}
