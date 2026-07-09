use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::error::WorthQueryGraphObligationDispatchError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationRuleIdentity {
    namespace: String,
    name: String,
    semantic_version: String,
    domain_invariant_family: String,
    identity_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphObligationRuleIdentity {
    pub fn new(
        namespace: impl Into<String>,
        name: impl Into<String>,
        semantic_version: impl Into<String>,
    ) -> Result<Self, WorthQueryGraphObligationDispatchError> {
        let namespace = non_empty(
            namespace.into(),
            WorthQueryGraphObligationDispatchError::EmptyRuleNamespace,
        )?;
        let name = non_empty(
            name.into(),
            WorthQueryGraphObligationDispatchError::EmptyRuleName,
        )?;
        let semantic_version = non_empty(
            semantic_version.into(),
            WorthQueryGraphObligationDispatchError::EmptyRuleVersion,
        )?;
        let identity_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationRuleIdentity)
                .field_value(WorthQueryEvidenceTag::new("namespace"), namespace.as_str())
                .field_value(WorthQueryEvidenceTag::new("name"), name.as_str())
                .field_value(
                    WorthQueryEvidenceTag::new("semantic_version"),
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

    pub(crate) fn identity_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity_digest
    }
}

fn non_empty(
    value: String,
    error: WorthQueryGraphObligationDispatchError,
) -> Result<String, WorthQueryGraphObligationDispatchError> {
    let value = value.trim().to_string();
    if value.is_empty() {
        return Err(error);
    }
    Ok(value)
}
