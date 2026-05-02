use std::collections::BTreeSet;

use super::denial::{
    graph_composition_error, ForgeQueryGraphCompositionDenial, ForgeQueryGraphCompositionDenialKind,
};
use super::existing_lifecycle::{require_retarget_intent, require_supersession_intent};
use super::relation_builder::ForgeQueryGraphRelationMutationBuilder;
use super::symbols::{ForgeQueryGraphEntitySymbol, ForgeQueryGraphRelationSymbol};
use crate::runtime::mutation::{ForgeQueryAspectMutationBuilder, ForgeQueryDeleteMutationBuilder};
use crate::runtime::{
    ForgeQueryExistingTruthTargetBinding, ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram, ForgeQueryGraphCompositionProgramStep,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryRuntimeError,
    ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, Default)]
pub struct ForgeQueryGraphCompositionBuilder {
    commands: Vec<ForgeQueryWriteCommand>,
    program_steps: Vec<ForgeQueryGraphCompositionProgramStep>,
    declared_symbols: BTreeSet<String>,
    symbolic_entity_declaration_count: usize,
    symbolic_relation_declaration_count: usize,
    error: Option<ForgeQueryGraphCompositionDenial>,
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
        let component_index = self.commands.len();
        let command = declaration(ForgeQueryAspectMutationBuilder::new())
            .build_insert_symbolic(reference.symbol().to_string(), collection)?;
        self.commands.push(command);
        self.program_steps
            .push(ForgeQueryGraphCompositionProgramStep::new(
                component_index,
                ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration,
                reference.target_collection().unwrap_or(""),
                Some(reference.symbol().to_string()),
            ));
        self.symbolic_entity_declaration_count += 1;
        Ok(ForgeQueryGraphEntitySymbol::new(reference))
    }
    pub fn insert_relation(
        &mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(
            ForgeQueryGraphRelationMutationBuilder,
        ) -> ForgeQueryGraphRelationMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let component_index = self.commands.len();
        let collection = collection.into();
        let command = declaration(ForgeQueryGraphRelationMutationBuilder::new())
            .into_inner()
            .build_insert(collection.clone())?;
        self.commands.push(command);
        self.program_steps
            .push(ForgeQueryGraphCompositionProgramStep::new(
                component_index,
                ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration,
                collection,
                None,
            ));
        self.symbolic_relation_declaration_count += 1;
        Ok(())
    }
    pub fn update_entity(
        &mut self,
        symbol: &ForgeQueryGraphEntitySymbol,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let component_index = self.commands.len();
        let command = declaration(ForgeQueryAspectMutationBuilder::new())
            .build_update_symbolic(symbol.reference())?;
        self.commands.push(command);
        self.program_steps
            .push(ForgeQueryGraphCompositionProgramStep::new(
                component_index,
                ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation,
                symbol.reference().target_collection().unwrap_or(""),
                Some(symbol.symbol().to_string()),
            ));
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
        let component_index = self.commands.len();
        let command = declaration(ForgeQueryGraphRelationMutationBuilder::new())
            .into_inner()
            .build_insert_symbolic(reference.symbol().to_string(), collection)?;
        self.commands.push(command);
        self.program_steps
            .push(ForgeQueryGraphCompositionProgramStep::new(
                component_index,
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration,
                reference.target_collection().unwrap_or(""),
                Some(reference.symbol().to_string()),
            ));
        self.symbolic_relation_declaration_count += 1;
        Ok(ForgeQueryGraphRelationSymbol::new(reference))
    }
    pub fn update_relation(
        &mut self,
        symbol: &ForgeQueryGraphRelationSymbol,
        declaration: impl FnOnce(
            ForgeQueryGraphRelationMutationBuilder,
        ) -> ForgeQueryGraphRelationMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let component_index = self.commands.len();
        let command = declaration(ForgeQueryGraphRelationMutationBuilder::new())
            .into_inner()
            .build_update_symbolic(symbol.reference())?;
        self.commands.push(command);
        self.program_steps
            .push(ForgeQueryGraphCompositionProgramStep::new(
                component_index,
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation,
                symbol.reference().target_collection().unwrap_or(""),
                Some(symbol.symbol().to_string()),
            ));
        Ok(())
    }
    pub fn delete_relation(
        &mut self,
        symbol: &ForgeQueryGraphRelationSymbol,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let component_index = self.commands.len();
        let command = declaration(ForgeQueryDeleteMutationBuilder::new())
            .build_delete_symbolic(symbol.reference())?;
        self.commands.push(command);
        self.program_steps
            .push(ForgeQueryGraphCompositionProgramStep::new(
                component_index,
                ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
                symbol.reference().target_collection().unwrap_or(""),
                Some(symbol.symbol().to_string()),
            ));
        Ok(())
    }
    pub fn update_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation,
            declared_collection,
        );
        Ok(())
    }
    pub fn retarget_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        require_retarget_intent(&command, &declared_collection)?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget,
            declared_collection,
        );
        Ok(())
    }
    pub fn supersede_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        require_supersession_intent(&command, &declared_collection)?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetSupersession,
            declared_collection,
        );
        Ok(())
    }
    pub fn update_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        update: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let command = build_verified_existing_update_command(
            binding,
            verify,
            update,
            "backend-verified existing-truth update",
        )?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation,
            declared_collection,
        );
        Ok(())
    }
    pub fn retarget_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        update: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let command = build_verified_existing_update_command(
            binding,
            verify,
            update,
            "backend-verified existing-truth retarget",
        )?;
        require_retarget_intent(&command, &declared_collection)?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            declared_collection,
        );
        Ok(())
    }
    pub fn supersede_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        update: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let command = build_verified_existing_update_command(
            binding,
            verify,
            update,
            "backend-verified existing-truth supersession",
        )?;
        require_supersession_intent(&command, &declared_collection)?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession,
            declared_collection,
        );
        Ok(())
    }
    pub fn delete_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let command =
            declaration(ForgeQueryDeleteMutationBuilder::new()).build_delete_existing(binding)?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement,
            declared_collection,
        );
        Ok(())
    }
    pub fn delete_existing_verified(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
        delete: impl FnOnce(ForgeQueryDeleteMutationBuilder) -> ForgeQueryDeleteMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = binding.target_collection().unwrap_or("").to_string();
        let asserted_aspects = verify(ForgeQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth delete")?;
        let command = delete(ForgeQueryDeleteMutationBuilder::new())
            .build_delete_existing_verified(binding, asserted_aspects)?;
        self.push_existing_target_step(
            command,
            ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement,
            declared_collection,
        );
        Ok(())
    }
    pub fn finish(
        self,
    ) -> Result<
        (
            Vec<ForgeQueryWriteCommand>,
            ForgeQueryGraphCompositionBreadth,
            ForgeQueryGraphCompositionProgram,
        ),
        ForgeQueryRuntimeError,
    > {
        if let Some(error) = self.error {
            return Err(ForgeQueryRuntimeError::GraphCompositionDenied(error));
        }
        if self.commands.is_empty() {
            return Err(graph_composition_error(
                ForgeQueryGraphCompositionDenialKind::EmptyComposition,
                None,
                None,
                "graph composition must declare at least one operation",
            ));
        }
        let breadth = ForgeQueryGraphCompositionBreadth::new(
            self.commands.len(),
            self.symbolic_entity_declaration_count,
            self.symbolic_relation_declaration_count,
        );
        let program = ForgeQueryGraphCompositionProgram::new(self.program_steps, &breadth);
        Ok((self.commands, breadth, program))
    }
    fn require_clean(&self) -> Result<(), ForgeQueryRuntimeError> {
        if let Some(error) = &self.error {
            return Err(ForgeQueryRuntimeError::GraphCompositionDenied(
                error.clone(),
            ));
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
            let denial = ForgeQueryGraphCompositionDenial::new(
                ForgeQueryGraphCompositionDenialKind::DuplicateSymbolDeclaration,
                Some(symbol),
                Some(collection.to_string()),
                message,
            );
            self.error = Some(denial.clone());
            return Err(ForgeQueryRuntimeError::GraphCompositionDenied(denial));
        }
        ForgeQuerySymbolicTargetReference::new(symbol)?
            .in_target_collection(collection)
            .map_err(ForgeQueryRuntimeError::from)
    }
    fn push_existing_target_step(
        &mut self,
        command: ForgeQueryWriteCommand,
        kind: ForgeQueryGraphCompositionProgramStepKind,
        declared_collection: String,
    ) {
        let component_index = self.commands.len();
        self.commands.push(command);
        self.program_steps
            .push(ForgeQueryGraphCompositionProgramStep::new(
                component_index,
                kind,
                declared_collection,
                None,
            ));
    }
}
fn build_verified_existing_update_command(
    binding: ForgeQueryExistingTruthTargetBinding,
    verify: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    update: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    verification_context: &'static str,
) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
    let asserted_aspects = verify(ForgeQueryAspectMutationBuilder::new())
        .finish_existing_truth_verification_aspects(verification_context)?;
    update(ForgeQueryAspectMutationBuilder::new())
        .build_update_existing_verified(binding, asserted_aspects)
}
