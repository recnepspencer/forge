use super::*;

impl PoolInner {
    pub(crate) fn fail_bounded_loading(
        &self,
        key: PhysicalBoundedFrameKey,
        identity: PhysicalFrameLoadingIdentity,
        kind: PhysicalFrameLoadTerminalKind,
    ) -> PhysicalFrameLoadTerminal {
        let mut state = self.lock();
        let terminal = Self::fail_bounded_loading_state(&mut state, key, identity, kind);
        self.changed.notify_all();
        terminal
    }

    pub(crate) fn abandon_bounded_loading(
        &self,
        key: PhysicalBoundedFrameKey,
        identity: PhysicalFrameLoadingIdentity,
    ) {
        self.fail_bounded_loading(
            key,
            identity,
            PhysicalFrameLoadTerminalKind::FaultOwnerAbandoned,
        );
    }

    pub(super) fn fail_bounded_loading_state(
        state: &mut PoolState,
        key: PhysicalBoundedFrameKey,
        identity: PhysicalFrameLoadingIdentity,
        kind: PhysicalFrameLoadTerminalKind,
    ) -> PhysicalFrameLoadTerminal {
        let terminal = PhysicalFrameLoadTerminal::new(identity, kind);
        let Some(entry) = state.frames.get_bounded(&key) else {
            return terminal;
        };
        let BoundedFrameEntry::Loading {
            identity: found,
            waiters,
            ..
        } = entry
        else {
            return terminal;
        };
        if *found != identity {
            return terminal;
        }
        let waiters = *waiters;
        state.loading_frames -= 1;
        state
            .accounting
            .fail_loading(u64::from(key.limit()), 1 + waiters, waiters > 0);
        if waiters == 0 {
            state.frames.remove_bounded(&key);
        } else {
            *state
                .frames
                .get_bounded_mut(&key)
                .expect("retained bounded failure remains indexed") =
                BoundedFrameEntry::LoadFailed { terminal, waiters };
        }
        terminal
    }
}
