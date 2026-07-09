use super::{WorthQueryAspectMutationBuilder, WorthQueryDeleteMutationBuilder};
use crate::memory_workspace::{WorthQueryEntityIdentity, WorthQueryWorkspaceError};
use crate::runtime::{
    WorthQueryExistingTruthTargetBinding, WorthQueryMutationMetadata, WorthQueryRuntimeError,
    WorthQuerySymbolicTargetReference, WorthQueryWriteCommand,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct WorthQueryMutationBatchBuilder {
    commands: Vec<WorthQueryWriteCommand>,
    error: Option<String>,
}

impl WorthQueryMutationBatchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryAspectMutationBuilder::new()).build_insert(collection) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn insert_symbolic(
        mut self,
        symbol: impl Into<String>,
        collection: impl Into<String>,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        let reference = match WorthQuerySymbolicTargetReference::new(symbol) {
            Ok(reference) => reference,
            Err(error) => {
                self.error = Some(error.to_string());
                return self;
            }
        };
        match declaration(WorthQueryAspectMutationBuilder::new())
            .build_insert_symbolic_reference(reference, collection)
        {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update(
        mut self,
        entity_identity: WorthQueryEntityIdentity,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryAspectMutationBuilder::new()).build_update(entity_identity) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update_existing(
        mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryAspectMutationBuilder::new()).build_update_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn assert_existing(
        mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryAspectMutationBuilder::new()).build_assert_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn verify_existing(
        mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryAspectMutationBuilder::new()).build_verify_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update_existing_verified(
        mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        verify: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
        update: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        let asserted_aspects = match verify(WorthQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth update")
        {
            Ok(aspects) => aspects,
            Err(error) => {
                self.error = Some(error.to_string());
                return self;
            }
        };
        match update(WorthQueryAspectMutationBuilder::new())
            .build_update_existing_verified(binding, asserted_aspects)
        {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update_symbolic(
        mut self,
        reference: WorthQuerySymbolicTargetReference,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryAspectMutationBuilder::new()).build_update_symbolic(reference) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete(mut self, entity_identity: WorthQueryEntityIdentity) -> Self {
        if self.error.is_some() {
            return self;
        }
        self.commands
            .push(WorthQueryWriteCommand::Delete { entity_identity });
        self
    }

    pub fn delete_with(
        mut self,
        entity_identity: WorthQueryEntityIdentity,
        declaration: impl FnOnce(WorthQueryDeleteMutationBuilder) -> WorthQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryDeleteMutationBuilder::new()).build_delete(entity_identity) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete_existing(mut self, binding: WorthQueryExistingTruthTargetBinding) -> Self {
        if self.error.is_some() {
            return self;
        }
        self.commands
            .push(WorthQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspects: Vec::new(),
                metadata: WorthQueryMutationMetadata::default(),
                naming_intent: None,
            });
        self
    }

    pub fn delete_symbolic(mut self, reference: WorthQuerySymbolicTargetReference) -> Self {
        if self.error.is_some() {
            return self;
        }
        self.commands
            .push(WorthQueryWriteCommand::DeleteSymbolicAspects {
                reference,
                touched_aspects: Vec::new(),
                metadata: WorthQueryMutationMetadata::default(),
                naming_intent: None,
            });
        self
    }

    pub fn delete_existing_with(
        mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryDeleteMutationBuilder) -> WorthQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryDeleteMutationBuilder::new()).build_delete_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete_existing_verified(
        mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        verify: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
        delete: impl FnOnce(WorthQueryDeleteMutationBuilder) -> WorthQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        let asserted_aspects = match verify(WorthQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth delete")
        {
            Ok(aspects) => aspects,
            Err(error) => {
                self.error = Some(error.to_string());
                return self;
            }
        };
        match delete(WorthQueryDeleteMutationBuilder::new())
            .build_delete_existing_verified(binding, asserted_aspects)
        {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete_symbolic_with(
        mut self,
        reference: WorthQuerySymbolicTargetReference,
        declaration: impl FnOnce(WorthQueryDeleteMutationBuilder) -> WorthQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(WorthQueryDeleteMutationBuilder::new()).build_delete_symbolic(reference) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn build(self) -> Result<Vec<WorthQueryWriteCommand>, WorthQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(error),
            ));
        }
        if self.commands.is_empty() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new("mutation batch must declare at least one operation"),
            ));
        }
        Ok(self.commands)
    }

    pub(crate) fn finish(self) -> Result<Vec<WorthQueryWriteCommand>, WorthQueryRuntimeError> {
        self.build()
    }
}
