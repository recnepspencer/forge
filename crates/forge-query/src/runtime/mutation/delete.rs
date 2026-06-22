use std::collections::BTreeSet;

use super::{ForgeQueryAspectTouch, ForgeQueryMutationMetadata};
use crate::memory_workspace::{ForgeQueryEntityIdentity, ForgeQueryWorkspaceError};
use crate::runtime::{
    ForgeQueryAspectValue, ForgeQueryExistingTruthTargetBinding, ForgeQueryNamingMutationIntent,
    ForgeQueryRuntimeError, ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryDeleteMutationBuilder {
    declared_collection: Option<String>,
    touched_aspects: Vec<ForgeQueryAspectTouch>,
    seen_aspects: BTreeSet<ForgeQueryAspectTouch>,
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

    pub fn touch(mut self, touch: ForgeQueryAspectTouch) -> Self {
        if self.error.is_some() {
            return self;
        }
        if !self.seen_aspects.insert(touch.clone()) {
            let aspect_touch_digest = touch.admitted_touch_digest_part();
            self.error = Some(format!(
                "delete touch aspect `{aspect_touch_digest}` may only be declared once per mutation"
            ));
            return self;
        }
        self.touched_aspects.push(touch);
        self
    }

    pub fn touches(mut self, touches: impl IntoIterator<Item = ForgeQueryAspectTouch>) -> Self {
        for touch in touches {
            self = self.touch(touch);
            if self.error.is_some() {
                break;
            }
        }
        self
    }

    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
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
            touched_aspects: self.touched_aspects,
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
            touched_aspects: self.touched_aspects,
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
            touched_aspects: self.touched_aspects,
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
            touched_aspects: self.touched_aspects,
            metadata: self.metadata,
            naming_intent: self.naming_intent,
        })
    }
}
