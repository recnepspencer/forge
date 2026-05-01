use std::collections::BTreeSet;

use serde::Serialize;

use super::ForgeQueryAspectMutationBuilder;
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::{
    ForgeQueryRuntimeError, ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphEntitySymbol {
    reference: ForgeQuerySymbolicTargetReference,
}

impl ForgeQueryGraphEntitySymbol {
    pub fn symbol(&self) -> &str {
        self.reference.symbol()
    }

    pub(crate) fn reference(&self) -> ForgeQuerySymbolicTargetReference {
        self.reference.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphRelationSymbol {
    reference: ForgeQuerySymbolicTargetReference,
}

impl ForgeQueryGraphRelationSymbol {
    pub fn symbol(&self) -> &str {
        self.reference.symbol()
    }

    pub(crate) fn reference(&self) -> ForgeQuerySymbolicTargetReference {
        self.reference.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ForgeQueryGraphRelationMutationBuilder {
    inner: ForgeQueryAspectMutationBuilder,
}

impl ForgeQueryGraphRelationMutationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn aspect<T: Serialize>(mut self, path: impl Into<String>, value: T) -> Self {
        self.inner = self.inner.aspect(path, value);
        self
    }

    pub fn existing_entity_identity(
        mut self,
        path: impl Into<String>,
        entity_identity: impl Into<String>,
    ) -> Self {
        self.inner = self.inner.aspect(path, entity_identity.into());
        self
    }

    pub fn symbolic_entity_identity(
        mut self,
        path: impl Into<String>,
        symbol: &ForgeQueryGraphEntitySymbol,
    ) -> Self {
        self.inner = self
            .inner
            .symbolic_entity_identity(path, symbol.reference());
        self
    }

    fn into_inner(self) -> ForgeQueryAspectMutationBuilder {
        self.inner
    }
}

#[derive(Clone, Debug, Default)]
pub struct ForgeQueryGraphCompositionBuilder {
    commands: Vec<ForgeQueryWriteCommand>,
    declared_symbols: BTreeSet<String>,
    error: Option<String>,
}

impl ForgeQueryGraphCompositionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_entity(
        &mut self,
        symbol: impl Into<String>,
        collection: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryGraphEntitySymbol, ForgeQueryRuntimeError> {
        self.require_clean()?;
        let collection = collection.into();
        let reference = self.declare_symbol(symbol, &collection)?;
        let command = declaration(ForgeQueryAspectMutationBuilder::new())
            .build_insert_symbolic(reference.symbol().to_string(), collection)?;
        self.commands.push(command);
        Ok(ForgeQueryGraphEntitySymbol { reference })
    }

    pub fn insert_relation(
        &mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(
            ForgeQueryGraphRelationMutationBuilder,
        ) -> ForgeQueryGraphRelationMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let command = declaration(ForgeQueryGraphRelationMutationBuilder::new())
            .into_inner()
            .build_insert(collection)?;
        self.commands.push(command);
        Ok(())
    }

    pub fn insert_symbolic_relation(
        &mut self,
        symbol: impl Into<String>,
        collection: impl Into<String>,
        declaration: impl FnOnce(
            ForgeQueryGraphRelationMutationBuilder,
        ) -> ForgeQueryGraphRelationMutationBuilder,
    ) -> Result<ForgeQueryGraphRelationSymbol, ForgeQueryRuntimeError> {
        self.require_clean()?;
        let collection = collection.into();
        let reference = self.declare_symbol(symbol, &collection)?;
        let command = declaration(ForgeQueryGraphRelationMutationBuilder::new())
            .into_inner()
            .build_insert_symbolic(reference.symbol().to_string(), collection)?;
        self.commands.push(command);
        Ok(ForgeQueryGraphRelationSymbol { reference })
    }

    pub fn update_relation(
        &mut self,
        symbol: &ForgeQueryGraphRelationSymbol,
        declaration: impl FnOnce(
            ForgeQueryGraphRelationMutationBuilder,
        ) -> ForgeQueryGraphRelationMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let command = declaration(ForgeQueryGraphRelationMutationBuilder::new())
            .into_inner()
            .build_update_symbolic(symbol.reference())?;
        self.commands.push(command);
        Ok(())
    }

    pub fn finish(self) -> Result<Vec<ForgeQueryWriteCommand>, ForgeQueryRuntimeError> {
        if let Some(error) = self.error {
            return Err(workspace_error(error));
        }
        if self.commands.is_empty() {
            return Err(workspace_error(
                "graph composition must declare at least one operation",
            ));
        }
        Ok(self.commands)
    }

    fn require_clean(&self) -> Result<(), ForgeQueryRuntimeError> {
        if let Some(error) = &self.error {
            return Err(workspace_error(error.clone()));
        }
        Ok(())
    }

    fn declare_symbol(
        &mut self,
        symbol: impl Into<String>,
        collection: &str,
    ) -> Result<ForgeQuerySymbolicTargetReference, ForgeQueryRuntimeError> {
        let symbol = symbol.into();
        if !self.declared_symbols.insert(symbol.clone()) {
            let message =
                format!("graph composition symbol `{symbol}` was declared more than once");
            self.error = Some(message.clone());
            return Err(workspace_error(message));
        }
        ForgeQuerySymbolicTargetReference::new(symbol)?
            .in_target_collection(collection)
            .map_err(ForgeQueryRuntimeError::from)
    }
}

fn workspace_error(message: impl Into<String>) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(message))
}
