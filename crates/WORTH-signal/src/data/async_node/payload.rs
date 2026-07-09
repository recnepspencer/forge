use serde::{Deserialize, Serialize};

use crate::data::resource::{ResourcePayloadContract, ResourcePayloadContractId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AsyncNodePayloadContractId(ResourcePayloadContractId);

impl AsyncNodePayloadContractId {
    pub const DEFAULT: Self = Self(ResourcePayloadContractId::DEFAULT);

    pub fn new(value: u64) -> Self {
        Self(ResourcePayloadContractId::new(value))
    }

    pub fn get(self) -> u64 {
        self.0.get()
    }

    pub(crate) fn into_resource(self) -> ResourcePayloadContractId {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AsyncNodePayloadContract {
    inner: ResourcePayloadContract,
}

impl AsyncNodePayloadContract {
    pub fn new(id: AsyncNodePayloadContractId) -> Self {
        Self {
            inner: ResourcePayloadContract::new(id.into_resource()),
        }
    }

    pub fn with_max_payload_bytes(mut self, max_payload_bytes: u64) -> Self {
        self.inner = self.inner.with_max_payload_bytes(max_payload_bytes);
        self
    }

    pub fn id(&self) -> AsyncNodePayloadContractId {
        AsyncNodePayloadContractId(self.inner.id())
    }

    pub fn max_payload_bytes(&self) -> Option<u64> {
        self.inner.max_payload_bytes()
    }

    pub(crate) fn as_resource(&self) -> &ResourcePayloadContract {
        &self.inner
    }

    pub(crate) fn from_resource(inner: ResourcePayloadContract) -> Self {
        Self { inner }
    }
}

impl Default for AsyncNodePayloadContract {
    fn default() -> Self {
        Self::new(AsyncNodePayloadContractId::DEFAULT)
    }
}
