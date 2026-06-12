use std::collections::BTreeSet;

use serde::Serialize;

use super::ForgeQueryMutationMetadata;
use crate::memory_workspace::{ForgeQueryEntityIdentity, ForgeQueryWorkspaceError};
use crate::runtime::{
    ForgeQueryAspectValue, ForgeQueryExistingTruthTargetBinding, ForgeQueryNamingMutationIntent,
    ForgeQueryRuntimeError, ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryDeleteMutationBuilder {
    declared_collection: Option<String>,
    touched_aspect_paths: Vec<String>,
    seen_aspects: BTreeSet<String>,
    metadata: ForgeQueryMutationMetadata,
    naming_intent: Option<ForgeQueryNamingMutationIntent>,
    error: Option<String>,
}

impl ForgeQueryDeleteMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn target_collection(mut self, collection: impl Into<String>) -> Self {
        if self.error.is_some() {
            return self;
        }
        let collection = collection.into();
        if collection.trim().is_empty() {
            self.error = Some("delete target collection may not be empty".to_string());
            return self;
        }
        self.declared_collection = Some(collection);
        self
    }

    pub fn touch(mut self, aspect_path: impl Into<String>) -> Self {
        if self.error.is_some() {
            return self;
        }
        let aspect_path = aspect_path.into();
        if aspect_path.trim().is_empty() {
            self.error = Some("delete touch aspect path may not be empty".to_string());
            return self;
        }
        if !self.seen_aspects.insert(aspect_path.clone()) {
            self.error = Some(format!(
                "delete touch aspect `{aspect_path}` may only be declared once per mutation"
            ));
            return self;
        }
        self.touched_aspect_paths.push(aspect_path);
        self
    }

    pub fn touches(mut self, aspect_paths: impl IntoIterator<Item = impl Into<String>>) -> Self {
        for aspect_path in aspect_paths {
            self = self.touch(aspect_path);
            if self.error.is_some() {
                break;
            }
        }
        self
    }

    pub fn metadata<T: Serialize>(mut self, key: impl Into<String>, value: T) -> Self {
        if self.error.is_some() {
            return self;
        }
        if let Err(error) = self.metadata.insert(key, value) {
            self.error = Some(error.to_string());
        }
        self
    }

    pub fn naming_intent(mut self, intent: ForgeQueryNamingMutationIntent) -> Self {
        if self.error.is_some() {
            return self;
        }
        if self.naming_intent.is_some() {
            self.error = Some("naming intent may only be declared once per mutation".to_string());
            return self;
        }
        self.naming_intent = Some(intent);
        self
    }

    pub fn naming_remove(
        self,
        attachment_identity: crate::runtime::ForgeQueryMutationAuthorityIdentity,
        prior_authoritative_identity: crate::runtime::ForgeQueryMutationAuthorityIdentity,
    ) -> Self {
        self.naming_intent(ForgeQueryNamingMutationIntent::remove(
            attachment_identity,
            prior_authoritative_identity,
        ))
    }

    pub fn build_delete(
        self,
        entity_identity: ForgeQueryEntityIdentity,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(error),
            ));
        }
        Ok(ForgeQueryWriteCommand::DeleteAspects {
            entity_identity,
            declared_collection: self.declared_collection,
            touched_aspect_paths: self.touched_aspect_paths,
            metadata: self.metadata,
            naming_intent: self.naming_intent,
        })
    }

    pub fn build_delete_existing(
        self,
        binding: ForgeQueryExistingTruthTargetBinding,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(error),
            ));
        }
        Ok(ForgeQueryWriteCommand::DeleteExistingAspects {
            binding,
            touched_aspect_paths: self.touched_aspect_paths,
            metadata: self.metadata,
            naming_intent: self.naming_intent,
        })
    }

    pub(crate) fn build_delete_existing_verified(
        self,
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(error),
            ));
        }
        Ok(ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            binding,
            asserted_aspects,
            touched_aspect_paths: self.touched_aspect_paths,
            metadata: self.metadata,
            naming_intent: self.naming_intent,
        })
    }

    pub fn build_delete_symbolic(
        self,
        reference: ForgeQuerySymbolicTargetReference,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(error),
            ));
        }
        Ok(ForgeQueryWriteCommand::DeleteSymbolicAspects {
            reference,
            touched_aspect_paths: self.touched_aspect_paths,
            metadata: self.metadata,
            naming_intent: self.naming_intent,
        })
    }
}
