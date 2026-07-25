use crate::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationPosture,
    UiGraphParticipationStatus,
};
use worth_ui_host_contract::{
    UiMountedParticipation, UiMountedParticipationFact, UiMountedParticipationInput,
    UiMountedParticipationStatus,
};

pub(super) fn lower_participation(posture: UiGraphParticipationPosture) -> UiMountedParticipation {
    UiMountedParticipation::new(UiMountedParticipationInput {
        paint: fact(posture.axis(UiGraphParticipationAxis::Paint)),
        clip: fact(posture.axis(UiGraphParticipationAxis::Visible)),
        input: fact(posture.axis(UiGraphParticipationAxis::Input)),
        focus: fact(posture.axis(UiGraphParticipationAxis::Focus)),
        hit_test: fact(posture.axis(UiGraphParticipationAxis::HitTest)),
        accessibility: fact(posture.axis(UiGraphParticipationAxis::Accessibility)),
        motion: UiMountedParticipationFact::new(UiMountedParticipationStatus::Deferred),
        diagnostic: fact(posture.axis(UiGraphParticipationAxis::Diagnostic)),
    })
}

fn fact(axis: UiGraphAxisParticipation) -> UiMountedParticipationFact {
    let status = match axis.status() {
        UiGraphParticipationStatus::Admitted => UiMountedParticipationStatus::Admitted,
        UiGraphParticipationStatus::Deferred => UiMountedParticipationStatus::Deferred,
        UiGraphParticipationStatus::Withheld => UiMountedParticipationStatus::Withheld,
    };
    UiMountedParticipationFact::new(status)
}
