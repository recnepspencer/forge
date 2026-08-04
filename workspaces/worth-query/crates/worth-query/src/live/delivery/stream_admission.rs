#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StreamConsumerShape {
    DetailCurrentState,
    CdcCollectionPatch,
}

impl StreamConsumerShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DetailCurrentState => "detail_current_state",
            Self::CdcCollectionPatch => "cdc_collection_patch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreamContractRequest {
    pub(in crate::live) digest: String,
    pub(in crate::live) query_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) consumer_shape: StreamConsumerShape,
}

impl StreamContractRequest {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn consumer_shape(&self) -> &StreamConsumerShape {
        &self.consumer_shape
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedStreamConsumerContract {
    pub(in crate::live) digest: String,
    pub(in crate::live) consumer_shape: StreamConsumerShape,
}

impl AdmittedStreamConsumerContract {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn consumer_shape(&self) -> &StreamConsumerShape {
        &self.consumer_shape
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StreamContractDigest(pub(in crate::live) String);

impl StreamContractDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
