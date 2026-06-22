use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::error::ForgeQueryGraphObligationDispatchError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationRuleIdentity {
    namespace: String,
    name: String,
    semantic_version: String,
    domain_invariant_family: String,
    identity_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationRuleIdentity {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        semantic_version: impl Into<String>,
    ) -> Result<Self, ForgeQueryGraphObligationDispatchError> {
        let namespace = non_empty(
            namespace.into(),
            ForgeQueryGraphObligationDispatchError::EmptyRuleNamespace,
        )?;
        let name = non_empty(
            name.into(),
            ForgeQueryGraphObligationDispatchError::EmptyRuleName,
        )?;
        let semantic_version = non_empty(
            semantic_version.into(),
            ForgeQueryGraphObligationDispatchError::EmptyRuleVersion,
        )?;
        let identity_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationRuleIdentity)
                .field_value(ForgeQueryEvidenceTag::new("namespace"), namespace.as_str())
                .field_value(ForgeQueryEvidenceTag::new("name"), name.as_str())
                .field_value(
                    ForgeQueryEvidenceTag::new("semantic_version"),
                    semantic_version.as_str(),
                )
                .seal();
        Ok(Self {
            domain_invariant_family: format!("{namespace}:{}:{semantic_version}", name.as_str()),
            namespace,
            name,
            semantic_version,
            identity_digest,
        })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn semantic_version(&self) -> &str {
        &self.semantic_version
    }

    pub fn identity_digest(&self) -> &str {
        self.identity_digest.as_str()
    }

    pub fn domain_invariant_family(&self) -> &str {
        &self.domain_invariant_family
    }

    pub(crate) fn identity_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.identity_digest
    }
}

fn non_empty(
    value: String,
    error: ForgeQueryGraphObligationDispatchError,
) -> Result<String, ForgeQueryGraphObligationDispatchError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(error);
    }
    Ok(value)
}
