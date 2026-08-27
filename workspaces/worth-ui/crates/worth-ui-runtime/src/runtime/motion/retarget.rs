#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionRetargetPredecessor {
    CurrentPresentationSample,
    CommittedSemanticPredecessor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionRetargetDisposition {
    Install {
        predecessor: UiMotionRetargetPredecessor,
    },
    FinishThenApply,
    SnapToTarget,
    CancelDrop,
}

pub(super) const fn resolve(
    policy: super::UiMotionInterruptionPolicy,
) -> UiMotionRetargetDisposition {
    match policy {
        super::UiMotionInterruptionPolicy::RetargetFromCurrentSample => {
            UiMotionRetargetDisposition::Install {
                predecessor: UiMotionRetargetPredecessor::CurrentPresentationSample,
            }
        }
        super::UiMotionInterruptionPolicy::RestartFromSemanticPredecessor => {
            UiMotionRetargetDisposition::Install {
                predecessor: UiMotionRetargetPredecessor::CommittedSemanticPredecessor,
            }
        }
        super::UiMotionInterruptionPolicy::FinishThenApply => {
            UiMotionRetargetDisposition::FinishThenApply
        }
        super::UiMotionInterruptionPolicy::SnapToTarget => {
            UiMotionRetargetDisposition::SnapToTarget
        }
        super::UiMotionInterruptionPolicy::CancelDrop => UiMotionRetargetDisposition::CancelDrop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_interruption_policy_has_one_explicit_disposition() {
        assert_eq!(
            resolve(super::super::UiMotionInterruptionPolicy::RetargetFromCurrentSample),
            UiMotionRetargetDisposition::Install {
                predecessor: UiMotionRetargetPredecessor::CurrentPresentationSample
            }
        );
        assert_eq!(
            resolve(super::super::UiMotionInterruptionPolicy::RestartFromSemanticPredecessor),
            UiMotionRetargetDisposition::Install {
                predecessor: UiMotionRetargetPredecessor::CommittedSemanticPredecessor
            }
        );
        assert_eq!(
            resolve(super::super::UiMotionInterruptionPolicy::FinishThenApply),
            UiMotionRetargetDisposition::FinishThenApply
        );
        assert_eq!(
            resolve(super::super::UiMotionInterruptionPolicy::SnapToTarget),
            UiMotionRetargetDisposition::SnapToTarget
        );
        assert_eq!(
            resolve(super::super::UiMotionInterruptionPolicy::CancelDrop),
            UiMotionRetargetDisposition::CancelDrop
        );
    }
}
