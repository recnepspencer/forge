use worth_ui_inspection::UiInspectionMeasurementDenialPosture;

use crate::evidence::measurement::basis::UiMeasurementBasisDenial;

use super::project_compatibility::project_generation_compatibility;
use super::project_evidence_maps::{
    project_basis_source, project_evidence_category, project_ownership_posture, project_slot,
};

pub(crate) fn project_denial(
    denial: &UiMeasurementBasisDenial,
) -> UiInspectionMeasurementDenialPosture {
    match denial {
        UiMeasurementBasisDenial::GenerationIncompatible { compatibility } => {
            UiInspectionMeasurementDenialPosture::GenerationIncompatible {
                compatibility: project_generation_compatibility(compatibility),
            }
        }
        UiMeasurementBasisDenial::MissingEvidence { slot } => {
            UiInspectionMeasurementDenialPosture::MissingEvidence {
                slot: project_slot(*slot),
            }
        }
        UiMeasurementBasisDenial::MissingBasisSourceEvidence { basis_source, slot } => {
            UiInspectionMeasurementDenialPosture::MissingBasisSourceEvidence {
                basis_source: project_basis_source(*basis_source),
                slot: project_slot(*slot),
            }
        }
        UiMeasurementBasisDenial::MissingOwnershipEvidence {
            ownership_posture,
            slot,
        } => UiInspectionMeasurementDenialPosture::MissingOwnershipEvidence {
            ownership_posture: project_ownership_posture(*ownership_posture),
            slot: project_slot(*slot),
        },
        UiMeasurementBasisDenial::MissingRequiredMeasurementEvidence { category, slot } => {
            UiInspectionMeasurementDenialPosture::MissingRequiredMeasurementEvidence {
                category: project_evidence_category(*category),
                slot: project_slot(*slot),
            }
        }
        UiMeasurementBasisDenial::ConflictingEvidenceInputs { slot } => {
            UiInspectionMeasurementDenialPosture::ConflictingEvidenceInputs {
                slot: project_slot(*slot),
            }
        }
    }
}
