use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LocalityScopeKind {
    Region,
    Partition,
}

impl LocalityScopeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Region => "region",
            Self::Partition => "partition",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LocalityScopeDigest(String);

impl LocalityScopeDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalityPredicateContract {
    pub(in crate::live) scope_kind: LocalityScopeKind,
    pub(in crate::live) scope: String,
    pub(in crate::live) digest: LocalityScopeDigest,
}

impl LocalityPredicateContract {
    pub fn scope_kind(&self) -> &LocalityScopeKind {
        &self.scope_kind
    }

    pub fn scope(&self) -> &str {
        &self.scope
    }

    pub fn digest(&self) -> &LocalityScopeDigest {
        &self.digest
    }

    pub fn region(scope: impl Into<String>) -> Self {
        let scope = scope.into();
        let digest =
            LocalityScopeDigest::from_parts(&["kind:region".to_string(), format!("scope:{scope}")]);
        Self {
            scope_kind: LocalityScopeKind::Region,
            scope,
            digest,
        }
    }

    pub fn partition(scope: impl Into<String>) -> Self {
        let scope = scope.into();
        let digest = LocalityScopeDigest::from_parts(&[
            "kind:partition".to_string(),
            format!("scope:{scope}"),
        ]);
        Self {
            scope_kind: LocalityScopeKind::Partition,
            scope,
            digest,
        }
    }
}
