use super::{
    WorthUiCanvasSpatialHostOutput, WorthUiHostOutputGeneration, WorthUiHostOutputGenerationDenial,
    WorthUiOrdinaryHostOutput, WorthUiRealtimeHostOutput, WorthUiVirtualizedDataHostOutput,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostOutputEnvelope {
    generation: WorthUiHostOutputGeneration,
    receipt_reference: WorthUiHostOutputReceiptReference,
    payload: WorthUiHostOutputPayload,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiHostOutputReceiptReference {
    lane: WorthUiHostOutputLane,
    digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WorthUiHostOutputLane {
    Ordinary,
    VirtualizedData,
    CanvasSpatial,
    RealtimeOverlay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum WorthUiHostOutputPayload {
    Ordinary(WorthUiOrdinaryHostOutput),
    VirtualizedData(WorthUiVirtualizedDataHostOutput),
    CanvasSpatial(WorthUiCanvasSpatialHostOutput),
    RealtimeOverlay(WorthUiRealtimeHostOutput),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHostOutputDisposition {
    Consumed,
    UnsupportedPayload,
}

impl WorthUiHostOutputEnvelope {
    pub fn ordinary(
        generation: WorthUiHostOutputGeneration,
        receipt_digest: u64,
        output: WorthUiOrdinaryHostOutput,
    ) -> Self {
        Self::new(
            generation,
            WorthUiHostOutputLane::Ordinary,
            receipt_digest,
            WorthUiHostOutputPayload::Ordinary(output),
        )
    }

    pub fn virtualized_data(
        generation: WorthUiHostOutputGeneration,
        receipt_digest: u64,
        output: WorthUiVirtualizedDataHostOutput,
    ) -> Self {
        Self::new(
            generation,
            WorthUiHostOutputLane::VirtualizedData,
            receipt_digest,
            WorthUiHostOutputPayload::VirtualizedData(output),
        )
    }

    pub fn canvas_spatial(
        generation: WorthUiHostOutputGeneration,
        receipt_digest: u64,
        output: WorthUiCanvasSpatialHostOutput,
    ) -> Self {
        Self::new(
            generation,
            WorthUiHostOutputLane::CanvasSpatial,
            receipt_digest,
            WorthUiHostOutputPayload::CanvasSpatial(output),
        )
    }

    pub fn realtime_overlay(
        generation: WorthUiHostOutputGeneration,
        receipt_digest: u64,
        output: WorthUiRealtimeHostOutput,
    ) -> Self {
        Self::new(
            generation,
            WorthUiHostOutputLane::RealtimeOverlay,
            receipt_digest,
            WorthUiHostOutputPayload::RealtimeOverlay(output),
        )
    }

    fn new(
        generation: WorthUiHostOutputGeneration,
        lane: WorthUiHostOutputLane,
        receipt_digest: u64,
        payload: WorthUiHostOutputPayload,
    ) -> Self {
        Self {
            generation,
            receipt_reference: WorthUiHostOutputReceiptReference::new(lane, receipt_digest),
            payload,
        }
    }

    pub fn validate_generation(
        self,
        expected: WorthUiHostOutputGeneration,
    ) -> Result<(), WorthUiHostOutputGenerationDenial> {
        self.generation.validate(expected)
    }

    pub fn generation(self) -> WorthUiHostOutputGeneration {
        self.generation
    }

    pub fn payload(self) -> WorthUiHostOutputPayload {
        self.payload
    }

    pub fn receipt_reference(self) -> WorthUiHostOutputReceiptReference {
        self.receipt_reference
    }
}

impl WorthUiHostOutputReceiptReference {
    fn new(lane: WorthUiHostOutputLane, digest: u64) -> Self {
        Self { lane, digest }
    }

    pub fn lane(self) -> WorthUiHostOutputLane {
        self.lane
    }

    pub fn digest(self) -> u64 {
        self.digest
    }
}
