use super::UiPresentedInteractionTarget;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiPointerGestureContinuityKind {
    ExactPresentation,
    OwnerWitnessedSuccessor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPointerGestureContinuityDenial {
    PresentationDidNotAdvance,
    SurfaceChanged,
    BindingChanged,
    MountedIncarnationChanged,
    TargetChangedWithinPresentation,
}

pub(crate) struct UiPresentedTargetContinuityWitness {
    kind: UiPointerGestureContinuityKind,
    digest: u64,
}

pub(crate) fn issue_continuity(
    pressed: &UiPresentedInteractionTarget,
    released: &UiPresentedInteractionTarget,
) -> Result<UiPresentedTargetContinuityWitness, UiPointerGestureContinuityDenial> {
    if pressed.presentation() == released.presentation() {
        require_same_exact_target(pressed, released)?;
        return Ok(UiPresentedTargetContinuityWitness::new(
            UiPointerGestureContinuityKind::ExactPresentation,
            pressed,
            released,
        ));
    }
    require_successor_presentation(pressed, released)?;
    require_same_mounted_incarnation(pressed, released)?;
    Ok(UiPresentedTargetContinuityWitness::new(
        UiPointerGestureContinuityKind::OwnerWitnessedSuccessor,
        pressed,
        released,
    ))
}

fn require_same_exact_target(
    pressed: &UiPresentedInteractionTarget,
    released: &UiPresentedInteractionTarget,
) -> Result<(), UiPointerGestureContinuityDenial> {
    require_same_mounted_incarnation(pressed, released)?;
    (pressed.node_receipt() == released.node_receipt()
        && pressed.semantic_digest() == released.semantic_digest())
    .then_some(())
    .ok_or(UiPointerGestureContinuityDenial::TargetChangedWithinPresentation)
}

fn require_same_mounted_incarnation(
    pressed: &UiPresentedInteractionTarget,
    released: &UiPresentedInteractionTarget,
) -> Result<(), UiPointerGestureContinuityDenial> {
    if pressed.surface() != released.surface() {
        return Err(UiPointerGestureContinuityDenial::SurfaceChanged);
    }
    if pressed.binding() != released.binding() {
        return Err(UiPointerGestureContinuityDenial::BindingChanged);
    }
    if pressed.mounted_instance() != released.mounted_instance() {
        return Err(UiPointerGestureContinuityDenial::MountedIncarnationChanged);
    }
    Ok(())
}

fn require_successor_presentation(
    pressed: &UiPresentedInteractionTarget,
    released: &UiPresentedInteractionTarget,
) -> Result<(), UiPointerGestureContinuityDenial> {
    let pressed_basis = pressed.presentation();
    let released_basis = released.presentation();
    let advanced = released_basis.frame() > pressed_basis.frame()
        || (released_basis.frame() == pressed_basis.frame()
            && released_basis.epoch() > pressed_basis.epoch());
    advanced
        .then_some(())
        .ok_or(UiPointerGestureContinuityDenial::PresentationDidNotAdvance)
}

impl UiPresentedTargetContinuityWitness {
    fn new(
        kind: UiPointerGestureContinuityKind,
        pressed: &UiPresentedInteractionTarget,
        released: &UiPresentedInteractionTarget,
    ) -> Self {
        let digest = [
            pressed.presentation().frame().diagnostic_value(),
            pressed.presentation().epoch().diagnostic_value(),
            released.presentation().frame().diagnostic_value(),
            released.presentation().epoch().diagnostic_value(),
            pressed.mounted_instance().diagnostic_value(),
            released.mounted_instance().diagnostic_value(),
        ]
        .into_iter()
        .fold(0x6765_7374_7572_655f, |digest, value| {
            (digest ^ value).wrapping_mul(0x100000001b3)
        });
        Self { kind, digest }
    }

    pub(crate) const fn kind(&self) -> UiPointerGestureContinuityKind {
        self.kind
    }

    pub(crate) const fn digest(&self) -> u64 {
        self.digest
    }
}
