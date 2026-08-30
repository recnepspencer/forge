#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionRetargetPredecessor {
    CurrentPresentationSample,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMotionRetargetDisposition {
    Install {
        predecessor: UiMotionRetargetPredecessor,
    },
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_sample_retargeting_has_one_explicit_disposition() {
        assert_eq!(
            resolve(super::super::UiMotionInterruptionPolicy::RetargetFromCurrentSample),
            UiMotionRetargetDisposition::Install {
                predecessor: UiMotionRetargetPredecessor::CurrentPresentationSample
            }
        );
    }
}
