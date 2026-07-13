use crate::identity::SchemaBasisDigest;

/// Query-owned authority for a schema basis.
///
/// The contained digest is a read-only projection. Only Query artifacts can
/// construct this handle, so a digest copied from diagnostics cannot be
/// promoted back into operational authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySchemaBasisAuthority {
    digest: SchemaBasisDigest,
}

impl QuerySchemaBasisAuthority {
    pub fn digest(&self) -> &SchemaBasisDigest {
        &self.digest
    }

    pub(crate) fn from_query_artifact(digest: &SchemaBasisDigest) -> Self {
        Self {
            digest: digest.clone(),
        }
    }

    pub(crate) fn into_digest(self) -> SchemaBasisDigest {
        self.digest
    }
}

/// Non-authoritative schema identity supplied by a boundary consumer.
///
/// This token can describe an external schema but cannot satisfy any
/// operational API that requires Query-owned schema authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryExternalSchemaBasisToken {
    domain_parts: Vec<String>,
}

impl QueryExternalSchemaBasisToken {
    pub fn from_domain_parts(parts: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            domain_parts: parts.into_iter().map(Into::into).collect(),
        }
    }

    pub fn domain_parts(&self) -> impl ExactSizeIterator<Item = &str> {
        self.domain_parts.iter().map(String::as_str)
    }

    pub(crate) fn admit(self) -> QuerySchemaBasisAuthority {
        QuerySchemaBasisAuthority::from_query_artifact(&SchemaBasisDigest::from_parts(
            &self.domain_parts,
        ))
    }
}
