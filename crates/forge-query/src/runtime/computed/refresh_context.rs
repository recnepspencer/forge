use crate::runtime::mutation::ForgeQueryMutationMetadata;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryRetainedRefreshOrigin {
    Mutation,
    DeclarationInitialization,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ForgeQueryRetainedRefreshContext {
    origin: ForgeQueryRetainedRefreshOrigin,
    refresh_identity: String,
    snapshot_token: String,
    touched_aspect_paths: Vec<String>,
    refresh_metadata: ForgeQueryMutationMetadata,
}

impl ForgeQueryRetainedRefreshContext {
    pub(in crate::runtime) fn from_mutation(
        refresh_identity: impl Into<String>,
        snapshot_token: impl Into<String>,
        touched_aspect_paths: impl IntoIterator<Item = String>,
        refresh_metadata: ForgeQueryMutationMetadata,
    ) -> Self {
        let mut touched_aspect_paths = touched_aspect_paths.into_iter().collect::<Vec<_>>();
        touched_aspect_paths.sort();
        touched_aspect_paths.dedup();
        Self {
            origin: ForgeQueryRetainedRefreshOrigin::Mutation,
            refresh_identity: refresh_identity.into(),
            snapshot_token: snapshot_token.into(),
            touched_aspect_paths,
            refresh_metadata,
        }
    }

    pub(in crate::runtime) fn from_declaration_initialization(
        refresh_identity: impl Into<String>,
        snapshot_token: impl Into<String>,
        refresh_metadata: ForgeQueryMutationMetadata,
    ) -> Self {
        Self {
            origin: ForgeQueryRetainedRefreshOrigin::DeclarationInitialization,
            refresh_identity: refresh_identity.into(),
            snapshot_token: snapshot_token.into(),
            touched_aspect_paths: Vec::new(),
            refresh_metadata,
        }
    }

    pub fn origin(&self) -> ForgeQueryRetainedRefreshOrigin {
        self.origin
    }

    pub fn refresh_identity(&self) -> &str {
        &self.refresh_identity
    }

    pub fn snapshot_token(&self) -> &str {
        &self.snapshot_token
    }

    pub fn touched_aspect_paths(&self) -> &[String] {
        &self.touched_aspect_paths
    }

    pub fn refresh_metadata(&self) -> &ForgeQueryMutationMetadata {
        &self.refresh_metadata
    }

    pub fn mutation_commit_identity(&self) -> Option<&str> {
        (self.origin == ForgeQueryRetainedRefreshOrigin::Mutation)
            .then_some(self.refresh_identity.as_str())
    }
}
