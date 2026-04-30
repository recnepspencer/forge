use super::{ForgeQueryAspectMutationBuilder, ForgeQueryDeleteMutationBuilder};
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::{
    ForgeQueryExistingTruthTargetBinding, ForgeQueryMutationMetadata, ForgeQueryRuntimeError,
    ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ForgeQueryMutationBatchBuilder {
    commands: Vec<ForgeQueryWriteCommand>,
    error: Option<String>,
}

impl ForgeQueryMutationBatchBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(
        mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_insert(collection) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn insert_symbolic(
        mut self,
        symbol: impl Into<String>,
        collection: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new())
            .build_insert_symbolic(symbol, collection)
        {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update(
        mut self,
        entity_identity: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_update(entity_identity) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update_existing(
        mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_update_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn assert_existing(
        mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_assert_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn verify_existing(
        mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_verify_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update_existing_verified(
        mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        update: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        let asserted_aspects = match verify(ForgeQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth update")
        {
            Ok(aspects) => aspects,
            Err(error) => {
                self.error = Some(error.to_string());
                return self;
            }
        };
        match update(ForgeQueryAspectMutationBuilder::new())
            .build_update_existing_verified(binding, asserted_aspects)
        {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn update_symbolic(
        mut self,
        reference: ForgeQuerySymbolicTargetReference,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryAspectMutationBuilder::new()).build_update_symbolic(reference) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete(mut self, entity_identity: impl Into<String>) -> Self {
        if self.error.is_some() {
            return self;
        }
        let entity_identity = entity_identity.into();
        if entity_identity.trim().is_empty() {
            self.error = Some("entity identity may not be empty".to_string());
            return self;
        }
        self.commands
            .push(ForgeQueryWriteCommand::Delete { entity_identity });
        self
    }

    pub fn delete_with(
        mut self,
        entity_identity: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete(entity_identity) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete_existing(mut self, binding: ForgeQueryExistingTruthTargetBinding) -> Self {
        if self.error.is_some() {
            return self;
        }
        self.commands
            .push(ForgeQueryWriteCommand::DeleteExistingAspects {
                binding,
                touched_aspect_paths: Vec::new(),
                metadata: ForgeQueryMutationMetadata::default(),
                naming_intent: None,
            });
        self
    }

    pub fn delete_symbolic(mut self, reference: ForgeQuerySymbolicTargetReference) -> Self {
        if self.error.is_some() {
            return self;
        }
        self.commands
            .push(ForgeQueryWriteCommand::DeleteSymbolicAspects {
                reference,
                touched_aspect_paths: Vec::new(),
                metadata: ForgeQueryMutationMetadata::default(),
                naming_intent: None,
            });
        self
    }

    pub fn delete_existing_with(
        mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete_existing(binding) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete_existing_verified(
        mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        delete: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        let asserted_aspects = match verify(ForgeQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth delete")
        {
            Ok(aspects) => aspects,
            Err(error) => {
                self.error = Some(error.to_string());
                return self;
            }
        };
        match delete(ForgeQueryDeleteMutationBuilder::new())
            .build_delete_existing_verified(binding, asserted_aspects)
        {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn delete_symbolic_with(
        mut self,
        reference: ForgeQuerySymbolicTargetReference,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Self {
        if self.error.is_some() {
            return self;
        }
        match declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete_symbolic(reference) {
            Ok(command) => self.commands.push(command),
            Err(error) => self.error = Some(error.to_string()),
        }
        self
    }

    pub fn build(self) -> Result<Vec<ForgeQueryWriteCommand>, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(error),
            ));
        }
        if self.commands.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new("mutation batch must declare at least one operation"),
            ));
        }
        Ok(self.commands)
    }

    pub(crate) fn finish(self) -> Result<Vec<ForgeQueryWriteCommand>, ForgeQueryRuntimeError> {
        self.build()
    }
}
