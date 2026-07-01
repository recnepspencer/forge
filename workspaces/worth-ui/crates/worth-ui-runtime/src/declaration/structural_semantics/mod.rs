mod admission;
mod containment_intent;
mod denial;
mod ordering_guarantee;
mod repetition_posture;
mod slot_participation_intent;
mod structural_semantics;
mod topology_role;

pub(crate) use admission::admit_declaration_structural_semantics;
pub use containment_intent::UiDeclarationContainmentIntent;
pub(crate) use denial::UiDeclarationStructuralSemanticsAdmission;
pub use denial::UiDeclarationStructuralSemanticsAdmissionDenial;
pub use ordering_guarantee::UiDeclarationOrderingGuarantee;
pub use repetition_posture::UiDeclarationRepetitionPosture;
pub use slot_participation_intent::UiDeclarationSlotParticipationIntent;
pub use structural_semantics::UiDeclarationStructuralSemantics;
pub use topology_role::UiDeclarationStructuralRole;
