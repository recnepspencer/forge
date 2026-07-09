use std::collections::BTreeSet;

use worth_foundational::facade::AspectValue;

use super::WorthQueryMutationMetadata;
use super::{
    WorthQueryDesiredAspectValue, WorthQueryParsedAspectTarget, WorthQueryParsedDesiredAspect,
};
use crate::evidence_identity::WorthQueryEvidenceIdentity;
use crate::memory_workspace::{WorthQueryEntityIdentity, WorthQueryWorkspaceError};
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAspectMutationOperationKind,
    WorthQueryAspectTouch, WorthQueryContinuityMutationIntent,
    WorthQueryMutationTargetCollectionIdentity, WorthQueryNamingMutationIntent,
    WorthQueryRuntimeError, WorthQuerySymbolicAspectReference, WorthQuerySymbolicTargetReference,
    WorthQueryWriteCommand,
};

#[path = "aspect_builder_helpers.rs"]
mod aspect_builder_helpers;
#[path = "aspect_existing_truth.rs"]
mod aspect_existing_truth;

use aspect_builder_helpers::{finish_aspects, reject_symbolic_aspect_references};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAdmittedAspectValue {
    parsed: WorthQueryParsedDesiredAspect,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryAuthoredAspectValue {
    value: AspectValue,
}

impl WorthQueryAuthoredAspectValue {
    pub fn string(value: impl Into<String>) -> Self {
        Self {
            value: AspectValue::String(value.into().into()),
        }
    }

    pub fn int64(value: i64) -> Self {
        Self {
            value: AspectValue::Int64(value),
        }
    }

    pub fn bool(value: bool) -> Self {
        Self {
            value: AspectValue::Bool(value),
        }
    }

    pub fn null() -> Self {
        Self {
            value: AspectValue::Null,
        }
    }

    #[cfg(test)]
    pub(crate) fn from_foundational_value(value: AspectValue) -> Self {
        Self { value }
    }

    pub(crate) fn into_foundational_value(self) -> AspectValue {
        self.value
    }
}

impl From<String> for WorthQueryAuthoredAspectValue {
    fn from(value: String) -> Self {
        Self::string(value)
    }
}

impl From<&str> for WorthQueryAuthoredAspectValue {
    fn from(value: &str) -> Self {
        Self::string(value)
    }
}

impl From<bool> for WorthQueryAuthoredAspectValue {
    fn from(value: bool) -> Self {
        Self::bool(value)
    }
}

impl From<i64> for WorthQueryAuthoredAspectValue {
    fn from(value: i64) -> Self {
        Self::int64(value)
    }
}

impl WorthQueryAdmittedAspectValue {
    pub(crate) fn native_string_value(value: impl Into<String>) -> AspectValue {
        AspectValue::String(value.into().into())
    }

    #[cfg(test)]
    pub(crate) fn new(
        aspect_touch: WorthQueryAspectTouch,
        value: AspectValue,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Self::new_set(aspect_touch, value)
    }

    pub(crate) fn new_set(
        aspect_touch: WorthQueryAspectTouch,
        value: AspectValue,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Ok(Self::from_touch_parts(
            aspect_touch,
            WorthQueryDesiredAspectValue::set_native(value),
        ))
    }

    pub(crate) fn new_set_evidence_identity(
        aspect_touch: WorthQueryAspectTouch,
        identity: &WorthQueryEvidenceIdentity,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Ok(Self::from_touch_parts(
            aspect_touch,
            WorthQueryDesiredAspectValue::set_native(Self::native_string_value(identity.as_str())),
        ))
    }

    pub(crate) fn new_clear(
        aspect_touch: WorthQueryAspectTouch,
    ) -> Result<Self, WorthQueryWorkspaceError> {
        Ok(Self {
            parsed: WorthQueryParsedDesiredAspect::new(
                aspect_touch.into_parsed_target(),
                WorthQueryDesiredAspectValue::clear(),
            ),
        })
    }

    pub fn aspect_touch(&self) -> WorthQueryAspectTouch {
        WorthQueryAspectTouch::from_parsed_target(self.parsed_target().clone())
    }

    pub(crate) fn parsed_target(&self) -> &WorthQueryParsedAspectTarget {
        self.parsed.target()
    }

    pub fn foundational_value(&self) -> Option<&AspectValue> {
        self.parsed.desired().value()
    }

    pub(crate) fn terminal_digest_material(&self) -> String {
        format!(
            "{}={}",
            WorthQueryAspectTouch::from_parsed_target(self.parsed_target().clone())
                .admitted_touch_digest_part(),
            self.parsed.desired().terminal_digest_material()
        )
    }

    pub fn clears_existing_value(&self) -> bool {
        self.parsed.desired().clears_existing_value()
    }

    pub fn declared_operation(&self) -> WorthQueryAspectMutationOperation {
        WorthQueryAspectMutationOperation::from_touch(
            WorthQueryAspectTouch::from_parsed_target(self.parsed_target().clone()),
            if self.clears_existing_value() {
                WorthQueryAspectMutationOperationKind::Clear
            } else {
                WorthQueryAspectMutationOperationKind::Set
            },
        )
    }

    fn from_touch_parts(
        aspect_touch: WorthQueryAspectTouch,
        desired: WorthQueryDesiredAspectValue,
    ) -> Self {
        Self {
            parsed: WorthQueryParsedDesiredAspect::new(aspect_touch.into_parsed_target(), desired),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorthQueryAspectMutationBuilder {
    aspects: Vec<WorthQueryAdmittedAspectValue>,
    symbolic_aspect_references: Vec<WorthQuerySymbolicAspectReference>,
    seen_aspects: BTreeSet<WorthQueryAspectTouch>,
    metadata: WorthQueryMutationMetadata,
    naming_intent: Option<WorthQueryNamingMutationIntent>,
    continuity_intent: Option<WorthQueryContinuityMutationIntent>,
    error: Option<String>,
}

impl WorthQueryAspectMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aspect(
        mut self,
        authored_touch_text: impl Into<String>,
        value: impl Into<WorthQueryAuthoredAspectValue>,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match WorthQueryAspectTouch::from_authoring_ingress_text(authored_touch_text) {
            Ok(touch) => self.set_aspect(touch, value.into()),
            Err(error) => {
                self.error = Some(error.to_string());
                self
            }
        }
    }

    pub fn set_aspect(
        mut self,
        aspect_touch: WorthQueryAspectTouch,
        value: WorthQueryAuthoredAspectValue,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match WorthQueryAdmittedAspectValue::new_set(aspect_touch, value.into_foundational_value())
        {
            Ok(aspect) => {
                let aspect_touch =
                    WorthQueryAspectTouch::from_parsed_target(aspect.parsed_target().clone());
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
        aspect_touch: WorthQueryAspectTouch,
        reference: WorthQuerySymbolicTargetReference,
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
            WorthQuerySymbolicAspectReference::same_batch_entity_identity(aspect_touch, reference),
        );
        self
    }

    pub fn clear(mut self, aspect_touch: WorthQueryAspectTouch) -> Self {
        if self.error.is_some() {
            return self;
        }
        match WorthQueryAdmittedAspectValue::new_clear(aspect_touch) {
            Ok(aspect) => {
                let aspect_touch =
                    WorthQueryAspectTouch::from_parsed_target(aspect.parsed_target().clone());
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

    pub fn naming_intent(mut self, intent: WorthQueryNamingMutationIntent) -> Self {
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

    pub fn continuity_intent(mut self, intent: WorthQueryContinuityMutationIntent) -> Self {
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
    ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError> {
        self.build_insert_internal(collection, None)
    }

    pub(crate) fn build_insert_symbolic_reference(
        self,
        reference: WorthQuerySymbolicTargetReference,
        collection: impl Into<String>,
    ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError> {
        self.build_insert_internal(collection, Some(reference))
    }

    fn build_insert_internal(
        self,
        collection: impl Into<String>,
        symbolic_target_reference: Option<WorthQuerySymbolicTargetReference>,
    ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError> {
        let WorthQueryAspectMutationBuilder {
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
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new("collection may not be empty"),
            ));
        }
        let collection =
            WorthQueryMutationTargetCollectionIdentity::new("write-command-declared", collection);
        if let Some(reference_collection) = symbolic_target_reference
            .as_ref()
            .and_then(WorthQuerySymbolicTargetReference::target_collection_identity)
        {
            if !collection.same_target_collection_as(reference_collection) {
                return Err(WorthQueryRuntimeError::Workspace(
                    WorthQueryWorkspaceError::new(format!(
                        "symbolic target collection `{}` does not match insert collection `{}`",
                        reference_collection.as_str(),
                        collection.as_str()
                    )),
                ));
            }
        }
        Ok(WorthQueryWriteCommand::InsertAspects {
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
        entity_identity: WorthQueryEntityIdentity,
    ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError> {
        let WorthQueryAspectMutationBuilder {
            aspects,
            symbolic_aspect_references,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        reject_symbolic_aspect_references(&symbolic_aspect_references, "update-family authoring")?;
        Ok(WorthQueryWriteCommand::UpdateAspects {
            entity_identity,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }

    pub fn build_update_existing(
        self,
        binding: crate::runtime::WorthQueryExistingTruthTargetBinding,
    ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError> {
        let WorthQueryAspectMutationBuilder {
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
        Ok(WorthQueryWriteCommand::UpdateExistingAspects {
            binding,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }
    pub fn build_update_symbolic(
        self,
        reference: WorthQuerySymbolicTargetReference,
    ) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError> {
        let WorthQueryAspectMutationBuilder {
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
        Ok(WorthQueryWriteCommand::UpdateSymbolicAspects {
            reference,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }
}
