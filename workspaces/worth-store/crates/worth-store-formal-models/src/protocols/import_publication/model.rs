use super::ImportPublicationAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportPublicationState {
    RawDeclaration,
    CurrentScopeReadmitted,
    RecoveredArtifactAdmitted,
    LayoutMaterialized,
    PublicationPending,
    PublicationDurable,
    PublicationDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImportPublicationModelDenial {
    CurrentScopeReadmissionRequired,
    RecoveredArtifactAdmissionRequired,
    LayoutMaterializationRequired,
    PublicationReadinessRequired,
    ExactPhysicalPublicationRequired,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportPublicationModel {
    state: ImportPublicationState,
    actions: Vec<ImportPublicationAction>,
}

impl ImportPublicationModel {
    pub fn from_raw_declaration() -> Self {
        Self {
            state: ImportPublicationState::RawDeclaration,
            actions: vec![ImportPublicationAction::RawDeclarationObserved],
        }
    }

    pub fn readmit_current_scope(&mut self) {
        self.state = ImportPublicationState::CurrentScopeReadmitted;
        self.actions
            .push(ImportPublicationAction::CurrentScopeReadmitted);
    }

    pub fn admit_recovered_artifact(&mut self) -> Result<(), ImportPublicationModelDenial> {
        if self.state != ImportPublicationState::CurrentScopeReadmitted {
            return Err(ImportPublicationModelDenial::CurrentScopeReadmissionRequired);
        }
        self.state = ImportPublicationState::RecoveredArtifactAdmitted;
        self.actions
            .push(ImportPublicationAction::RecoveredArtifactAdmitted);
        Ok(())
    }

    pub fn admit_layout_materialization(&mut self) -> Result<(), ImportPublicationModelDenial> {
        if self.state != ImportPublicationState::RecoveredArtifactAdmitted {
            return Err(ImportPublicationModelDenial::RecoveredArtifactAdmissionRequired);
        }
        self.state = ImportPublicationState::LayoutMaterialized;
        self.actions
            .push(ImportPublicationAction::LayoutMaterializationAdmitted);
        Ok(())
    }

    pub fn admit_publication_readiness(&mut self) -> Result<(), ImportPublicationModelDenial> {
        if self.state != ImportPublicationState::LayoutMaterialized {
            return Err(ImportPublicationModelDenial::LayoutMaterializationRequired);
        }
        self.state = ImportPublicationState::PublicationPending;
        self.actions
            .push(ImportPublicationAction::PublicationPending);
        Ok(())
    }

    pub fn complete_publication(
        &mut self,
        exact_physical_publication: bool,
    ) -> Result<(), ImportPublicationModelDenial> {
        if self.state != ImportPublicationState::PublicationPending {
            return Err(ImportPublicationModelDenial::PublicationReadinessRequired);
        }
        if !exact_physical_publication {
            self.state = ImportPublicationState::PublicationDenied;
            self.actions
                .push(ImportPublicationAction::PublicationDenied);
            return Err(ImportPublicationModelDenial::ExactPhysicalPublicationRequired);
        }
        self.state = ImportPublicationState::PublicationDurable;
        self.actions
            .push(ImportPublicationAction::PublicationDurable);
        Ok(())
    }

    pub fn crash(&mut self) {
        if self.state == ImportPublicationState::PublicationPending {
            self.state = ImportPublicationState::LayoutMaterialized;
            self.actions
                .push(ImportPublicationAction::CrashBeforePublication);
        }
    }

    pub const fn state(&self) -> ImportPublicationState {
        self.state
    }

    pub fn actions(&self) -> impl Iterator<Item = ImportPublicationAction> + '_ {
        self.actions.iter().copied()
    }
}
