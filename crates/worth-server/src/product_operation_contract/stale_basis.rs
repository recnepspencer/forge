#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductStaleBasisDenial {
    expected_base_digest: String,
    observed_base_digest: String,
    canonical_digest: String,
}

impl WorthServerProductStaleBasisDenial {
    pub(crate) fn new(
        expected_base_digest: impl Into<String>,
        observed_base_digest: impl Into<String>,
    ) -> Self {
        let expected_base_digest = expected_base_digest.into();
        let observed_base_digest = observed_base_digest.into();
        let canonical_digest = format!(
            "worth-server-product-stale-basis-denial-v1|expected={expected_base_digest}|observed={observed_base_digest}"
        );
        Self {
            expected_base_digest,
            observed_base_digest,
            canonical_digest,
        }
    }

    pub fn expected_base_digest(&self) -> &str {
        &self.expected_base_digest
    }

    pub fn observed_base_digest(&self) -> &str {
        &self.observed_base_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductRebaseRequired {
    denial: WorthServerProductStaleBasisDenial,
}

impl WorthServerProductRebaseRequired {
    pub fn new(denial: WorthServerProductStaleBasisDenial) -> Self {
        Self { denial }
    }

    pub fn denial(&self) -> &WorthServerProductStaleBasisDenial {
        &self.denial
    }

    pub fn expected_base_digest(&self) -> &str {
        self.denial.expected_base_digest()
    }

    pub fn observed_base_digest(&self) -> &str {
        self.denial.observed_base_digest()
    }
}
