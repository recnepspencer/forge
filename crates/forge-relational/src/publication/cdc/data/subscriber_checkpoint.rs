use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::ReplaySchemaVersion;
use crate::schema::data::SchemaVersionId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriberCheckpoint {
    position: PatchStreamPosition,
    replay_schema_version: ReplaySchemaVersion,
    schema_version: SchemaVersionId,
}

impl SubscriberCheckpoint {
    pub(crate) fn new(
        position: PatchStreamPosition,
        replay_schema_version: ReplaySchemaVersion,
        schema_version: SchemaVersionId,
    ) -> Self {
        Self {
            position,
            replay_schema_version,
            schema_version,
        }
    }

    pub fn position(&self) -> PatchStreamPosition {
        self.position
    }

    pub fn replay_schema_version(&self) -> &ReplaySchemaVersion {
        &self.replay_schema_version
    }

    pub fn schema_version(&self) -> SchemaVersionId {
        self.schema_version
    }
}
