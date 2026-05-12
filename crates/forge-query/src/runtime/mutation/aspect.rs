use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::Value;

use super::ForgeQueryMutationMetadata;
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::{
    ForgeQueryContinuityMutationIntent, ForgeQueryNamingMutationIntent, ForgeQueryRuntimeError,
    ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[path = "aspect_builder_helpers.rs"]
mod aspect_builder_helpers;
#[path = "aspect_existing_truth.rs"]
mod aspect_existing_truth;

use aspect_builder_helpers::{finish_aspects, reject_symbolic_aspect_references};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryAspectMutationOperationKind {
    Set,
    Clear,
}

impl ForgeQueryAspectMutationOperationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Set => "set",
            Self::Clear => "clear",
        }
    }
}

impl std::fmt::Display for ForgeQueryAspectMutationOperationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ForgeQueryAspectMutationOperation {
    aspect_path: String,
    kind: ForgeQueryAspectMutationOperationKind,
}

impl ForgeQueryAspectMutationOperation {
    pub(crate) fn new(
        aspect_path: impl Into<String>,
        kind: ForgeQueryAspectMutationOperationKind,
    ) -> Self {
        Self {
            aspect_path: aspect_path.into(),
            kind,
        }
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn kind(&self) -> ForgeQueryAspectMutationOperationKind {
        self.kind
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAspectValue {
    aspect_path: String,
    value: Value,
    clears_existing_value: bool,
}

impl ForgeQueryAspectValue {
    pub fn new<T: Serialize>(
        aspect_path: impl Into<String>,
        value: T,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new_set(aspect_path, value)
    }

    pub fn new_set<T: Serialize>(
        aspect_path: impl Into<String>,
        value: T,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        let aspect_path = aspect_path.into();
        if aspect_path.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "aspect path may not be empty",
            ));
        }
        let value = serde_json::to_value(value).map_err(|error| {
            ForgeQueryWorkspaceError::new(format!(
                "aspect `{aspect_path}` could not serialize into a mutation value: {error}"
            ))
        })?;
        Ok(Self {
            aspect_path,
            value,
            clears_existing_value: false,
        })
    }

    pub fn new_clear(aspect_path: impl Into<String>) -> Result<Self, ForgeQueryWorkspaceError> {
        let aspect_path = aspect_path.into();
        if aspect_path.trim().is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "aspect path may not be empty",
            ));
        }
        Ok(Self {
            aspect_path,
            value: Value::Null,
            clears_existing_value: true,
        })
    }

    pub fn aspect_path(&self) -> &str {
        &self.aspect_path
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn clears_existing_value(&self) -> bool {
        self.clears_existing_value
    }

    pub fn declared_operation(&self) -> ForgeQueryAspectMutationOperation {
        ForgeQueryAspectMutationOperation::new(
            self.aspect_path.clone(),
            if self.clears_existing_value {
                ForgeQueryAspectMutationOperationKind::Clear
            } else {
                ForgeQueryAspectMutationOperationKind::Set
            },
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryAspectMutationBuilder {
    aspects: Vec<ForgeQueryAspectValue>,
    symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
    seen_aspects: BTreeSet<String>,
    metadata: ForgeQueryMutationMetadata,
    naming_intent: Option<ForgeQueryNamingMutationIntent>,
    continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    error: Option<String>,
}

impl ForgeQueryAspectMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aspect<T: Serialize>(mut self, aspect_path: impl Into<String>, value: T) -> Self {
        if self.error.is_some() {
            return self;
        }
        match ForgeQueryAspectValue::new_set(aspect_path, value) {
            Ok(aspect) => {
                if !self.seen_aspects.insert(aspect.aspect_path.clone()) {
                    self.error = Some(format!(
                        "aspect `{}` may only be declared once per mutation",
                        aspect.aspect_path()
                    ));
                } else {
                    self.aspects.push(aspect);
                }
            }
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn symbolic_entity_identity(
        mut self,
        aspect_path: impl Into<String>,
        reference: ForgeQuerySymbolicTargetReference,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        let aspect_path = aspect_path.into();
        if !self.seen_aspects.insert(aspect_path.clone()) {
            self.error = Some(format!(
                "aspect `{aspect_path}` may only be declared once per mutation"
            ));
            return self;
        }
        match ForgeQuerySymbolicAspectReference::same_batch_entity_identity(aspect_path, reference)
        {
            Ok(reference) => self.symbolic_aspect_references.push(reference),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn clear(mut self, aspect_path: impl Into<String>) -> Self {
        if self.error.is_some() {
            return self;
        }
        match ForgeQueryAspectValue::new_clear(aspect_path) {
            Ok(aspect) => {
                if !self.seen_aspects.insert(aspect.aspect_path.clone()) {
                    self.error = Some(format!(
                        "aspect `{}` may only be declared once per mutation",
                        aspect.aspect_path()
                    ));
                } else {
                    self.aspects.push(aspect);
                }
            }
            Err(error) => self.error = Some(error.to_string()),
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

    pub fn continuity_intent(mut self, intent: ForgeQueryContinuityMutationIntent) -> Self {
        if self.error.is_some() {
            return self;
        }
        if self.continuity_intent.is_some() {
            self.error =
                Some("continuity intent may only be declared once per mutation".to_string());
            return self;
        }
        self.continuity_intent = Some(intent);
        self
    }

    pub fn build_insert(
        self,
        collection: impl Into<String>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        self.build_insert_internal(collection, None)
    }

    pub fn build_insert_symbolic(
        self,
        symbol: impl Into<String>,
        collection: impl Into<String>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        self.build_insert_internal(
            collection,
            Some(
                ForgeQuerySymbolicTargetReference::new(symbol)
                    .map_err(ForgeQueryRuntimeError::Workspace)?,
            ),
        )
    }

    fn build_insert_internal(
        self,
        collection: impl Into<String>,
        symbolic_target_reference: Option<ForgeQuerySymbolicTargetReference>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            symbolic_aspect_references,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        let collection = collection.into();
        if collection.trim().is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("collection may not be empty"),
            ));
        }
        Ok(ForgeQueryWriteCommand::InsertAspects {
            collection,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
            symbolic_target_reference,
            symbolic_aspect_references,
        })
    }

    pub fn build_update(
        self,
        entity_identity: impl Into<String>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            symbolic_aspect_references,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        reject_symbolic_aspect_references(&symbolic_aspect_references, "update-family authoring")?;
        let entity_identity = entity_identity.into();
        if entity_identity.trim().is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("entity identity may not be empty"),
            ));
        }
        Ok(ForgeQueryWriteCommand::UpdateAspects {
            entity_identity,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }

    pub fn build_update_existing(
        self,
        binding: crate::runtime::ForgeQueryExistingTruthTargetBinding,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            symbolic_aspect_references,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        reject_symbolic_aspect_references(
            &symbolic_aspect_references,
            "existing-target update-family authoring",
        )?;
        Ok(ForgeQueryWriteCommand::UpdateExistingAspects {
            binding,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }
    pub fn build_update_symbolic(
        self,
        reference: ForgeQuerySymbolicTargetReference,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            symbolic_aspect_references,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        reject_symbolic_aspect_references(
            &symbolic_aspect_references,
            "symbolic-target update-family authoring",
        )?;
        Ok(ForgeQueryWriteCommand::UpdateSymbolicAspects {
            reference,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }
}
