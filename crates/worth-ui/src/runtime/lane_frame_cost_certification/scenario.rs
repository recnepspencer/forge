use crate::runtime::{WorthUiFrameExecutionReceipt, WorthUiLaneParityCertification};

#[derive(Clone, Debug)]
pub struct WorthUiLaneFrameCostCertificationScenario {
    name: String,
    steady_frame_receipt: Option<WorthUiFrameExecutionReceipt>,
    virtualized_data_scale_samples: Vec<WorthUiFrameExecutionReceipt>,
    realtime_scale_samples: Vec<WorthUiFrameExecutionReceipt>,
    cross_lane_parity: Option<WorthUiLaneParityCertification>,
    require_foundational_readiness: bool,
}

impl WorthUiLaneFrameCostCertificationScenario {
    pub fn named(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steady_frame_receipt: None,
            virtualized_data_scale_samples: Vec::new(),
            realtime_scale_samples: Vec::new(),
            cross_lane_parity: None,
            require_foundational_readiness: false,
        }
    }

    pub fn with_steady_frame_receipt(mut self, receipt: WorthUiFrameExecutionReceipt) -> Self {
        self.steady_frame_receipt = Some(receipt);
        self
    }

    pub fn with_virtualized_data_scale_sample(
        mut self,
        receipt: WorthUiFrameExecutionReceipt,
    ) -> Self {
        self.virtualized_data_scale_samples.push(receipt);
        self
    }

    pub fn with_realtime_scale_sample(mut self, receipt: WorthUiFrameExecutionReceipt) -> Self {
        self.realtime_scale_samples.push(receipt);
        self
    }

    pub fn with_cross_lane_parity(mut self, parity: WorthUiLaneParityCertification) -> Self {
        self.cross_lane_parity = Some(parity);
        self
    }

    pub fn require_foundational_readiness(mut self) -> Self {
        self.require_foundational_readiness = true;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn steady_frame_receipt(&self) -> Option<&WorthUiFrameExecutionReceipt> {
        self.steady_frame_receipt.as_ref()
    }

    pub(crate) fn virtualized_data_scale_samples(&self) -> &[WorthUiFrameExecutionReceipt] {
        &self.virtualized_data_scale_samples
    }

    pub(crate) fn realtime_scale_samples(&self) -> &[WorthUiFrameExecutionReceipt] {
        &self.realtime_scale_samples
    }

    pub(crate) fn cross_lane_parity(&self) -> Option<&WorthUiLaneParityCertification> {
        self.cross_lane_parity.as_ref()
    }

    pub(crate) fn requires_foundational_readiness(&self) -> bool {
        self.require_foundational_readiness
    }
}
