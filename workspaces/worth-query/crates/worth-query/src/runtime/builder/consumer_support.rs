use crate::domain_installation::{
    WorthQueryConsumerSupportDimension, WorthQueryConsumerSupportPosture,
};

use super::WorthQueryRuntimeBuilder;

impl WorthQueryRuntimeBuilder {
    pub fn consumer_support_posture(
        mut self,
        dimension: WorthQueryConsumerSupportDimension,
        posture: WorthQueryConsumerSupportPosture,
    ) -> Self {
        self.consumer_support_postures[dimension.index()] = Some(posture);
        self
    }
}
