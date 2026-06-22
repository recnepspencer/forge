use super::*;
use crate::runtime::mutation::graph_composition::existing_lifecycle::{
    require_retarget_intent, require_supersession_intent,
};

impl ForgeQueryGraphCompositionBuilder {
    pub fn update_existing(
        &mut self,
        binding: ForgeQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<(), ForgeQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
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
        let declared_collection = existing_truth_declared_collection(&binding);
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
        let declared_collection = existing_truth_declared_collection(&binding);
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
        let declared_collection = existing_truth_declared_collection(&binding);
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
        let declared_collection = existing_truth_declared_collection(&binding);
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
        let declared_collection = existing_truth_declared_collection(&binding);
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
        let declared_collection = existing_truth_declared_collection(&binding);
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
        let declared_collection = existing_truth_declared_collection(&binding);
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
}

fn existing_truth_declared_collection(binding: &ForgeQueryExistingTruthTargetBinding) -> String {
    binding
        .terminal_target_collection_projection()
        .unwrap_or("")
        .to_string()
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
