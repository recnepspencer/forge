#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredMotionPolicyContract {
    ReducedMotionAware,
}

impl UiDeclaredMotionPolicyContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Motion
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum UiReducedMotionBehavior {
    SnapDecorativeToFinalState,
    PreserveSemanticTransition,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct UiMotionPolicy {
    decorative: UiReducedMotionBehavior,
    semantic: UiReducedMotionBehavior,
}

impl UiMotionPolicy {
    pub const fn system_respecting() -> Self {
        Self {
            decorative: UiReducedMotionBehavior::SnapDecorativeToFinalState,
            semantic: UiReducedMotionBehavior::PreserveSemanticTransition,
        }
    }

    pub const fn with_decorative_reduced_motion(
        mut self,
        behavior: UiReducedMotionBehavior,
    ) -> Self {
        self.decorative = behavior;
        self
    }

    pub const fn decorative_reduced_motion(self) -> UiReducedMotionBehavior {
        self.decorative
    }

    pub const fn semantic_reduced_motion(self) -> UiReducedMotionBehavior {
        self.semantic
    }

    pub(crate) const fn digest_basis(self) -> u64 {
        self.decorative as u64 | (self.semantic as u64) << 8
    }
}
