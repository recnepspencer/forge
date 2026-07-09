use crate::{
    WorthServerCompatibilityPreparedRequest, WorthServerDirectRemaskDisposition,
    WorthServerDirectRemaskPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerCompatibilityCachePolicy {
    cache_control: String,
    vary: Vec<String>,
    publicly_reusable: bool,
    canonical_digest: String,
}

impl WorthServerCompatibilityCachePolicy {
    pub(crate) fn for_scoped_read(
        prepared_request: &WorthServerCompatibilityPreparedRequest,
        remask_posture: &WorthServerDirectRemaskPosture,
    ) -> Self {
        let branch_digest = prepared_request
            .admission()
            .request_context()
            .branch_target()
            .branch_digest();
        let remask_label = match remask_posture.disposition() {
            WorthServerDirectRemaskDisposition::Visible => "visible",
            WorthServerDirectRemaskDisposition::Remasked => "remasked",
            WorthServerDirectRemaskDisposition::Denied => "denied",
        };
        let vary = vec![
            "authorization".to_string(),
            "x-WORTH-branch".to_string(),
            "x-WORTH-diagnostics".to_string(),
        ];
        let cache_control = "private, no-store".to_string();
        let canonical_digest = format!(
            "compat-http-cache-policy-v1|cache-control:{cache_control}|vary:{}|public:false|branch:{branch_digest}|remask:{remask_label}",
            vary.join(","),
        );
        Self {
            cache_control,
            vary,
            publicly_reusable: false,
            canonical_digest,
        }
    }

    pub fn cache_control(&self) -> &str {
        &self.cache_control
    }

    pub fn vary(&self) -> &[String] {
        &self.vary
    }

    pub fn publicly_reusable(&self) -> bool {
        self.publicly_reusable
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
