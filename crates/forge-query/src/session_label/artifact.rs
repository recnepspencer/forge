use super::digest::derive_session_label_identity;
use super::error::ForgeQuerySessionLabelError;
use super::namespace::ForgeQuerySessionNamespace;
use super::sealed::SealedForgeQuerySessionLabel;
use super::segment::ForgeQuerySessionLabelSegment;
use crate::evidence_identity::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope};

#[derive(Clone, Debug)]
pub struct ForgeQuerySessionLabel {
    namespace: ForgeQuerySessionNamespace,
    name_segments: Vec<ForgeQuerySessionLabelSegment>,
    display: String,
    identity_digest: ForgeQueryEvidenceIdentity,
}

impl PartialEq for ForgeQuerySessionLabel {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.name_segments == other.name_segments
    }
}

impl Eq for ForgeQuerySessionLabel {}

impl std::hash::Hash for ForgeQuerySessionLabel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.namespace.hash(state);
        self.name_segments.hash(state);
    }
}

impl ForgeQuerySessionLabel {
    pub fn scoped(
        namespace: ForgeQuerySessionNamespace,
        name_segments: impl IntoIterator<Item = ForgeQuerySessionLabelSegment>,
    ) -> Result<Self, ForgeQuerySessionLabelError> {
        let name_segments = name_segments.into_iter().collect::<Vec<_>>();
        if name_segments.is_empty() {
            return Err(ForgeQuerySessionLabelError::MissingNameSegments);
        }
        let display = render_display(&namespace, &name_segments);
        let identity_digest = derive_session_label_identity(&namespace, &name_segments);
        Ok(Self::new(SealedForgeQuerySessionLabel::new(
            namespace,
            name_segments,
            display,
            identity_digest,
        )))
    }

    pub fn scoped_strs<I, S>(
        namespace: impl Into<String>,
        name_segments: I,
    ) -> Result<Self, ForgeQuerySessionLabelError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::scoped(
            ForgeQuerySessionNamespace::new(namespace)?,
            name_segments
                .into_iter()
                .map(ForgeQuerySessionLabelSegment::new)
                .collect::<Result<Vec<_>, _>>()?,
        )
    }

    pub fn namespace(&self) -> &ForgeQuerySessionNamespace {
        &self.namespace
    }

    pub fn name_segments(&self) -> &[ForgeQuerySessionLabelSegment] {
        &self.name_segments
    }

    pub fn identity_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity_digest
    }

    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn identity_scope() -> ForgeQueryEvidenceScope {
        ForgeQueryEvidenceScope::SessionLabelIdentity
    }

    pub(crate) fn new(sealed: SealedForgeQuerySessionLabel) -> Self {
        Self {
            namespace: sealed.namespace,
            name_segments: sealed.name_segments,
            display: sealed.display,
            identity_digest: sealed.identity_digest,
        }
    }
}

impl std::fmt::Display for ForgeQuerySessionLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.display())
    }
}

fn render_display(
    namespace: &ForgeQuerySessionNamespace,
    name_segments: &[ForgeQuerySessionLabelSegment],
) -> String {
    let mut parts = Vec::with_capacity(name_segments.len() + 1);
    parts.push(namespace.as_str().to_string());
    parts.extend(
        name_segments
            .iter()
            .map(ForgeQuerySessionLabelSegment::as_str)
            .map(str::to_string),
    );
    parts.join(".")
}
