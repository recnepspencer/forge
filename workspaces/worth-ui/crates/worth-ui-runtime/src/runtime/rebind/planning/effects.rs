use super::{UiRebindPlanTarget, UiRebindSubsystemKind};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiRebindDeclarativeEffect {
    subsystem: UiRebindSubsystemKind,
    target: UiRebindPlanTarget,
}

#[derive(Debug)]
pub struct UiRebindEffectSet {
    effects: Box<[UiRebindDeclarativeEffect]>,
}

impl UiRebindDeclarativeEffect {
    pub(crate) const fn new(subsystem: UiRebindSubsystemKind, target: UiRebindPlanTarget) -> Self {
        Self { subsystem, target }
    }

    pub const fn subsystem(&self) -> UiRebindSubsystemKind {
        self.subsystem
    }

    pub const fn target(&self) -> &UiRebindPlanTarget {
        &self.target
    }
}

impl UiRebindEffectSet {
    pub(crate) fn new(mut effects: Vec<UiRebindDeclarativeEffect>) -> Self {
        effects.sort();
        effects.dedup();
        Self {
            effects: effects.into_boxed_slice(),
        }
    }

    pub fn effects(&self) -> &[UiRebindDeclarativeEffect] {
        &self.effects
    }
}
