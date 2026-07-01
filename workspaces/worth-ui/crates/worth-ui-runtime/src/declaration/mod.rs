mod artifact;
mod aspect_contract;
mod closeout;
mod declaration_handoff;
mod declared_posture;
mod family;
mod structural_semantics;
mod support;

pub(crate) use artifact::ui_declaration_lowering::UiDeclarationLowering;
pub use artifact::{
    UiDeclarationArtifact, UiDeclarationArtifactDigest, UiDeclarationAspectDigest,
    UiDeclarationDigestProjection, UiDeclarationEquivalenceContract, UiDeclarationFamilyDigest,
    UiDeclarationIdentity, UiDeclarationIdentityDigest, UiDeclarationPostureDigest,
    UiDeclarationProvenance, UiDeclarationStructuralDigest, UiDeclarationSupportDigest,
};
pub(crate) use artifact::stable_text_digest;
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
    UiDeclaredHostCapabilityPosture, UiDeclaredMeasurementPolicyPosture,
    UiDeclaredPostureAdmissionDenial, UiDeclaredPostureApplicability, UiDeclaredPostureContract,
    UiDeclaredPostureLane, UiDeclaredPostureLaneKind, UiDeclaredQueryBindingPosture,
    UiDeclaredServiceUsagePosture, UiDeclaredTouchMeaningPosture,
};
pub(crate) use family::UiDeclarationFamilyAdmission;
pub use family::{
    UiDeclarationFamily, UiDeclarationFamilyAdmissionDenial, UiDeclarationFamilyCatalog,
    UiDeclarationFamilyKind,
};
pub(crate) use structural_semantics::UiDeclarationStructuralSemanticsAdmission;
pub use structural_semantics::{
    UiDeclarationContainmentIntent, UiDeclarationOrderingGuarantee, UiDeclarationRepetitionPosture,
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
mod declared_posture_tests;
#[cfg(test)]
mod support_tests;
#[cfg(test)]
mod tests;
