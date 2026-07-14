use serde::{Deserialize, Serialize};

use super::{SignalSchemaId, SignalSchemaName, SignalSchemaVersion};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalSchemaBinding {
    schema_id: SignalSchemaId,
    semantic_name: SignalSchemaName,
    version: SignalSchemaVersion,
    descriptor_digest: String,
}

impl SignalSchemaBinding {
    pub fn new(
        schema_id: SignalSchemaId,
        semantic_name: SignalSchemaName,
        version: SignalSchemaVersion,
        descriptor_digest: impl Into<String>,
    ) -> Self {
        Self {
            schema_id,
            semantic_name,
            version,
            descriptor_digest: descriptor_digest.into(),
        }
    }

    pub fn schema_id(&self) -> SignalSchemaId {
        self.schema_id
    }

    pub fn semantic_name(&self) -> &SignalSchemaName {
        &self.semantic_name
    }

    pub fn version(&self) -> SignalSchemaVersion {
        self.version
    }

    pub fn descriptor_digest(&self) -> &str {
        &self.descriptor_digest
    }
}
