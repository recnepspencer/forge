use worth_store_layout_indexes::{
    LsmMaintenanceAdmissionDenialKind as OwnerDenial,
    LsmMaintenanceDisposition as OwnerDisposition, LsmMaintenanceOperation as OwnerOperation,
    LsmMaintenanceOwnerCaseId,
};

use crate::protocols::compaction_visibility::{
    CompactionVisibilityAction, LsmMaintenanceAction, LsmMaintenanceDenial, ModeledOutcome,
};

pub(super) fn expected_action(owner_case: LsmMaintenanceOwnerCaseId) -> CompactionVisibilityAction {
    let operation = match owner_case.operation() {
        OwnerOperation::AdmitRunPublication => LsmMaintenanceAction::AdmitRunPublication,
        OwnerOperation::AdmitReplay => LsmMaintenanceAction::AdmitReplay,
        OwnerOperation::AdmitCompaction => LsmMaintenanceAction::AdmitCompaction,
    };
    let outcome = match owner_case.disposition() {
        OwnerDisposition::Admitted => ModeledOutcome::Admitted,
        OwnerDisposition::Denied(denial) => ModeledOutcome::Denied(expected_denial(denial)),
    };
    CompactionVisibilityAction::LsmMaintenance { operation, outcome }
}

const fn expected_denial(denial: OwnerDenial) -> LsmMaintenanceDenial {
    match denial {
        OwnerDenial::ArtifactFamily => LsmMaintenanceDenial::ArtifactFamily,
        OwnerDenial::SecurityScope => LsmMaintenanceDenial::SecurityScope,
        OwnerDenial::KeyDomain => LsmMaintenanceDenial::KeyDomain,
        OwnerDenial::ConcreteKey => LsmMaintenanceDenial::ConcreteKey,
        OwnerDenial::Shape => LsmMaintenanceDenial::Shape,
        OwnerDenial::RequestAdmission => LsmMaintenanceDenial::RequestAdmission,
        OwnerDenial::NoEligibleLayout => LsmMaintenanceDenial::NoEligibleLayout,
        OwnerDenial::Cost => LsmMaintenanceDenial::Cost,
        OwnerDenial::Budget => LsmMaintenanceDenial::Budget,
        OwnerDenial::UnexpectedSelectedOperation => {
            LsmMaintenanceDenial::UnexpectedSelectedOperation
        }
    }
}
