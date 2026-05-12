use serde_json::Value;

use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryWorkspaceError;

use super::ForgeQueryExistingTruthTargetBinding;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthProbeMode {
    BackendVerifiedProbe,
}

impl ForgeQueryExistingTruthProbeMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackendVerifiedProbe => "backend_verified_probe",
        }
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthProbeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthProbeDenialKind {
    BackendProbeUnsupported,
    ResolvedTargetUnavailable,
    MissingProbedAspect,
}

impl ForgeQueryExistingTruthProbeDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackendProbeUnsupported => "backend_probe_unsupported",
            Self::ResolvedTargetUnavailable => "resolved_target_unavailable",
            Self::MissingProbedAspect => "missing_probed_aspect",
        }
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthProbeDenialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthProbeDenial {
    binding: ForgeQueryExistingTruthTargetBinding,
    kind: ForgeQueryExistingTruthProbeDenialKind,
    probed_aspect_path: Option<String>,
    message: String,
    denial_digest: String,
}

impl ForgeQueryExistingTruthProbeDenial {
    pub fn new(
        binding: &ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthProbeDenialKind,
        probed_aspect_path: Option<String>,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let denial_digest = hash_parts(&[
            "forge_query_existing_truth_probe_denial_v1".to_string(),
            format!("family:{}", binding.family()),
            format!("authoritative:{}", binding.authoritative_identity()),
            format!("resolved:{}", binding.resolved_target_identity()),
            format!(
                "collection:{}",
                binding.target_collection().unwrap_or("none")
            ),
            format!("kind:{kind}"),
            format!("aspect:{}", probed_aspect_path.as_deref().unwrap_or("none")),
            format!("message:{message}"),
        ]);
        Self {
            binding: binding.clone(),
            kind,
            probed_aspect_path,
            message,
            denial_digest,
        }
    }

    pub fn binding(&self) -> &ForgeQueryExistingTruthTargetBinding {
        &self.binding
    }

    pub fn kind(&self) -> ForgeQueryExistingTruthProbeDenialKind {
        self.kind
    }

    pub fn probed_aspect_path(&self) -> Option<&str> {
        self.probed_aspect_path.as_deref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryExistingTruthProbeDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "existing-truth probe denied for authoritative `{}` during {}: {}",
            self.binding.authoritative_identity(),
            self.kind,
            self.message
        )
    }
}

impl std::error::Error for ForgeQueryExistingTruthProbeDenial {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthProbeRequest {
    binding: ForgeQueryExistingTruthTargetBinding,
    aspect_paths: Vec<String>,
    request_digest: String,
}

impl ForgeQueryExistingTruthProbeRequest {
    pub fn new<I, S>(
        binding: ForgeQueryExistingTruthTargetBinding,
        aspect_paths: I,
    ) -> Result<Self, ForgeQueryWorkspaceError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut aspect_paths = aspect_paths
            .into_iter()
            .map(|path| normalize_non_empty(path.into(), "probe aspect path may not be empty"))
            .collect::<Result<Vec<_>, _>>()?;
        aspect_paths.sort();
        aspect_paths.dedup();
        if aspect_paths.is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "existing-truth probe must declare at least one aspect path",
            ));
        }
        let request_digest = hash_parts(
            &std::iter::once("forge_query_existing_truth_probe_request_v1".to_string())
                .chain(std::iter::once(format!(
                    "binding:{}",
                    binding.binding_digest()
                )))
                .chain(aspect_paths.iter().map(|path| format!("aspect:{path}")))
                .collect::<Vec<_>>(),
        );
        Ok(Self {
            binding,
            aspect_paths,
            request_digest,
        })
    }

    pub fn binding(&self) -> &ForgeQueryExistingTruthTargetBinding {
        &self.binding
    }

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryExistingTruthProbeField {
    aspect_path: String,
    value: Value,
    value_json: String,
    value_digest: String,
}

impl ForgeQueryExistingTruthProbeField {
    fn new(aspect_path: String, value: Value) -> Self {
        let value_json = serde_json::to_string(&value).unwrap_or_else(|_| value.to_string());
        let value_digest = hash_parts(&[
            "forge_query_existing_truth_probe_field_v1".to_string(),
            format!("aspect:{aspect_path}"),
            format!("value:{value_json}"),
        ]);
        Self {
            aspect_path,
            value,
            value_json,
            value_digest,
        }
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn value_json(&self) -> &str {
        &self.value_json
    }

    pub fn value_digest(&self) -> &str {
        &self.value_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryExistingTruthProbe {
    binding: ForgeQueryExistingTruthTargetBinding,
    mode: ForgeQueryExistingTruthProbeMode,
    fields: Vec<ForgeQueryExistingTruthProbeField>,
    probe_digest: String,
}

impl ForgeQueryExistingTruthProbe {
    pub(crate) fn backend_verified(
        request: &ForgeQueryExistingTruthProbeRequest,
        fields: Vec<(String, Value)>,
    ) -> Self {
        let fields = fields
            .into_iter()
            .map(|(aspect_path, value)| ForgeQueryExistingTruthProbeField::new(aspect_path, value))
            .collect::<Vec<_>>();
        let probe_digest =
            hash_parts(
                &std::iter::once("forge_query_existing_truth_probe_v1".to_string())
                    .chain(std::iter::once(format!(
                        "binding:{}",
                        request.binding().binding_digest()
                    )))
                    .chain(std::iter::once(format!(
                        "request:{}",
                        request.request_digest()
                    )))
                    .chain(std::iter::once("mode:backend_verified_probe".to_string()))
                    .chain(fields.iter().map(|field| {
                        format!("field:{}:{}", field.aspect_path(), field.value_digest())
                    }))
                    .collect::<Vec<_>>(),
            );
        Self {
            binding: request.binding().clone(),
            mode: ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe,
            fields,
            probe_digest,
        }
    }

    pub fn binding(&self) -> &ForgeQueryExistingTruthTargetBinding {
        &self.binding
    }

    pub fn mode(&self) -> ForgeQueryExistingTruthProbeMode {
        self.mode
    }

    pub fn fields(&self) -> &[ForgeQueryExistingTruthProbeField] {
        &self.fields
    }

    pub fn field(&self, aspect_path: &str) -> Option<&ForgeQueryExistingTruthProbeField> {
        self.fields
            .iter()
            .find(|field| field.aspect_path() == aspect_path)
    }

    pub fn probe_digest(&self) -> &str {
        &self.probe_digest
    }
}

fn normalize_non_empty(
    value: String,
    message: &'static str,
) -> Result<String, ForgeQueryWorkspaceError> {
    if value.trim().is_empty() {
        return Err(ForgeQueryWorkspaceError::new(message));
    }
    Ok(value)
}
