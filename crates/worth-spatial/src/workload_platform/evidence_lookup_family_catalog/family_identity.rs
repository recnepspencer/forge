use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EvidenceLookupFamilyIdentity {
    value: String,
    digest: String,
}

impl EvidenceLookupFamilyIdentity {
    pub(crate) fn declared(value: impl Into<String>) -> Self {
        let value = value.into();
        let digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-family-identity:v1".to_string(),
                value.clone(),
            ],
        );
        Self { value, digest }
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}
