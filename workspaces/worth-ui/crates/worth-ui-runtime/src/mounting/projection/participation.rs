use crate::graph::{
    UiGraphAxisParticipation, UiGraphParticipationAxis, UiGraphParticipationPosture,
    UiGraphParticipationStatus,
};
use worth_ui_host_contract::{
    UiMountedParticipation, UiMountedParticipationFact, UiMountedParticipationInput,
    UiMountedParticipationStatus,
};

pub(super) fn lower_participation(
    posture: UiGraphParticipationPosture,
    admits_static_paint: bool,
) -> UiMountedParticipation {
    UiMountedParticipation::new(UiMountedParticipationInput {
        paint: projected_fact(
            posture.axis(UiGraphParticipationAxis::Paint),
            admits_static_paint,
        ),
        clip: projected_fact(
            posture.axis(UiGraphParticipationAxis::Visible),
            admits_static_paint,
        ),
        input: fact(posture.axis(UiGraphParticipationAxis::Input)),
        focus: fact(posture.axis(UiGraphParticipationAxis::Focus)),
        hit_test: fact(posture.axis(UiGraphParticipationAxis::HitTest)),
        accessibility: fact(posture.axis(UiGraphParticipationAxis::Accessibility)),
        motion: UiMountedParticipationFact::new(UiMountedParticipationStatus::Deferred),
        diagnostic: fact(posture.axis(UiGraphParticipationAxis::Diagnostic)),
    })
}

fn projected_fact(
    axis: UiGraphAxisParticipation,
    admitted_by_complete_static_paint: bool,
) -> UiMountedParticipationFact {
    if admitted_by_complete_static_paint {
        UiMountedParticipationFact::new(UiMountedParticipationStatus::Admitted)
    } else {
        fact(axis)
    }
}

fn fact(axis: UiGraphAxisParticipation) -> UiMountedParticipationFact {
    let status = match axis.status() {
        UiGraphParticipationStatus::Admitted => UiMountedParticipationStatus::Admitted,
        UiGraphParticipationStatus::Deferred => UiMountedParticipationStatus::Deferred,
        UiGraphParticipationStatus::Withheld => UiMountedParticipationStatus::Withheld,
    };
    UiMountedParticipationFact::new(status)
}
