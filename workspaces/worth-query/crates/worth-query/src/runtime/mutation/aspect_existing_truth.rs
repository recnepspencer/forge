use super::{WorthQueryAspectMutationBuilder, WorthQueryAuthoredAspectMutation};
use crate::memory_workspace::WorthQueryWorkspaceError;
use crate::runtime::{
    WorthQueryExistingTruthTargetBinding, WorthQueryRuntimeError, WorthQueryWriteCommand,
};

use super::aspect_builder_helpers::{finish_aspects, reject_symbolic_aspect_references};

impl WorthQueryAspectMutationBuilder {
    pub(crate) fn finish_existing_truth_verification_aspects(
        self,
        lane_description: &'static str,
    ) -> Result<Vec<WorthQueryAuthoredAspectMutation>, WorthQueryRuntimeError> {
        let WorthQueryAspectMutationBuilder {
            aspects,
            symbolic_aspect_references,
            metadata,
            naming_intent,
            continuity_intent,
            error,
            ..
        } = self;
        reject_symbolic_aspect_references(&symbolic_aspect_references, lane_description)?;
        if !metadata.is_empty() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(format!(
                    "{lane_description} may not declare metadata on the verification side"
                )),
            ));
        }
        if naming_intent.is_some() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(format!(
                    "{lane_description} may not declare naming intent on the verification side"
                )),
            ));
        }
        if continuity_intent.is_some() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(format!(
                    "{lane_description} may not declare continuity intent on the verification side"
                )),
            ));
        }
        finish_aspects(aspects, error)
    }

    pub fn build_assert_existing(
        self,
        binding: WorthQueryExistingTruthTargetBinding,
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
        reject_symbolic_aspect_references(&symbolic_aspect_references, "existing-truth assertion")?;
        if naming_intent.is_some() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(
                    "existing-truth assertion may not declare naming intent",
                ),
            ));
        }
        if continuity_intent.is_some() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(
                    "existing-truth assertion may not declare continuity intent",
                ),
            ));
        }
        Ok(WorthQueryWriteCommand::AssertExistingAspects {
            binding,
            aspects: finish_aspects(aspects, error)?,
            metadata,
        })
    }

    pub fn build_verify_existing(
        self,
        binding: WorthQueryExistingTruthTargetBinding,
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
            "backend-verified existing-truth assertion",
        )?;
        if naming_intent.is_some() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(
                    "backend-verified existing-truth assertion may not declare naming intent",
                ),
            ));
        }
        if continuity_intent.is_some() {
            return Err(WorthQueryRuntimeError::Workspace(
                WorthQueryWorkspaceError::new(
                    "backend-verified existing-truth assertion may not declare continuity intent",
                ),
            ));
        }
        Ok(WorthQueryWriteCommand::VerifyExistingAspects {
            binding,
            aspects: finish_aspects(aspects, error)?,
            metadata,
        })
    }

    pub(crate) fn build_update_existing_verified(
        self,
        binding: WorthQueryExistingTruthTargetBinding,
        asserted_aspects: Vec<WorthQueryAuthoredAspectMutation>,
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
        Ok(WorthQueryWriteCommand::VerifyThenUpdateExistingAspects {
            binding,
            asserted_aspects,
            aspects: finish_aspects(aspects, error)?,
            symbolic_aspect_references,
            metadata,
            naming_intent,
            continuity_intent,
        })
    }
}
