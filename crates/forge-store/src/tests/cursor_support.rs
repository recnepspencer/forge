use serde_json::Value;

use crate::{
    backend::records::{
        EmbeddedCheckpointClassification as StoredCheckpointClassification,
        EmbeddedCheckpointRecord,
    },
    BasisBoundCheckpoint, DerivedDurableCheckpointKind, DurableCursorAcknowledgeRequest,
    DurableCursorResumeRequest, ForgeStoreBuilder, HistoricalIdentityRequest, NoContainedCommits,
    StoreErrorKind,
};
use forge_relational::facade::identity::LineageId;

use super::harness::fixtures::{
    runtime::{create_entity, latest_envelope, runtime_with_demo_schema},
    stores::unique_test_sqlite_path,
};
use super::harness::{
    corruption::local_file::{
        force_cursor_checkpoint_gap, force_embedded_checkpoint_shape_violation,
    },
    fixtures::stores::unique_test_store_path,
};

#[path = "cursor_support/cursor_resume.rs"]
mod cursor_resume;
#[path = "cursor_support/embedded_checkpoints.rs"]
mod embedded_checkpoints;
#[path = "cursor_support/historical_identity.rs"]
mod historical_identity;
