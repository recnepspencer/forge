use super::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectValue, ForgeQueryContinuityMutationIntent,
    ForgeQueryNamingMutationIntent, ForgeQueryRuntimeError, ForgeQuerySymbolicAspectReference,
};
use crate::memory_workspace::ForgeQueryWorkspaceError;

impl ForgeQueryAspectMutationBuilder {
    pub fn naming_attach_new_target(self, attachment_identity: impl Into<String>) -> Self {
        match ForgeQueryNamingMutationIntent::attach_new_target(attachment_identity) {
            Ok(intent) => self.naming_intent(intent),
            Err(error) => Self {
                error: Some(error.to_string()),
                ..self
            },
        }
    }

    pub fn naming_attach_existing_target(
        self,
        attachment_identity: impl Into<String>,
        target_authoritative_identity: impl Into<String>,
    ) -> Self {
        match ForgeQueryNamingMutationIntent::attach_existing_target(
            attachment_identity,
            target_authoritative_identity,
        ) {
            Ok(intent) => self.naming_intent(intent),
            Err(error) => Self {
                error: Some(error.to_string()),
                ..self
            },
        }
    }

    pub fn naming_rebind_target(
        self,
        attachment_identity: impl Into<String>,
        prior_authoritative_identity: impl Into<String>,
        target_authoritative_identity: impl Into<String>,
    ) -> Self {
        match ForgeQueryNamingMutationIntent::rebind_target(
            attachment_identity,
            prior_authoritative_identity,
            target_authoritative_identity,
        ) {
            Ok(intent) => self.naming_intent(intent),
            Err(error) => Self {
                error: Some(error.to_string()),
                ..self
            },
        }
    }

    pub fn naming_remove_target(
        self,
        attachment_identity: impl Into<String>,
        prior_authoritative_identity: impl Into<String>,
    ) -> Self {
        match ForgeQueryNamingMutationIntent::remove(
            attachment_identity,
            prior_authoritative_identity,
        ) {
            Ok(intent) => self.naming_intent(intent),
            Err(error) => Self {
                error: Some(error.to_string()),
                ..self
            },
        }
    }

    pub fn continuity_rebind_existing_target(
        self,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identity: impl Into<String>,
    ) -> Self {
        match ForgeQueryContinuityMutationIntent::rebind_existing_target(
            prior_authoritative_identity,
            successor_authoritative_identity,
        ) {
            Ok(intent) => self.continuity_intent(intent),
            Err(error) => Self {
                error: Some(error.to_string()),
                ..self
            },
        }
    }

    pub fn continuity_rebind_merge_successor(
        self,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identity: impl Into<String>,
    ) -> Self {
        match ForgeQueryContinuityMutationIntent::rebind_merge_successor(
            prior_authoritative_identity,
            successor_authoritative_identity,
        ) {
            Ok(intent) => self.continuity_intent(intent),
            Err(error) => Self {
                error: Some(error.to_string()),
                ..self
            },
        }
    }

    pub fn continuity_split_successors<I, S>(
        self,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identities: I,
    ) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        match ForgeQueryContinuityMutationIntent::split_existing_target(
            prior_authoritative_identity,
            successor_authoritative_identities,
        ) {
            Ok(intent) => self.continuity_intent(intent),
            Err(error) => Self {
                error: Some(error.to_string()),
                ..self
            },
        }
    }
}

pub(super) fn finish_aspects(
    aspects: Vec<ForgeQueryAspectValue>,
    error: Option<String>,
) -> Result<Vec<ForgeQueryAspectValue>, ForgeQueryRuntimeError> {
    if let Some(error) = error {
        return Err(ForgeQueryRuntimeError::Workspace(
            ForgeQueryWorkspaceError::new(error),
        ));
    }
    if aspects.is_empty() {
        return Err(ForgeQueryRuntimeError::Workspace(
            ForgeQueryWorkspaceError::new("aspect mutation must declare at least one aspect"),
        ));
    }
    Ok(aspects)
}

pub(super) fn reject_symbolic_aspect_references(
    symbolic_aspect_references: &[ForgeQuerySymbolicAspectReference],
    lane_description: &str,
) -> Result<(), ForgeQueryRuntimeError> {
    if symbolic_aspect_references.is_empty() {
        return Ok(());
    }
    Err(ForgeQueryRuntimeError::Workspace(
        ForgeQueryWorkspaceError::new(format!(
            "{lane_description} does not admit symbolic aspect references yet"
        )),
    ))
}
