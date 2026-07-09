use super::*;
use crate::runtime::mutation::graph_composition::existing_lifecycle::{
    require_retarget_intent, require_supersession_intent,
};

impl WorthQueryGraphCompositionBuilder {
    pub fn update_existing(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
        let command =
            declaration(WorthQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        self.push_existing_target_step(
            command,
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation,
            declared_collection,
        );
        Ok(())
    }

    pub fn retarget_existing(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
        let command =
            declaration(WorthQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        require_retarget_intent(&command, declared_collection.as_ref())?;
        self.push_existing_target_step(
            command,
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetRetarget,
            declared_collection,
        );
        Ok(())
    }

    pub fn supersede_existing(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
        let command =
            declaration(WorthQueryAspectMutationBuilder::new()).build_update_existing(binding)?;
        require_supersession_intent(&command, declared_collection.as_ref())?;
        self.push_existing_target_step(
            command,
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetSupersession,
            declared_collection,
        );
        Ok(())
    }

    pub fn update_existing_verified(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        verify: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
        update: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
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
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation,
            declared_collection,
        );
        Ok(())
    }

    pub fn retarget_existing_verified(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        verify: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
        update: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
        let command = build_verified_existing_update_command(
            binding,
            verify,
            update,
            "backend-verified existing-truth retarget",
        )?;
        require_retarget_intent(&command, declared_collection.as_ref())?;
        self.push_existing_target_step(
            command,
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget,
            declared_collection,
        );
        Ok(())
    }

    pub fn supersede_existing_verified(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        verify: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
        update: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
        let command = build_verified_existing_update_command(
            binding,
            verify,
            update,
            "backend-verified existing-truth supersession",
        )?;
        require_supersession_intent(&command, declared_collection.as_ref())?;
        self.push_existing_target_step(
            command,
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession,
            declared_collection,
        );
        Ok(())
    }

    pub fn delete_existing(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        declaration: impl FnOnce(WorthQueryDeleteMutationBuilder) -> WorthQueryDeleteMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
        let command =
            declaration(WorthQueryDeleteMutationBuilder::new()).build_delete_existing(binding)?;
        self.push_existing_target_step(
            command,
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetRetirement,
            declared_collection,
        );
        Ok(())
    }

    pub fn delete_existing_verified(
        &mut self,
        binding: WorthQueryExistingTruthTargetBinding,
        verify: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
        delete: impl FnOnce(WorthQueryDeleteMutationBuilder) -> WorthQueryDeleteMutationBuilder,
    ) -> Result<(), WorthQueryRuntimeError> {
        self.require_clean()?;
        let declared_collection = existing_truth_declared_collection(&binding);
        let asserted_aspects = verify(WorthQueryAspectMutationBuilder::new())
            .finish_existing_truth_verification_aspects("backend-verified existing-truth delete")?;
        let command = delete(WorthQueryDeleteMutationBuilder::new())
            .build_delete_existing_verified(binding, asserted_aspects)?;
        self.push_existing_target_step(
            command,
            WorthQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement,
            declared_collection,
        );
        Ok(())
    }
}

fn existing_truth_declared_collection(
    binding: &WorthQueryExistingTruthTargetBinding,
) -> Option<WorthQueryMutationTargetCollectionIdentity> {
    binding.target_collection_identity().cloned()
}

fn build_verified_existing_update_command(
    binding: WorthQueryExistingTruthTargetBinding,
    verify: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    update: impl FnOnce(WorthQueryAspectMutationBuilder) -> WorthQueryAspectMutationBuilder,
    verification_context: &'static str,
) -> Result<WorthQueryWriteCommand, WorthQueryRuntimeError> {
    let asserted_aspects = verify(WorthQueryAspectMutationBuilder::new())
        .finish_existing_truth_verification_aspects(verification_context)?;
    update(WorthQueryAspectMutationBuilder::new())
        .build_update_existing_verified(binding, asserted_aspects)
}
