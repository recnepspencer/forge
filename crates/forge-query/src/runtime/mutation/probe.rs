use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::ForgeQueryMutationTargetCollectionIdentity;
use forge_foundational::facade::AspectValue;

use super::denied_aspect_touch::ForgeQueryDeniedAspectTouch;
use super::{
    aspect_value_native_digest_text, ForgeQueryAspectTouch, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryParsedAspectTarget,
};

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
    UnsupportedProbedAspectValue,
}

impl ForgeQueryExistingTruthProbeDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BackendProbeUnsupported => "backend_probe_unsupported",
            Self::ResolvedTargetUnavailable => "resolved_target_unavailable",
            Self::MissingProbedAspect => "missing_probed_aspect",
            Self::UnsupportedProbedAspectValue => "unsupported_probed_aspect_value",
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
    probed_aspect_touch: Option<ForgeQueryDeniedAspectTouch>,
    message: String,
    denial_digest: String,
}

impl ForgeQueryExistingTruthProbeDenial {
    pub fn new(
        binding: &ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthProbeDenialKind,
        probed_aspect_touch: Option<ForgeQueryAspectTouch>,
        message: impl Into<String>,
    ) -> Self {
        let probed_aspect_touch = probed_aspect_touch.map(ForgeQueryDeniedAspectTouch::Admitted);
        Self::from_denied_aspect_touch(binding, kind, probed_aspect_touch, message)
    }

    fn from_admitted_touch(
        binding: &ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthProbeDenialKind,
        probed_aspect_touch: ForgeQueryAspectTouch,
        message: impl Into<String>,
    ) -> Self {
        Self::from_denied_aspect_touch(
            binding,
            kind,
            Some(ForgeQueryDeniedAspectTouch::Admitted(probed_aspect_touch)),
            message,
        )
    }

    fn from_denied_aspect_touch(
        binding: &ForgeQueryExistingTruthTargetBinding,
        kind: ForgeQueryExistingTruthProbeDenialKind,
        probed_aspect_touch: Option<ForgeQueryDeniedAspectTouch>,
        message: impl Into<String>,
    ) -> Self {
        let message = message.into();
        let probed_aspect_touch_digest = probed_aspect_touch
            .as_ref()
            .map(ForgeQueryDeniedAspectTouch::admitted_touch_digest_part);
        let target_collection_identity = binding.target_collection_identity();
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
                        .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .optional_value(
                    ForgeQueryEvidenceTag::new("aspect_touch"),
                    probed_aspect_touch_digest.as_deref(),
                )
                .field_value(ForgeQueryEvidenceTag::new("message"), &message)
                .seal()
                .as_str()
                .to_string();
        Self {
            binding: binding.clone(),
            kind,
            probed_aspect_touch,
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

    pub fn probed_aspect_touch(&self) -> Option<&ForgeQueryAspectTouch> {
        self.probed_aspect_touch
            .as_ref()
            .and_then(ForgeQueryDeniedAspectTouch::admitted_touch)
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
    aspect_touches: Vec<ForgeQueryAspectTouch>,
    request_digest: String,
}

impl ForgeQueryExistingTruthProbeRequest {
    pub fn new(
        binding: ForgeQueryExistingTruthTargetBinding,
        aspect_touches: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let mut aspect_touches = aspect_touches.into_iter().collect::<Vec<_>>();
        aspect_touches.sort();
        aspect_touches.dedup();
        if aspect_touches.is_empty() {
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
                    aspect_touches
                        .iter()
                        .map(ForgeQueryAspectTouch::admitted_touch_digest_part),
                )
                .seal()
                .as_str()
                .to_string();
        Ok(Self {
            binding,
            aspect_touches,
            request_digest,
        })
    }

    pub fn binding(&self) -> &ForgeQueryExistingTruthTargetBinding {
        &self.binding
    }

    pub fn aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.aspect_touches
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryExistingTruthProbeField {
    target: ForgeQueryParsedAspectTarget,
    value: AspectValue,
    value_digest: String,
}

impl ForgeQueryExistingTruthProbeField {
    pub fn new_native(aspect_touch: ForgeQueryAspectTouch, value: AspectValue) -> Self {
        let target = aspect_touch.into_parsed_target();
        let aspect_touch = ForgeQueryAspectTouch::from_parsed_target(target.clone());
        let value_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "existing-truth-probe-field",
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("aspect_touch"),
                    aspect_touch.admitted_touch_digest_part(),
                )
                .field_value(
                    ForgeQueryEvidenceTag::new("value"),
                    aspect_value_native_digest_text(&value),
                )
                .seal()
                .as_str()
                .to_string();
        Self {
            target,
            value,
            value_digest,
        }
    }

    pub fn aspect_touch(&self) -> ForgeQueryAspectTouch {
        ForgeQueryAspectTouch::from_parsed_target(self.target.clone())
    }

    pub fn foundational_value(&self) -> &AspectValue {
        &self.value
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
        fields: Vec<ForgeQueryExistingTruthProbeField>,
    ) -> Result<Self, ForgeQueryExistingTruthProbeDenial> {
        for requested in request.aspect_touches() {
            if !fields
                .iter()
                .any(|field| field.target == *requested.parsed_target())
            {
                return Err(ForgeQueryExistingTruthProbeDenial::from_admitted_touch(
                    request.binding(),
                    ForgeQueryExistingTruthProbeDenialKind::MissingProbedAspect,
                    requested.clone(),
                    "backend probe response did not return the requested aspect",
                ));
            }
        }
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
                .field_value(
                    ForgeQueryEvidenceTag::new("aspect_touch"),
                    field.aspect_touch().admitted_touch_digest_part(),
                )
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
        Ok(Self {
            binding: request.binding().clone(),
            mode: ForgeQueryExistingTruthProbeMode::BackendVerifiedProbe,
            fields,
            probe_digest,
        })
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

    pub fn field_for_touch(
        &self,
        aspect_touch: &ForgeQueryAspectTouch,
    ) -> Option<&ForgeQueryExistingTruthProbeField> {
        self.fields
            .iter()
            .find(|field| field.target == *aspect_touch.parsed_target())
    }

    pub fn probe_digest(&self) -> &str {
        &self.probe_digest
    }
}
