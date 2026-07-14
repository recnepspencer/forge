mod artifact;
mod aspect_contract;
mod closeout;
mod declaration_handoff;
mod declared_posture;
mod family;
mod inspection;
mod measurement_dependency;
mod structural_semantics;
mod support;

pub(crate) use artifact::ui_declaration_lowering::UiDeclarationLowering;
pub(crate) use artifact::{authored_source_provenance_digest, stable_text_digest};
pub use artifact::{
    UiDeclarationArtifact, UiDeclarationArtifactDigest, UiDeclarationAspectDigest,
    UiDeclarationDigestProjection, UiDeclarationEquivalenceContract, UiDeclarationFamilyDigest,
    UiDeclarationIdentity, UiDeclarationIdentityDigest, UiDeclarationPostureDigest,
    UiDeclarationProvenance, UiDeclarationStructuralDigest, UiDeclarationSupportDigest,
};
pub(crate) use aspect_contract::UiAspectContractAdmission;
pub use aspect_contract::{
    UiAspectContract, UiAspectContractAdmissionDenial, UiAspectCoverageEntry,
    UiAspectCoverageReport, UiAspectFamily, UiAspectName, UiAspectSemanticSlice,
    UiConsumedAspectContract, UiPublishedAspectContract,
};
pub use closeout::{
    UiDeclarationClosedSemanticLane, UiDeclarationCloseoutGuarantee, UiDeclarationCloseoutNonGoal,
    UiDeclarationCloseoutReport,
};
pub use declaration_handoff::{UiDeclarationGraphHandoff, UiDeclarationGraphHandoffDenial};
pub(crate) use declaration_handoff::{
    UiDeclaredAspectPayload, UiDeclaredPosturePayload, UiStructuralDeclarationPayload,
};
pub(crate) use declared_posture::UiDeclaredPostureAdmission;
pub use declared_posture::{
    UiDeclaredHostCapabilityPosture, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementConstraintModifier, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementMode, UiDeclaredMeasurementOwnershipPosture,
    UiDeclaredMeasurementPolicyPosture, UiDeclaredPostureAdmissionDenial,
    UiDeclaredPostureApplicability, UiDeclaredPostureContract, UiDeclaredPostureLane,
    UiDeclaredPostureLaneKind, UiDeclaredQueryBindingPosture, UiDeclaredServiceUsagePosture,
    UiDeclaredTouchMeaningPosture,
};
pub(crate) use family::UiDeclarationFamilyAdmission;
pub use family::{
    UiDeclarationFamily, UiDeclarationFamilyAdmissionDenial, UiDeclarationFamilyCatalog,
    UiDeclarationFamilyKind,
};
pub(crate) use inspection::{UiDeclarationAuthoredEvidenceIndex, UiDeclarationEvidenceRecord};
pub(crate) use measurement_dependency::declared_measurement_basis_requirements;
pub(crate) use measurement_dependency::declared_query_measurement_dependencies;
pub use measurement_dependency::{
    UiDeclaredMeasurementBasisRequirementSet, UiDeclaredMeasurementQueryDependencySet,
};
pub(crate) use structural_semantics::UiDeclarationStructuralSemanticsAdmission;
pub use structural_semantics::{
    UiDeclarationContainmentIntent, UiDeclarationOrderingGuarantee,
    UiDeclarationPlanningOperatorKind, UiDeclarationRepetitionPosture,
    UiDeclarationSlotParticipationIntent, UiDeclarationStructuralRole,
    UiDeclarationStructuralSemantics, UiDeclarationStructuralSemanticsAdmissionDenial,
};
pub(crate) use support::{
    derive_declaration_inspection_support_projection, UiDeclarationInspectionSupportProjection,
    UiDeclarationSupportSnapshotAdmission,
};
pub use support::{
    UiDeclarationSupportMilestoneExpectation, UiDeclarationSupportRow,
    UiDeclarationSupportRowSchemaKind, UiDeclarationSupportSnapshot,
    UiDeclarationSupportSnapshotAdmissionDenial, UiDeclarationUnsupportedPosture,
};

#[cfg(test)]
mod declaration_measurement_registration_tests;
#[cfg(test)]
mod declared_measurement_posture_tests;
#[cfg(test)]
mod declared_posture_tests;
#[cfg(test)]
mod structural_operator_tests;
#[cfg(test)]
mod support_inspection_tests;
#[cfg(test)]
mod support_measurement_tests;
#[cfg(test)]
mod support_tests;
#[cfg(test)]
mod tests;
