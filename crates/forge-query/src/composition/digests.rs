use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CompositionDigest(String);

impl CompositionDigest {
    pub fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ScopeLineageDigest(String);

impl ScopeLineageDigest {
    pub fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TemplateBindingDigest(String);

impl TemplateBindingDigest {
    pub fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
