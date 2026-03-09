use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::artifact::AttachmentRecord;
use crate::extension::ExtensionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    JsonPretty,
    JsonCompact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactPayloadKind {
    Utf8,
    Binary,
    Reference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchiveAsset {
    pub asset_name: String,
    pub logical_path: String,
    pub media_type: String,
    pub payload_kind: ArtifactPayloadKind,
    pub text_content: Option<String>,
    pub binary_content: Option<Vec<u8>>,
    pub content_reference: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordArchive {
    pub archive_name: String,
    pub format_name: String,
    pub records: BTreeMap<String, String>,
    pub assets: Vec<ArchiveAsset>,
    pub attachments: Vec<AttachmentRecord>,
}

impl RecordArchive {
    pub fn new(archive_name: impl Into<String>, format: ExportFormat) -> Self {
        Self {
            archive_name: archive_name.into(),
            format_name: format.label().to_string(),
            records: BTreeMap::new(),
            assets: Vec::new(),
            attachments: Vec::new(),
        }
    }

    pub fn insert_serialized<Record>(
        &mut self,
        record_name: impl Into<String>,
        record: &Record,
        format: ExportFormat,
    ) -> Result<(), serde_json::Error>
    where
        Record: Serialize,
    {
        self.records
            .insert(record_name.into(), export_record(record, format)?);
        Ok(())
    }

    pub fn insert_text_asset(
        &mut self,
        asset_name: impl Into<String>,
        logical_path: impl Into<String>,
        media_type: impl Into<String>,
        text_content: impl Into<String>,
    ) {
        self.assets.push(ArchiveAsset {
            asset_name: asset_name.into(),
            logical_path: logical_path.into(),
            media_type: media_type.into(),
            payload_kind: ArtifactPayloadKind::Utf8,
            text_content: Some(text_content.into()),
            binary_content: None,
            content_reference: None,
            metadata: BTreeMap::new(),
        });
    }

    pub fn insert_binary_asset(
        &mut self,
        asset_name: impl Into<String>,
        logical_path: impl Into<String>,
        media_type: impl Into<String>,
        binary_content: Vec<u8>,
    ) {
        self.assets.push(ArchiveAsset {
            asset_name: asset_name.into(),
            logical_path: logical_path.into(),
            media_type: media_type.into(),
            payload_kind: ArtifactPayloadKind::Binary,
            text_content: None,
            binary_content: Some(binary_content),
            content_reference: None,
            metadata: BTreeMap::new(),
        });
    }

    pub fn insert_reference_asset(
        &mut self,
        asset_name: impl Into<String>,
        logical_path: impl Into<String>,
        media_type: impl Into<String>,
        content_reference: impl Into<String>,
    ) {
        self.assets.push(ArchiveAsset {
            asset_name: asset_name.into(),
            logical_path: logical_path.into(),
            media_type: media_type.into(),
            payload_kind: ArtifactPayloadKind::Reference,
            text_content: None,
            binary_content: None,
            content_reference: Some(content_reference.into()),
            metadata: BTreeMap::new(),
        });
    }

    pub fn attach(mut self, attachment: AttachmentRecord) -> Self {
        self.attachments.push(attachment);
        self
    }

    pub fn add_attachment(&mut self, attachment: AttachmentRecord) {
        self.attachments.push(attachment);
    }
}

pub trait ArchiveExportSink {
    fn export_archive(&self, archive: &RecordArchive) -> ExtensionResult;
}

impl ExportFormat {
    pub fn label(self) -> &'static str {
        match self {
            Self::JsonPretty => "json-pretty",
            Self::JsonCompact => "json-compact",
        }
    }
}

pub fn export_record<T>(record: &T, format: ExportFormat) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    match format {
        ExportFormat::JsonPretty => serde_json::to_string_pretty(record),
        ExportFormat::JsonCompact => serde_json::to_string(record),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::Serialize;

    use crate::artifact::{AttachmentKind, AttachmentRecord};

    use super::{export_record, ArchiveAsset, ArtifactPayloadKind, ExportFormat, RecordArchive};

    #[derive(Serialize)]
    struct Example<'a> {
        name: &'a str,
    }

    #[test]
    fn export_record_serializes_json() {
        let text = export_record(&Example { name: "forge" }, ExportFormat::JsonCompact).unwrap();
        assert!(text.contains("forge"));
    }

    #[test]
    fn record_archive_collects_named_exports() {
        let mut archive = RecordArchive::new("bundle", ExportFormat::JsonPretty);
        archive
            .insert_serialized(
                "example",
                &Example { name: "forge" },
                ExportFormat::JsonCompact,
            )
            .unwrap();
        archive.insert_text_asset("report", "reports/report.txt", "text/plain", "forge");
        archive.insert_reference_asset(
            "mesh",
            "geometry/wing.mesh",
            "application/octet-stream",
            "s3://bucket/wing.mesh",
        );
        archive.add_attachment(
            AttachmentRecord::with_reference(
                "trace",
                AttachmentKind::Trace,
                "application/json",
                "trace://record",
            ),
        );
        assert_eq!(archive.format_name, "json-pretty");
        assert!(archive.records.get("example").unwrap().contains("forge"));
        assert_eq!(archive.assets.len(), 2);
        assert_eq!(archive.attachments.len(), 1);
        assert_eq!(archive.assets[0].payload_kind, ArtifactPayloadKind::Utf8);
        assert_eq!(archive.assets[1].payload_kind, ArtifactPayloadKind::Reference);
    }

    #[test]
    fn archive_asset_can_hold_binary_payloads() {
        let asset = ArchiveAsset {
            asset_name: "mesh".to_string(),
            logical_path: "mesh.bin".to_string(),
            media_type: "application/octet-stream".to_string(),
            payload_kind: ArtifactPayloadKind::Binary,
            text_content: None,
            binary_content: Some(vec![1, 2, 3]),
            content_reference: None,
            metadata: BTreeMap::new(),
        };
        assert_eq!(asset.binary_content.unwrap(), vec![1, 2, 3]);
    }
}
