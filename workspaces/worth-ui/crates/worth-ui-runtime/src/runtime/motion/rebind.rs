pub(crate) struct UiPreparedMotionRebind {
    rebound_away: Box<[super::UiMotionTargetIdentity]>,
}

impl super::UiMotionRuntimeState {
    pub(crate) fn prepare_mounted_rebind(
        &self,
        successor: &crate::mounting::UiMountedGraphReplacementSuccessor,
    ) -> UiPreparedMotionRebind {
        self.prepare_rebind_with(|target| {
            successor.contains_mounted_instance(target.mounted_instance())
        })
    }

    fn prepare_rebind_with(
        &self,
        survives: impl Fn(super::UiMotionTargetIdentity) -> bool,
    ) -> UiPreparedMotionRebind {
        UiPreparedMotionRebind {
            rebound_away: self
                .tracks
                .keys()
                .copied()
                .filter(|target| !survives(*target))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub(crate) fn commit_mounted_rebind(
        &mut self,
        prepared: UiPreparedMotionRebind,
    ) -> Box<[super::UiMotionTerminalReceipt]> {
        prepared
            .rebound_away
            .iter()
            .filter_map(|target| {
                self.terminalize_target(*target, super::UiMotionTerminalCause::ReboundAway)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}

#[cfg(test)]
impl super::UiMotionRuntimeState {
    pub(super) fn prepare_rebind_for_test(
        &self,
        survives: impl Fn(super::UiMotionTargetIdentity) -> bool,
    ) -> UiPreparedMotionRebind {
        self.prepare_rebind_with(survives)
    }
}
