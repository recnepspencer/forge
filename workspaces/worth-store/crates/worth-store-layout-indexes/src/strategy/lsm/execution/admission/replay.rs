use super::super::BaselineLsmExecutionAdmissionDenial;
use crate::planning::SelectedLsmReplayRecovery;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmReplayAdmission {
    selected: SelectedLsmReplayRecovery,
    source: worth_store_lsm_authority::AdmittedLsmReplaySource,
    current_materialization: crate::CurrentLayoutMaterialization,
}

impl BaselineLsmReplayAdmission {
    pub fn admit(
        selected: SelectedLsmReplayRecovery,
        source: worth_store_lsm_authority::AdmittedLsmReplaySource,
        current_materialization: crate::CurrentLayoutMaterialization,
    ) -> Result<Self, BaselineLsmExecutionAdmissionDenial> {
        if selected.request_identity().canonical_key() != source.membership().key_ref().canonical()
        {
            return Err(BaselineLsmExecutionAdmissionDenial::SelectedOperationKeyMismatch);
        }
        if selected.materialization() != current_materialization.materialization() {
            return Err(BaselineLsmExecutionAdmissionDenial::ReplayBindingMismatch);
        }
        Ok(Self {
            selected,
            source,
            current_materialization,
        })
    }

    pub const fn selected(&self) -> &SelectedLsmReplayRecovery {
        &self.selected
    }
    pub const fn source(&self) -> &worth_store_lsm_authority::AdmittedLsmReplaySource {
        &self.source
    }
    pub fn into_source(self) -> worth_store_lsm_authority::AdmittedLsmReplaySource {
        self.source
    }
    pub const fn current_materialization(&self) -> &crate::CurrentLayoutMaterialization {
        &self.current_materialization
    }
    pub fn into_execution_basis(
        self,
    ) -> (
        worth_store_lsm_authority::AdmittedLsmReplaySource,
        crate::CurrentLayoutMaterialization,
    ) {
        (self.source, self.current_materialization)
    }
}
