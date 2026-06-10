use super::{ForgeServerBinaryDownloadAuthorization, ForgeServerBinaryResumeRequest};
use crate::surfaces::compat_http::binary_digest::stable_byte_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerBinaryDownloadRequest {
    body_bytes: Vec<u8>,
    content_type: String,
    authorization: ForgeServerBinaryDownloadAuthorization,
    resume_request: Option<ForgeServerBinaryResumeRequest>,
    payload_digest: String,
    canonical_digest: String,
}

impl ForgeServerBinaryDownloadRequest {
    pub fn new(body_bytes: Vec<u8>) -> Self {
        let authorization =
            ForgeServerBinaryDownloadAuthorization::entire_representation(body_bytes.len());
        let mut request = Self {
            body_bytes,
            content_type: "application/octet-stream".to_string(),
            authorization,
            resume_request: None,
            payload_digest: String::new(),
            canonical_digest: String::new(),
        };
        request.rebuild_digest();
        request
    }

    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = content_type.into().trim().to_string();
        self.rebuild_digest();
        self
    }

    pub fn with_authorization(
        mut self,
        authorization: ForgeServerBinaryDownloadAuthorization,
    ) -> Self {
        self.authorization = authorization;
        self.rebuild_digest();
        self
    }

    pub fn with_resume_request(mut self, resume_request: ForgeServerBinaryResumeRequest) -> Self {
        self.resume_request = Some(resume_request);
        self.rebuild_digest();
        self
    }

    pub fn body_bytes(&self) -> &[u8] {
        &self.body_bytes
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn authorization(&self) -> &ForgeServerBinaryDownloadAuthorization {
        &self.authorization
    }

    pub fn resume_request(&self) -> Option<&ForgeServerBinaryResumeRequest> {
        self.resume_request.as_ref()
    }

    pub fn payload_digest(&self) -> &str {
        &self.payload_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    fn rebuild_digest(&mut self) {
        self.payload_digest = stable_byte_digest(&self.body_bytes);
        self.canonical_digest = format!(
            "compat-http-binary-download-request-v1|payload={}|content_type={}|authorization={}|resume={}",
            self.payload_digest,
            self.content_type,
            self.authorization.canonical_digest(),
            self.resume_request
                .as_ref()
                .map_or("none", ForgeServerBinaryResumeRequest::canonical_digest),
        );
    }
}
