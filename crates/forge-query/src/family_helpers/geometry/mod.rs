mod continuation;
mod material_attachment;

pub use material_attachment::ForgeQueryGeometryMaterialAttachmentInput;

use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryAdmittedDeclarationProgression,
    ForgeQueryDeclarationEntryProgressionError, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::contribution_composed_orchestration::{
    ForgeQueryContributionComposedOrchestration,
    ForgeQueryContributionComposedOrchestrationChecked,
    ForgeQueryContributionComposedOrchestrationOutcome,
    ForgeQueryContributionComposedOrchestrationTranscript,
};
use crate::ordinary_outcome::ForgeQueryOrdinaryOutcome;
use crate::signal_compatibility_orchestration::{
    ForgeQuerySignalCompatibilityOrchestration, ForgeQuerySignalCompatibilityOrchestrationChecked,
    ForgeQuerySignalCompatibilityOrchestrationOutcome,
    ForgeQuerySignalCompatibilityOrchestrationTranscript,
};

pub trait ForgeQueryGeometryActiveFaceSelectionHelperFamily<D: ForgeQueryDomainEntryMarker>:
    ForgeQueryDeclarationFamilyMarker<D>
{
}

pub trait ForgeQueryGeometryMaterialAttachmentHelperFamily<D: ForgeQueryDomainEntryMarker>:
    ForgeQueryDeclarationFamilyMarker<D>
{
}

pub struct ForgeQueryGeometryFamilyHelpers<
    'a,
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
> {
    handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
}

impl<'a, D: ForgeQueryDomainEntryMarker, C: ForgeQueryDomainOperatingContext<D>>
    ForgeQueryGeometryFamilyHelpers<'a, D, C>
{
    pub(crate) fn new(handle: &'a ForgeQueryAdmittedConfiguredDomainHandle<D, C>) -> Self {
        Self { handle }
    }

    pub fn progress_active_face_selection<I>(
        &self,
        input: I,
    ) -> Result<
        ForgeQueryAdmittedDeclarationProgression<D, I>,
        ForgeQueryDeclarationEntryProgressionError<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        self.handle.declare_review_and_progress(input)
    }

    pub fn prepare_preview_for_active_face_selection<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_preview_for_active_face_selection(self.handle, progression)
    }

    pub fn prepare_preview_for_active_face_selection_outcome<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_preview_for_active_face_selection_outcome(self.handle, progression)
    }

    pub fn prepare_preview_for_active_face_selection_checked<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_preview_for_active_face_selection_checked(self.handle, progression)
    }

    pub fn prepare_preview_for_active_face_selection_proof<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_preview_for_active_face_selection_proof(self.handle, progression)
    }

    pub fn prepare_runtime_route_for_active_face_selection<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_runtime_route_for_active_face_selection(self.handle, progression)
    }

    pub fn prepare_runtime_route_for_active_face_selection_outcome<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_runtime_route_for_active_face_selection_outcome(
            self.handle,
            progression,
        )
    }

    pub fn prepare_runtime_route_for_active_face_selection_checked<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_runtime_route_for_active_face_selection_checked(
            self.handle,
            progression,
        )
    }

    pub fn prepare_runtime_route_for_active_face_selection_proof<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_runtime_route_for_active_face_selection_proof(
            self.handle,
            progression,
        )
    }

    pub fn prepare_current_truth_view_for_active_face_selection<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_current_truth_view_for_active_face_selection(self.handle, progression)
    }

    pub fn prepare_current_truth_view_for_active_face_selection_outcome<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_current_truth_view_for_active_face_selection_outcome(
            self.handle,
            progression,
        )
    }

    pub fn prepare_current_truth_view_for_active_face_selection_checked<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_current_truth_view_for_active_face_selection_checked(
            self.handle,
            progression,
        )
    }

    pub fn prepare_current_truth_view_for_active_face_selection_proof<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_current_truth_view_for_active_face_selection_proof(
            self.handle,
            progression,
        )
    }

    pub fn prepare_historical_truth_view_for_active_face_selection<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationOutcome<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_historical_truth_view_for_active_face_selection(
            self.handle,
            progression,
        )
    }

    pub fn prepare_historical_truth_view_for_active_face_selection_outcome<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQuerySignalCompatibilityOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_historical_truth_view_for_active_face_selection_outcome(
            self.handle,
            progression,
        )
    }

    pub fn prepare_historical_truth_view_for_active_face_selection_checked<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_historical_truth_view_for_active_face_selection_checked(
            self.handle,
            progression,
        )
    }

    pub fn prepare_historical_truth_view_for_active_face_selection_proof<I>(
        &self,
        progression: ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> ForgeQuerySignalCompatibilityOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryActiveFaceSelectionHelperFamily<D>,
    {
        continuation::prepare_historical_truth_view_for_active_face_selection_proof(
            self.handle,
            progression,
        )
    }

    pub fn orchestrate_material_attachment_for_active_face_selection<I>(
        &self,
        input: ForgeQueryGeometryMaterialAttachmentInput<D, I>,
    ) -> Result<
        ForgeQueryContributionComposedOrchestration<D, I>,
        ForgeQueryContributionComposedOrchestrationOutcome<D, I>,
    >
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryMaterialAttachmentHelperFamily<D>,
    {
        self.handle
            .orchestrate_declaration_with_contributions(input.into_composed_input())
    }

    pub fn orchestrate_material_attachment_for_active_face_selection_outcome<I>(
        &self,
        input: ForgeQueryGeometryMaterialAttachmentInput<D, I>,
    ) -> ForgeQueryOrdinaryOutcome<ForgeQueryContributionComposedOrchestration<D, I>>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryMaterialAttachmentHelperFamily<D>,
    {
        self.handle
            .orchestrate_declaration_with_contributions_outcome(input.into_composed_input())
    }

    pub fn orchestrate_material_attachment_for_active_face_selection_checked<I>(
        &self,
        input: ForgeQueryGeometryMaterialAttachmentInput<D, I>,
    ) -> ForgeQueryContributionComposedOrchestrationChecked<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryMaterialAttachmentHelperFamily<D>,
    {
        self.handle
            .orchestrate_declaration_with_contributions_checked(input.into_composed_input())
    }

    pub fn orchestrate_material_attachment_for_active_face_selection_proof<I>(
        &self,
        input: ForgeQueryGeometryMaterialAttachmentInput<D, I>,
    ) -> ForgeQueryContributionComposedOrchestrationTranscript<D, I>
    where
        I: ForgeQueryDeclarationInput<D>,
        I::Family: ForgeQueryGeometryMaterialAttachmentHelperFamily<D>,
    {
        self.handle
            .orchestrate_declaration_with_contributions_proof(input.into_composed_input())
    }
}
