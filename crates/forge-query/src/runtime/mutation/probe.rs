use serde_json::Value;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::ForgeQueryMutationTargetCollectionIdentity;

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
        let target_collection_identity = binding.target_collection().map(|collection| {
            ForgeQueryMutationTargetCollectionIdentity::new("existing-truth-probe", collection)
        });
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-probe-denial",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("family"),
                    binding.family().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("authoritative"),
                    binding.authoritative_identity().evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved"),
                    &binding.resolved_target_identity().evidence_identity(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("collection"),
                    target_collection_identity
                        .as_ref()
                        .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .optional_value(
                    ForgeQueryEvidenceTag::new("aspect"),
                    probed_aspect_path.as_deref(),
                )
                .field_value(ForgeQueryEvidenceTag::new("message"), &message)
                .seal()
                .as_str()
                .to_string();
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
            self.binding.authoritative_identity().as_str(),
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
        let request_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-probe-request",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("binding"),
                    binding.binding_evidence_identity(),
                )
                .field_value_sequence(
                    ForgeQueryEvidenceTag::new("aspect"),
                    aspect_paths.iter().map(String::as_str),
                )
                .seal()
                .as_str()
                .to_string();
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
    external_value_json: String,
    value_digest: String,
}

impl ForgeQueryExistingTruthProbeField {
    fn new(aspect_path: String, value: Value) -> Self {
        let external_value_json =
            serde_json::to_string(&value).unwrap_or_else(|_| value.to_string());
        let value_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-probe-field",
                )
                .field_value(ForgeQueryEvidenceTag::new("aspect"), &aspect_path)
                .field_value(ForgeQueryEvidenceTag::new("value"), &external_value_json)
                .seal()
                .as_str()
                .to_string();
        Self {
            aspect_path,
            value,
            external_value_json,
            value_digest,
        }
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn external_value_json(&self) -> &str {
        &self.external_value_json
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
        let field_identities = fields
            .iter()
            .map(|field| {
                forge_query_evidence_identity(
                    ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-probe-row",
                )
                .field_value(ForgeQueryEvidenceTag::new("aspect"), field.aspect_path())
                .field_value(ForgeQueryEvidenceTag::new("field"), field.value_digest())
                .seal()
            })
            .collect::<Vec<_>>();
        let probe_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "existing-truth-probe")
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("binding"),
                    request.binding().binding_evidence_identity(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("request"),
                    request.request_digest(),
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("mode"),
                    ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe.as_str(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("field"),
                    field_identities.iter(),
                )
                .seal()
                .as_str()
                .to_string();
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
