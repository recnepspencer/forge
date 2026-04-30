use super::{ForgeQueryAspectMutationBuilder, ForgeQueryAspectValue};
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::{
    ForgeQueryExistingTruthTargetBinding, ForgeQueryRuntimeError, ForgeQueryWriteCommand,
};

use super::aspect_builder_helpers::finish_aspects;

impl ForgeQueryAspectMutationBuilder {
    pub(crate) fn finish_existing_truth_verification_aspects(
        self,
        lane_description: &'static str,
    ) -> Result<Vec<ForgeQueryAspectValue>, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        if !metadata.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(format!(
                    "{lane_description} may not declare metadata on the verification side"
                )),
            ));
        }
        if naming_intent.is_some() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(format!(
                    "{lane_description} may not declare naming intent on the verification side"
                )),
            ));
        }
        if continuity_intent.is_some() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(format!(
                    "{lane_description} may not declare continuity intent on the verification side"
                )),
            ));
        }
        finish_aspects(aspects, error)
    }

    pub fn build_assert_existing(
        self,
        binding: ForgeQueryExistingTruthTargetBinding,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        if naming_intent.is_some() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(
                    "existing-truth assertion may not declare naming intent",
                ),
            ));
        }
        if continuity_intent.is_some() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(
                    "existing-truth assertion may not declare continuity intent",
                ),
            ));
        }
        Ok(ForgeQueryWriteCommand::AssertExistingAspects {
            binding,
            aspects: finish_aspects(aspects, error)?,
            metadata,
        })
    }

    pub fn build_verify_existing(
        self,
        binding: ForgeQueryExistingTruthTargetBinding,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        if naming_intent.is_some() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(
                    "backend-verified existing-truth assertion may not declare naming intent",
                ),
            ));
        }
        if continuity_intent.is_some() {
            return Err(ForgeQueryRuntimeError::Workspace(
                ForgeQueryWorkspaceError::new(
                    "backend-verified existing-truth assertion may not declare continuity intent",
                ),
            ));
        }
        Ok(ForgeQueryWriteCommand::VerifyExistingAspects {
            binding,
            aspects: finish_aspects(aspects, error)?,
            metadata,
        })
    }

    pub(crate) fn build_update_existing_verified(
        self,
        binding: ForgeQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<ForgeQueryAspectValue>,
    ) -> Result<ForgeQueryWriteCommand, ForgeQueryRuntimeError> {
        let ForgeQueryAspectMutationBuilder {
            aspects,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        Ok(ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects {
            binding,
            asserted_aspects,
            aspects: finish_aspects(aspects, error)?,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }
}
