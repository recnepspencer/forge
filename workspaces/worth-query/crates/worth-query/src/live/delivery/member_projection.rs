use super::stream_admission::StreamConsumerShape;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamMemberProjection {
    pub(in crate::live) digest: String,
    pub(in crate::live) consumer_shape: StreamConsumerShape,
    pub(in crate::live) member_count: usize,
    pub(in crate::live) delivery_width: usize,
}

impl StreamMemberProjection {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn consumer_shape(&self) -> &StreamConsumerShape {
        &self.consumer_shape
    }

    pub fn member_count(&self) -> usize {
        self.member_count
    }

    pub fn delivery_width(&self) -> usize {
        self.delivery_width
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamWindowCompatibility {
    pub(in crate::live) digest: String,
    pub(in crate::live) consumer_shape: StreamConsumerShape,
    pub(in crate::live) window_width: usize,
    pub(in crate::live) budget_limit: usize,
}

impl StreamWindowCompatibility {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn consumer_shape(&self) -> &StreamConsumerShape {
        &self.consumer_shape
    }

    pub fn window_width(&self) -> usize {
        self.window_width
    }

    pub fn budget_limit(&self) -> usize {
        self.budget_limit
    }
}
