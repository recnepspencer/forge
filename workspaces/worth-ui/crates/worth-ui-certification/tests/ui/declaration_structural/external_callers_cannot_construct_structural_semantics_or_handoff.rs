use worth_ui::facade::declaration::{
    UiDeclarationContainmentIntent, UiDeclarationFamily, UiDeclarationFamilyKind,
    UiDeclarationGraphHandoff, UiDeclarationIdentity, UiDeclarationOrderingGuarantee,
    UiDeclarationRepetitionPosture, UiDeclarationSlotParticipationIntent,
    UiDeclarationStructuralRole, UiDeclarationStructuralSemantics, UiStructuralDeclarationPayload,
};

fn main() {
    let _semantics = UiDeclarationStructuralSemantics::new(
        UiDeclarationFamilyKind::Control,
        UiDeclarationStructuralRole::Control,
        UiDeclarationContainmentIntent::DeclaredControlAttachment {
            control_name: "save".into(),
        },
        UiDeclarationSlotParticipationIntent::None,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
    );
    let fake_family =
        unsafe { std::mem::MaybeUninit::<UiDeclarationFamily>::zeroed().assume_init() };
    let _payload = UiStructuralDeclarationPayload::new(
        fake_family,
        UiDeclarationStructuralRole::Control,
        UiDeclarationContainmentIntent::DeclaredControlAttachment {
            control_name: "save".into(),
        },
        UiDeclarationSlotParticipationIntent::None,
        UiDeclarationOrderingGuarantee::NotSemanticallyClaimed,
        UiDeclarationRepetitionPosture::NotAdmitted,
    );
    let fake_identity =
        unsafe { std::mem::MaybeUninit::<UiDeclarationIdentity>::zeroed().assume_init() };
    let fake_payload =
        unsafe { std::mem::MaybeUninit::<UiStructuralDeclarationPayload>::zeroed().assume_init() };
    let _handoff = UiDeclarationGraphHandoff::new(fake_identity, fake_payload, unsafe {
        std::mem::MaybeUninit::zeroed().assume_init()
    });
}
