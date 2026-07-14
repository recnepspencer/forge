use std::collections::BTreeMap;

use serde_json::Value;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerUploadExpectation {
    ContinueOptional,
    ContinueRequired,
}

impl WorthServerUploadExpectation {
    pub fn continue_optional() -> Self {
        Self::ContinueOptional
    }

    pub fn continue_required() -> Self {
        Self::ContinueRequired
    }

    pub fn requires_early_admission(self) -> bool {
        matches!(self, Self::ContinueRequired)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerUploadTransferMode {
    DeclaredLength,
    UnknownLength,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerUploadContentEncoding {
    Identity,
    Gzip,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerUploadChunk {
    wire_bytes: Vec<u8>,
}

impl WorthServerUploadChunk {
    pub fn new(wire_bytes: impl Into<Vec<u8>>) -> Self {
        Self {
            wire_bytes: wire_bytes.into(),
        }
    }

    pub fn wire_bytes(&self) -> &[u8] {
        &self.wire_bytes
    }

    pub fn wire_len(&self) -> u64 {
        self.wire_bytes.len() as u64
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerUploadPart {
    name: String,
    content_type: String,
    declared_length: u64,
    transfer_mode: WorthServerUploadTransferMode,
    content_encoding: WorthServerUploadContentEncoding,
    authoritative_bytes: Vec<u8>,
    wire_chunks: Vec<WorthServerUploadChunk>,
    declared_integrity_digest: Option<String>,
}

impl WorthServerUploadPart {
    pub fn file(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content_type: "application/octet-stream".to_string(),
            declared_length: 0,
            transfer_mode: WorthServerUploadTransferMode::DeclaredLength,
            content_encoding: WorthServerUploadContentEncoding::Identity,
            authoritative_bytes: Vec::new(),
            wire_chunks: Vec::new(),
            declared_integrity_digest: None,
        }
    }

    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = content_type.into();
        self
    }

    pub fn with_declared_length(mut self, declared_length: u64) -> Self {
        self.declared_length = declared_length;
        self
    }

    pub fn with_transfer_mode(mut self, transfer_mode: WorthServerUploadTransferMode) -> Self {
        self.transfer_mode = transfer_mode;
        self
    }

    pub fn with_content_encoding(
        mut self,
        content_encoding: WorthServerUploadContentEncoding,
    ) -> Self {
        self.content_encoding = content_encoding;
        self
    }

    pub fn with_body_bytes(mut self, bytes: impl Into<Vec<u8>>) -> Self {
        self.authoritative_bytes = bytes.into();
        self
    }

    pub fn with_wire_chunk(mut self, chunk: WorthServerUploadChunk) -> Self {
        self.wire_chunks.push(chunk);
        self
    }

    pub fn with_integrity_digest(mut self, digest: impl Into<String>) -> Self {
        self.declared_integrity_digest = Some(digest.into().trim().to_string());
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn declared_length(&self) -> u64 {
        self.declared_length
    }

    pub fn transfer_mode(&self) -> WorthServerUploadTransferMode {
        self.transfer_mode
    }

    pub fn content_encoding(&self) -> WorthServerUploadContentEncoding {
        self.content_encoding
    }

    pub fn authoritative_bytes(&self) -> &[u8] {
        &self.authoritative_bytes
    }

    pub fn wire_chunks(&self) -> &[WorthServerUploadChunk] {
        &self.wire_chunks
    }

    pub fn declared_integrity_digest(&self) -> Option<&str> {
        self.declared_integrity_digest.as_deref()
    }

    pub fn authoritative_len(&self) -> u64 {
        if self.authoritative_bytes.is_empty() {
            self.declared_length
        } else {
            self.authoritative_bytes.len() as u64
        }
    }

    pub fn total_wire_len(&self) -> u64 {
        if self.wire_chunks.is_empty() {
            self.authoritative_len()
        } else {
            self.wire_chunks
                .iter()
                .map(WorthServerUploadChunk::wire_len)
                .sum()
        }
    }

    pub fn effective_authoritative_bytes(&self) -> Vec<u8> {
        if self.authoritative_bytes.is_empty() {
            vec![0; self.declared_length as usize]
        } else {
            self.authoritative_bytes.clone()
        }
    }

    pub fn effective_wire_chunks(&self) -> Vec<WorthServerUploadChunk> {
        if self.wire_chunks.is_empty() {
            vec![WorthServerUploadChunk::new(
                self.effective_authoritative_bytes(),
            )]
        } else {
            self.wire_chunks.clone()
        }
    }

    pub fn canonical_digest(&self) -> String {
        format!(
            "part={}|content_type={}|declared_length={}|transfer_mode={:?}|encoding={:?}|authoritative_length={}|wire_length={}|declared_integrity={}",
            self.name.trim(),
            self.content_type.trim().to_ascii_lowercase(),
            self.declared_length,
            self.transfer_mode,
            self.content_encoding,
            self.authoritative_len(),
            self.total_wire_len(),
            self.declared_integrity_digest.as_deref().unwrap_or("none"),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerUploadManifest {
    metadata_body: Value,
    declared_file_parts: Vec<String>,
    declared_integrity_digest: Option<String>,
}

impl WorthServerUploadManifest {
    pub fn new(metadata_body: Value) -> Self {
        Self {
            metadata_body,
            declared_file_parts: Vec::new(),
            declared_integrity_digest: None,
        }
    }

    pub fn with_file_part(mut self, part_name: impl Into<String>) -> Self {
        self.declared_file_parts.push(part_name.into());
        self
    }

    pub fn with_integrity_digest(mut self, digest: impl Into<String>) -> Self {
        self.declared_integrity_digest = Some(digest.into().trim().to_string());
        self
    }

    pub fn metadata_body(&self) -> &Value {
        &self.metadata_body
    }

    pub fn declared_file_parts(&self) -> &[String] {
        &self.declared_file_parts
    }

    pub fn declared_integrity_digest(&self) -> Option<&str> {
        self.declared_integrity_digest.as_deref()
    }

    pub fn integrity_basis(&self) -> String {
        let mut declared_parts = self
            .declared_file_parts
            .iter()
            .map(|value| value.trim().to_string())
            .collect::<Vec<_>>();
        declared_parts.sort();
        format!(
            "metadata={}|file_parts={}",
            canonical_json(&self.metadata_body),
            declared_parts.join(","),
        )
    }

    pub fn canonical_digest(&self) -> String {
        format!(
            "{}|declared_integrity={}",
            self.integrity_basis(),
            self.declared_integrity_digest.as_deref().unwrap_or("none"),
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerMultipartUpload {
    manifest: WorthServerUploadManifest,
    expectation: WorthServerUploadExpectation,
    parts: Vec<WorthServerUploadPart>,
    canonical_digest: String,
}

impl WorthServerMultipartUpload {
    pub fn new(manifest: WorthServerUploadManifest) -> Self {
        let mut upload = Self {
            manifest,
            expectation: WorthServerUploadExpectation::continue_optional(),
            parts: Vec::new(),
            canonical_digest: String::new(),
        };
        upload.refresh_canonical_digest();
        upload
    }

    pub fn with_expectation(mut self, expectation: WorthServerUploadExpectation) -> Self {
        self.expectation = expectation;
        self.refresh_canonical_digest();
        self
    }

    pub fn with_part(mut self, part: WorthServerUploadPart) -> Self {
        self.parts.push(part);
        self.refresh_canonical_digest();
        self
    }

    pub fn manifest(&self) -> &WorthServerUploadManifest {
        &self.manifest
    }

    pub fn expectation(&self) -> WorthServerUploadExpectation {
        self.expectation
    }

    pub fn parts(&self) -> &[WorthServerUploadPart] {
        &self.parts
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    fn refresh_canonical_digest(&mut self) {
        let mut part_digests = self
            .parts
            .iter()
            .map(WorthServerUploadPart::canonical_digest)
            .collect::<Vec<_>>();
        part_digests.sort();
        self.canonical_digest = format!(
            "worth-server-multipart-upload-v1|expectation={:?}|manifest={}|parts={}",
            self.expectation,
            self.manifest.canonical_digest(),
            part_digests.join("||"),
        );
    }
}

fn canonical_json(value: &Value) -> String {
    match value {
        Value::Null => "null".to_string(),
        Value::Bool(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::String(value) => format!("{value:?}"),
        Value::Array(values) => format!(
            "[{}]",
            values
                .iter()
                .map(canonical_json)
                .collect::<Vec<_>>()
                .join(",")
        ),
        Value::Object(values) => {
            let ordered = values
                .iter()
                .map(|(name, value)| (name.as_str(), value))
                .collect::<BTreeMap<_, _>>();
            format!(
                "{{{}}}",
                ordered
                    .iter()
                    .map(|(name, value)| format!("{name:?}:{}", canonical_json(value)))
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
    }
}
