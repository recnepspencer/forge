#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialCanonicalDeclarationField {
    locus: String,
    value: String,
}

impl SpatialCanonicalDeclarationField {
    pub fn new(locus: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            locus: locus.into(),
            value: value.into(),
        }
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
