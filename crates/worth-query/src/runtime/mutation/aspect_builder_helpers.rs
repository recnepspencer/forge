use super::{
    WorthQueryAspectMutationBuilder, WorthQueryAuthoredAspectMutation,
    WorthQueryContinuityMutationIntent, WorthQueryNamingMutationIntent, WorthQueryRuntimeError,
    WorthQuerySymbolicAspectReference,
};
use crate::memory_workspace::WorthQueryWorkspaceError;
use crate::runtime::WorthQueryMutationAuthorityIdentity;

impl WorthQueryAspectMutationBuilder {
    pub fn naming_attach_new_target(
        self,
        attachment_identity: WorthQueryMutationAuthorityIdentity,
    ) -> Self {
        self.naming_intent(WorthQueryNamingMutationIntent::attach_new_target(
            attachment_identity,
        ))
    }

    pub fn naming_attach_existing_target(
        self,
        attachment_identity: WorthQueryMutationAuthorityIdentity,
        target_authoritative_identity: WorthQueryMutationAuthorityIdentity,
    ) -> Self {
        self.naming_intent(WorthQueryNamingMutationIntent::attach_existing_target(
            attachment_identity,
            target_authoritative_identity,
        ))
    }

    pub fn naming_rebind_target(
        self,
        attachment_identity: WorthQueryMutationAuthorityIdentity,
        prior_authoritative_identity: WorthQueryMutationAuthorityIdentity,
        target_authoritative_identity: WorthQueryMutationAuthorityIdentity,
    ) -> Self {
        self.naming_intent(WorthQueryNamingMutationIntent::rebind_target(
            attachment_identity,
            prior_authoritative_identity,
            target_authoritative_identity,
        ))
    }

    pub fn naming_remove_target(
        self,
        attachment_identity: WorthQueryMutationAuthorityIdentity,
        prior_authoritative_identity: WorthQueryMutationAuthorityIdentity,
    ) -> Self {
        self.naming_intent(WorthQueryNamingMutationIntent::remove(
            attachment_identity,
            prior_authoritative_identity,
        ))
    }

    pub fn continuity_rebind_existing_target(
        self,
        prior_authoritative_identity: WorthQueryMutationAuthorityIdentity,
        successor_authoritative_identity: WorthQueryMutationAuthorityIdentity,
    ) -> Self {
        match WorthQueryContinuityMutationIntent::rebind_existing_target(
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
        prior_authoritative_identity: WorthQueryMutationAuthorityIdentity,
        successor_authoritative_identity: WorthQueryMutationAuthorityIdentity,
    ) -> Self {
        match WorthQueryContinuityMutationIntent::rebind_merge_successor(
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

    pub fn continuity_split_successors<I>(
        self,
        prior_authoritative_identity: WorthQueryMutationAuthorityIdentity,
        successor_authoritative_identities: I,
    ) -> Self
    where
        I: IntoIterator<Item = WorthQueryMutationAuthorityIdentity>,
    {
        match WorthQueryContinuityMutationIntent::split_existing_target(
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
    aspects: Vec<WorthQueryAuthoredAspectMutation>,
    error: Option<String>,
) -> Result<Vec<WorthQueryAuthoredAspectMutation>, WorthQueryRuntimeError> {
    if let Some(error) = error {
        return Err(WorthQueryRuntimeError::Workspace(
            WorthQueryWorkspaceError::new(error),
        ));
    }
    if aspects.is_empty() {
        return Err(WorthQueryRuntimeError::Workspace(
            WorthQueryWorkspaceError::new("aspect mutation must declare at least one aspect"),
        ));
    }
    Ok(aspects)
}

pub(super) fn reject_symbolic_aspect_references(
    symbolic_aspect_references: &[WorthQuerySymbolicAspectReference],
    lane_description: &str,
) -> Result<(), WorthQueryRuntimeError> {
    if symbolic_aspect_references.is_empty() {
        return Ok(());
    }
    Err(WorthQueryRuntimeError::Workspace(
        WorthQueryWorkspaceError::new(format!(
            "{lane_description} does not admit symbolic aspect references yet"
        )),
    ))
}
