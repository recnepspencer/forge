use std::collections::BTreeSet;

use forge_foundational::facade::AspectValue;

use super::ForgeQueryMutationMetadata;
use super::{
    ForgeQueryDesiredAspectValue, ForgeQueryParsedAspectTarget, ForgeQueryParsedDesiredAspect,
};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::{ForgeQueryEntityIdentity, ForgeQueryWorkspaceError};
use crate::runtime::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectMutationOperationKind,
    ForgeQueryAspectTouch, ForgeQueryContinuityMutationIntent, ForgeQueryNamingMutationIntent,
    ForgeQueryRuntimeError, ForgeQuerySymbolicAspectReference, ForgeQuerySymbolicTargetReference,
    ForgeQueryWriteCommand,
};

#[path = "aspect_builder_helpers.rs"]
mod aspect_builder_helpers;
#[path = "aspect_existing_truth.rs"]
mod aspect_existing_truth;

use aspect_builder_helpers::{finish_aspects, reject_symbolic_aspect_references};

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAspectValue {
    parsed: ForgeQueryParsedDesiredAspect,
}

impl ForgeQueryAspectValue {
    #[cfg(test)]
    pub(crate) fn new(
        aspect_touch: ForgeQueryAspectTouch,
        value: AspectValue,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new_set(aspect_touch, value)
    }

    pub(crate) fn new_set(
        aspect_touch: ForgeQueryAspectTouch,
        value: AspectValue,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::from_touch_parts(
            aspect_touch,
            ForgeQueryDesiredAspectValue::set_native(value),
        ))
    }

    pub(crate) fn new_set_evidence_identity(
        aspect_touch: ForgeQueryAspectTouch,
        identity: &ForgeQueryEvidenceIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self::from_touch_parts(
            aspect_touch,
            ForgeQueryDesiredAspectValue::set_native(AspectValue::String(identity.as_str().into())),
        ))
    }

    pub(crate) fn new_clear(
        aspect_touch: ForgeQueryAspectTouch,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            parsed: ForgeQueryParsedDesiredAspect::new(
                aspect_touch.into_parsed_target(),
                ForgeQueryDesiredAspectValue::clear(),
            ),
        })
    }

    pub fn aspect_touch(&self) -> ForgeQueryAspectTouch {
        ForgeQueryAspectTouch::from_parsed_target(self.parsed_target().clone())
    }

    pub(crate) fn parsed_target(&self) -> &ForgeQueryParsedAspectTarget {
        self.parsed.target()
    }

    pub fn foundational_value(&self) -> Option<&AspectValue> {
        self.parsed.desired().value()
    }

    pub(crate) fn native_digest_material(&self) -> String {
        format!(
            "{}={}",
            ForgeQueryAspectTouch::from_parsed_target(self.parsed_target().clone())
                .admitted_touch_digest_part(),
            self.parsed.desired().native_digest_material()
        )
    }

    pub fn clears_existing_value(&self) -> bool {
        self.parsed.desired().clears_existing_value()
    }

    pub fn declared_operation(&self) -> ForgeQueryAspectMutationOperation {
        ForgeQueryAspectMutationOperation::from_touch(
            ForgeQueryAspectTouch::from_parsed_target(self.parsed_target().clone()),
            if self.clears_existing_value() {
                ForgeQueryAspectMutationOperationKind::Clear
            } else {
                ForgeQueryAspectMutationOperationKind::Set
            },
        )
    }

    fn from_touch_parts(
        aspect_touch: ForgeQueryAspectTouch,
        desired: ForgeQueryDesiredAspectValue,
    ) -> Self {
        Self {
            parsed: ForgeQueryParsedDesiredAspect::new(aspect_touch.into_parsed_target(), desired),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryAspectMutationBuilder {
    aspects: Vec<ForgeQueryAspectValue>,
    symbolic_aspect_references: Vec<ForgeQuerySymbolicAspectReference>,
    seen_aspects: BTreeSet<ForgeQueryAspectTouch>,
    metadata: ForgeQueryMutationMetadata,
    naming_intent: Option<ForgeQueryNamingMutationIntent>,
    continuity_intent: Option<ForgeQueryContinuityMutationIntent>,
    error: Option<String>,
}

impl ForgeQueryAspectMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aspect(mut self, aspect_touch: ForgeQueryAspectTouch, value: AspectValue) -> Self {
        if self.error.is_some() {
            return self;
        }
        match ForgeQueryAspectValue::new_set(aspect_touch, value) {
            Ok(aspect) => {
                let aspect_touch =
                    ForgeQueryAspectTouch::from_parsed_target(aspect.parsed_target().clone());
                if !self.seen_aspects.insert(aspect_touch) {
                    self.error = Some(format!(
                        "aspect `{}` may only be declared once per mutation",
                        aspect.aspect_touch().admitted_touch_digest_part()
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
        aspect_touch: ForgeQueryAspectTouch,
        reference: ForgeQuerySymbolicTargetReference,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        if !self.seen_aspects.insert(aspect_touch.clone()) {
            let aspect_touch_digest = aspect_touch.admitted_touch_digest_part();
            self.error = Some(format!(
                "aspect `{aspect_touch_digest}` may only be declared once per mutation"
            ));
            return self;
        }
        self.symbolic_aspect_references.push(
            ForgeQuerySymbolicAspectReference::same_batch_entity_identity(aspect_touch, reference),
        );
        self
    }

    pub fn clear(mut self, aspect_touch: ForgeQueryAspectTouch) -> Self {
        if self.error.is_some() {
            return self;
        }
        match ForgeQueryAspectValue::new_clear(aspect_touch) {
            Ok(aspect) => {
                let aspect_touch =
                    ForgeQueryAspectTouch::from_parsed_target(aspect.parsed_target().clone());
                if !self.seen_aspects.insert(aspect_touch) {
                    self.error = Some(format!(
                        "aspect `{}` may only be declared once per mutation",
                        aspect.aspect_touch().admitted_touch_digest_part()
                    ));
                } else {
                    self.aspects.push(aspect);
                }
            }
            Err(error) => self.error = Some(error.to_string()),
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
        entity_identity: ForgeQueryEntityIdentity,
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
