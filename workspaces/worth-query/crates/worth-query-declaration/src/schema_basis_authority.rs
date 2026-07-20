use crate::identity::SchemaBasisDigest;

/// Typed carrier for Query schema-basis identity.
///
/// Schema-view construction is the only minting path. The compatibility edge
/// may carry this proof downstream, but a digest or reporting artifact cannot
/// reconstruct it. This type alone grants no installation or execution
/// capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySchemaBasisAuthority {
    digest: SchemaBasisDigest,
}

impl QuerySchemaBasisAuthority {
    pub fn digest(&self) -> &SchemaBasisDigest {
        &self.digest
    }

    pub(crate) fn mint(digest: SchemaBasisDigest) -> Self {
        Self { digest }
    }

    pub fn into_digest(self) -> SchemaBasisDigest {
        self.digest
    }
}
