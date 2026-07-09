use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::capture::RecordSchemaVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AttachmentKind {
    Json,
    Text,
    Binary,
    Trace,
    Mesh,
    Snapshot,
    GeometryDump,
    Report,
    AuditPackage,
    MarketFeed,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlobDescriptor {
    pub logical_name: String,
    pub media_type: String,
    pub content_hash: Option<String>,
    pub dedup_key: Option<String>,
    pub size_bytes: Option<u64>,
    pub content_reference: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachmentRecord {
    pub schema_version: RecordSchemaVersion,
    pub name: String,
    pub kind: AttachmentKind,
    pub media_type: String,
    pub inline_value: Option<Value>,
    pub content_reference: Option<String>,
    pub blob: Option<BlobDescriptor>,
    pub metadata: BTreeMap<String, String>,
}

impl AttachmentRecord {
    pub fn json(name: impl Into<String>, value: Value) -> Self {
        Self {
            schema_version: RecordSchemaVersion::V1,
            name: name.into(),
            kind: AttachmentKind::Json,
            media_type: "application/json".to_string(),
            inline_value: Some(value),
            content_reference: None,
            blob: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn text(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            schema_version: RecordSchemaVersion::V1,
            name: name.into(),
            kind: AttachmentKind::Text,
            media_type: "text/plain".to_string(),
            inline_value: Some(Value::String(text.into())),
            content_reference: None,
            blob: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_reference(
        name: impl Into<String>,
        kind: AttachmentKind,
        media_type: impl Into<String>,
        content_reference: impl Into<String>,
    ) -> Self {
        Self {
            schema_version: RecordSchemaVersion::V1,
            name: name.into(),
            kind,
            media_type: media_type.into(),
            inline_value: None,
            content_reference: Some(content_reference.into()),
            blob: None,
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_blob(name: impl Into<String>, kind: AttachmentKind, blob: BlobDescriptor) -> Self {
        Self {
            schema_version: RecordSchemaVersion::V1,
            name: name.into(),
            kind,
            media_type: blob.media_type.clone(),
            inline_value: None,
            content_reference: Some(blob.content_reference.clone()),
            blob: Some(blob),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
